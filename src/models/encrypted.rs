#[derive(Debug)]
pub struct EncryptedData {
    pub ciphertext: Vec<u8>,
    pub nonce: [u8; 12],
}