use crate::error::{NimbusError, NimbusResult};
use rand::TryRngCore;
use rand::rngs::OsRng;

pub trait RngProvider {
    type Error;
    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), Self::Error>;
    fn try_next_u64(&mut self) -> Result<u64, Self::Error>;
}

impl RngProvider for OsRng {
    type Error = <OsRng as TryRngCore>::Error;

    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), Self::Error> {
        TryRngCore::try_fill_bytes(self, dest)
    }

    fn try_next_u64(&mut self) -> Result<u64, Self::Error> {
        TryRngCore::try_next_u64(self)
    }
}

fn generate_bytes_generic<R: RngProvider>(rng: &mut R, size: usize) -> NimbusResult<Vec<u8>> {
    let mut bytes = vec![0u8; size];
    rng.try_fill_bytes(&mut bytes)
        .map_err(|_| NimbusError::RandomGenerationFailed)?;
    Ok(bytes)
}

fn generate_u64_generic<R: RngProvider>(rng: &mut R) -> NimbusResult<u64> {
    rng.try_next_u64()
        .map_err(|_| NimbusError::RandomGenerationFailed)
}

pub fn generate_nonce_92bit() -> NimbusResult<Vec<u8>> {
    let mut rng = OsRng;
    generate_bytes_generic(&mut rng, 12)
}

pub fn generate_nonce_192bit() -> NimbusResult<Vec<u8>> {
    let mut rng = OsRng;
    generate_bytes_generic(&mut rng, 24)
}

pub fn generate_u64() -> NimbusResult<u64> {
    let mut rng = OsRng;
    generate_u64_generic(&mut rng)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Error as IoError;

    struct MockRng {
        fill_should_fail: bool,
        u64_should_fail: bool,
    }

    impl RngProvider for MockRng {
        type Error = IoError;

        fn try_fill_bytes(&mut self, _dest: &mut [u8]) -> Result<(), Self::Error> {
            if self.fill_should_fail {
                Err(IoError::other("Mock fill error"))
            } else {
                Ok(())
            }
        }

        fn try_next_u64(&mut self) -> Result<u64, Self::Error> {
            if self.u64_should_fail {
                Err(IoError::other("Mock u64 error"))
            } else {
                Ok(42)
            }
        }
    }

    #[test]
    fn test_bytes_generic_success() {
        let mut mock = MockRng {
            fill_should_fail: false,
            u64_should_fail: false,
        };
        let result = generate_bytes_generic(&mut mock, 12);
        assert_eq!(result.unwrap().len(), 12);
    }

    #[test]
    fn test_bytes_generic_failure() {
        let mut mock = MockRng {
            fill_should_fail: true,
            u64_should_fail: false,
        };
        let result = generate_bytes_generic(&mut mock, 12);
        assert_eq!(result.unwrap_err(), NimbusError::RandomGenerationFailed);
    }

    #[test]
    fn test_u64_generic_success() {
        let mut mock = MockRng {
            fill_should_fail: false,
            u64_should_fail: false,
        };
        let result = generate_u64_generic(&mut mock);
        assert_eq!(result.unwrap(), 42);
    }

    #[test]
    fn test_u64_generic_failure() {
        let mut mock = MockRng {
            fill_should_fail: false,
            u64_should_fail: true,
        };
        let result = generate_u64_generic(&mut mock);
        assert_eq!(result.unwrap_err(), NimbusError::RandomGenerationFailed);
    }

    #[test]
    fn test_nonce_92bit() {
        let result = generate_nonce_92bit();
        assert_eq!(result.unwrap().len(), 12);
    }

    #[test]
    fn test_nonce_192bit() {
        let result = generate_nonce_192bit();
        assert_eq!(result.unwrap().len(), 24);
    }

    #[test]
    fn test_u64() {
        let result = generate_u64();
        assert!(result.is_ok());
    }
}
