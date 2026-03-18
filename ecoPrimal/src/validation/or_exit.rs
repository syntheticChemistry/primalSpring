// SPDX-License-Identifier: AGPL-3.0-or-later

//! Zero-panic exit trait for validation binaries.
//!
//! Absorbed from groundSpring/wetSpring/healthSpring. Replaces verbose
//! `let Ok(v) = expr else { eprintln!(...); process::exit(1); }` boilerplate
//! in experiment binaries with a clean `.or_exit(msg)` call.

use std::fmt;

/// Exit code for general errors in validation binaries.
const GENERAL_ERROR: i32 = 1;

/// Unwrap a fallible value or print an error and exit with code 1.
///
/// Implemented for both `Result<T, E>` and `Option<T>`.
pub trait OrExit<T> {
    /// Unwrap the value or print `msg` to stderr and exit with code 1.
    fn or_exit(self, msg: &str) -> T;
}

impl<T, E: fmt::Display> OrExit<T> for Result<T, E> {
    fn or_exit(self, msg: &str) -> T {
        match self {
            Ok(v) => v,
            Err(e) => {
                eprintln!("FATAL: {msg}: {e}");
                std::process::exit(GENERAL_ERROR);
            }
        }
    }
}

impl<T> OrExit<T> for Option<T> {
    fn or_exit(self, msg: &str) -> T {
        self.unwrap_or_else(|| {
            eprintln!("FATAL: {msg}");
            std::process::exit(GENERAL_ERROR);
        })
    }
}
