#[derive(Debug)]
pub struct EncryptedData {
    pub ciphertext: Vec<u8>,
    pub nonce: [u8; 12],
    pub encrypted_dek: Vec<u8>,
    pub dek_nonce: [u8; 12],
}