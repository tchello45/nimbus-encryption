use crate::error::{NimbusError, NimbusResult};
use base64ct::{Base64, Encoding};

pub const BASE64_MAX_INPUT_SIZE: usize = usize::MAX / 4;

pub trait Encoder {
    type Error;

    fn encode(&self, data: &[u8]) -> Result<String, Self::Error>;

    fn decode(&self, data: &str) -> Result<Vec<u8>, Self::Error>;

    fn max_input_size(&self) -> usize;
}

impl Encoder for Base64 {
    type Error = NimbusError;

    fn encode(&self, data: &[u8]) -> Result<String, Self::Error> {
        if data.len() > self.max_input_size() {
            return Err(NimbusError::InvalidLength);
        }
        Ok(Base64::encode_string(data))
    }

    fn decode(&self, data: &str) -> Result<Vec<u8>, Self::Error> {
        Base64::decode_vec(data).map_err(|_| NimbusError::InvalidInput)
    }

    fn max_input_size(&self) -> usize {
        BASE64_MAX_INPUT_SIZE
    }
}

fn encode_with<E: Encoder>(encoder: &E, data: &[u8]) -> NimbusResult<String> {
    encoder.encode(data).map_err(|_| NimbusError::InvalidInput)
}

fn decode_with<E: Encoder>(encoder: &E, data: &str) -> NimbusResult<Vec<u8>> {
    encoder.decode(data).map_err(|_| NimbusError::InvalidInput)
}

pub fn encode_base64(data: &[u8]) -> NimbusResult<String> {
    encode_with(&Base64, data)
}

pub fn decode_base64(data: &str) -> NimbusResult<Vec<u8>> {
    decode_with(&Base64, data)
}
