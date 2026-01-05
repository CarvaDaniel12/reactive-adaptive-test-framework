//! PKCE (Proof Key for Code Exchange) utilities.
//!
//! Implements RFC 7636 for secure OAuth 2.0 authorization code flow.
//!
//! Note: As of 2024, Jira Cloud does NOT support PKCE - it requires `client_secret`.
//! However, we implement PKCE utilities for Jira Data Center/Server support
//! and for future-proofing when Jira Cloud adds PKCE support.

use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
use rand::Rng;
use sha2::{Digest, Sha256};

/// PKCE code verifier and challenge pair.
#[derive(Debug, Clone)]
pub struct PkceChallenge {
    /// The code verifier (43-128 chars, URL-safe random string)
    pub verifier: String,
    /// The code challenge (SHA256 hash of verifier, base64url encoded)
    pub challenge: String,
}

impl PkceChallenge {
    /// Generate a new PKCE challenge pair.
    ///
    /// Creates a cryptographically random code verifier and derives
    /// the code challenge using the S256 method (SHA256 + base64url).
    #[must_use]
    pub fn new() -> Self {
        let verifier = generate_code_verifier();
        let challenge = generate_code_challenge(&verifier);
        Self { verifier, challenge }
    }

    /// Get the challenge method string for OAuth requests.
    #[must_use]
    pub const fn method() -> &'static str {
        "S256"
    }
}

impl Default for PkceChallenge {
    fn default() -> Self {
        Self::new()
    }
}

/// Generate a cryptographically random code verifier.
///
/// The verifier is 43-128 characters using only unreserved URI characters:
/// `[A-Z] / [a-z] / [0-9] / "-" / "." / "_" / "~"`
///
/// We use 32 random bytes encoded as base64url, resulting in 43 characters.
#[must_use]
pub fn generate_code_verifier() -> String {
    let mut rng = rand::thread_rng();
    let bytes: Vec<u8> = (0..32).map(|_| rng.gen()).collect();
    URL_SAFE_NO_PAD.encode(bytes)
}

/// Generate code challenge from verifier using S256 method.
///
/// Computes: `BASE64URL(SHA256(ASCII(code_verifier)))`
///
/// The result is a 43-character base64url string (256 bits / 6 bits per char).
#[must_use]
pub fn generate_code_challenge(verifier: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(verifier.as_bytes());
    let hash = hasher.finalize();
    URL_SAFE_NO_PAD.encode(hash)
}

/// Generate a random state parameter for CSRF protection.
///
/// Uses the same generation as code verifier for simplicity.
#[must_use]
pub fn generate_state() -> String {
    generate_code_verifier()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_verifier_length() {
        let verifier = generate_code_verifier();
        // 32 bytes base64url encoded = 43 characters
        assert_eq!(verifier.len(), 43);
        // Must be within RFC 7636 bounds (43-128)
        assert!(verifier.len() >= 43 && verifier.len() <= 128);
    }

    #[test]
    fn test_verifier_is_url_safe() {
        let verifier = generate_code_verifier();
        // Should only contain URL-safe characters
        assert!(verifier
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_'));
    }

    #[test]
    fn test_challenge_length() {
        let verifier = generate_code_verifier();
        let challenge = generate_code_challenge(&verifier);
        // SHA256 = 32 bytes, base64url encoded = 43 characters
        assert_eq!(challenge.len(), 43);
    }

    #[test]
    fn test_challenge_is_deterministic() {
        let verifier = "test_verifier_string_for_testing_purposes_ok";
        let challenge1 = generate_code_challenge(verifier);
        let challenge2 = generate_code_challenge(verifier);
        assert_eq!(challenge1, challenge2);
    }

    #[test]
    fn test_different_verifiers_produce_different_challenges() {
        let verifier1 = generate_code_verifier();
        let verifier2 = generate_code_verifier();
        let challenge1 = generate_code_challenge(&verifier1);
        let challenge2 = generate_code_challenge(&verifier2);
        assert_ne!(challenge1, challenge2);
    }

    #[test]
    fn test_pkce_challenge_struct() {
        let pkce = PkceChallenge::new();
        assert_eq!(pkce.verifier.len(), 43);
        assert_eq!(pkce.challenge.len(), 43);
        assert_eq!(PkceChallenge::method(), "S256");
    }

    #[test]
    fn test_known_challenge() {
        // Test vector from RFC 7636 Appendix B
        // Note: The RFC uses a specific verifier, we verify our implementation
        // produces consistent results
        let verifier = "dBjftJeZ4CVP-mB92K27uhbUJU1p1r_wW1gFWFOEjXk";
        let challenge = generate_code_challenge(verifier);
        // Expected: E9Melhoa2OwvFrEMTJguCHaoeK1t8URWbuGJSstw-cM
        assert_eq!(challenge, "E9Melhoa2OwvFrEMTJguCHaoeK1t8URWbuGJSstw-cM");
    }
}
