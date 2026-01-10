//! Encrypted file-based token storage implementation.
//!
//! Stores OAuth tokens encrypted with AES-256-GCM.

use anyhow::{Context, Result};
use async_trait::async_trait;
use qa_pms_config::Encryptor;
use qa_pms_core::{AuthStateStore, StoredTokens, TokenStore};
use secrecy::ExposeSecret;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, info, warn};

/// File-based token storage with encryption.
pub struct FileTokenStore {
    /// Path to the tokens file
    file_path: PathBuf,
    /// Encryptor for securing tokens
    encryptor: Encryptor,
    /// In-memory cache of tokens
    cache: RwLock<HashMap<String, StoredTokens>>,
}

/// Serializable format for the tokens file.
#[derive(Serialize, Deserialize)]
struct TokensFile {
    version: u32,
    tokens: HashMap<String, EncryptedTokens>,
}

/// Encrypted token data for storage.
#[derive(Serialize, Deserialize)]
struct EncryptedTokens {
    access_token: String,
    refresh_token: String,
    expires_at: String,
    updated_at: String,
}

impl FileTokenStore {
    /// Current file format version.
    const VERSION: u32 = 1;

    /// Create a new file-based token store.
    ///
    /// # Errors
    ///
    /// Returns an error if the tokens file cannot be loaded.
    pub async fn new(file_path: PathBuf, encryptor: Encryptor) -> Result<Self> {
        let store = Self {
            file_path,
            encryptor,
            cache: RwLock::new(HashMap::new()),
        };

        // Load existing tokens
        store.load_from_file().await?;

        Ok(store)
    }

    /// Load tokens from file into cache.
    async fn load_from_file(&self) -> Result<()> {
        if !self.file_path.exists() {
            debug!(path = %self.file_path.display(), "Token file does not exist, starting fresh");
            return Ok(());
        }

        let contents = tokio::fs::read_to_string(&self.file_path)
            .await
            .context("Failed to read tokens file")?;

        let file: TokensFile =
            serde_json::from_str(&contents).context("Failed to parse tokens file")?;

        if file.version != Self::VERSION {
            warn!(
                file_version = file.version,
                expected = Self::VERSION,
                "Token file version mismatch, clearing tokens"
            );
            return Ok(());
        }

        let mut cache = self.cache.write().await;
        for (integration, encrypted) in file.tokens {
            match self.decrypt_tokens(&integration, &encrypted) {
                Ok(tokens) => {
                    cache.insert(integration, tokens);
                }
                Err(e) => {
                    warn!(integration = %integration, error = %e, "Failed to decrypt tokens");
                }
            }
        }

        info!(count = cache.len(), "Loaded tokens from file");
        Ok(())
    }

    /// Save all tokens to file.
    #[allow(clippy::significant_drop_tightening)]
    async fn save_to_file(&self) -> Result<()> {
        let cache = self.cache.read().await;

        let mut tokens_map = HashMap::new();
        for (integration, tokens) in cache.iter() {
            let encrypted = self.encrypt_tokens(tokens)?;
            tokens_map.insert(integration.clone(), encrypted);
        }
        drop(cache); // Release lock before I/O

        let file = TokensFile {
            version: Self::VERSION,
            tokens: tokens_map,
        };

        let contents = serde_json::to_string_pretty(&file)?;

        // Ensure parent directory exists
        if let Some(parent) = self.file_path.parent() {
            tokio::fs::create_dir_all(parent).await?;
        }

        tokio::fs::write(&self.file_path, contents)
            .await
            .context("Failed to write tokens file")?;

        debug!(path = %self.file_path.display(), "Saved tokens to file");
        Ok(())
    }

    /// Encrypt tokens for storage.
    fn encrypt_tokens(&self, tokens: &StoredTokens) -> Result<EncryptedTokens> {
        Ok(EncryptedTokens {
            access_token: self.encryptor.encrypt(&tokens.access_token)?,
            refresh_token: self.encryptor.encrypt(&tokens.refresh_token)?,
            expires_at: tokens.expires_at.to_rfc3339(),
            updated_at: tokens.updated_at.to_rfc3339(),
        })
    }

    /// Decrypt tokens from storage.
    fn decrypt_tokens(
        &self,
        integration: &str,
        encrypted: &EncryptedTokens,
    ) -> Result<StoredTokens> {
        Ok(StoredTokens {
            access_token: self
                .encryptor
                .decrypt(&encrypted.access_token)?
                .expose_secret()
                .clone(),
            refresh_token: self
                .encryptor
                .decrypt(&encrypted.refresh_token)?
                .expose_secret()
                .clone(),
            expires_at: chrono::DateTime::parse_from_rfc3339(&encrypted.expires_at)?
                .with_timezone(&chrono::Utc),
            updated_at: chrono::DateTime::parse_from_rfc3339(&encrypted.updated_at)?
                .with_timezone(&chrono::Utc),
            integration: integration.to_string(),
        })
    }
}

#[async_trait]
impl TokenStore for FileTokenStore {
    async fn store_tokens(&self, tokens: StoredTokens) -> Result<()> {
        let integration = tokens.integration.clone();

        {
            let mut cache = self.cache.write().await;
            cache.insert(integration.clone(), tokens);
        }

        self.save_to_file().await?;
        info!(integration = %integration, "Stored tokens");
        Ok(())
    }

    async fn get_tokens(&self, integration: &str) -> Result<Option<StoredTokens>> {
        let cache = self.cache.read().await;
        Ok(cache.get(integration).cloned())
    }

    async fn delete_tokens(&self, integration: &str) -> Result<()> {
        {
            let mut cache = self.cache.write().await;
            cache.remove(integration);
        }

        self.save_to_file().await?;
        info!(integration = %integration, "Deleted tokens");
        Ok(())
    }
}

// ============================================================================
// In-memory Auth State Store
// ============================================================================

/// In-memory auth state storage for OAuth flows.
///
/// Stores PKCE code verifiers temporarily during OAuth authorization.
/// States expire after 10 minutes.
pub struct InMemoryAuthStateStore {
    /// State -> (`code_verifier`, `created_at`)
    #[allow(clippy::type_complexity)]
    states: Arc<RwLock<HashMap<String, (String, chrono::DateTime<chrono::Utc>)>>>,
}

impl InMemoryAuthStateStore {
    /// State expiry time in seconds.
    const EXPIRY_SECONDS: i64 = 600; // 10 minutes

    /// Create a new in-memory auth state store.
    #[must_use]
    pub fn new() -> Self {
        Self {
            states: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

impl Default for InMemoryAuthStateStore {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl AuthStateStore for InMemoryAuthStateStore {
    async fn store(&self, state: &str, code_verifier: &str) -> Result<()> {
        let mut states = self.states.write().await;
        states.insert(
            state.to_string(),
            (code_verifier.to_string(), chrono::Utc::now()),
        );
        debug!(state_prefix = &state[..8], "Stored auth state");
        Ok(())
    }

    async fn get_and_remove(&self, state: &str) -> Result<Option<String>> {
        let mut states = self.states.write().await;

        if let Some((code_verifier, created_at)) = states.remove(state) {
            let age = chrono::Utc::now() - created_at;

            if age.num_seconds() > Self::EXPIRY_SECONDS {
                debug!(
                    state_prefix = &state[..8.min(state.len())],
                    "Auth state expired"
                );
                return Ok(None);
            }

            debug!(
                state_prefix = &state[..8.min(state.len())],
                "Retrieved and removed auth state"
            );
            return Ok(Some(code_verifier));
        }

        Ok(None)
    }

    async fn cleanup_expired(&self) -> Result<()> {
        let mut states = self.states.write().await;
        let now = chrono::Utc::now();

        let before = states.len();
        states
            .retain(|_, (_, created_at)| (now - *created_at).num_seconds() <= Self::EXPIRY_SECONDS);
        let removed = before - states.len();

        if removed > 0 {
            debug!(removed = removed, "Cleaned up expired auth states");
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_in_memory_auth_state() {
        let store = InMemoryAuthStateStore::new();

        // Store state
        store.store("state123", "verifier456").await.unwrap();

        // Retrieve and remove
        let verifier = store.get_and_remove("state123").await.unwrap();
        assert_eq!(verifier, Some("verifier456".to_string()));

        // Should be gone now
        let verifier = store.get_and_remove("state123").await.unwrap();
        assert_eq!(verifier, None);
    }

    #[tokio::test]
    async fn test_auth_state_unknown() {
        let store = InMemoryAuthStateStore::new();
        let verifier = store.get_and_remove("unknown").await.unwrap();
        assert_eq!(verifier, None);
    }
}
