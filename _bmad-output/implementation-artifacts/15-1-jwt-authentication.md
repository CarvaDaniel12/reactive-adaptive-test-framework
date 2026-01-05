# Story 15.1: JWT Authentication

Status: ready-for-dev

## Story

As a QA Engineer,
I want to authenticate securely using JWT tokens with password-based login,
So that I can access the platform with my credentials and sessions are properly managed.

## Acceptance Criteria

1. **Given** a user account exists in the database
   **When** the user provides valid email and password
   **Then** the system generates a JWT access token
   **And** generates a refresh token
   **And** returns both tokens to the client
   **And** refresh token is stored securely in HTTP-only cookie

2. **Given** a user provides invalid credentials
   **When** authentication is attempted
   **Then** the system returns 401 Unauthorized
   **And** provides clear error message
   **And** logs the failed attempt

3. **Given** a user logs in with valid credentials
   **When** the password is stored in the database
   **Then** it is hashed using argon2id with appropriate parameters
   **And** the hash is stored (never the plaintext password)
   **And** a salt is generated and stored with the hash

4. **Given** an access token is issued
   **When** the token is used for API requests
   **Then** it expires after 15 minutes
   **And** contains user_id, role, and expiration claims
   **And** is signed with RS256 algorithm
   **And** includes jti (JWT ID) claim for revocation tracking

5. **Given** an access token expires
   **When** the client uses the refresh token
   **Then** a new access token is issued
   **And** a new refresh token is issued (token rotation)
   **And** the old refresh token is invalidated
   **And** the new access token has a fresh expiration

6. **Given** a protected API endpoint is called
   **When** the request includes a valid JWT
   **Then** the request proceeds
   **And** user context is available to the handler
   **And** request is correlated with jti for tracing

7. **Given** a protected API endpoint is called
   **When** the request has no token or invalid token
   **Then** the system returns 401 Unauthorized
   **And** includes "WWW-Authenticate: Bearer" header
   **And** logs the unauthorized access attempt

8. **Given** a user logs out
   **When** logout is requested
   **Then** the refresh token is invalidated
   **And** the jti is added to token blacklist
   **And** all active sessions for the user are terminated

9. **Given** password validation is required
   **When** a password is set or changed
   **Then** it must be at least 12 characters
   **And** must contain uppercase, lowercase, number, and special character
   **And** cannot contain parts of the email address
   **And** password history prevents reuse of last 5 passwords

10. **Given** authentication endpoint receives a request
    **When** rate limiting is applied
    **Then** the endpoint allows 5 requests per 15 minutes per IP
    **And** returns 429 Too Many Requests when limit exceeded
    **And** includes Retry-After header

## Tasks / Subtasks

- [ ] Task 1: Setup authentication crate structure (AC: #1)
  - [ ] 1.1: Create `qa-pms-auth` crate in workspace
  - [ ] 1.2: Add `authentication` module
  - [ ] 1.3: Add `tokens` module
  - [ ] 1.4: Add `middleware` module
  - [ ] 1.5: Add `password` module
  - [ ] 1.6: Configure crate dependencies (jsonwebtoken, argon2, etc.)

- [ ] Task 2: Implement password hashing utilities (AC: #3, #9)
  - [ ] 2.1: Create `password.rs` module
  - [ ] 2.2: Implement `hash_password()` using argon2id
  - [ ] 2.3: Implement `verify_password()` using argon2id
  - [ ] 2.4: Implement `validate_password_strength()` with all rules
  - [ ] 2.5: Add unit tests for password hashing

- [ ] Task 3: Implement JWT token generation and validation (AC: #1, #4)
  - [ ] 3.1: Create `tokens.rs` module
  - [ ] 3.2: Implement `Claims` struct with user_id, role, exp, jti
  - [ ] 3.3: Implement `generate_access_token()` with 15 min expiry
  - [ ] 3.4: Implement `generate_refresh_token()` with 7 day expiry
  - [ ] 3.5: Implement `validate_token()` with signature and expiry checks
  - [ ] 3.6: Add unit tests for token operations

- [ ] Task 4: Create database schema for authentication (AC: #3, #5)
  - [ ] 4.1: Create `users` table migration
  - [ ] 4.2: Add email, password_hash, salt columns
  - [ ] 4.3: Create `refresh_tokens` table migration
  - [ ] 4.4: Add user_id, token_hash, expires_at columns
  - [ ] 4.5: Create `password_history` table for 5-password limit
  - [ ] 4.6: Add indexes on email and token_hash

- [ ] Task 5: Implement authentication service (AC: #1, #2, #10)
  - [ ] 5.1: Create `AuthService` in `qa-pms-auth`
  - [ ] 5.2: Implement `login()` with password verification
  - [ ] 5.3: Implement rate limiting using tower-governor
  - [ ] 5.4: Return access and refresh tokens on success
  - [ ] 5.5: Return appropriate errors on failure
  - [ ] 5.6: Log all authentication attempts with tracing

- [ ] Task 6: Implement token refresh mechanism (AC: #5, #8)
  - [ ] 6.1: Implement `refresh_token()` in AuthService
  - [ ] 6.2: Validate existing refresh token from cookie
  - [ ] 6.3: Generate new access and refresh tokens
  - [ ] 6.4: Invalidate old refresh token
  - [ ] 6.5: Set new refresh token in cookie
  - [ ] 6.6: Return new access token

- [ ] Task 7: Implement token blacklist for logout (AC: #7, #8)
  - [ ] 7.1: Create `token_blacklist` table migration
  - [ ] 7.2: Add jti and expires_at columns
  - [ ] 7.3: Implement `blacklist_token()` function
  - [ ] 7.4: Implement `is_token_blacklisted()` check
  - [ ] 7.5: Add cleanup job for expired blacklisted tokens

- [ ] Task 8: Create authentication middleware (AC: #6, #7)
  - [ ] 8.1: Create `auth_middleware.rs` in middleware module
  - [ ] 8.2: Extract JWT from Authorization header
  - [ ] 8.3: Validate token signature and expiry
  - [ ] 8.4: Check token blacklist
  - [ ] 8.5: Extract user context and add to request extensions
  - [ ] 8.6: Return 401 on invalid/missing tokens
  - [ ] 8.7: Add request_id tracking from jti claim

- [ ] Task 9: Implement logout functionality (AC: #8)
  - [ ] 9.1: Create `logout()` handler
  - [ ] 9.2: Get refresh token from cookie
  - [ ] 9.3: Blacklist access token jti
  - [ ] 9.4: Delete refresh token from database
  - [ ] 9.5: Clear auth cookies
  - [ ] 9.6: Return 204 No Content

- [ ] Task 10: Create API endpoints (AC: #1, #2, #8, #9)
  - [ ] 10.1: Add `POST /api/v1/auth/login` endpoint
  - [ ] 10.2: Add `POST /api/v1/auth/refresh` endpoint
  - [ ] 10.3: Add `POST /api/v1/auth/logout` endpoint
  - [ ] 10.4: Apply rate limiting to login endpoint
  - [ ] 10.5: Apply auth middleware to protected routes
  - [ ] 10.6: Add OpenAPI documentation with utoipa

- [ ] Task 11: Implement frontend auth client (AC: #1, #5, #6)
  - [ ] 11.1: Create `useAuth` hook in frontend
  - [ ] 11.2: Implement `login()` function with credentials
  - [ ] 11.3: Implement token storage in memory/cookies
  - [ ] 11.4: Implement automatic token refresh on expiry
  - [ ] 11.5: Implement `logout()` function
  - [ ] 11.6: Add axios interceptor for Authorization header

- [ ] Task 12: Create login page UI (AC: #2, #9)
  - [ ] 12.1: Create `LoginPage` component
  - [ ] 12.2: Add email and password inputs
  - [ ] 12.3: Implement client-side password validation
  - [ ] 12.4: Show loading state during auth
  - [ ] 12.5: Display error messages clearly
  - [ ] 12.6: Redirect to dashboard on successful login

## Dev Notes

### Architecture Alignment

This story implements **JWT Authentication** per Epic 15 requirements:

- **Backend Location**: `crates/qa-pms-auth/src/`
- **Middleware**: `crates/qa-pms-auth/src/middleware/auth_middleware.rs`
- **Security**: NFR-SEC-01 compliant password hashing, JWT with RS256
- **Token Management**: Access tokens (15min), Refresh tokens (7 days)

### Technical Implementation Details

#### Dependencies to Add

```toml
# crates/qa-pms-auth/Cargo.toml
[dependencies]
jsonwebtoken = "9"
argon2 = "0.5"
serde = { version = "1.0", features = ["derive"] }
chrono = "0.4"
rand = "0.8"
tower-governor = "0.4"
tower = "0.4"
axum = "0.7"
```

#### Database Schema

```sql
-- Users table
CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    email TEXT NOT NULL UNIQUE,
    password_hash TEXT NOT NULL,
    salt TEXT NOT NULL,
    is_active BOOLEAN DEFAULT TRUE,
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW(),
    last_login_at TIMESTAMP
);

-- Refresh tokens table
CREATE TABLE refresh_tokens (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id),
    token_hash TEXT NOT NULL UNIQUE,
    jti TEXT NOT NULL UNIQUE,
    expires_at TIMESTAMP NOT NULL,
    created_at TIMESTAMP DEFAULT NOW(),
    last_used_at TIMESTAMP DEFAULT NOW(),
    ip_address INET,
    user_agent TEXT
);

-- Password history table
CREATE TABLE password_history (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id),
    password_hash TEXT NOT NULL,
    created_at TIMESTAMP DEFAULT NOW()
);

-- Token blacklist table
CREATE TABLE token_blacklist (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    jti TEXT NOT NULL UNIQUE,
    expires_at TIMESTAMP NOT NULL,
    created_at TIMESTAMP DEFAULT NOW()
);

-- Indexes
CREATE INDEX idx_users_email ON users(email);
CREATE INDEX idx_refresh_tokens_user_id ON refresh_tokens(user_id);
CREATE INDEX idx_refresh_tokens_jti ON refresh_tokens(jti);
CREATE INDEX idx_token_blacklist_jti ON token_blacklist(jti);
```

#### API Endpoints

```rust
// POST /api/v1/auth/login
#[utoipa::path(
    request_body = LoginRequest,
    responses(
        (status = 200, description = "Successful login", body = AuthResponse),
        (status = 401, description = "Invalid credentials"),
        (status = 429, description = "Rate limit exceeded")
    ),
    tag = "Authentication"
)]
pub async fn login(
    State(auth_service): State<Arc<AuthService>>,
    Json(request): Json<LoginRequest>,
) -> Result<Json<AuthResponse>, ApiError>

// POST /api/v1/auth/refresh
#[utoipa::path(
    request_body = RefreshRequest,
    responses(
        (status = 200, description = "Token refreshed", body = RefreshResponse),
        (status = 401, description = "Invalid refresh token")
    ),
    tag = "Authentication"
)]
pub async fn refresh_token(
    State(auth_service): State<Arc<AuthService>>,
    req: Request,
) -> Result<Json<RefreshResponse>, ApiError>

// POST /api/v1/auth/logout
#[utoipa::path(
    responses(
        (status = 204, description = "Logged out successfully")
    ),
    tag = "Authentication"
)]
pub async fn logout(
    State(auth_service): State<Arc<AuthService>>,
    req: Request,
) -> Result<StatusCode, ApiError>
```

#### Authentication Middleware

```rust
use axum::{
    extract::{Request, State},
    http::{HeaderMap, StatusCode},
    middleware::Next,
    response::Response,
};
use jsonwebtoken::{decode, Validation};
use tower::ServiceBuilder;

pub async fn auth_middleware<B>(
    State(jwt_secret): State<String>,
    mut req: Request<B>,
    next: Next<B>,
) -> Result<Response, StatusCode> {
    // Extract token from Authorization header
    let auth_header = req.headers()
        .get("authorization")
        .and_then(|h| h.to_str().ok())
        .and_then(|h| h.strip_prefix("Bearer "));
    
    let token = match auth_header {
        Some(token) => token,
        None => return Err(StatusCode::UNAUTHORIZED),
    };
    
    // Validate token
    let claims = match decode::<Claims>(token, &Validation::new(&jwt_secret)) {
        Ok(data) => data.claims,
        Err(_) => return Err(StatusCode::UNAUTHORIZED),
    };
    
    // Check if token is blacklisted
    if auth_service.is_token_blacklisted(claims.jti).await {
        return Err(StatusCode::UNAUTHORIZED);
    }
    
    // Add user context to request extensions
    req.extensions_mut().insert(UserContext::from(claims));
    
    // Add request ID for tracing
    req.extensions_mut().insert(RequestId::new(claims.jti));
    
    Ok(next.run(req).await)
}
```

#### Password Validation

```rust
use argonautica::Argon2;
use regex::Regex;

pub fn validate_password_strength(password: &str, email: &str) -> Result<(), PasswordError> {
    // Length check
    if password.len() < 12 {
        return Err(PasswordError::TooShort);
    }
    
    // Complexity checks
    if !password.chars().any(|c| c.is_uppercase()) {
        return Err(PasswordError::MissingUppercase);
    }
    
    if !password.chars().any(|c| c.is_lowercase()) {
        return Err(PasswordError::MissingLowercase);
    }
    
    if !password.chars().any(|c| c.is_numeric()) {
        return Err(PasswordError::MissingNumber);
    }
    
    if !password.chars().any(|c| !c.is_alphanumeric()) {
        return Err(PasswordError::MissingSpecial);
    }
    
    // Email parts check
    let email_parts: Vec<&str> = email.split('@').collect();
    if let Some(username) = email_parts.get(0) {
        if password.to_lowercase().contains(&username.to_lowercase()) {
            return Err(PasswordError::ContainsEmail);
        }
    }
    
    Ok(())
}
```

### Testing Strategy

#### Unit Tests

- Password hashing and verification
- Password validation rules
- JWT token generation and parsing
- Token blacklist operations
- Rate limiting logic

#### Integration Tests

- Complete login flow
- Token refresh flow
- Logout flow
- Middleware with valid/invalid tokens
- Rate limiting enforcement

#### Security Tests

- Attempt login with wrong password
- Attempt login with non-existent user
- Test token expiry
- Test token blacklisting
- Brute force attempts (rate limiting)

#### End-to-End Tests

- Login through frontend
- Access protected endpoint
- Token refresh automatically
- Logout and verify tokens invalid