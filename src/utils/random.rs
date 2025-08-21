use crate::error::{NimbusError, NimbusResult};
use rand::TryRngCore;
use rand::rngs::OsRng;

pub trait RandomNumberGenerator {
    fn generate_nonce_92bit(&mut self) -> NimbusResult<Vec<u8>>;
    fn generate_nonce_192bit(&mut self) -> NimbusResult<Vec<u8>>;
    fn generate_random_u64(&mut self) -> NimbusResult<u64>;
}

impl RandomNumberGenerator for OsRng {
    fn generate_nonce_92bit(&mut self) -> NimbusResult<Vec<u8>> {
        let mut nonce = vec![0; 12];
        self.try_fill_bytes(&mut nonce)
            .map_err(|_| NimbusError::RandomGenerationFailed)?;
        Ok(nonce)
    }

    fn generate_nonce_192bit(&mut self) -> NimbusResult<Vec<u8>> {
        let mut nonce = vec![0; 24];
        self.try_fill_bytes(&mut nonce)
            .map_err(|_| NimbusError::RandomGenerationFailed)?;
        Ok(nonce)
    }

    fn generate_random_u64(&mut self) -> NimbusResult<u64> {
        let random_u64 = self
            .try_next_u64()
            .map_err(|_| NimbusError::RandomGenerationFailed)?;
        Ok(random_u64)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_nonce_92bit() {
        let mut rng = OsRng;
        let nonce = rng.generate_nonce_92bit().unwrap();
        assert_eq!(nonce.len(), 12);
    }

    #[test]
    fn test_generate_nonce_192bit() {
        let mut rng = OsRng;
        let nonce = rng.generate_nonce_192bit().unwrap();
        assert_eq!(nonce.len(), 24);
    }

    #[test]
    fn test_generate_random_u64() {
        let mut rng = OsRng;
        let random_u64 = rng.generate_random_u64().unwrap();
        assert!(random_u64 < u64::MAX);
    }
}
