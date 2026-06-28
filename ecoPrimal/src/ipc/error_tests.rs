// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

use super::*;

#[test]
fn socket_not_found_is_connection_error() {
    let err = IpcError::SocketNotFound {
        primal: "beardog".to_owned(),
    };
    assert!(err.is_connection_error());
    assert!(!err.is_retriable());
    assert!(!err.is_timeout_likely());
    assert!(!err.is_method_not_found());
}

#[test]
fn connection_refused_is_connection_error_not_retriable() {
    let err = IpcError::ConnectionRefused(std::io::Error::new(
        std::io::ErrorKind::ConnectionRefused,
        "refused",
    ));
    assert!(err.is_connection_error());
    assert!(!err.is_retriable());
}

#[test]
fn connection_reset_is_retriable_and_connection_error() {
    let err = IpcError::ConnectionReset(std::io::Error::new(
        std::io::ErrorKind::ConnectionReset,
        "reset",
    ));
    assert!(err.is_retriable());
    assert!(err.is_connection_error());
}

#[test]
fn timeout_is_retriable_and_timeout_likely() {
    let err = IpcError::Timeout(std::io::Error::new(
        std::io::ErrorKind::TimedOut,
        "timed out",
    ));
    assert!(err.is_retriable());
    assert!(err.is_timeout_likely());
    assert!(!err.is_connection_error());
}

#[test]
fn method_not_found_queries() {
    let err = IpcError::MethodNotFound {
        method: "foo.bar".to_owned(),
    };
    assert!(err.is_method_not_found());
    assert!(!err.is_retriable());
    assert!(!err.is_connection_error());
}

#[test]
fn application_error_with_method_not_found_code() {
    let err = IpcError::ApplicationError {
        code: error_codes::METHOD_NOT_FOUND,
        message: "compute.submit".to_owned(),
        data: None,
    };
    assert!(err.is_method_not_found());
}

#[test]
fn protocol_error_with_serialized_method_not_found() {
    let err = IpcError::ProtocolError {
        detail: r#"{"code":-32601,"message":"no such method"}"#.to_owned(),
    };
    assert!(err.is_method_not_found());
}

#[test]
fn socket_not_found_is_not_method_not_found() {
    let err = IpcError::SocketNotFound {
        primal: "beardog".to_owned(),
    };
    assert!(!err.is_method_not_found());
}

#[test]
fn application_error_is_not_retriable() {
    let err = IpcError::ApplicationError {
        code: -32_603,
        message: "internal".to_owned(),
        data: None,
    };
    assert!(!err.is_retriable());
    assert!(!err.is_method_not_found());
}

#[test]
fn from_jsonrpc_method_not_found() {
    let rpc_err = JsonRpcError {
        code: error_codes::METHOD_NOT_FOUND,
        message: "compute.submit".to_owned(),
        data: None,
    };
    let err = IpcError::from(rpc_err);
    assert!(err.is_method_not_found());
}

#[test]
fn from_jsonrpc_application_error() {
    let rpc_err = JsonRpcError {
        code: error_codes::INTERNAL_ERROR,
        message: "boom".to_owned(),
        data: None,
    };
    let err = IpcError::from(rpc_err);
    assert!(!err.is_method_not_found());
    assert!(matches!(err, IpcError::ApplicationError { .. }));
}

#[test]
fn classify_io_connection_refused() {
    let io_err = std::io::Error::new(std::io::ErrorKind::ConnectionRefused, "refused");
    let err = classify_io_error(io_err);
    assert!(matches!(err, IpcError::ConnectionRefused(_)));
}

#[test]
fn classify_io_timed_out() {
    let io_err = std::io::Error::new(std::io::ErrorKind::TimedOut, "slow");
    let err = classify_io_error(io_err);
    assert!(matches!(err, IpcError::Timeout(_)));
}

#[test]
fn classify_io_broken_pipe() {
    let io_err = std::io::Error::new(std::io::ErrorKind::BrokenPipe, "pipe");
    let err = classify_io_error(io_err);
    assert!(matches!(err, IpcError::ConnectionReset(_)));
}

#[test]
fn display_socket_not_found() {
    let err = IpcError::SocketNotFound {
        primal: "beardog".to_owned(),
    };
    assert!(err.to_string().contains("beardog"));
}

#[test]
fn display_method_not_found() {
    let err = IpcError::MethodNotFound {
        method: "foo.bar".to_owned(),
    };
    let s = err.to_string();
    assert!(s.contains("method not found"));
    assert!(s.contains("foo.bar"));
}

#[test]
fn is_retriable_true_for_reset() {
    let err = IpcError::ConnectionReset(std::io::Error::new(
        std::io::ErrorKind::ConnectionReset,
        "reset",
    ));
    assert!(err.is_retriable());
    assert!(!err.is_timeout_likely());
    assert!(err.is_connection_error());
}

#[test]
fn is_retriable_true_for_timeout() {
    let err = IpcError::Timeout(std::io::Error::new(std::io::ErrorKind::TimedOut, "slow"));
    assert!(err.is_retriable());
    assert!(err.is_timeout_likely());
    assert!(!err.is_connection_error());
}

#[test]
fn is_retriable_false_for_socket_not_found() {
    let err = IpcError::SocketNotFound {
        primal: "beardog".to_owned(),
    };
    assert!(!err.is_retriable());
    assert!(err.is_connection_error());
}

#[test]
fn is_retriable_false_for_application_error() {
    let err = IpcError::ApplicationError {
        code: -32603,
        message: "internal".to_owned(),
        data: None,
    };
    assert!(!err.is_retriable());
    assert!(!err.is_connection_error());
    assert!(!err.is_method_not_found());
}

#[test]
fn error_source_returns_io_error_for_connection_types() {
    use std::error::Error;
    let err = IpcError::ConnectionRefused(std::io::Error::new(
        std::io::ErrorKind::ConnectionRefused,
        "refused",
    ));
    assert!(err.source().is_some());

    let err2 = IpcError::ProtocolError {
        detail: "bad".to_owned(),
    };
    assert!(err2.source().is_none());
}

#[test]
fn from_jsonrpc_error_method_not_found() {
    let rpc_err = JsonRpcError {
        code: error_codes::METHOD_NOT_FOUND,
        message: "health.check".to_owned(),
        data: None,
    };
    let ipc_err: IpcError = rpc_err.into();
    assert!(ipc_err.is_method_not_found());
}

#[test]
fn from_jsonrpc_error_application_error() {
    let rpc_err = JsonRpcError {
        code: error_codes::INTERNAL_ERROR,
        message: "boom".to_owned(),
        data: Some(serde_json::json!({"detail": "stack trace"})),
    };
    let ipc_err: IpcError = rpc_err.into();
    assert!(!ipc_err.is_method_not_found());
    assert!(matches!(ipc_err, IpcError::ApplicationError { .. }));
}

#[test]
fn display_all_variants() {
    let variants: Vec<IpcError> = vec![
        IpcError::SocketNotFound {
            primal: "x".to_owned(),
        },
        IpcError::ConnectionRefused(std::io::Error::new(
            std::io::ErrorKind::ConnectionRefused,
            "r",
        )),
        IpcError::ConnectionReset(std::io::Error::new(
            std::io::ErrorKind::ConnectionReset,
            "r",
        )),
        IpcError::Timeout(std::io::Error::new(std::io::ErrorKind::TimedOut, "t")),
        IpcError::ProtocolError {
            detail: "bad".to_owned(),
        },
        IpcError::MethodNotFound {
            method: "m".to_owned(),
        },
        IpcError::ApplicationError {
            code: -1,
            message: "e".to_owned(),
            data: None,
        },
        IpcError::SerializationError {
            detail: "s".to_owned(),
        },
        IpcError::PermissionDenied {
            method: "compute.submit".to_owned(),
            reason: "no token".to_owned(),
        },
    ];
    for v in &variants {
        assert!(!v.to_string().is_empty());
    }
}

#[test]
fn ipc_error_phase_display() {
    assert_eq!(IpcErrorPhase::Connect.to_string(), "connect");
    assert_eq!(IpcErrorPhase::Serialize.to_string(), "serialize");
    assert_eq!(IpcErrorPhase::Send.to_string(), "send");
    assert_eq!(IpcErrorPhase::Receive.to_string(), "receive");
    assert_eq!(IpcErrorPhase::Parse.to_string(), "parse");
}

#[test]
fn phased_error_display_includes_phase() {
    let err = IpcError::ProtocolError {
        detail: "bad json".to_owned(),
    };
    let phased = err.in_phase(IpcErrorPhase::Parse);
    let display = phased.to_string();
    assert!(display.starts_with("[parse]"));
    assert!(display.contains("bad json"));
}

#[test]
fn phased_error_preserves_source() {
    use std::error::Error;
    let err = IpcError::Timeout(std::io::Error::new(std::io::ErrorKind::TimedOut, "slow"));
    let phased = err.in_phase(IpcErrorPhase::Receive);
    assert!(phased.source().is_some());
    assert_eq!(phased.phase, IpcErrorPhase::Receive);
}

#[test]
fn phased_error_source_always_returns_inner() {
    use std::error::Error;
    let err = IpcError::ProtocolError {
        detail: "x".to_owned(),
    };
    let phased = err.in_phase(IpcErrorPhase::Connect);
    assert!(
        phased.source().is_some(),
        "PhasedIpcError wraps IpcError as source"
    );
}

// ── is_recoverable tests ──

#[test]
fn is_recoverable_true_for_connection_refused() {
    let err = IpcError::ConnectionRefused(std::io::Error::new(
        std::io::ErrorKind::ConnectionRefused,
        "refused",
    ));
    assert!(err.is_recoverable());
}

#[test]
fn is_recoverable_true_for_connection_reset() {
    let err = IpcError::ConnectionReset(std::io::Error::new(
        std::io::ErrorKind::ConnectionReset,
        "reset",
    ));
    assert!(err.is_recoverable());
    assert!(err.is_retriable());
}

#[test]
fn is_recoverable_true_for_timeout() {
    let err = IpcError::Timeout(std::io::Error::new(std::io::ErrorKind::TimedOut, "slow"));
    assert!(err.is_recoverable());
    assert!(err.is_retriable());
}

#[test]
fn is_recoverable_true_for_application_error() {
    let err = IpcError::ApplicationError {
        code: -32_603,
        message: "internal".to_owned(),
        data: None,
    };
    assert!(err.is_recoverable());
    assert!(!err.is_retriable());
}

#[test]
fn is_recoverable_false_for_method_not_found() {
    let err = IpcError::MethodNotFound {
        method: "foo.bar".to_owned(),
    };
    assert!(!err.is_recoverable());
}

#[test]
fn is_recoverable_false_for_serialization_error() {
    let err = IpcError::SerializationError {
        detail: "bad json".to_owned(),
    };
    assert!(!err.is_recoverable());
}

#[test]
fn is_recoverable_false_for_socket_not_found() {
    let err = IpcError::SocketNotFound {
        primal: "beardog".to_owned(),
    };
    assert!(!err.is_recoverable());
}

#[test]
fn is_recoverable_false_for_protocol_error() {
    let err = IpcError::ProtocolError {
        detail: "bad".to_owned(),
    };
    assert!(!err.is_recoverable());
}

// ── PermissionDenied tests ──

#[test]
fn permission_denied_queries() {
    let err = IpcError::PermissionDenied {
        method: "compute.submit".to_owned(),
        reason: "no token".to_owned(),
    };
    assert!(err.is_permission_denied());
    assert!(!err.is_retriable());
    assert!(!err.is_recoverable());
    assert!(!err.is_connection_error());
    assert!(!err.is_method_not_found());
}

#[test]
fn from_jsonrpc_permission_denied() {
    let rpc_err = JsonRpcError {
        code: error_codes::PERMISSION_DENIED,
        message: "capability token missing".to_owned(),
        data: Some(serde_json::json!({"method": "compute.submit"})),
    };
    let err = IpcError::from(rpc_err);
    assert!(err.is_permission_denied());
    if let IpcError::PermissionDenied { method, reason } = &err {
        assert_eq!(method, "compute.submit");
        assert_eq!(reason, "capability token missing");
    }
}

#[test]
fn from_jsonrpc_unauthorized() {
    let rpc_err = JsonRpcError {
        code: error_codes::UNAUTHORIZED,
        message: "identity unknown".to_owned(),
        data: None,
    };
    let err = IpcError::from(rpc_err);
    assert!(err.is_permission_denied());
    if let IpcError::PermissionDenied { method, .. } = &err {
        assert_eq!(method, "unknown");
    }
}

#[test]
fn display_permission_denied() {
    let err = IpcError::PermissionDenied {
        method: "storage.put".to_owned(),
        reason: "scope insufficient".to_owned(),
    };
    let s = err.to_string();
    assert!(s.contains("permission denied"));
    assert!(s.contains("storage.put"));
    assert!(s.contains("scope insufficient"));
}
