use crate::error::{NimbusError, NimbusResult};
use base64ct::{Base64, Encoding};

pub const BASE64_MAX_INPUT_SIZE: usize = usize::MAX / 4;

pub trait Encoder {
    type Error;

    /// Encodes the given byte data into a string representation.
    ///
    /// # Errors
    ///
    /// Returns an error if the input data is too large or if encoding fails.
    fn encode(&self, data: &[u8]) -> Result<String, Self::Error>;

    /// Decodes the given string data into bytes.
    ///
    /// # Errors
    ///
    /// Returns an error if the input string is invalid or malformed.
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

/// Encodes the given byte data into a Base64 string.
///
/// # Errors
///
/// Returns `NimbusError::InvalidLength` if the input data is too large,
/// or `NimbusError::InvalidInput` if encoding fails.
pub fn encode_base64(data: &[u8]) -> NimbusResult<String> {
    encode_with(&Base64, data)
}

/// Decodes the given Base64 string into bytes.
///
/// # Errors
///
/// Returns `NimbusError::InvalidInput` if the input string is invalid
/// or malformed Base64.
pub fn decode_base64(data: &str) -> NimbusResult<Vec<u8>> {
    decode_with(&Base64, data)
}

#[cfg(test)]
mod tests {
    use super::*;

    struct MockEncoder {
        input_too_large: bool,
        decode_failure: bool,
        encode_failure: bool,
    }

    impl MockEncoder {
        fn new_success() -> Self {
            Self {
                input_too_large: false,
                decode_failure: false,
                encode_failure: false,
            }
        }

        fn new_input_too_large() -> Self {
            Self {
                input_too_large: true,
                decode_failure: false,
                encode_failure: false,
            }
        }

        fn new_decode_failure() -> Self {
            Self {
                input_too_large: false,
                decode_failure: true,
                encode_failure: false,
            }
        }

        fn new_encode_failure() -> Self {
            Self {
                input_too_large: false,
                decode_failure: false,
                encode_failure: true,
            }
        }
    }

    impl Encoder for MockEncoder {
        type Error = NimbusError;

        fn encode(&self, _data: &[u8]) -> Result<String, Self::Error> {
            if self.input_too_large {
                return Err(NimbusError::InvalidLength);
            }
            if self.encode_failure {
                return Err(NimbusError::InvalidInput);
            }
            Ok(String::from("test"))
        }

        fn decode(&self, _data: &str) -> Result<Vec<u8>, Self::Error> {
            if self.decode_failure {
                return Err(NimbusError::InvalidInput);
            }
            Ok(b"test".to_vec())
        }

        fn max_input_size(&self) -> usize {
            usize::MAX
        }
    }

    #[test]
    fn base64_encoder_encode_success() {
        let encoder = Base64;
        let data = b"Hello, World!";
        let result = encoder.encode(data);
        assert!(result.is_ok());
        let encoded_result = result.unwrap();
        assert_eq!(encoded_result, "SGVsbG8sIFdvcmxkIQ==");
    }

    #[test]
    fn base64_encoder_empty_data_roundtrip() {
        let encoder = Base64;

        // Test encoding empty data
        let empty_bytes = b"";
        let encoded_result = encoder.encode(empty_bytes).unwrap();
        assert_eq!(encoded_result, "");

        // Test decoding empty string
        let empty_string = "";
        let decoded = encoder.decode(empty_string).unwrap();
        assert_eq!(decoded, b"");

        // Test full roundtrip
        let roundtrip = encoder.decode(&encoded_result).unwrap();
        assert_eq!(roundtrip, empty_bytes);
    }

    #[test]
    fn base64_encoder_encode_input_too_large() {
        // We can't actually allocate usize::MAX/4 + 1 bytes in memory,
        // but we can test the logic by using a mock encoder that simulates this condition
        let mock = MockEncoder::new_input_too_large();
        let data = b"test data";
        let result = mock.encode(data);
        assert_eq!(result.unwrap_err(), NimbusError::InvalidLength);
    }

    #[test]
    fn base64_encoder_decode_success() {
        let encoder = Base64;
        let data = "SGVsbG8sIFdvcmxkIQ==";
        let result = encoder.decode(data);
        assert!(result.is_ok());
        let decoded = result.unwrap();
        assert_eq!(decoded, b"Hello, World!");
    }

    #[test]
    fn base64_encoder_decode_invalid_input() {
        let encoder = Base64;
        let data = "Invalid Base64!@#$%";
        let result = encoder.decode(data);
        assert_eq!(result.unwrap_err(), NimbusError::InvalidInput);
    }

    #[test]
    fn base64_encoder_max_input_size() {
        let encoder = Base64;
        assert_eq!(encoder.max_input_size(), BASE64_MAX_INPUT_SIZE);
    }

    #[test]
    fn encode_with_success() {
        let mock = MockEncoder::new_success();
        let data = b"test data";
        let result = encode_with(&mock, data);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "test");
    }

    #[test]
    fn encode_with_input_too_large() {
        let mock = MockEncoder::new_input_too_large();
        let data = b"test data";
        let result = encode_with(&mock, data);
        assert_eq!(result.unwrap_err(), NimbusError::InvalidInput);
    }

    #[test]
    fn encode_with_encode_failure() {
        let mock = MockEncoder::new_encode_failure();
        let data = b"test data";
        let result = encode_with(&mock, data);
        assert_eq!(result.unwrap_err(), NimbusError::InvalidInput);
    }

    #[test]
    fn decode_with_success() {
        let mock = MockEncoder::new_success();
        let data = "test data";
        let result = decode_with(&mock, data);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), b"test");
    }

    #[test]
    fn decode_with_failure() {
        let mock = MockEncoder::new_decode_failure();
        let data = "test data";
        let result = decode_with(&mock, data);
        assert_eq!(result.unwrap_err(), NimbusError::InvalidInput);
    }

    #[test]
    fn encode_base64_success() {
        let data = b"Hello, Base64!";
        let result = encode_base64(data);
        assert!(result.is_ok());
        let encoded = result.unwrap();
        assert_eq!(encoded, "SGVsbG8sIEJhc2U2NCE=");
    }

    #[test]
    fn base64_api_empty_data_roundtrip() {
        // Test encoding empty data
        let empty_bytes = b"";
        let encoded = encode_base64(empty_bytes).unwrap();
        assert_eq!(encoded, "");

        // Test decoding empty string
        let empty_string = "";
        let decoded = decode_base64(empty_string).unwrap();
        assert_eq!(decoded, b"");

        // Test full roundtrip
        let roundtrip = decode_base64(&encoded).unwrap();
        assert_eq!(roundtrip, empty_bytes);
    }

    #[test]
    fn decode_base64_success() {
        let data = "SGVsbG8sIEJhc2U2NCE=";
        let result = decode_base64(data);
        assert!(result.is_ok());
        let decoded = result.unwrap();
        assert_eq!(decoded, b"Hello, Base64!");
    }

    #[test]
    fn decode_base64_invalid_input() {
        let data = "Invalid Base64 Data!@#";
        let result = decode_base64(data);
        assert_eq!(result.unwrap_err(), NimbusError::InvalidInput);
    }

    #[test]
    fn mock_encoder_behavior_configurations() {
        // Test success configuration
        let success_mock = MockEncoder::new_success();
        assert!(!success_mock.input_too_large);
        assert!(!success_mock.decode_failure);
        assert!(!success_mock.encode_failure);
        assert_eq!(success_mock.max_input_size(), usize::MAX);

        // Test input too large configuration
        let large_input_mock = MockEncoder::new_input_too_large();
        assert!(large_input_mock.input_too_large);
        assert!(!large_input_mock.decode_failure);
        assert!(!large_input_mock.encode_failure);

        // Test decode failure configuration
        let decode_fail_mock = MockEncoder::new_decode_failure();
        assert!(!decode_fail_mock.input_too_large);
        assert!(decode_fail_mock.decode_failure);
        assert!(!decode_fail_mock.encode_failure);

        // Test encode failure configuration
        let encode_fail_mock = MockEncoder::new_encode_failure();
        assert!(!encode_fail_mock.input_too_large);
        assert!(!encode_fail_mock.decode_failure);
        assert!(encode_fail_mock.encode_failure);
    }

    #[test]
    fn roundtrip_encoding_decoding() {
        let original_data = b"This is a test message for roundtrip encoding/decoding!";
        let encoded = encode_base64(original_data).unwrap();
        let decoded = decode_base64(&encoded).unwrap();
        assert_eq!(decoded, original_data);
    }

    #[test]
    fn roundtrip_with_special_characters() {
        let original_data = b"Special chars: \x00\x01\x02\xFF\xFE\xFD";
        let encoded = encode_base64(original_data).unwrap();
        let decoded = decode_base64(&encoded).unwrap();
        assert_eq!(decoded, original_data);
    }
}
