# Story 3.1: Jira OAuth 2.0 + PKCE Authentication Flow

Status: ready-for-dev

## Story

As a user,
I want secure OAuth authentication with Jira,
So that my credentials are protected using industry standards.

## Acceptance Criteria

1. **Given** Jira credentials configured in setup
   **When** the application needs to authenticate with Jira
   **Then** OAuth 2.0 + PKCE flow generates code verifier and code challenge

2. **Given** authentication is initiated
   **When** user needs to authorize
   **Then** redirect to Jira authorization endpoint occurs

3. **Given** user authorizes on Jira
   **When** callback is received
   **Then** authorization code is handled and exchanged for access token

4. **Given** tokens are received
   **When** storage is needed
   **Then** tokens are stored securely (encrypted with AES-256-GCM)

5. **Given** access token is about to expire
   **When** expiry approaches
   **Then** token refresh happens automatically before expiry

6. **Given** authentication fails
   **When** error occurs
   **Then** clear error with re-auth option is shown

7. **Given** OAuth flow is implemented
   **When** security is audited
   **Then** flow complies with NFR-SEC-04 (OAuth 2.0 + PKCE standard)

## Tasks / Subtasks

- [ ] Task 1: Implement PKCE utilities (AC: #1)
  - [ ] 1.1: Create `pkce.rs` module in `qa-pms-jira`
  - [ ] 1.2: Implement code verifier generation (43-128 chars, URL-safe)
  - [ ] 1.3: Implement code challenge generation (SHA256, base64url)
  - [ ] 1.4: Add unit tests for PKCE generation

- [ ] Task 2: Implement OAuth client (AC: #1, #2, #3)
  - [ ] 2.1: Create `oauth.rs` module with JiraOAuthClient struct
  - [ ] 2.2: Implement `build_authorization_url()` method
  - [ ] 2.3: Implement `exchange_code_for_tokens()` method
  - [ ] 2.4: Implement `refresh_access_token()` method

- [ ] Task 3: Create token storage service (AC: #4)
  - [ ] 3.1: Create `TokenStore` trait in `qa-pms-core`
  - [ ] 3.2: Implement encrypted file-based storage
  - [ ] 3.3: Use `qa-pms-config` encryption utilities
  - [ ] 3.4: Add token expiry tracking

- [ ] Task 4: Implement token refresh scheduler (AC: #5)
  - [ ] 4.1: Create background task for token monitoring
  - [ ] 4.2: Trigger refresh 5 minutes before expiry
  - [ ] 4.3: Handle refresh failures gracefully
  - [ ] 4.4: Log refresh events with tracing

- [ ] Task 5: Create OAuth callback endpoint (AC: #3)
  - [ ] 5.1: Add `GET /api/v1/auth/jira/callback` route
  - [ ] 5.2: Extract authorization code from query params
  - [ ] 5.3: Exchange code for tokens
  - [ ] 5.4: Redirect to success/error page

- [ ] Task 6: Create authorization initiation endpoint (AC: #2)
  - [ ] 6.1: Add `GET /api/v1/auth/jira/authorize` route
  - [ ] 6.2: Generate and store PKCE state
  - [ ] 6.3: Redirect to Jira authorization URL

- [ ] Task 7: Implement error handling (AC: #6)
  - [ ] 7.1: Create `JiraAuthError` enum with thiserror
  - [ ] 7.2: Map OAuth errors to user-friendly messages
  - [ ] 7.3: Provide re-authorization link on failure

- [ ] Task 8: Create frontend auth flow UI (AC: #2, #6)
  - [ ] 8.1: Create `JiraAuthButton` component
  - [ ] 8.2: Handle OAuth redirect flow
  - [ ] 8.3: Show auth status and errors
  - [ ] 8.4: Add "Re-authenticate" button on failure

## Dev Notes

### Architecture Alignment

This story implements **Jira OAuth 2.0 + PKCE Authentication** per Epic 3 requirements:

- **Backend Location**: `crates/qa-pms-jira/src/oauth.rs`
- **Security**: NFR-SEC-04 compliant OAuth 2.0 + PKCE
- **Token Storage**: Encrypted via `qa-pms-config`

### Technical Implementation Details

#### PKCE Module

```rust
// crates/qa-pms-jira/src/pkce.rs
use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
use rand::Rng;
use sha2::{Digest, Sha256};

/// Generate a cryptographically random code verifier (43-128 chars)
pub fn generate_code_verifier() -> String {
    let mut rng = rand::thread_rng();
    let bytes: Vec<u8> = (0..32).map(|_| rng.gen()).collect();
    URL_SAFE_NO_PAD.encode(&bytes)
}

/// Generate code challenge from verifier using S256 method
pub fn generate_code_challenge(verifier: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(verifier.as_bytes());
    let hash = hasher.finalize();
    URL_SAFE_NO_PAD.encode(&hash)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_verifier_length() {
        let verifier = generate_code_verifier();
        assert!(verifier.len() >= 43 && verifier.len() <= 128);
    }

    #[test]
    fn test_challenge_is_base64url() {
        let verifier = generate_code_verifier();
        let challenge = generate_code_challenge(&verifier);
        // Should be 43 chars (256 bits / 6 bits per char)
        assert_eq!(challenge.len(), 43);
    }
}
```

#### OAuth Client

```rust
// crates/qa-pms-jira/src/oauth.rs
use crate::pkce::{generate_code_challenge, generate_code_verifier};
use anyhow::Result;
use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub struct JiraOAuthConfig {
    pub client_id: String,
    pub client_secret: String,
    pub redirect_uri: String,
    pub scopes: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenResponse {
    pub access_token: String,
    pub refresh_token: String,
    pub expires_in: u64,
    pub token_type: String,
    pub scope: String,
}

#[derive(Debug, Clone)]
pub struct AuthorizationState {
    pub code_verifier: String,
    pub state: String,
}

pub struct JiraOAuthClient {
    config: JiraOAuthConfig,
    http_client: Client,
}

impl JiraOAuthClient {
    const AUTH_URL: &'static str = "https://auth.atlassian.com/authorize";
    const TOKEN_URL: &'static str = "https://auth.atlassian.com/oauth/token";

    pub fn new(config: JiraOAuthConfig) -> Self {
        Self {
            config,
            http_client: Client::new(),
        }
    }

    /// Build the authorization URL with PKCE
    pub fn build_authorization_url(&self) -> (String, AuthorizationState) {
        let code_verifier = generate_code_verifier();
        let code_challenge = generate_code_challenge(&code_verifier);
        let state = generate_code_verifier(); // Use same function for random state

        let scopes = self.config.scopes.join(" ");
        
        let url = format!(
            "{}?audience=api.atlassian.com&client_id={}&scope={}&redirect_uri={}&state={}&response_type=code&prompt=consent&code_challenge={}&code_challenge_method=S256",
            Self::AUTH_URL,
            &self.config.client_id,
            urlencoding::encode(&scopes),
            urlencoding::encode(&self.config.redirect_uri),
            &state,
            &code_challenge,
        );

        let auth_state = AuthorizationState {
            code_verifier,
            state,
        };

        (url, auth_state)
    }

    /// Exchange authorization code for tokens
    pub async fn exchange_code_for_tokens(
        &self,
        code: &str,
        code_verifier: &str,
    ) -> Result<TokenResponse> {
        let params = [
            ("grant_type", "authorization_code"),
            ("client_id", &self.config.client_id),
            ("client_secret", &self.config.client_secret),
            ("code", code),
            ("redirect_uri", &self.config.redirect_uri),
            ("code_verifier", code_verifier),
        ];

        let response = self
            .http_client
            .post(Self::TOKEN_URL)
            .form(&params)
            .send()
            .await?
            .error_for_status()?
            .json::<TokenResponse>()
            .await?;

        Ok(response)
    }

    /// Refresh access token using refresh token
    pub async fn refresh_access_token(&self, refresh_token: &str) -> Result<TokenResponse> {
        let params = [
            ("grant_type", "refresh_token"),
            ("client_id", &self.config.client_id),
            ("client_secret", &self.config.client_secret),
            ("refresh_token", refresh_token),
        ];

        let response = self
            .http_client
            .post(Self::TOKEN_URL)
            .form(&params)
            .send()
            .await?
            .error_for_status()?
            .json::<TokenResponse>()
            .await?;

        Ok(response)
    }
}
```

#### Token Store Trait

```rust
// crates/qa-pms-core/src/auth.rs
use async_trait::async_trait;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct StoredTokens {
    pub access_token: String,
    pub refresh_token: String,
    pub expires_at: DateTime<Utc>,
    pub integration: String,
}

#[async_trait]
pub trait TokenStore: Send + Sync {
    async fn store_tokens(&self, tokens: StoredTokens) -> anyhow::Result<()>;
    async fn get_tokens(&self, integration: &str) -> anyhow::Result<Option<StoredTokens>>;
    async fn delete_tokens(&self, integration: &str) -> anyhow::Result<()>;
    async fn is_token_expired(&self, integration: &str) -> anyhow::Result<bool>;
}
```

#### API Routes

```rust
// crates/qa-pms-api/src/routes/auth.rs
use axum::{
    extract::{Query, State},
    response::Redirect,
    routing::get,
    Router,
};
use serde::Deserialize;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/jira/authorize", get(initiate_jira_auth))
        .route("/jira/callback", get(handle_jira_callback))
}

#[derive(Deserialize)]
pub struct CallbackParams {
    code: String,
    state: String,
}

async fn initiate_jira_auth(
    State(state): State<AppState>,
) -> Redirect {
    let (auth_url, auth_state) = state.jira_oauth.build_authorization_url();
    
    // Store auth_state in session/cache for callback validation
    state.auth_state_store.store(&auth_state.state, &auth_state.code_verifier).await;
    
    Redirect::temporary(&auth_url)
}

async fn handle_jira_callback(
    State(state): State<AppState>,
    Query(params): Query<CallbackParams>,
) -> Result<Redirect, ApiError> {
    // Retrieve and validate state
    let code_verifier = state
        .auth_state_store
        .get_and_remove(&params.state)
        .await
        .ok_or(ApiError::BadRequest("Invalid state".into()))?;
    
    // Exchange code for tokens
    let tokens = state
        .jira_oauth
        .exchange_code_for_tokens(&params.code, &code_verifier)
        .await?;
    
    // Store tokens (encrypted)
    let stored = StoredTokens {
        access_token: tokens.access_token,
        refresh_token: tokens.refresh_token,
        expires_at: Utc::now() + chrono::Duration::seconds(tokens.expires_in as i64),
        integration: "jira".to_string(),
    };
    state.token_store.store_tokens(stored).await?;
    
    Ok(Redirect::temporary("/settings?auth=success"))
}
```

#### Token Refresh Background Task

```rust
// crates/qa-pms-jira/src/token_refresh.rs
use tokio::time::{interval, Duration};
use tracing::{info, warn};

pub async fn start_token_refresh_task(
    token_store: Arc<dyn TokenStore>,
    oauth_client: Arc<JiraOAuthClient>,
) {
    let mut interval = interval(Duration::from_secs(60)); // Check every minute

    loop {
        interval.tick().await;
        
        if let Ok(Some(tokens)) = token_store.get_tokens("jira").await {
            let now = Utc::now();
            let refresh_threshold = tokens.expires_at - chrono::Duration::minutes(5);
            
            if now >= refresh_threshold {
                info!("Refreshing Jira access token (expires at {})", tokens.expires_at);
                
                match oauth_client.refresh_access_token(&tokens.refresh_token).await {
                    Ok(new_tokens) => {
                        let stored = StoredTokens {
                            access_token: new_tokens.access_token,
                            refresh_token: new_tokens.refresh_token,
                            expires_at: Utc::now() + chrono::Duration::seconds(new_tokens.expires_in as i64),
                            integration: "jira".to_string(),
                        };
                        if let Err(e) = token_store.store_tokens(stored).await {
                            warn!("Failed to store refreshed tokens: {}", e);
                        }
                    }
                    Err(e) => {
                        warn!("Failed to refresh Jira token: {}", e);
                        // TODO: Notify user that re-auth is needed
                    }
                }
            }
        }
    }
}
```

### Jira OAuth Scopes Required

```
read:jira-user
read:jira-work
write:jira-work
offline_access
```

### Error Types

```rust
// crates/qa-pms-jira/src/error.rs
use thiserror::Error;

#[derive(Debug, Error)]
pub enum JiraAuthError {
    #[error("Invalid OAuth state - possible CSRF attack")]
    InvalidState,
    
    #[error("Authorization code expired or invalid")]
    InvalidCode,
    
    #[error("Token refresh failed: {0}")]
    RefreshFailed(String),
    
    #[error("User denied authorization")]
    UserDenied,
    
    #[error("Network error: {0}")]
    Network(#[from] reqwest::Error),
}
```

### Frontend Component

```tsx
// frontend/src/components/JiraAuthButton.tsx
import { useState } from "react";
import { ExternalLinkIcon, CheckCircledIcon, CrossCircledIcon } from "@radix-ui/react-icons";

interface JiraAuthButtonProps {
  isAuthenticated: boolean;
  onReauth?: () => void;
}

export function JiraAuthButton({ isAuthenticated, onReauth }: JiraAuthButtonProps) {
  const [isLoading, setIsLoading] = useState(false);

  const handleAuth = () => {
    setIsLoading(true);
    // Redirect to backend OAuth initiation
    window.location.href = "/api/v1/auth/jira/authorize";
  };

  if (isAuthenticated) {
    return (
      <div className="flex items-center gap-2 text-success-500">
        <CheckCircledIcon className="w-5 h-5" />
        <span>Connected to Jira</span>
        <button
          onClick={onReauth}
          className="text-sm text-neutral-500 hover:text-neutral-700 underline ml-2"
        >
          Re-authenticate
        </button>
      </div>
    );
  }

  return (
    <button
      onClick={handleAuth}
      disabled={isLoading}
      className="flex items-center gap-2 px-4 py-2 bg-primary-500 text-white rounded-lg
                 hover:bg-primary-600 disabled:bg-neutral-300 transition-colors"
    >
      {isLoading ? (
        <div className="w-4 h-4 border-2 border-white border-t-transparent rounded-full animate-spin" />
      ) : (
        <ExternalLinkIcon className="w-4 h-4" />
      )}
      Connect to Jira
    </button>
  );
}
```

### Project Structure Notes

Files to create:
```
crates/qa-pms-jira/src/
├── lib.rs              # Module exports
├── pkce.rs             # PKCE utilities
├── oauth.rs            # OAuth client
├── token_refresh.rs    # Background refresh task
└── error.rs            # Error types

crates/qa-pms-api/src/routes/
└── auth.rs             # OAuth callback routes

crates/qa-pms-core/src/
└── auth.rs             # TokenStore trait

frontend/src/components/
└── JiraAuthButton.tsx  # Auth UI component
```

### Security Notes

- PKCE prevents authorization code interception attacks
- State parameter prevents CSRF attacks
- Tokens encrypted at rest with AES-256-GCM
- Refresh token rotation on each refresh
- Never log tokens or sensitive data

### Dependencies

```toml
# crates/qa-pms-jira/Cargo.toml
[dependencies]
sha2 = "0.10"
base64 = "0.21"
rand = "0.8"
urlencoding = "2.1"
```

### Testing Notes

- Unit test PKCE generation (verifier length, challenge format)
- Unit test authorization URL construction
- Integration test: Mock OAuth flow end-to-end
- Test token refresh logic with mocked time
- Test error handling for all failure modes

### References

- [Source: _bmad-output/planning-artifacts/epics.md#Story 3.1]
- [Source: Atlassian OAuth 2.0 Documentation]
- [Source: _bmad-output/planning-artifacts/prd.md#NFR-SEC-04]

## Dev Agent Record

### Agent Model Used

{{agent_model_name_version}}

### Debug Log References

### Completion Notes List

### File List
