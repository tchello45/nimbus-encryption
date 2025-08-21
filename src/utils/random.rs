use crate::error::{NimbusError, NimbusResult};
use rand::TryRngCore;
use rand::rngs::OsRng;

/// Standard nonce sizes in bytes for common cryptographic operations
pub const NONCE_96_BIT_SIZE: usize = 12; // 96 bits = 12 bytes
pub const NONCE_192_BIT_SIZE: usize = 24; // 192 bits = 24 bytes

/// A trait for secure random number generators that can fail gracefully.
/// This abstraction allows for easy testing and potential future RNG implementations.
pub trait SecureRandomSource {
    type Error;

    /// Fills the given buffer with cryptographically secure random bytes.
    ///
    /// # Errors
    ///
    /// Returns an error if the underlying random number generator fails to produce
    /// secure random data or if the system's entropy source is unavailable.
    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), Self::Error>;

    /// Generates a cryptographically secure random u64 value.
    ///
    /// # Errors
    ///
    /// Returns an error if the underlying random number generator fails to produce
    /// secure random data or if the system's entropy source is unavailable.
    fn try_next_u64(&mut self) -> Result<u64, Self::Error>;
}

impl SecureRandomSource for OsRng {
    type Error = <OsRng as TryRngCore>::Error;

    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), Self::Error> {
        TryRngCore::try_fill_bytes(self, dest)
    }

    fn try_next_u64(&mut self) -> Result<u64, Self::Error> {
        TryRngCore::try_next_u64(self)
    }
}

fn secure_random_bytes<R: SecureRandomSource>(
    rng: &mut R,
    byte_count: usize,
) -> NimbusResult<Vec<u8>> {
    let mut buffer = vec![0u8; byte_count];
    rng.try_fill_bytes(&mut buffer)
        .map_err(|_| NimbusError::RandomGenerationFailed)?;
    Ok(buffer)
}

fn secure_random_u64<R: SecureRandomSource>(rng: &mut R) -> NimbusResult<u64> {
    rng.try_next_u64()
        .map_err(|_| NimbusError::RandomGenerationFailed)
}

fn generate_nonce(byte_count: usize) -> NimbusResult<Vec<u8>> {
    let mut rng = OsRng;
    secure_random_bytes(&mut rng, byte_count)
}

/// Generate a 96-bit nonce suitable for AES-GCM and similar AEAD ciphers.
///
/// # Errors
///
/// Returns [`NimbusError::RandomGenerationFailed`] if the system's secure random
/// number generator fails to produce cryptographically secure random data.
pub fn generate_aes_gcm_nonce() -> NimbusResult<Vec<u8>> {
    generate_nonce(NONCE_96_BIT_SIZE)
}

/// Generate a 192-bit nonce suitable for XChaCha20-Poly1305.
///
/// # Errors
///
/// Returns [`NimbusError::RandomGenerationFailed`] if the system's secure random
/// number generator fails to produce cryptographically secure random data.
pub fn generate_extended_nonce() -> NimbusResult<Vec<u8>> {
    generate_nonce(NONCE_192_BIT_SIZE)
}

/// Generate a cryptographically secure random u64 value.
///
/// # Errors
///
/// Returns [`NimbusError::RandomGenerationFailed`] if the system's secure random
/// number generator fails to produce cryptographically secure random data.
pub fn generate_random_u64() -> NimbusResult<u64> {
    let mut rng = OsRng;
    secure_random_u64(&mut rng)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Error as IoError;

    struct MockRandomSource {
        bytes_should_fail: bool,
        u64_should_fail: bool,
        test_value: u64,
    }

    impl MockRandomSource {
        fn new_success() -> Self {
            Self {
                bytes_should_fail: false,
                u64_should_fail: false,
                test_value: 42,
            }
        }

        fn new_bytes_failure() -> Self {
            Self {
                bytes_should_fail: true,
                u64_should_fail: false,
                test_value: 42,
            }
        }

        fn new_u64_failure() -> Self {
            Self {
                bytes_should_fail: false,
                u64_should_fail: true,
                test_value: 42,
            }
        }
    }

    impl SecureRandomSource for MockRandomSource {
        type Error = IoError;

        fn try_fill_bytes(&mut self, _dest: &mut [u8]) -> Result<(), Self::Error> {
            if self.bytes_should_fail {
                Err(IoError::other("Mock random bytes generation failed"))
            } else {
                Ok(())
            }
        }

        fn try_next_u64(&mut self) -> Result<u64, Self::Error> {
            if self.u64_should_fail {
                Err(IoError::other("Mock u64 generation failed"))
            } else {
                Ok(self.test_value)
            }
        }
    }

    #[test]
    fn secure_random_bytes_generates_correct_length() {
        let mut mock = MockRandomSource::new_success();
        let result = secure_random_bytes(&mut mock, 12);
        assert_eq!(result.unwrap().len(), 12);
    }

    #[test]
    fn secure_random_bytes_handles_rng_failure() {
        let mut mock = MockRandomSource::new_bytes_failure();
        let result = secure_random_bytes(&mut mock, 12);
        assert_eq!(result.unwrap_err(), NimbusError::RandomGenerationFailed);
    }

    #[test]
    fn secure_random_u64_returns_expected_value() {
        let mut mock = MockRandomSource::new_success();
        let result = secure_random_u64(&mut mock);
        assert_eq!(result.unwrap(), 42);
    }

    #[test]
    fn secure_random_u64_handles_rng_failure() {
        let mut mock = MockRandomSource::new_u64_failure();
        let result = secure_random_u64(&mut mock);
        assert_eq!(result.unwrap_err(), NimbusError::RandomGenerationFailed);
    }

    #[test]
    fn aes_gcm_nonce_has_correct_size() {
        let result = generate_aes_gcm_nonce();
        assert_eq!(result.unwrap().len(), NONCE_96_BIT_SIZE);
    }

    #[test]
    fn extended_nonce_has_correct_size() {
        let result = generate_extended_nonce();
        assert_eq!(result.unwrap().len(), NONCE_192_BIT_SIZE);
    }

    #[test]
    fn random_u64_generation_succeeds() {
        let result = generate_random_u64();
        assert!(result.is_ok());
    }

    #[test]
    fn nonce_constants_have_correct_values() {
        assert_eq!(NONCE_96_BIT_SIZE, 12);
        assert_eq!(NONCE_192_BIT_SIZE, 24);
    }

    #[test]
    fn generated_nonces_are_different() {
        let nonce1 = generate_aes_gcm_nonce().unwrap();
        let nonce2 = generate_aes_gcm_nonce().unwrap();
        assert_ne!(nonce1, nonce2);
    }
}
