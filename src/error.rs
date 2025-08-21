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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cryptographic_failure_display() {
        assert_eq!(
            NimbusError::CryptographicFailure.to_string(),
            "Cryptographic operation failed"
        );
    }

    #[test]
    fn test_authentication_failed_display() {
        assert_eq!(
            NimbusError::AuthenticationFailed.to_string(),
            "Authentication verification failed"
        );
    }

    #[test]
    fn test_key_operation_failed_display() {
        assert_eq!(
            NimbusError::KeyOperationFailed.to_string(),
            "Key operation failed"
        );
    }

    #[test]
    fn test_invalid_input_display() {
        assert_eq!(
            NimbusError::InvalidInput.to_string(),
            "Invalid input provided"
        );
    }

    #[test]
    fn test_invalid_length_display() {
        assert_eq!(
            NimbusError::InvalidLength.to_string(),
            "Invalid data length"
        );
    }

    #[test]
    fn test_random_generation_failed_display() {
        assert_eq!(
            NimbusError::RandomGenerationFailed.to_string(),
            "Secure random generation failed"
        );
    }

    #[test]
    fn test_system_error_display() {
        assert_eq!(
            NimbusError::SystemError.to_string(),
            "System operation failed"
        );
    }

    #[test]
    fn test_webassembly_error_display() {
        assert_eq!(
            NimbusError::WebAssemblyError.to_string(),
            "WebAssembly operation failed"
        );
    }
}
