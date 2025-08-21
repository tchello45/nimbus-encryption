//! Error types for the Nimbus E2EE toolkit.

use std::fmt;

pub type NimbusResult<T> = Result<T, NimbusError>;

#[derive(Debug, Clone, PartialEq)]
pub enum NimbusError {
    CryptographicFailure,
    AuthenticationFailed,
    KeyOperationFailed,
    InvalidInput,
    InvalidLength,
    RandomGenerationFailed,
    SystemError,
    WebAssemblyError,
}

impl fmt::Display for NimbusError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            NimbusError::CryptographicFailure => write!(f, "Cryptographic operation failed"),
            NimbusError::AuthenticationFailed => write!(f, "Authentication verification failed"),
            NimbusError::KeyOperationFailed => write!(f, "Key operation failed"),
            NimbusError::InvalidInput => write!(f, "Invalid input provided"),
            NimbusError::InvalidLength => write!(f, "Invalid data length"),
            NimbusError::RandomGenerationFailed => write!(f, "Secure random generation failed"),
            NimbusError::SystemError => write!(f, "System operation failed"),
            NimbusError::WebAssemblyError => write!(f, "WebAssembly operation failed"),
        }
    }
}

impl std::error::Error for NimbusError {}
