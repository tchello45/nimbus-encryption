use crate::error::{NimbusError, NimbusResult};
use rand::TryRngCore;
use rand::rngs::OsRng;

trait RandomNumberGenerator {
    type Error;
    fn generate_nonce_92bit(&mut self) -> NimbusResult<Vec<u8>>;
    fn generate_nonce_192bit(&mut self) -> NimbusResult<Vec<u8>>;
    fn generate_random_u64(&mut self) -> NimbusResult<u64>;
}

impl RandomNumberGenerator for OsRng {
    type Error = NimbusError;
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
