// SPDX-License-Identifier: AGPL-3.0-or-later

//! IPC resilience primitives — `CircuitBreaker`, `RetryPolicy`, `resilient_call`.
//!
//! Converged ecosystem pattern adopted by wetSpring, healthSpring, groundSpring,
//! airSpring, neuralSpring, and ludoSpring. Wraps IPC calls with automatic
//! retry (exponential backoff) and circuit-breaking (fail-fast when a primal
//! is demonstrably down).

use std::time::{Duration, Instant};

use super::error::IpcError;

/// Circuit breaker state machine.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CircuitState {
    /// Normal operation — calls pass through.
    Closed,
    /// Too many recent failures — calls are rejected immediately.
    Open,
    /// Recovery window — one probe call is allowed through.
    HalfOpen,
}

/// Fail-fast circuit breaker for IPC calls.
///
/// Tracks consecutive failures and trips open after `failure_threshold`.
/// After `recovery_timeout`, transitions to half-open and allows one probe.
/// A successful probe resets the breaker; a failed probe re-opens it.
#[derive(Debug)]
pub struct CircuitBreaker {
    failure_threshold: u32,
    recovery_timeout: Duration,
    consecutive_failures: u32,
    last_failure_at: Option<Instant>,
    state: CircuitState,
}

impl CircuitBreaker {
    /// Create a new circuit breaker.
    ///
    /// - `failure_threshold`: consecutive failures before opening the circuit.
    /// - `recovery_timeout`: how long to wait in open state before probing.
    #[must_use]
    pub const fn new(failure_threshold: u32, recovery_timeout: Duration) -> Self {
        Self {
            failure_threshold,
            recovery_timeout,
            consecutive_failures: 0,
            last_failure_at: None,
            state: CircuitState::Closed,
        }
    }

    /// Current circuit state.
    #[must_use]
    pub fn state(&self) -> CircuitState {
        if self.state == CircuitState::Open {
            if let Some(last) = self.last_failure_at {
                if last.elapsed() >= self.recovery_timeout {
                    return CircuitState::HalfOpen;
                }
            }
        }
        self.state
    }

    /// Whether a call is currently permitted.
    #[must_use]
    pub fn is_call_permitted(&self) -> bool {
        match self.state() {
            CircuitState::Closed | CircuitState::HalfOpen => true,
            CircuitState::Open => false,
        }
    }

    /// Record a successful call — resets failure count and closes the circuit.
    pub const fn record_success(&mut self) {
        self.consecutive_failures = 0;
        self.last_failure_at = None;
        self.state = CircuitState::Closed;
    }

    /// Record a failed call — increments failure count and may trip the circuit.
    pub fn record_failure(&mut self) {
        self.consecutive_failures += 1;
        self.last_failure_at = Some(Instant::now());
        if self.consecutive_failures >= self.failure_threshold {
            self.state = CircuitState::Open;
        }
    }
}

/// Exponential backoff retry policy.
#[derive(Debug, Clone)]
pub struct RetryPolicy {
    /// Maximum number of retry attempts (0 = no retries).
    pub max_retries: u32,
    /// Base delay between retries (doubles each attempt).
    pub base_delay: Duration,
    /// Maximum delay cap.
    pub max_delay: Duration,
}

impl RetryPolicy {
    /// Create a new retry policy.
    #[must_use]
    pub const fn new(max_retries: u32, base_delay: Duration, max_delay: Duration) -> Self {
        Self {
            max_retries,
            base_delay,
            max_delay,
        }
    }

    /// Compute the delay for a given attempt number (0-indexed).
    #[must_use]
    pub fn delay_for(&self, attempt: u32) -> Duration {
        let multiplier = 1u64.checked_shl(attempt).unwrap_or(u64::MAX);
        let delay = self
            .base_delay
            .saturating_mul(u32::try_from(multiplier).unwrap_or(u32::MAX));
        delay.min(self.max_delay)
    }
}

impl Default for RetryPolicy {
    /// Sensible defaults: 3 retries, 100ms base, 2s max.
    fn default() -> Self {
        Self::new(3, Duration::from_millis(100), Duration::from_secs(2))
    }
}

/// Execute a fallible IPC call with circuit-breaking and retry.
///
/// - If the circuit is open, returns the last error immediately.
/// - On retriable failures, backs off per `policy` and retries.
/// - On non-retriable failures, returns immediately.
///
/// # Errors
///
/// Returns the last `IpcError` if all attempts fail or the circuit is open.
pub fn resilient_call<F, T>(
    breaker: &mut CircuitBreaker,
    policy: &RetryPolicy,
    mut f: F,
) -> Result<T, IpcError>
where
    F: FnMut() -> Result<T, IpcError>,
{
    if !breaker.is_call_permitted() {
        return Err(IpcError::ProtocolError {
            detail: "circuit breaker is open".to_owned(),
        });
    }

    let mut last_err = None;

    for attempt in 0..=policy.max_retries {
        match f() {
            Ok(val) => {
                breaker.record_success();
                return Ok(val);
            }
            Err(e) => {
                let retriable = e.is_retriable();
                breaker.record_failure();
                last_err = Some(e);

                if !retriable || attempt == policy.max_retries {
                    break;
                }

                std::thread::sleep(policy.delay_for(attempt));
            }
        }
    }

    Err(last_err.unwrap_or_else(|| IpcError::ProtocolError {
        detail: "resilient_call exhausted with no error".to_owned(),
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_breaker_is_closed() {
        let cb = CircuitBreaker::new(3, Duration::from_secs(5));
        assert_eq!(cb.state(), CircuitState::Closed);
        assert!(cb.is_call_permitted());
    }

    #[test]
    fn breaker_opens_after_threshold() {
        let mut cb = CircuitBreaker::new(2, Duration::from_secs(60));
        cb.record_failure();
        assert_eq!(cb.state(), CircuitState::Closed);
        cb.record_failure();
        assert_eq!(cb.state(), CircuitState::Open);
        assert!(!cb.is_call_permitted());
    }

    #[test]
    fn breaker_resets_on_success() {
        let mut cb = CircuitBreaker::new(3, Duration::from_secs(60));
        cb.record_failure();
        cb.record_failure();
        cb.record_success();
        assert_eq!(cb.state(), CircuitState::Closed);
        assert!(cb.is_call_permitted());
    }

    #[test]
    fn breaker_transitions_to_half_open_after_timeout() {
        let mut cb = CircuitBreaker::new(1, Duration::from_millis(1));
        cb.record_failure();
        assert_eq!(cb.state(), CircuitState::Open);

        std::thread::sleep(Duration::from_millis(5));
        assert_eq!(cb.state(), CircuitState::HalfOpen);
        assert!(cb.is_call_permitted());
    }

    #[test]
    fn retry_policy_delay_doubles() {
        let policy = RetryPolicy::new(5, Duration::from_millis(100), Duration::from_secs(10));
        assert_eq!(policy.delay_for(0), Duration::from_millis(100));
        assert_eq!(policy.delay_for(1), Duration::from_millis(200));
        assert_eq!(policy.delay_for(2), Duration::from_millis(400));
    }

    #[test]
    fn retry_policy_caps_at_max() {
        let policy = RetryPolicy::new(5, Duration::from_millis(500), Duration::from_secs(1));
        assert_eq!(policy.delay_for(5), Duration::from_secs(1));
    }

    #[test]
    fn default_retry_policy() {
        let policy = RetryPolicy::default();
        assert_eq!(policy.max_retries, 3);
        assert_eq!(policy.base_delay, Duration::from_millis(100));
    }

    #[test]
    fn resilient_call_succeeds_immediately() {
        let mut cb = CircuitBreaker::new(3, Duration::from_secs(5));
        let policy = RetryPolicy::new(2, Duration::from_millis(1), Duration::from_millis(10));
        let result = resilient_call(&mut cb, &policy, || Ok::<_, IpcError>(42));
        assert_eq!(result.unwrap(), 42);
        assert_eq!(cb.state(), CircuitState::Closed);
    }

    #[test]
    fn resilient_call_retries_on_retriable_error() {
        let mut cb = CircuitBreaker::new(10, Duration::from_secs(60));
        let policy = RetryPolicy::new(2, Duration::from_millis(1), Duration::from_millis(10));
        let mut attempts = 0u32;
        let result = resilient_call(&mut cb, &policy, || {
            attempts += 1;
            if attempts < 3 {
                Err(IpcError::Timeout(std::io::Error::new(
                    std::io::ErrorKind::TimedOut,
                    "slow",
                )))
            } else {
                Ok(99)
            }
        });
        assert_eq!(result.unwrap(), 99);
        assert_eq!(attempts, 3);
    }

    #[test]
    fn resilient_call_stops_on_non_retriable() {
        let mut cb = CircuitBreaker::new(10, Duration::from_secs(60));
        let policy = RetryPolicy::new(5, Duration::from_millis(1), Duration::from_millis(10));
        let mut attempts = 0u32;
        let result: Result<i32, _> = resilient_call(&mut cb, &policy, || {
            attempts += 1;
            Err(IpcError::MethodNotFound {
                method: "nope".to_owned(),
            })
        });
        assert!(result.is_err());
        assert_eq!(attempts, 1);
    }

    #[test]
    fn resilient_call_fails_when_circuit_open() {
        let mut cb = CircuitBreaker::new(1, Duration::from_secs(60));
        cb.record_failure();
        let policy = RetryPolicy::default();
        let result: Result<i32, _> = resilient_call(&mut cb, &policy, || Ok(1));
        assert!(result.is_err());
    }
}
