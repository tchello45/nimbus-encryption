/// A trait for cryptographic cipher implementations providing authenticated encryption.
///
/// This trait defines the interface for symmetric encryption algorithms that provide
/// both confidentiality and authenticity, such as AES-GCM or XChaCha20-Poly1305.
pub trait CryptoCipherTrait {
    /// The error type returned by operations in this trait.
    type Error;
    /// The key type used for encryption and decryption.
    type Key;
    /// The nonce type used for encryption and decryption.
    type Nonce;

    /// The size of the key in bytes.
    const KEY_SIZE: usize;
    /// The size of the nonce in bytes.
    const NONCE_SIZE: usize;

    /// Creates a key from a byte array.
    ///
    /// # Errors
    ///
    /// Returns an error if the provided byte array has an invalid length or format
    /// for the key type.
    fn get_key_from_u8_array(key: &[u8]) -> Result<Self::Key, Self::Error>;

    /// Creates a nonce from a byte array.
    ///
    /// # Errors
    ///
    /// Returns an error if the provided byte array has an invalid length or format
    /// for the nonce type.
    fn get_nonce_from_u8_array(nonce: &[u8]) -> Result<Self::Nonce, Self::Error>;

    /// Encrypts plaintext using authenticated encryption.
    ///
    /// # Errors
    ///
    /// Returns an error if the encryption operation fails due to invalid parameters
    /// or internal cryptographic errors.
    fn encrypt(
        key: &Self::Key,
        nonce: &Self::Nonce,
        plaintext: &[u8],
        additional_associated_data: &[u8],
    ) -> Result<Vec<u8>, Self::Error>;

    /// Decrypts ciphertext using authenticated decryption.
    ///
    /// # Errors
    ///
    /// Returns an error if the decryption operation fails due to authentication
    /// failure, invalid parameters, or internal cryptographic errors.
    fn decrypt(
        key: &Self::Key,
        nonce: &Self::Nonce,
        ciphertext: &[u8],
        additional_associated_data: &[u8],
    ) -> Result<Vec<u8>, Self::Error>;

    /// Generates a new cryptographically secure key.
    ///
    /// # Errors
    ///
    /// Returns an error if the key generation fails due to insufficient entropy
    /// or other system-level issues.
    fn generate_key() -> Result<Self::Key, Self::Error>;

    /// Generates a new cryptographically secure nonce.
    ///
    /// # Errors
    ///
    /// Returns an error if the nonce generation fails due to insufficient entropy
    /// or other system-level issues.
    fn generate_nonce() -> Result<Self::Nonce, Self::Error>;
}
