# Story 15.2: OAuth 2.0 + PKCE Authentication

Status: ready-for-dev

## Story

As an Enterprise QA Lead,
I want to log in using my corporate SSO (Okta, Azure AD, Google),
So that I don't need to remember another password and leverage existing security policies.

## Acceptance Criteria

1. **Given** OAuth 2.0 provider is configured (e.g., Okta)
   **When** user clicks "Login with [Provider]" button
   **Then** the system initiates PKCE flow
   **And** generates code_verifier and code_challenge
   **And** redirects user to provider's consent screen
   **And** includes state parameter for CSRF protection

2. **Given** user approves consent on provider
   **When** provider redirects back with authorization code
   **Then** the system validates state parameter
   **And** exchanges code for tokens using code_verifier
   **And** retrieves user info from OIDC userinfo endpoint
   **And** creates or updates user account
   **And** generates JWT tokens for the application
   **And** redirects user to dashboard

3. **Given** user doesn't exist yet
   **When** OAuth login succeeds
   **Then** the system creates new user account
   **And** assigns default role based on email domain rules
   **And** sends welcome email

4. **Given** multiple OAuth providers are configured
   **When** user sees login options
   **Then** all enabled providers are displayed
   **And** each provider has appropriate branding/icon
   **And** user can choose which provider to use

5. **Given** OAuth tokens are received
   **When** tokens need to be stored
   **Then** access_token is stored encrypted
   **And** refresh_token is stored encrypted
   **And** expiry timestamps are tracked
   **And** provider-specific metadata is saved

6. **Given** OAuth access token is about to expire
   **When** auto-refresh is triggered
   **Then** system uses refresh_token to get new access_token
   **And** updates stored tokens
   **And** maintains user session seamlessly

7. **Given** OAuth flow fails or is cancelled
   **When** error callback is received
   **Then** user-friendly error message is displayed
   **And** provides option to try again
   **And** logs error details for debugging

8. **Given** OAuth integration is configured
   **When** OAuth flow is implemented
   **Then** it follows PKCE (Proof Key for Code Exchange) specification
   **And** uses OIDC (OpenID Connect) for user info
   **And** implements state parameter to prevent CSRF
   **And** secures redirect URIs properly

9. **Given** user wants to link another OAuth provider
   **When** user adds additional provider
   **Then** system links to existing user account
   **And** allows login with any linked provider
   **And** maintains single user identity

10. **Given** OAuth configuration needs to be managed
   **When** admin configures providers
   **Then** they can enable/disable each provider
   **And** set client_id and client_secret securely
   **And** configure redirect URIs
   **And** define default role mapping rules

## Tasks / Subtasks

- [ ] Task 1: Setup OAuth provider infrastructure (AC: #4, #10)
  - [ ] 1.1: Create `oauth_providers` table migration
  - [ ] 1.2: Add provider_id, name, client_id, client_secret_encrypted columns
  - [ ] 1.3: Add redirect_uri, enabled, scopes columns
  - [ ] 1.4: Create `oauth_accounts` table for user-provider links
  - [ ] 1.5: Add indexes on provider_id and user_id

- [ ] Task 2: Implement PKCE utilities (AC: #1, #8)
  - [ ] 2.1: Create `pkce.rs` module in `qa-pms-auth`
  - [ ] 2.2: Implement `generate_code_verifier()` (crypto-secure random, 43-128 chars)
  - [ ] 2.3: Implement `generate_code_challenge()` (SHA256, base64url)
  - [ ] 2.4: Implement `generate_state()` for CSRF protection
  - [ ] 2.5: Add unit tests for PKCE generation

- [ ] Task 3: Implement OAuth 2.0 client (AC: #1, #2, #8)
  - [ ] 3.1: Create `oauth_client.rs` module
  - [ ] 3.2: Create `OAuthProvider` trait for provider abstraction
  - [ ] 3.3: Implement Okta provider
  - [ ] 3.4: Implement Azure AD provider
  - [ ] 3.5: Implement Google provider
  - [ ] 3.6: Implement GitHub provider
  - [ ] 3.7: Add unit tests for each provider

- [ ] Task 4: Create OAuth authorization endpoints (AC: #1, #2)
  - [ ] 4.1: Add `GET /api/v1/auth/oauth/{provider}/authorize` route
  - [ ] 4.2: Generate and store PKCE code_verifier and state
  - [ ] 4.3: Build authorization URL with client_id, redirect_uri, code_challenge
  - [ ] 4.4: Redirect to provider authorization endpoint
  - [ ] 4.5: Set HTTP-only, Secure cookie for code_verifier

- [ ] Task 5: Implement OAuth callback handler (AC: #2, #3, #7)
  - [ ] 5.1: Add `GET /api/v1/auth/oauth/{provider}/callback` route
  - [ ] 5.2: Validate state parameter against stored value
  - [ ] 5.3: Extract authorization code from query params
  - [ ] 5.4: Retrieve code_verifier from cookie
  - [ ] 5.5: Exchange code for tokens using PKCE
  - [ ] 5.6: Fetch user info from OIDC userinfo endpoint
  - [ ] 5.7: Create or update user account
  - [ ] 5.8: Generate JWT tokens for application
  - [ ] 5.9: Store OAuth tokens encrypted
  - [ ] 5.10: Redirect to frontend with JWT tokens

- [ ] Task 6: Implement token refresh for OAuth providers (AC: #5, #6)
  - [ ] 6.1: Create `refresh_oauth_token()` function
  - [ ] 6.2: Use stored refresh_token to get new access_token
  - [ ] 6.3: Update encrypted tokens in database
  - [ ] 6.4: Handle refresh token expiry
  - [ ] 6.5: Add error handling for failed refreshes

- [ ] Task 7: Implement account linking (AC: #3, #9)
  - [ ] 7.1: Create `link_provider()` endpoint
  - [ ] 7.2: Verify user is authenticated with JWT
  - [ ] 7.3: Initiate OAuth flow for new provider
  - [ ] 7.4: On callback, link to existing user account
  - [ ] 7.5: Update `oauth_accounts` table
  - [ ] 7.6: Notify user of successful linking

- [ ] Task 8: Create provider configuration API (AC: #10)
  - [ ] 8.1: Add `GET /api/admin/oauth-providers` endpoint
  - [ ] 8.2: Add `POST /api/admin/oauth-providers` endpoint
  - [ ] 8.3: Add `PUT /api/admin/oauth-providers/{id}` endpoint
  - [ ] 8.4: Add `DELETE /api/admin/oauth-providers/{id}` endpoint
  - [ ] 8.5: Encrypt client_secret before storage
  - [ ] 8.6: Implement validation of redirect URIs

- [ ] Task 9: Create role mapping system (AC: #3)
  - [ ] 9.1: Create `role_mapping_rules` table
  - [ ] 9.2: Add email_domain_pattern, role_id columns
  - [ ] 9.3: Implement `map_email_domain_to_role()` function
  - [ ] 9.4: Apply default role on user creation
  - [ ] 9.5: Allow admin to customize rules

- [ ] Task 10: Implement frontend OAuth UI (AC: #1, #4, #7)
  - [ ] 10.1: Create `OAuthProviderButton` component
  - [ ] 10.2: Create `OAuthLogin` page with provider list
  - [ ] 10.3: Handle OAuth redirect flow
  - [ ] 10.4: Handle OAuth callback and store tokens
  - [ ] 10.5: Show provider branding/icons
  - [ ] 10.6: Display loading state during OAuth flow
  - [ ] 10.7: Show error messages on failure

- [ ] Task 11: Create OAuth settings management UI (AC: #10)
  - [ ] 11.1: Create `OAuthSettings` page component
  - [ ] 11.2: List all configured providers
  - [ ] 11.3: Add/edit provider configuration form
  - [ ] 11.4: Toggle provider enabled/disabled
  - [ ] 11.5: Configure role mapping rules
  - [ ] 11.6: Test provider connection

- [ ] Task 12: Add comprehensive error handling and logging (AC: #7)
  - [ ] 12.1: Create OAuth error enum (OAuthError)
  - [ ] 12.2: Map provider errors to user-friendly messages
  - [ ] 12.3: Log all OAuth flow steps with tracing
  - [ ] 12.4: Track OAuth metrics (success rate, errors by type)
  - [ ] 12.5: Implement alerting for OAuth failures

## Dev Notes

### Architecture Alignment

This story implements **OAuth 2.0 + PKCE Authentication** per Epic 15 requirements:

- **Backend Location**: `crates/qa-pms-auth/src/oauth/`
- **Provider Trait**: `crates/qa-pms-auth/src/oauth/provider.rs`
- **Security**: PKCE-compliant, OIDC standard, CSRF protection via state
- **Token Storage**: Encrypted via `qa-pms-config`

### Technical Implementation Details

#### Dependencies to Add
```toml
# crates/qa-pms-auth/Cargo.toml
[dependencies]
oauth2 = "4.4"
base64 = "0.22"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
reqwest = { version = "0.12", features = ["json"] }
chrono = "0.4"
uuid = { version = "1.6", features = ["v4", "serde"] }
```

#### Database Schema
```sql
-- OAuth providers configuration
CREATE TABLE oauth_providers (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    provider_id TEXT NOT NULL UNIQUE, -- 'okta', 'azure', 'google', 'github'
    name TEXT NOT NULL,
    client_id TEXT NOT NULL,
    client_secret_encrypted TEXT NOT NULL,
    authorization_url TEXT NOT NULL,
    token_url TEXT NOT NULL,
    userinfo_url TEXT NOT NULL,
    scopes TEXT NOT NULL,
    redirect_uri TEXT NOT NULL,
    enabled BOOLEAN DEFAULT TRUE,
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW()
);

-- User OAuth account links
CREATE TABLE oauth_accounts (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id),
    provider_id TEXT NOT NULL REFERENCES oauth_providers(provider_id),
    provider_user_id TEXT NOT NULL, -- ID from provider
    access_token_encrypted TEXT,
    refresh_token_encrypted TEXT,
    token_expires_at TIMESTAMP,
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW(),
    UNIQUE(user_id, provider_id)
);

-- PKCE temporary storage
CREATE TABLE oauth_pkce_state (
    state TEXT PRIMARY KEY,
    provider_id TEXT NOT NULL,
    code_verifier TEXT NOT NULL,
    redirect_uri TEXT NOT NULL,
    expires_at TIMESTAMP DEFAULT NOW() + INTERVAL '10 minutes'
);

-- Role mapping rules
CREATE TABLE role_mapping_rules (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    email_domain_pattern TEXT NOT NULL,
    role_id UUID NOT NULL REFERENCES roles(id),
    created_at TIMESTAMP DEFAULT NOW()
);

-- Indexes
CREATE INDEX idx_oauth_providers_provider_id ON oauth_providers(provider_id);
CREATE INDEX idx_oauth_accounts_user_id ON oauth_accounts(user_id);
CREATE INDEX idx_oauth_accounts_provider_id ON oauth_accounts(provider_id);
CREATE INDEX idx_oauth_pkce_state_expires_at ON oauth_pkce_state(expires_at);
```

#### OAuth Provider Trait
```rust
use oauth2::{basic::BasicClient, AuthorizationRequest, CsrfToken, TokenResponse};
use reqwest::Client;
use serde::{Deserialize, Serialize};

#[async_trait]
pub trait OAuthProvider: Send + Sync {
    fn provider_id(&self) -> &'static str;
    fn provider_name(&self) -> &'static str;
    
    async fn build_authorization_url(
        &self,
        state: &str,
        code_challenge: &str,
        redirect_uri: &str,
    ) -> Result<String, OAuthError>;
    
    async fn exchange_code_for_tokens(
        &self,
        code: &str,
        code_verifier: &str,
        redirect_uri: &str,
    ) -> Result<TokenResponse, OAuthError>;
    
    async fn get_user_info(
        &self,
        access_token: &str,
    ) -> Result<OAuthUserInfo, OAuthError>;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OAuthUserInfo {
    pub id: String,
    pub email: String,
    pub name: Option<String>,
    pub picture: Option<String>,
}
```

#### PKCE Implementation
```rust
use rand::{thread_rng, Rng};
use sha2::{Sha256, Digest};
use base64::{Engine, engine::general_purpose::URL_SAFE_NO_PAD};

pub struct PkceState {
    pub code_verifier: String,
    pub code_challenge: String,
    pub state: String,
}

pub fn generate_pkce_state() -> Result<PkceState, anyhow::Error> {
    // Generate cryptographically secure random code verifier
    let mut rng = thread_rng();
    let code_verifier: String = (0..128)
        .map(|_| {
            const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-._~";
            CHARSET[rng.gen_range(0..CHARSET.len())] as char
        })
        .collect();
    
    // Generate code challenge (SHA256 hash, base64url encoded)
    let mut hasher = Sha256::new();
    hasher.update(code_verifier.as_bytes());
    let code_challenge = URL_SAFE_NO_PAD.encode(hasher.finalize());
    
    // Generate state for CSRF protection
    let state: String = (0..32)
        .map(|_| {
            const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
            CHARSET[rng.gen_range(0..CHARSET.len())] as char
        })
        .collect();
    
    Ok(PkceState {
        code_verifier,
        code_challenge,
        state,
    })
}

pub fn generate_state() -> String {
    let mut rng = thread_rng();
    (0..32)
        .map(|_| {
            const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
            CHARSET[rng.gen_range(0..CHARSET.len())] as char
        })
        .collect()
}
```

#### Authorization Endpoint
```rust
use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::Redirect,
    Json, Request,
};
use tower_cookies::{Cookie, CookieManagerLayer, Cookies};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct AuthorizeQuery {
    provider: String,
}

#[utoipa::path(
    path = "/api/v1/auth/oauth/{provider}/authorize",
    params(
        ("provider", description = "OAuth provider (okta, azure, google, github)"),
    ),
    responses(
        (status = 302, description = "Redirect to OAuth provider"),
    ),
    tag = "OAuth"
)]
pub async fn oauth_authorize(
    Query(query): Query<AuthorizeQuery>,
    State(oauth_service): State<Arc<OAuthService>>,
    cookies: Cookies,
) -> Result<Redirect, ApiError> {
    // Get provider configuration
    let provider = oauth_service.get_provider(&query.provider)?;
    
    // Generate PKCE state
    let pkce_state = generate_pkce_state()?;
    
    // Store PKCE state temporarily
    oauth_service.store_pkce_state(&pkce_state.state, &pkce_state.code_verifier).await?;
    
    // Build authorization URL
    let redirect_uri = format!(
        "http://localhost:3000/api/v1/auth/oauth/{}/callback",
        query.provider
    );
    
    let auth_url = provider
        .build_authorization_url(&pkce_state.state, &pkce_state.code_challenge, &redirect_uri)
        .await?;
    
    // Set code_verifier cookie (HTTP-only, Secure)
    let cookie = Cookie::build(("pkce_verifier", pkce_state.code_verifier))
        .http_only(true)
        .secure(true)
        .same_site(tower_cookies::cookie::SameSite::Lax);
    
    cookies.add(cookie);
    
    Ok(Redirect::to(&auth_url))
}
```

#### Callback Handler
```rust
use serde::Deserialize;

#[derive(Deserialize)]
pub struct CallbackQuery {
    code: String,
    state: String,
    error: Option<String>,
    error_description: Option<String>,
}

#[utoipa::path(
    path = "/api/v1/auth/oauth/{provider}/callback",
    params(
        ("provider", description = "OAuth provider"),
        ("code", description = "Authorization code from provider"),
        ("state", description = "State parameter for CSRF protection"),
    ),
    responses(
        (status = 302, description = "Redirect to frontend with tokens"),
        (status = 400, description = "OAuth error or invalid state"),
    ),
    tag = "OAuth"
)]
pub async fn oauth_callback(
    Path(provider): Path<String>,
    Query(query): Query<CallbackQuery>,
    State(oauth_service): State<Arc<OAuthService>>,
    State(auth_service): State<Arc<AuthService>>,
    cookies: Cookies,
) -> Result<Redirect, ApiError> {
    // Check for OAuth error
    if let Some(error) = query.error {
        let message = query.error_description.as_deref().unwrap_or(&error);
        let error_url = format!(
            "http://localhost:5173/auth/error?error={}&description={}",
            urlencoding::encode(&error),
            urlencoding::encode(&message)
        );
        return Ok(Redirect::to(&error_url));
    }
    
    // Validate state and retrieve code_verifier
    let pkce_data = oauth_service
        .validate_and_consume_pkce_state(&query.state)
        .await?;
    
    // Get provider
    let provider = oauth_service.get_provider(&provider)?;
    
    // Exchange authorization code for tokens
    let redirect_uri = format!(
        "http://localhost:3000/api/v1/auth/oauth/{}/callback",
        provider
    );
    let token_response = provider
        .exchange_code_for_tokens(&query.code, &pkce_data.code_verifier, &redirect_uri)
        .await?;
    
    // Get user info from OIDC userinfo endpoint
    let user_info = provider
        .get_user_info(&token_response.access_token())
        .await?;
    
    // Create or update user account
    let user = oauth_service
        .find_or_create_user(&user_info, &provider)
        .await?;
    
    // Generate JWT tokens for our application
    let (access_token, refresh_token) = auth_service
        .generate_tokens_for_user(&user.id)
        .await?;
    
    // Store OAuth tokens
    oauth_service
        .store_oauth_tokens(&user.id, &provider, &token_response)
        .await?;
    
    // Clear PKCE cookie
    let mut clear_cookie = Cookie::from(("pkce_verifier", ""));
    clear_cookie.set_max_age(time::Duration::ZERO);
    cookies.add(clear_cookie);
    
    // Redirect to frontend with JWT tokens
    let redirect_url = format!(
        "http://localhost:5173/auth/callback?access_token={}&refresh_token={}",
        urlencoding::encode(&access_token),
        urlencoding::encode(&refresh_token)
    );
    
    Ok(Redirect::to(&redirect_url))
}
```

#### Role Mapping
```rust
use regex::Regex;

pub async fn map_email_domain_to_role(
    email: &str,
    pool: &PgPool,
) -> Result<String, ApiError> {
    // Extract email domain
    let domain = email
        .split('@')
        .nth(1)
        .ok_or_else(|| anyhow!("Invalid email"))?;
    
    // Check for matching rule
    let query = sqlx::query!(
        r#"
        SELECT r.name 
        FROM role_mapping_rules rmr
        JOIN roles r ON r.id = rmr.role_id
        WHERE $1 ~ rmr.email_domain_pattern
        ORDER BY LENGTH(rmr.email_domain_pattern) DESC
        LIMIT 1
        "#,
        domain
    );
    
    if let Some(result) = query.fetch_one(pool).await? {
        Ok(result.name)
    } else {
        // Default role
        Ok("qa_engineer".to_string())
    }
}
```

### Testing Strategy

#### Unit Tests

- PKCE state generation
- Code challenge generation
- OAuth provider abstraction
- Role mapping logic
- Token encryption/decryption

#### Integration Tests

- Complete OAuth authorization flow
- OAuth callback with valid code
- OAuth callback with error
- Token exchange
- User creation from OAuth
- Account linking

#### End-to-End Tests

- Login via Okta
- Login via Google
- Login via Azure AD
- Login via GitHub
- Account linking flow
- Logout OAuth session

#### Security Tests

- CSRF protection (state validation)
- PKCE flow correctness
- Token storage encryption
- Session management
- Provider switching

### File List

**Files to be created:**
- `crates/qa-pms-auth/src/oauth/mod.rs`
- `crates/qa-pms-auth/src/oauth/provider.rs`
- `crates/qa-pms-auth/src/oauth/pkce.rs`
- `crates/qa-pms-auth/src/oauth/okta.rs`
- `crates/qa-pms-auth/src/oauth/azure.rs`
- `crates/qa-pms-auth/src/oauth/google.rs`
- `crates/qa-pms-auth/src/oauth/github.rs`
- `migrations/create_oauth_providers_table.sql`
- `migrations/create_oauth_accounts_table.sql`
- `migrations/create_oauth_pkce_state_table.sql`
- `migrations/create_role_mapping_rules_table.sql`
- `frontend/src/components/auth/OAuthProviderButton.tsx`
- `frontend/src/pages/auth/OAuthLogin.tsx`
- `frontend/src/pages/admin/OAuthSettings.tsx`

**Files to be modified:**
- `crates/qa-pms-auth/Cargo.toml` (add oauth2 dependency)
- `crates/qa-pms-api/src/main.rs` (add OAuth routes)
- `frontend/src/api/auth.ts` (add OAuth methods)
- `frontend/src/stores/authStore.ts` (handle OAuth auth)