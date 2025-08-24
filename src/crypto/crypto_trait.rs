pub trait CryptoCipherTrait {
    type Error;
    type Key;
    type Nonce;

    const KEY_SIZE: usize;
    const NONCE_SIZE: usize;

    fn get_key_from_u8_array(key: &[u8]) -> Result<Self::Key, Self::Error>;
    fn get_nonce_from_u8_array(nonce: &[u8]) -> Result<Self::Nonce, Self::Error>;

    fn encrypt(
        key: &Self::Key,
        nonce: &Self::Nonce,
        plaintext: &[u8],
        additional_associated_data: &[u8],
    ) -> Result<Vec<u8>, Self::Error>;
    fn decrypt(
        key: &Self::Key,
        nonce: &Self::Nonce,
        ciphertext: &[u8],
        additional_associated_data: &[u8],
    ) -> Result<Vec<u8>, Self::Error>;

    fn generate_key() -> Result<Self::Key, Self::Error>;
    fn generate_nonce() -> Result<Self::Nonce, Self::Error>;
}
