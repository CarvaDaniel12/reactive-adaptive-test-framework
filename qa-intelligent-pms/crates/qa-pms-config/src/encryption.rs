//! AES-256-GCM encryption for sensitive configuration data.
//!
//! Uses the `aes-gcm` crate (security audited by NCC Group) for encryption
//! and `secrecy` for secure secret handling.

use aes_gcm::{
    aead::{Aead, KeyInit},
    Aes256Gcm, Nonce,
};
use anyhow::{Context, Result};
use rand::RngCore;
use secrecy::{ExposeSecret, SecretString};
use zeroize::Zeroizing;

/// Nonce size for AES-256-GCM (96 bits = 12 bytes)
const NONCE_SIZE: usize = 12;

/// Encryptor for sensitive configuration data.
///
/// Uses AES-256-GCM for authenticated encryption.
#[derive(Clone)]
pub struct Encryptor {
    cipher: Aes256Gcm,
}

impl Encryptor {
    /// Create a new encryptor from a hex-encoded 256-bit key.
    ///
    /// # Errors
    ///
    /// Returns an error if the key is not a valid 64-character hex string.
    pub fn from_hex_key(hex_key: &str) -> Result<Self> {
        let key_bytes = Zeroizing::new(
            hex::decode(hex_key).context("Invalid hex encoding for encryption key")?,
        );

        if key_bytes.len() != 32 {
            anyhow::bail!(
                "Encryption key must be 256 bits (64 hex characters), got {} bytes",
                key_bytes.len()
            );
        }

        let key = aes_gcm::Key::<Aes256Gcm>::from_slice(&key_bytes);
        let cipher = Aes256Gcm::new(key);

        Ok(Self { cipher })
    }

    /// Encrypt a plaintext string.
    ///
    /// Returns the ciphertext as a hex-encoded string with the nonce prepended.
    ///
    /// # Errors
    ///
    /// Returns an error if encryption fails.
    pub fn encrypt(&self, plaintext: &str) -> Result<String> {
        let mut nonce_bytes = [0u8; NONCE_SIZE];
        rand::thread_rng().fill_bytes(&mut nonce_bytes);
        let nonce = Nonce::from_slice(&nonce_bytes);

        let ciphertext = self
            .cipher
            .encrypt(nonce, plaintext.as_bytes())
            .map_err(|e| anyhow::anyhow!("Encryption failed: {e}"))?;

        // Prepend nonce to ciphertext and hex encode
        let mut result = nonce_bytes.to_vec();
        result.extend(ciphertext);
        Ok(hex::encode(result))
    }

    /// Decrypt a hex-encoded ciphertext (with prepended nonce).
    ///
    /// Returns the plaintext as a `SecretString` to prevent accidental logging.
    ///
    /// # Errors
    ///
    /// Returns an error if decryption fails or the ciphertext is malformed.
    pub fn decrypt(&self, ciphertext_hex: &str) -> Result<SecretString> {
        let data = hex::decode(ciphertext_hex).context("Invalid hex encoding for ciphertext")?;

        if data.len() < NONCE_SIZE {
            anyhow::bail!("Ciphertext too short (must include nonce)");
        }

        let (nonce_bytes, ciphertext) = data.split_at(NONCE_SIZE);
        let nonce = Nonce::from_slice(nonce_bytes);

        let plaintext = self
            .cipher
            .decrypt(nonce, ciphertext)
            .map_err(|e| anyhow::anyhow!("Decryption failed: {e}"))?;

        let plaintext_str =
            String::from_utf8(plaintext).context("Decrypted data is not valid UTF-8")?;

        Ok(SecretString::from(plaintext_str))
    }

    /// Encrypt a `SecretString`.
    ///
    /// # Errors
    ///
    /// Returns an error if encryption fails.
    pub fn encrypt_secret(&self, secret: &SecretString) -> Result<String> {
        self.encrypt(secret.expose_secret())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_encryptor() -> Encryptor {
        // Test key: 256 bits = 64 hex characters
        Encryptor::from_hex_key(
            "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef",
        )
        .expect("Failed to create test encryptor")
    }

    #[test]
    fn test_encrypt_decrypt_roundtrip() {
        let encryptor = test_encryptor();
        let plaintext = "my-secret-api-key";

        let ciphertext = encryptor.encrypt(plaintext).expect("Encryption failed");
        let decrypted = encryptor.decrypt(&ciphertext).expect("Decryption failed");

        assert_eq!(decrypted.expose_secret(), plaintext);
    }

    #[test]
    fn test_different_ciphertexts_for_same_plaintext() {
        let encryptor = test_encryptor();
        let plaintext = "same-secret";

        let ciphertext1 = encryptor.encrypt(plaintext).expect("Encryption failed");
        let ciphertext2 = encryptor.encrypt(plaintext).expect("Encryption failed");

        // Different nonces should produce different ciphertexts
        assert_ne!(ciphertext1, ciphertext2);

        // But both should decrypt to the same plaintext
        let decrypted1 = encryptor.decrypt(&ciphertext1).expect("Decryption failed");
        let decrypted2 = encryptor.decrypt(&ciphertext2).expect("Decryption failed");
        assert_eq!(decrypted1.expose_secret(), decrypted2.expose_secret());
    }

    #[test]
    fn test_invalid_key_length() {
        let result = Encryptor::from_hex_key("tooshort");
        assert!(result.is_err());
    }

    #[test]
    fn test_invalid_ciphertext() {
        let encryptor = test_encryptor();
        let result = encryptor.decrypt("invalid");
        assert!(result.is_err());
    }
}
