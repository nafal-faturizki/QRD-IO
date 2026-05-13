//! Cryptographic operations
//!
//! All cryptographic operations follow FIPS-140-3 aligned practices:
//! - ChaCha20-Poly1305 for AEAD encryption
//! - HKDF-SHA256 for key derivation
//! - Secure random number generation via getrandom

use chacha20poly1305::{
    aead::{Aead, KeyInit, Payload},
    ChaCha20Poly1305, Nonce,
};
use hkdf::Hkdf;
use sha2::Sha256;
use zeroize::Zeroize;

/// 32-byte encryption key
pub type EncryptionKey = [u8; 32];

/// 12-byte nonce (IV)
pub type Nonce12 = [u8; 12];

/// Cryptographic operations
pub struct Crypto;

impl Crypto {
    /// Derive a key from a password using HKDF-SHA256
    pub fn derive_key(password: &[u8], salt: &[u8]) -> crate::Result<EncryptionKey> {
        let hkdf = Hkdf::<Sha256>::new(Some(salt), password);
        let mut key = [0u8; 32];
        hkdf.expand(b"qrd-key", &mut key)
            .map_err(|_| crate::Error::InvalidKey("Key derivation failed".to_string()))?;
        Ok(key)
    }

    /// Generate a random nonce
    pub fn generate_nonce() -> crate::Result<Nonce12> {
        let mut nonce = [0u8; 12];
        getrandom::getrandom(&mut nonce)
            .map_err(|e| crate::Error::Other(format!("Random generation failed: {}", e)))?;
        Ok(nonce)
    }

    /// Encrypt data with ChaCha20-Poly1305
    pub fn encrypt(
        key: &EncryptionKey,
        nonce: &Nonce12,
        plaintext: &[u8],
        aad: &[u8],
    ) -> crate::Result<Vec<u8>> {
        let cipher = ChaCha20Poly1305::new(key.into());
        let nonce_obj = Nonce::from(*nonce);
        let payload = Payload {
            msg: plaintext,
            aad,
        };

        cipher
            .encrypt(&nonce_obj, payload)
            .map_err(|_| crate::Error::DecryptionFailed("Encryption failed".to_string()))
    }

    /// Decrypt data with ChaCha20-Poly1305
    pub fn decrypt(
        key: &EncryptionKey,
        nonce: &Nonce12,
        ciphertext: &[u8],
        aad: &[u8],
    ) -> crate::Result<Vec<u8>> {
        let cipher = ChaCha20Poly1305::new(key.into());
        let nonce_obj = Nonce::from(*nonce);
        let payload = Payload {
            msg: ciphertext,
            aad,
        };

        cipher
            .decrypt(&nonce_obj, payload)
            .map_err(|_| crate::Error::DecryptionFailed("Decryption failed".to_string()))
    }

    /// Zero-out a key from memory
    pub fn zeroize_key(key: &mut EncryptionKey) {
        key.zeroize();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_key_derivation() {
        let password = b"password";
        let salt = b"salt";
        let key = Crypto::derive_key(password, salt).unwrap();
        assert_eq!(key.len(), 32);
    }

    #[test]
    fn test_encrypt_decrypt() {
        let key = [0x42u8; 32];
        let nonce = [0x01u8; 12];
        let plaintext = b"Hello, QRD!";
        let aad = b"metadata";

        let ciphertext = Crypto::encrypt(&key, &nonce, plaintext, aad).unwrap();
        let decrypted = Crypto::decrypt(&key, &nonce, &ciphertext, aad).unwrap();
        assert_eq!(plaintext.to_vec(), decrypted);
    }

    #[test]
    fn test_wrong_key_fails() {
        let key = [0x42u8; 32];
        let wrong_key = [0x43u8; 32];
        let nonce = [0x01u8; 12];
        let plaintext = b"Hello, QRD!";
        let aad = b"metadata";

        let ciphertext = Crypto::encrypt(&key, &nonce, plaintext, aad).unwrap();
        let result = Crypto::decrypt(&wrong_key, &nonce, &ciphertext, aad);
        assert!(result.is_err());
    }
}
