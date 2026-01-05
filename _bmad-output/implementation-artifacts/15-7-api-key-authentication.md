# Story 15.7: API Key Authentication

Status: ready-for-dev

## Story

As an Enterprise QA Lead or DevOps Engineer,
I want to use API keys for automated scripts and CI/CD pipelines without user authentication,
So that I can integrate the platform with automation tools and external systems.

## Acceptance Criteria

1. **Given** an admin user wants to create a service account
   **When** they configure service account details
   **Then** system generates API key
   **And** shows it only once
   **And** marks as service account type

2. **Given** a service account needs to be created
   **When** admin submits form
   **Then** system creates user record with service_account role
   **And** generates API key
   **And** stores key hash
   **And** returns key

3. **Given** an API key is issued
   **When** key is used for authentication
   **Then** system validates key hash
   **And** checks if key is active
   **And** returns user context without password/OAuth

4. **Given** an API key is used
   **When** making API request
   **Then** system validates key via Bearer header
   **And** returns user context
   **And** checks key permissions

5. **Given** an admin wants to manage API keys
   **When** they access service accounts
   **Then** system shows list of all API keys
   **And** allows regenerating keys
   **And** allows revoking keys

6. **Given** an API key needs to be revoked
   **When** admin deletes or revokes key
   **Then** system marks key as inactive
   **And** prevents further access
   **And** logs revocation

7. **Given** API key needs to have scope limitations
   **When** key is created
   **Then** system can assign specific permissions (api.read, api.write)
   **And** limits key usage with rate limits

8. **Given** an API key needs to be trackable
   **When** key is used
   **Then** system tracks last used timestamp
   **And** tracks request count
   **And** maintains usage history

9. **Given** an API key is compromised
   **When** admin rotates keys
   **Then** system can generate new key
   **And** invalidate old key
   **And** maintain continuity

10. **Given** CI/CD needs integration
   **When** pipeline is configured
   **Then** system provides key as environment variable
   **Or** secret management integration

## Tasks / Subtasks

- [ ] Task 1: Setup API key database schema (AC: #2, #3)
  - [ ] 1.1: Create `api_keys` table migration
  - [ ] 1.2: Add columns: id, service_account_id, key_hash, key_prefix, permissions, is_active, created_at, last_used_at, expires_at
  - [ ] 1.3: Add indexes on service_account_id, key_hash, is_active
  - [ ] 1.4: Create migration file
  - [ ] 1.5: Add unit tests for schema changes

- [ ] Task 2: Implement API key generation utilities (AC: #1, #2)
  - [ ] 2.1: Create `api_key_generator.rs` module in `qa-pms-auth`
  - [ ] 2.2: Implement `generate_api_key()` function
  - [ ] 2.3: Generate cryptographically secure 32-byte key
  - [ ] 2.4: Generate key prefix: `qapms_sk_`
  - [ ] 2.5: Hash key using SHA-256
  - [ ] 2.6: Store only hash in database (never store plain key)
  - [ ] 2.7: Return plain key only once (on creation)
  - [ ] 2.8: Add unit tests for key generation

- [ ] Task 3: Create service account user type (AC: #2)
  - [ ] 3.1: Add `service_account` enum to User roles
  - [ ] 3.2: Update user table to support service_account type
  - [ ] 3.3: Add `is_service_account` boolean column
  - [ ] 3.4: Implement service account validation logic
  - [ ] 3.5: Add migration to update users table
  - [ ] 3.6: Add unit tests for service account logic

- [ ] Task 4: Implement API key authentication middleware (AC: #3, #4)
  - [ ] 4.1: Create `api_key_middleware.rs` in `qa-pms-auth/middleware/`
  - [ ] 4.2: Extract API key from Authorization header (Bearer: qapms_sk_...)
  - [ ] 4.3: Lookup key hash in database
  - [ ] 4.4: Check if key is active and not expired
  - [ ] 4.5: Retrieve associated service_account and user
  - [ ] 4.6: Validate key permissions against required permissions
  - [ ] 4.7: Add user context with service_account role to request extensions
  - [ ] 4.8: Handle invalid/revoked/inactive keys with 401
  - [ ] 4.9: Track key usage (update last_used_at, increment request count)
  - [ ] 4.10: Add unit tests for middleware logic

- [ ] Task 5: Create service account management API (AC: #5, #6, #9)
  - [ ] 5.1: Add `POST /api/v1/admin/service-accounts` endpoint
  - [ ] 5.2: Implement create service account with name, description, permissions
  - [ ] 5.3: Generate API key and return it (only time)
  - [ ] 5.4: Add `GET /api/v1/admin/service-accounts` endpoint
  - [ ] 5.5: Implement pagination support
  - [ ] 5.6: Add `PUT /api/v1/admin/service-accounts/{id}` endpoint
  - [ ] 5.7: Implement name/description/permissions update
  - [ ] 5.8: Add `DELETE /api/v1/admin/service-accounts/{id}` endpoint
  - [ ] 5.9: Implement delete with cascade (delete API keys too)
  - [ ] 5.10: Add `POST /api/v1/admin/service-accounts/{id}/regenerate-key` endpoint
  - [ ] 5.11: Generate new key, invalidate old key, return new key (only once)
  - [ ] 5.12: Add `POST /api/v1/admin/service-accounts/{id}/revoke-key` endpoint
  - [ ] 5.13: Mark key as inactive, prevent further access
  - [ ] 5.14: Add OpenAPI documentation with utoipa
  - [ ] 5.15: Protect all endpoints with admin permission

- [ ] Task 6: Implement API key usage tracking (AC: #8)
  - [ ] 6.1: Create `ApiKeyTracker` service in `qa-pms-auth`
  - [ ] 6.2: Implement `track_key_usage(key_hash)` function
  - [ ] 6.3: Update last_used_at timestamp on successful auth
  - [ ] 6.4: Increment request_count counter
  - [ ] 6.5: Add rate limiting check (key-level limits)
  - [ ] 6.6: Store usage metrics in `api_keys_usage` table
  - [ ] 6.7: Implement `get_key_usage_stats(key_hash)` for admin dashboard
  - [ ] 6.8: Add unit tests for tracking logic

- [ ] Task 7: Create API key management UI (AC: #5, #6)
  - [ ] 7.1: Create `ServiceAccounts` page component
  - [ ] 7.2: Create `ServiceAccountForm` for creating/editing accounts
  - [ ] 7.3: Implement API key display (masked: qapms_sk_...XxxX)
  - [ ] 7.4: Add "Copy Key" button (copies to clipboard)
  - [ ] 7.5: Add "Regenerate Key" button with confirmation
  - [ ] 7.6: Add "Revoke Key" button with confirmation
  - [ ] 7.7: Create `ApiKeyCard` component showing usage stats
  - [ ] 7.8: Show last used timestamp, request count
  - [ ] 7.9: Implement permissions selector (checkboxes for api.read, api.write)
  - [ ] 7.10: Add loading states for actions
  - [ ] 7.11: Add success/error toasts

- [ ] Task 8: Implement rate limiting per API key (AC: #7)
  - [ ] 8.1: Extend rate limiter to support key-based limits
  - [ ] 8.2: Implement `ApiKeyGovernor` with custom key limit
  - [ ] 8.3: Configure different limits per permission level
  - [ ] 8.4: api.read: 1000 req/hour, api.write: 500 req/hour
  - [ ] 8.5: Track usage per key for rate limiting
  - [ ] 8.6: Return 429 with Retry-After header when limit exceeded
  - [ ] 8.7: Add unit tests for key-level rate limiting

- [ ] Task 9: Add API key rotation support (AC: #9)
  - [ ] 9.1: Implement `rotate_api_key()` function in ServiceAccountService
  - [ ] 9.2: Generate new key, invalidate old key
  - [ ] 9.3: Update database with new key hash
  - [ ] 9.4: Maintain mapping from old key to new key during rotation period
  - [ ] 9.5: Allow old key to work during grace period (e.g., 5 minutes)
  - [ ] 9.6: After grace period, old key is permanently invalid
  - [ ] 9.7: Log rotation event with old and new key IDs
  - [ ] 9.8: Add unit tests for rotation logic

- [ ] Task 10: Implement CI/CD integration helpers (AC: #10)
  - [ ] 10.1: Create `generate_env_file()` function
  - [ ] 10.2: Generate .env file with QA_PMS_API_KEY variable
  - [ ] 10.3: Add option for GitHub Actions (secrets)
  - [ ] 10.4: Add option for GitLab CI (variables)
  - [ ] 10.5: Add option for CircleCI (contexts)
  - [ ] 10.6: Create CLI command: `qapms service-account env-file`
  - [ ] 10.7: Implement `export_github_secrets()` command
  - [ ] 10.8: Add unit tests for CI/CD helpers

- [ ] Task 11: Add comprehensive error handling and logging (AC: Todas)
  - [ ] 11.1: Create `ApiKeyError` enum (InvalidKey, KeyNotFound, ExpiredKey, RateLimitExceeded, PermissionDenied)
  - [ ] 11.2: Map all API key errors to user-friendly messages
  - [ ] 11.3: Log all API key authentication attempts (success/failure)
  - [ ] 11.4: Log key generation, rotation, revocation events
  - [ ] 11.5: Include service_account_id, key_hash, IP address in logs
  - [ ] 11.6: Add structured logging with tracing crate
  - [ ] 11.7: Add security alerting for suspicious patterns
  - [ ] 11.8: Add unit tests for error handling

- [ ] Task 12: Create frontend API key components (AC: #5, #6)
  - [ ] 12.1: Create `useApiKeys` hook for state management
  - [ ] 12.2: Implement `fetchServiceAccounts()` function
  - [ ] 12.3: Implement `createServiceAccount()` function
  - [ ] 12.4: Implement `regenerateApiKey()` function
  - [ ] 12.5: Implement `revokeApiKey()` function
  - [ ] 12.6: Add error handling and toasts for all operations
  - [ ] 12.7: Create `ApiKeyDisplay` component
  - [ ] 12.8: Implement masked key display with show/hide toggle
  - [ ] 12.9: Add key usage visualization (requests over time)
  - [ ] 12.10: Add permissions management UI
  - [ ] 12.11: Add loading states and optimistic updates

## Dev Notes

### Architecture Alignment

This story implements **API Key Authentication** per Epic 15 requirements:

- **Backend Location**: `crates/qa-pms-auth/src/api_keys/`
- **Middleware**: `crates/qa-pms-auth/src/middleware/api_key_middleware.rs`
- **Security**: SHA-256 key hashing, key stored only as hash, Bearer token authentication, key-level rate limiting
- **Performance**: Indexed queries on key_hash, usage tracking, rate limiting with Redis/in-memory cache

### Technical Implementation Details

#### Dependencies to Add

```toml
# crates/qa-pms-auth/Cargo.toml
[dependencies]
# Existing
sqlx = { version = "0.7", features = ["postgres", "runtime-tokio"] }
serde = { version = "1.0", features = ["derive"] }
tokio = { version = "1.35", features = ["full"] }
axum = "0.7"

# New for API keys
uuid = { version = "1.6", features = ["v4", "serde"] }
sha2 = "0.10"
rand = "0.8"
# For key hash comparison
hex = "0.4"
```

#### Database Schema

```sql
-- Users table update (from Task 3)
ALTER TABLE users ADD COLUMN is_service_account BOOLEAN DEFAULT FALSE;

-- API keys table
CREATE TABLE api_keys (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    service_account_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    key_hash TEXT NOT NULL UNIQUE,
    key_prefix TEXT NOT NULL, -- 'qapms_sk_'
    key_secret_encrypted TEXT NOT NULL, -- Encrypted plain key for future reference
    permissions TEXT[] NOT NULL, -- ['api.read', 'api.write']
    is_active BOOLEAN DEFAULT TRUE,
    created_at TIMESTAMP DEFAULT NOW(),
    last_used_at TIMESTAMP,
    expires_at TIMESTAMP DEFAULT NULL, -- Optional key expiration
    revoked_at TIMESTAMP DEFAULT NULL,
    revoked_reason TEXT,
    request_count INTEGER DEFAULT 0
    rate_limit_window_start TIMESTAMP DEFAULT NOW()
);

-- API keys usage tracking table
CREATE TABLE api_keys_usage (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    api_key_id UUID NOT NULL REFERENCES api_keys(id) ON DELETE CASCADE,
    endpoint TEXT NOT NULL,
    method TEXT NOT NULL, -- GET, POST, PUT, DELETE, etc.
    status_code INTEGER,
    response_time_ms INTEGER,
    ip_address INET,
    user_agent TEXT,
    created_at TIMESTAMP DEFAULT NOW()
);

-- Indexes for performance
CREATE INDEX idx_api_keys_service_account_id ON api_keys(service_account_id);
CREATE INDEX idx_api_keys_key_hash ON api_keys(key_hash);
CREATE INDEX idx_api_keys_is_active ON api_keys(is_active) WHERE is_active = TRUE;
CREATE INDEX idx_api_keys_usage_api_key_id ON api_keys_usage(api_key_id);
CREATE INDEX idx_api_keys_usage_created_at ON api_keys_usage(created_at);
```

#### API Key Generation Service

```rust
use rand::{thread_rng, Rng};
use sha2::{Sha256, Digest};
use base64::{Engine, engine::general_purpose::URL_SAFE_NO_PAD};
use sqlx::PgPool;

pub struct ApiKeyGenerator {
    db: Arc<PgPool>,
}

impl ApiKeyGenerator {
    pub fn new(db: Arc<PgPool>) -> Self {
        Self { db }
    }
    
    /// Generate a new API key for a service account
    pub async fn generate_api_key(
        &self,
        service_account_id: &Uuid,
        permissions: Vec<String>,
    ) -> Result<ApiKey, ApiKeyError> {
        // Generate 32-byte cryptographically secure key
        let mut rng = thread_rng();
        let key_bytes: Vec<u8> = (0..32)
            .map(|_| {
                const CHARSET: &[u8] = b"abcdefghijklmnopqrstuvwxyz0123456789";
                CHARSET[rng.gen_range(0..CHARSET.len())] as u8
            })
            .collect();
        
        let key = String::from_utf8(&key_bytes).unwrap();
        
        // Hash key for database storage (SHA-256)
        let mut hasher = Sha256::new();
        hasher.update(key.as_bytes());
        let key_hash = hex::encode(hasher.finalize());
        
        // Generate prefix: qapms_sk_ + first 8 chars of key (for identification)
        let key_prefix = format!("qapms_sk_{}", &key[..8]);
        
        // Store in database (only hash)
        let expires_at = if permissions.contains(&"api.write".to_string()) {
            Some(chrono::Utc::now() + chrono::Duration::days(90)) // Write keys expire in 90 days
        } else {
            None // Read keys don't expire
        };
        
        let api_key_id = sqlx::query!(
            r#"
            INSERT INTO api_keys 
                (service_account_id, key_hash, key_prefix, key_secret_encrypted, 
                 permissions, is_active, expires_at, created_at)
            VALUES ($1, $2, $3, $4, $5, $6, TRUE, $7, NOW())
            RETURNING id, key_prefix, created_at
            "#
        )
        .bind(
            service_account_id,
            &key_hash,
            &key_prefix,
            &encryption_service.encrypt(&key)?, // Encrypt plain key for future reference
            &serde_json::to_string(&permissions),
            expires_at
        )
        .fetch_one(&*self.db)
        .await?
        .ok_or(ApiKeyError::DatabaseError("Failed to create API key"))?;
        
        Ok(ApiKey {
            id: api_key_id,
            key_hash,
            key_prefix,
            key, // Plain key returned ONLY ONCE
            permissions,
            is_active: true,
            created_at: api_key_id.created_at,
        })
    }
    
    /// Validate an API key hash
    pub async fn validate_api_key(
        &self,
        key_hash: &str,
    ) -> Result<bool, ApiKeyError> {
        let result = sqlx::query!(
            r#"
            SELECT id, is_active, expires_at, revoked_at 
            FROM api_keys 
            WHERE key_hash = $1
            "#
        )
        .bind(key_hash)
        .fetch_optional(&*self.db)
        .await?;
        
        match result {
            Some(api_key) => {
                // Check if key is revoked
                if api_key.revoked_at.is_some() {
                    return Ok(false);
                }
                
                // Check if key is inactive
                if !api_key.is_active {
                    return Ok(false);
                }
                
                // Check if key is expired
                if let Some(expires_at) = api_key.expires_at {
                    if expires_at < chrono::Utc::now() {
                        return Ok(false);
                    }
                }
                
                // Key is valid
                Ok(true)
            }
            None => Ok(false),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ApiKey {
    pub id: Uuid,
    pub key_hash: String,
    pub key_prefix: String,
    pub key: String,
    pub permissions: Vec<String>,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
}
```

#### API Key Authentication Middleware

```rust
use axum::{
    extract::{Request, State},
    http::{HeaderMap, StatusCode},
    middleware::Next,
    response::Response,
    Extension,
};
use sqlx::PgPool;

#[derive(Debug)]
pub struct ApiKeyContext {
    pub user_id: Uuid,
    pub service_account_id: Uuid,
    pub api_key_id: Uuid,
    pub permissions: Vec<String>,
    pub key_prefix: String,
}

pub async fn api_key_middleware<B>(
    State(db): State<Arc<PgPool>>,
    State(rate_limiter): State<Arc<GovernorConfig>>,
    mut req: Request<B>,
    next: Next<B>,
) -> Result<Response, StatusCode> {
    // Extract API key from Authorization header
    let auth_header = req.headers()
        .get("authorization")
        .and_then(|h| h.to_str().ok())
        .and_then(|h| h.strip_prefix("Bearer "))
        .ok_or(ApiKeyError::MissingApiKey)?;
    
    // Extract key hash (key is in format: qapms_sk_ABCDEFGH)
    let key = match auth_header {
        Some(key) => key,
        None => return Err(StatusCode::UNAUTHORIZED),
    };
    
    // Parse prefix and key parts
    let parts: Vec<&str> = key.split('_').collect();
    if parts.len() != 2 || !parts[0].starts_with("qapms_sk_") {
        return Err(StatusCode::UNAUTHORIZED);
    }
    
    let key_hash = format!("sha256:{}", key);
    
    // Validate API key
    let is_valid = validate_api_key(&db, &key_hash).await?;
    if !is_valid {
        log_api_key_access_denied(&key_hash, req).await;
        return Err(StatusCode::UNAUTHORIZED);
    }
    
    // Check rate limiting
    let rate_limit_key = format!("api_key:{}", key_hash);
    if !rate_limiter.check(&rate_limit_key).await {
        log_api_key_rate_limit_exceeded(&key_hash, req).await;
        return Err(StatusCode::TOO_MANY_REQUESTS);
    }
    
    // Lookup API key details from database
    let api_key = sqlx::query!(
        r#"
            SELECT ak.id, ak.service_account_id, ak.key_prefix, ak.permissions, ak.request_count, ak.last_used_at,
                   u.id, u.email
            FROM api_keys ak
            JOIN users u ON ak.service_account_id = u.id
            WHERE ak.key_hash = $1 AND ak.is_active = TRUE AND ak.revoked_at IS NULL
            "#
        )
        .bind(&key_hash)
        .fetch_optional(&*db)
        .await;
    
    let api_key = match api_key {
        Some(key) => key,
        None => {
            log_api_key_not_found(&key_hash, req).await;
            return Err(StatusCode::UNAUTHORIZED);
        }
    };
    
    // Track usage (update last_used_at, increment request_count)
    sqlx::query!(
        r#"
            UPDATE api_keys
            SET last_used_at = NOW(), request_count = request_count + 1
            WHERE id = $1
            "#
        )
        .bind(api_key.id)
        .execute(&*db)
        .await
        .ok_or_else(|e| {
            error!("Failed to track API key usage: {}", e);
        })?;
    
    // Store usage in tracking table
    sqlx::query!(
        r#"
            INSERT INTO api_keys_usage (api_key_id, endpoint, method, status_code, ip_address, user_agent)
            VALUES ($1, $2, $3, 200, $4, $5)
            "#
        )
        .bind(
            api_key.id,
            req.uri().path(),
            req.method().to_string(),
            req.uri().path(),
            extract_client_ip(&req),
            extract_user_agent(&req),
        )
        .execute(&*db)
        .await
        .ok_or_else(|e| {
            error!("Failed to log API key usage: {}", e);
        })?;
    
    // Add user context to request extensions
    let context = ApiKeyContext {
        user_id: api_key.user_id,
        service_account_id: api_key.service_account_id,
        api_key_id: api_key.id,
        permissions: api_key.permissions,
        key_prefix: api_key.key_prefix,
    };
    
    req.extensions_mut().insert(context);
    
    Ok(next.run(req).await)
}

// Helper functions
fn extract_client_ip(req: &Request) -> String {
    req.headers()
        .get("x-forwarded-for")
        .or_else(|| {
            req.headers()
                .get("x-real-ip")
                .or_else(|| {
                    req.connect_info()
                        .map(|info| info.ipaddr().to_string())
                        .unwrap_or_else(|| "unknown".to_string())
                })
        })
        })
        .unwrap_or_else(|| "unknown".to_string())
}

fn extract_user_agent(req: &Request) -> String {
    req.headers()
        .get("user-agent")
        .unwrap_or_else(|| "unknown".to_string())
}

async fn log_api_key_access_denied(key_hash: &str, req: &Request) {
    info!(
        api_key_hash = %key_hash,
        ip_address = %extract_client_ip(req),
        user_agent = %extract_user_agent(req),
        endpoint = %req.uri().path(),
        "API key authentication failed"
    );
}

async fn log_api_key_not_found(key_hash: &str, req: &Request) {
    warn!(
        api_key_hash = %key_hash,
        ip_address = %extract_client_ip(req),
        user_agent = %extract_user_agent(req),
        endpoint = %req.uri().path(),
        "API key not found or inactive"
    );
}

async fn log_api_key_rate_limit_exceeded(key_hash: &str, req: &Request) {
    warn!(
        api_key_hash = %key_hash,
        ip_address = %extract_client_ip(req),
        user_agent = %extract_user_agent(req),
        endpoint = %req.uri().path(),
        "API key rate limit exceeded"
    );
}
```

#### Service Account Management API

```rust
use axum::{
    extract::{Json, Path, State},
    http::StatusCode,
    response::{IntoResponse, Json},
};
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Deserialize)]
pub struct CreateServiceAccountRequest {
    #[validate(length(min = 1, max = 100))]
    pub name: String,
    
    #[validate(length(min = 0, max = 500))]
    pub description: String,
    
    #[serde(default)]
    pub permissions: Vec<String>,
    
    #[serde(default = "api.read")]
    pub default_permission: String,
}

#[utoipa::path(
    request_body = CreateServiceAccountRequest,
    responses(
        (status = 200, description = "Service account created", body = CreateServiceAccountResponse),
        (status = 400, description = "Validation errors", body = ApiError),
        (status = 401, description = "Unauthorized"),
    ),
    tag = "Service Accounts"
)]
pub async fn create_service_account(
    State(auth_service): State<Arc<AuthService>>,
    State(api_key_service): State<Arc<ApiKeyService>>,
    Json(request): Json<CreateServiceAccountRequest>,
) -> Result<Json<CreateServiceAccountResponse>, ApiError> {
    // Validate admin permission
    let user_context = req.extensions().get::<UserContext>().unwrap();
    if !user_context.permissions.contains(&Permission::UsersManage) {
        return Err(ApiError::Forbidden);
    }
    
    // Set default permissions
    let permissions = if request.permissions.is_empty() {
        vec![request.default_permission]
    } else {
        request.permissions
    };
    
    // Validate permissions
    for perm in &permissions {
        if perm != "api.read" && perm != "api.write" {
            return Err(ApiError::Validation("Invalid permission. Only 'api.read' and 'api.write' allowed".to_string()));
        }
    }
    
    // Create user with service_account role
    let user_id = auth_service.create_user_with_role(
        "api_key",
        &format!("{}@service.qapms.com", Uuid::new_v4()), // Generate unique email
        &format!("{} Service Account", &request.name),
    ).await?;
    
    // Generate API key
    let api_key = api_key_service.generate_api_key(
        &user_id,
        permissions,
    ).await?;
    
    Ok(Json(CreateServiceAccountResponse {
        service_account_id: user_id,
        api_key: api_key.key,
        key_prefix: api_key.key_prefix,
        permissions: api_key.permissions,
        created_at: api_key.created_at,
    }))
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateServiceAccountResponse {
    pub service_account_id: Uuid,
    pub api_key: String,
    pub key_prefix: String,
    pub permissions: Vec<String>,
    pub created_at: DateTime<Utc>,
}

#[utoipa::path(
    path = "/admin/service-accounts/{id}",
    params(
        ("id", description = "Service account ID"),
    ),
    responses(
        (status = 200, description = "Service account updated", body = ServiceAccount),
        (status = 404, description = "Not found"),
        (status = 401, description = "Unauthorized"),
    ),
    tag = "Service Accounts"
)]
pub async fn get_service_account(
    State(db): State<Arc<PgPool>>,
    Path(id): Path<Uuid>,
    Json(request): Json<GetServiceAccountRequest>,
) -> Result<Json<ServiceAccount>, ApiError> {
    // Validate admin permission
    let user_context = req.extensions().get::<UserContext>().unwrap();
    if !user_context.permissions.contains(&Permission::UsersView) {
        return Err(ApiError::Forbidden);
    }
    
    // Fetch service account
    let service_account = sqlx::query!(
        r#"
            SELECT u.id, u.email, u.created_at, u.is_service_account,
                   ak.id, ak.key_prefix, ak.permissions, ak.is_active, ak.created_at,
                   ak.last_used_at, ak.request_count
            FROM users u
            JOIN api_keys ak ON ak.service_account_id = u.id
            WHERE u.id = $1
            "#
        )
        .bind(id)
        .fetch_optional(&*db)
        .await?
        .ok_or(ApiError::NotFound)?
    .map(|(user, api_key)| ServiceAccount {
            id: user.id,
            email: user.email,
            name: user.email.split('@').next().unwrap_or_default(&"Service Account"),
            created_at: user.created_at,
            api_key: Some(api_key),
        })
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServiceAccount {
    pub id: Uuid,
    pub email: String,
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub api_key: Option<ApiKey>,
}

#[utoipa::path(
    path = "/admin/service-accounts/{id}/regenerate-key",
    params(
        ("id", description = "Service account ID"),
    ),
    responses(
        (status = 200, description = "New API key generated", body = RegenerateKeyResponse),
        (status = 404, description = "Not found"),
        (status = 401, description = "Unauthorized"),
    ),
    tag = "Service Accounts"
)]
pub async fn regenerate_api_key(
    State(db): State<Arc<PgPool>>,
    State(api_key_service): State<Arc<ApiKeyService>>,
    Path(id): Path<Uuid>,
) -> Result<Json<RegenerateKeyResponse>, ApiError> {
    // Validate admin permission
    let user_context = req.extensions().get::<UserContext>().unwrap();
    if !user_context.permissions.contains(&Permission::UsersManage) {
        return Err(ApiError::Forbidden);
    }
    
    // Fetch current API key
    let current_key = sqlx::query!(
        r#"
            SELECT id, key_hash, key_prefix, permissions
            FROM api_keys
            WHERE service_account_id = $1 AND is_active = TRUE
            LIMIT 1
            "#
        )
        .bind(id)
        .fetch_optional(&*db)
        .await?
        .ok_or(ApiError::NotFound)?
        .ok_or(ApiKeyError::NoActiveKey)?
        .ok_or(ApiKeyError::AlreadyHasActiveKey)?;
    
    // Generate new API key
    let new_key = api_key_service.generate_api_key(
        &current_key.service_account_id,
        current_key.permissions.clone(),
    ).await?;
    
    // Invalidate old key
    sqlx::query!(
        r#"
            UPDATE api_keys
            SET is_active = FALSE, revoked_at = NOW(), revoked_reason = 'rotated'
            WHERE id = $1
            "#
        )
        .bind(current_key.id)
        .execute(&*db)
        .await
        .ok_or_else(|e| {
            error!("Failed to invalidate old API key: {}", e);
        })?;
    
    // Log rotation event
    info!(
        old_key_id = %current_key.id,
        new_key_id = %new_key.id,
        service_account_id = %id,
        "API key rotated"
    );
    
    Ok(Json(RegenerateKeyResponse {
        api_key: new_key.key,
        key_prefix: new_key.key_prefix,
        permissions: new_key.permissions,
        created_at: new_key.created_at,
        old_key_prefix: current_key.key_prefix,
    }))
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RegenerateKeyResponse {
    pub api_key: String,
    pub key_prefix: String,
    pub permissions: Vec<String>,
    pub created_at: DateTime<Utc>,
    pub old_key_prefix: String,
}

#[utoipa::path(
    path = "/admin/service-accounts/{id}/revoke-key",
    params(
        ("id", description = "Service account ID"),
    ),
    responses(
        (status = 204, description = "Key revoked successfully"),
        (status = 404, description = "Not found"),
        (status = 401, description = "Unauthorized"),
    ),
    tag = "Service Accounts"
)]
pub async fn revoke_api_key(
    State(db): State<Arc<PgPool>>,
    State(api_key_service): State<Arc<ApiKeyService>>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, ApiError> {
    // Validate admin permission
    let user_context = req.extensions().get::<UserContext>().unwrap();
    if !user_context.permissions.contains(&Permission::UsersManage) {
        return Err(ApiError::Forbidden);
    }
    
    // Revoke all API keys for service account
    sqlx::query!(
        r#"
            UPDATE api_keys
            SET is_active = FALSE, revoked_at = NOW(), revoked_reason = 'admin_revoked'
            WHERE service_account_id = $1 AND is_active = TRUE
            "#
        )
        .bind(id)
        .execute(&*db)
        .await
        .ok_or_else(|e| {
            error!("Failed to revoke API keys: {}", e);
        })?;
    
    Ok(StatusCode::NO_CONTENT)
}
```

#### Frontend Components

```typescript
// frontend/src/components/admin/ServiceAccounts.tsx
import { useState } from 'react';
import { api } from '@/api';
import { Trash2, RefreshCw, Key, Copy, Plus, Shield } from 'lucide-react';

interface ServiceAccount {
  id: string;
  email: string;
  name: string;
  created_at: string;
  api_key?: {
    id: string;
    key_prefix: string;
    key: string;
    permissions: string[];
    created_at: string;
  };
}

export const ServiceAccounts: React.FC = () => {
  const [accounts, setAccounts] = useState<ServiceAccount[]>([]);
  const [showCreateModal, setShowCreateModal] = useState(false);
  const [loading, setLoading] = useState(false);

  useEffect(() => {
    loadServiceAccounts();
  }, []);

  const loadServiceAccounts = async () => {
    setLoading(true);
    try {
      const response = await api.get('/api/v1/admin/service-accounts');
      setAccounts(response.data);
    } catch (error) {
      toast.error('Failed to load service accounts');
    } finally {
      setLoading(false);
    }
  };

  const createServiceAccount = async (data: CreateServiceAccountRequest) => {
    setLoading(true);
    try {
      const response = await api.post('/api/v1/admin/service-accounts', data);
      toast.success(`Service account created! API Key: ${response.data.api_key}`);
      setShowCreateModal(false);
      loadServiceAccounts();
    } catch (error) {
      toast.error('Failed to create service account');
    } finally {
      setLoading(false);
    }
  };

  const regenerateKey = async (accountId: string) => {
    setLoading(true);
    try {
      const response = await api.post(`/api/v1/admin/service-accounts/${accountId}/regenerate-key`);
      toast.success(`API Key regenerated! New key: ${response.data.api_key}`);
      loadServiceAccounts();
    } catch (error) {
      toast.error('Failed to regenerate API key');
    } finally {
      setLoading(false);
    }
  };

  const revokeKey = async (accountId: string, keyPrefix: string) => {
    if (!confirm(`Are you sure you want to revoke the API key ${keyPrefix}?`)) return;

    setLoading(true);
    try {
      await api.post(`/api/v1/admin/service-accounts/${accountId}/revoke-key`);
      toast.success('API key revoked');
      loadServiceAccounts();
    } catch (error) {
      toast.error('Failed to revoke API key');
    } finally {
      setLoading(false);
    }
  };

  return (
    <div className="service-accounts">
      <div className="header">
        <h2>Service Accounts & API Keys</h2>
        <button 
          onClick={() => setShowCreateModal(true)}
          className="create-button"
        >
          <Plus className="icon" />
          Create Service Account
        </button>
      </div>

      <div className="accounts-list">
        {loading ? (
          <div className="loading">Loading service accounts...</div>
        ) : accounts.length === 0 ? (
          <div className="empty-state">
            <Shield className="icon" />
            <p>No service accounts yet</p>
          </div>
        ) : (
          accounts.map(account => (
            <div key={account.id} className="account-card">
              <div className="account-header">
                <h3>{account.name}</h3>
                <p>{account.email}</p>
                <span className="created-at">Created: {new Date(account.created_at).toLocaleDateString()}</span>
              </div>

              <div className="api-key-section">
                {account.api_key ? (
                  <>
                    <div className="api-key-info">
                      <Key className="icon" />
                      <span className="label">API Key:</span>
                      <code className="key-value">
                        {account.api_key.key}
                      </code>
                      <button 
                        className="copy-button"
                        onClick={() => navigator.clipboard.writeText(account.api_key.key)}
                      >
                        <Copy className="icon" />
                      </button>
                    </div>
                    <span className="key-prefix">({account.api_key.key_prefix}...)</span>
                  </div>

                  <div className="permissions">
                    <strong>Permissions:</strong>
                    <ul>
                      {account.api_key.permissions.map(perm => (
                        <li key={perm}>{perm}</li>
                      ))}
                    </ul>
                  </div>

                  <div className="actions">
                    <button 
                      className="regenerate-button"
                      onClick={() => regenerateKey(account.id)}
                    >
                      <RefreshCw className="icon" />
                      Regenerate Key
                    </button>
                    <button 
                      className="revoke-button"
                      onClick={() => revokeKey(account.id, account.api_key.key_prefix)}
                    >
                      <Trash2 className="icon" />
                      Revoke Key
                    </button>
                  </div>
                </>
              ) : (
                <div className="no-api-key">
                  <p className="no-key-message">No active API key</p>
                </div>
              )}

              <div className="usage-stats">
                <div className="stat">
                  <span className="label">Last Used:</span>
                  <span className="value">
                    {account.api_key?.last_used_at 
                      ? new Date(account.api_key.last_used_at).toLocaleString()
                      : 'Never'
                    }
                  </span>
                </div>
                <div className="stat">
                  <span className="label">Requests:</span>
                  <span className="value">
                    {account.api_key?.request_count || 0}
                  </span>
                </div>
              </div>
            </div>
          ))}
        )}
      </div>

      {showCreateModal && (
        <CreateServiceAccountModal
          onClose={() => setShowCreateModal(false)}
          onSubmit={createServiceAccount}
        />
      )}
    </div>
  );
};
```

### Testing Strategy

#### Unit Tests
- API key generation (randomness, uniqueness)
- Key hashing verification
- Permission validation logic
- Rate limiting calculations
- Key rotation logic

#### Integration Tests
- Create service account with API key
- Regenerate API key
- Revoke API key
- API key authentication middleware
- Usage tracking functionality
- Rate limiting enforcement per key

#### End-to-End Tests
- Admin creates service account
- Admin views service accounts list
- Admin regenerates API key
- Admin revokes API key
- Service account authenticates with API key
- API key reaches rate limit
- Multiple concurrent requests with same key

#### Security Tests
- Attempt authentication with invalid API key
- Attempt authentication with revoked API key
- Attempt authentication with expired API key
- Attempt to use API key beyond rate limit
- Attempt to regenerate key without permission
- Attempt to revoke key without permission
- Brute force attempts on API key endpoints
- API key extraction attempts

### File List

**Files to be created:**
- `crates/qa-pms-auth/src/api_keys/mod.rs`
- `crates/qa-pms-auth/src/api_keys/generator.rs`
- `crates/qa-pms-auth/src/api_keys/service.rs`
- `crates/qa-pms-auth/src/middleware/api_key_middleware.rs`
- `migrations/create_api_keys_table.sql`
- `migrations/create_api_keys_usage_table.sql`
- `migrations/add_service_account_to_users.sql`
- `frontend/src/components/admin/ServiceAccounts.tsx`
- `frontend/src/components/admin/CreateServiceAccountModal.tsx`

**Files to be modified:**
- `crates/qa-pms-auth/Cargo.toml` (add sha2, rand, hex dependencies)
- `crates/qa-pms-auth/src/lib.rs` (add api_keys module export)
- `crates/qa-pms-api/src/main.rs` (add service account routes, api_key middleware)
- `frontend/src/api/admin.ts` (add service account methods)
- `crates/qa-pms-auth/src/rbac/permission.rs` (add api.read, api.write permissions)
```
