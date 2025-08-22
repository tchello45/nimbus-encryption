use crate::error::{NimbusError, NimbusResult};
use base64ct::{Base64, Encoding};

pub const MAX_BASE64_INPUT_SIZE: usize = usize::MAX / 4;

pub trait SecureEncodingProvider {
    type Error;

    fn encode(&self, data: &[u8]) -> Result<String, Self::Error>;

    fn decode(&self, data: &str) -> Result<Vec<u8>, Self::Error>;

    fn max_input_size(&self) -> usize;
}

impl SecureEncodingProvider for Base64 {
    type Error = NimbusError;

    fn encode(&self, data: &[u8]) -> Result<String, Self::Error> {
        if data.len() > self.max_input_size() {
            return Err(NimbusError::InvalidInput);
        }
        Ok(Base64::encode_string(data))
    }

    fn decode(&self, data: &str) -> Result<Vec<u8>, Self::Error> {
        Base64::decode_vec(data).map_err(|_| NimbusError::InvalidInput)
    }

    fn max_input_size(&self) -> usize {
        MAX_BASE64_INPUT_SIZE
    }
}

fn secure_encode<E: SecureEncodingProvider>(provider: &E, data: &[u8]) -> NimbusResult<String> {
    provider.encode(data).map_err(|_| NimbusError::InvalidInput)
}

fn secure_decode<E: SecureEncodingProvider>(provider: &E, data: &str) -> NimbusResult<Vec<u8>> {
    provider.decode(data).map_err(|_| NimbusError::InvalidInput)
}

pub fn encode_base64(data: &[u8]) -> NimbusResult<String> {
    secure_encode(&Base64, data)
}

pub fn decode_base64(data: &str) -> NimbusResult<Vec<u8>> {
    secure_decode(&Base64, data)
}
