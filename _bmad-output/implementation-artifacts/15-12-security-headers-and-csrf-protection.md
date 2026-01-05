# Story 15.12: Security Headers & CSRF Protection

Status: ready-for-dev

Epic: 15 - Authentication & Authorization
Priority: P0 (Critical for Security)
Estimated Effort: 2 days
Sprint: 1

<!-- Note: Validation is optional. Run validate-create-story for quality check before dev-story. -->

## Story

As a **Security Engineer**,
I want to **implement security headers and CSRF protection**,
So that **the application is protected against common web vulnerabilities**.

## Acceptance Criteria

1. **Given** security middleware is configured
   **When** any HTTP response is returned
   **Then** security headers are set on all responses
   **And** `X-Content-Type-Options: nosniff` header is present
   **And** `X-Frame-Options: DENY` header is present
   **And** `X-XSS-Protection: 1; mode=block` header is present
   **And** `Strict-Transport-Security` header is present (HTTPS only)
   **And** `Content-Security-Policy` header is present
   **And** `Referrer-Policy: strict-origin-when-cross-origin` header is present
   **And** `Permissions-Policy` header is present

2. **Given** CORS middleware is configured
   **When** cross-origin requests are made
   **Then** CORS is properly configured with allowed origins
   **And** allowed methods are restricted (GET, POST, PUT, DELETE, PATCH)
   **And** allowed headers are restricted (Content-Type, Authorization, X-CSRF-Token)
   **And** credentials are allowed for authenticated requests
   **And** CORS preflight requests are handled correctly

3. **Given** CSRF protection is enabled
   **When** state-changing requests (POST, PUT, DELETE, PATCH) are made
   **Then** CSRF token is required in `X-CSRF-Token` header
   **And** token is validated against stored token
   **And** request is rejected with 403 Forbidden if token invalid
   **And** token is regenerated after successful use (single-use)

4. **Given** CSRF token endpoint exists
   **When** frontend requests CSRF token
   **Then** system generates cryptographically secure token (32 bytes)
   **And** token is stored in encrypted cookie
   **And** token is returned in response body
   **And** token expires after 1 hour (configurable)
   **And** token is single-use (regenerated after use)

5. **Given** Content Security Policy is configured
   **When** responses are returned
   **Then** CSP header is set with appropriate policy
   **And** policy allows self-origin for scripts/styles
   **And** policy allows specific external domains (AI providers, if needed)
   **And** policy blocks inline scripts/styles in production
   **And** policy allows inline scripts/styles in development mode
   **And** CSP violation reporting endpoint exists

6. **Given** security headers are configured
   **When** application is in development mode
   **Then** CSP allows unsafe-inline and unsafe-eval for development
   **And** security headers are less restrictive for local development
   **And** HTTPS-only headers are disabled for localhost
   **When** application is in production mode
   **Then** security headers are fully restrictive
   **And** HTTPS-only headers are enabled
   **And** CSP blocks unsafe-inline and unsafe-eval

7. **Given** CSP violation reporting is configured
   **When** CSP violation occurs
   **Then** violation is logged to server
   **And** violation endpoint accepts POST requests with violation data
   **And** violations are stored for security monitoring
   **And** violations are rate-limited to prevent abuse

8. **Given** security middleware is applied
   **When** OPTIONS requests (CORS preflight) are made
   **Then** requests are handled correctly
   **And** appropriate CORS headers are returned
   **And** CSRF validation is skipped for OPTIONS requests
   **And** no security headers are bypassed

9. **Given** health check endpoints exist
   **When** health check requests are made
   **Then** CSRF validation is skipped (read-only endpoint)
   **And** security headers are still applied
   **And** performance is not impacted (< 1ms overhead)

10. **Given** security headers middleware is implemented
    **When** requests are processed
    **Then** middleware adds minimal overhead (< 1ms per request)
    **And** security headers don't interfere with API functionality
    **And** all endpoints are protected consistently

## Tasks / Subtasks

- [ ] Task 1: Implement security headers middleware (AC: #1, #6)
  - [ ] 1.1: Create `crates/qa-pms-api/src/middleware/security.rs` module
  - [ ] 1.2: Create `SecurityHeadersLayer` using `tower_http::set_header::SetResponseHeaderLayer`
  - [ ] 1.3: Set `X-Content-Type-Options: nosniff` header
  - [ ] 1.4: Set `X-Frame-Options: DENY` header
  - [ ] 1.5: Set `X-XSS-Protection: 1; mode=block` header
  - [ ] 1.6: Set `Strict-Transport-Security: max-age=31536000; includeSubDomains` header (HTTPS only)
  - [ ] 1.7: Set `Referrer-Policy: strict-origin-when-cross-origin` header
  - [ ] 1.8: Set `Permissions-Policy` header with appropriate policy
  - [ ] 1.9: Make headers conditional based on environment (dev vs prod)
  - [ ] 1.10: Disable HTTPS-only headers for localhost in development
  - [ ] 1.11: Add unit tests for security headers middleware

- [ ] Task 2: Implement Content Security Policy middleware (AC: #5, #6, #7)
  - [ ] 2.1: Create CSP builder function in `security.rs`
  - [ ] 2.2: Build CSP policy: `default-src 'self'`
  - [ ] 2.3: Add `script-src 'self'` (with unsafe-inline/unsafe-eval for dev)
  - [ ] 2.4: Add `style-src 'self' 'unsafe-inline'` (for Tailwind CSS)
  - [ ] 2.5: Add `img-src 'self' data: https:`
  - [ ] 2.6: Add `connect-src 'self' https://*.openai.com https://*.anthropic.com` (for AI features)
  - [ ] 2.7: Add `font-src 'self'`
  - [ ] 2.8: Add `object-src 'none'`
  - [ ] 2.9: Add `base-uri 'self'`
  - [ ] 2.10: Add `form-action 'self'`
  - [ ] 2.11: Add `frame-ancestors 'none'`
  - [ ] 2.12: Add `upgrade-insecure-requests` (production only)
  - [ ] 2.13: Set different CSP for development mode (allow unsafe-inline/unsafe-eval)
  - [ ] 2.14: Add CSP violation reporting endpoint `/api/v1/security/csp-report`
  - [ ] 2.15: Log CSP violations with tracing::warn!
  - [ ] 2.16: Rate limit CSP violation endpoint (prevent abuse)
  - [ ] 2.17: Add unit tests for CSP generation

- [ ] Task 3: Configure CORS properly (AC: #2, #8)
  - [ ] 3.1: Update `CorsLayer` in `app.rs` to be more restrictive
  - [ ] 3.2: Configure allowed origins from Settings (not `Any`)
  - [ ] 3.3: Set allowed methods: `[Method::GET, Method::POST, Method::PUT, Method::DELETE, Method::PATCH, Method::OPTIONS]`
  - [ ] 3.4: Set allowed headers: `[CONTENT_TYPE, AUTHORIZATION, X_CSRF_TOKEN]`
  - [ ] 3.5: Allow credentials: `allow_credentials(true)`
  - [ ] 3.6: Set max age: `max_age(Duration::from_secs(3600))` (1 hour)
  - [ ] 3.7: Add CORS origin configuration to Settings (environment variable)
  - [ ] 3.8: Handle CORS preflight requests (OPTIONS) correctly
  - [ ] 3.9: Add unit tests for CORS configuration

- [ ] Task 4: Implement CSRF token generation and storage (AC: #3, #4)
  - [ ] 4.1: Create `crates/qa-pms-core/src/security/csrf.rs` module
  - [ ] 4.2: Create `CsrfTokenService` struct
  - [ ] 4.3: Implement `generate_token()` method using `rand::thread_rng()` (32 bytes)
  - [ ] 4.4: Encode token as hex string or base64
  - [ ] 4.5: Store token in encrypted cookie using `tower-cookies` (or encrypted session)
  - [ ] 4.6: Set cookie: `HttpOnly, Secure, SameSite=Strict` (production)
  - [ ] 4.7: Set cookie: `HttpOnly, SameSite=Lax` (development, allow localhost)
  - [ ] 4.8: Set token expiration (1 hour, configurable)
  - [ ] 4.9: Implement `validate_token(stored_token, provided_token)` method
  - [ ] 4.10: Use constant-time comparison for token validation (prevent timing attacks)
  - [ ] 4.11: Implement token regeneration after successful use
  - [ ] 4.12: Add `GET /api/v1/csrf-token` endpoint to return CSRF token
  - [ ] 4.13: Add unit tests for CSRF token generation and validation

- [ ] Task 5: Implement CSRF protection middleware (AC: #3, #4, #8, #9)
  - [ ] 5.1: Create `csrf_middleware()` function in `security.rs`
  - [ ] 5.2: Skip CSRF validation for GET, HEAD, OPTIONS requests
  - [ ] 5.3: Skip CSRF validation for health check endpoints (`/api/v1/health/*`)
  - [ ] 5.4: Extract CSRF token from `X-CSRF-Token` header
  - [ ] 5.5: Extract stored CSRF token from cookie or session
  - [ ] 5.6: Validate tokens using constant-time comparison
  - [ ] 5.7: Return 403 Forbidden if token invalid or missing
  - [ ] 5.8: Return descriptive error message for CSRF failures
  - [ ] 5.9: Log CSRF violations with tracing::warn!
  - [ ] 5.10: Regenerate token after successful validation (single-use)
  - [ ] 5.11: Update cookie with new token
  - [ ] 5.12: Add unit tests for CSRF middleware
  - [ ] 5.13: Add integration tests for CSRF protection

- [ ] Task 6: Add CSRF token endpoint (AC: #4)
  - [ ] 6.1: Create `crates/qa-pms-api/src/routes/security.rs` module
  - [ ] 6.2: Add `GET /api/v1/csrf-token` endpoint
  - [ ] 6.3: Generate new CSRF token
  - [ ] 6.4: Store token in encrypted cookie
  - [ ] 6.5: Return token in response body: `{ "token": "..." }`
  - [ ] 6.6: Set appropriate cookie headers
  - [ ] 6.7: Add OpenAPI documentation
  - [ ] 6.8: Add integration tests

- [ ] Task 7: Add CSP violation reporting endpoint (AC: #7)
  - [ ] 7.1: Add `POST /api/v1/security/csp-report` endpoint
  - [ ] 7.2: Accept CSP violation report format (JSON)
  - [ ] 7.3: Parse violation report (document-uri, blocked-uri, violated-directive, etc.)
  - [ ] 7.4: Log violation with tracing::warn! (include full report)
  - [ ] 7.5: Store violation in database (optional, for monitoring)
  - [ ] 7.6: Rate limit endpoint (prevent abuse: max 10 reports/minute per IP)
  - [ ] 7.7: Return 204 No Content (no response body needed)
  - [ ] 7.8: Skip CSRF validation for CSP report endpoint (it's a POST from browser)
  - [ ] 7.9: Add integration tests

- [ ] Task 8: Integrate security middleware into application (AC: #1, #2, #3, #10)
  - [ ] 8.1: Update `app.rs` to include security headers middleware
  - [ ] 8.2: Update CORS configuration to be more restrictive
  - [ ] 8.3: Add CSRF middleware to router (after CORS, before handlers)
  - [ ] 8.4: Apply CSRF middleware to all routes except GET, HEAD, OPTIONS, health checks
  - [ ] 8.5: Add `CsrfTokenService` to `AppState`
  - [ ] 8.6: Configure security settings from environment variables
  - [ ] 8.7: Make security headers configurable via Settings
  - [ ] 8.8: Test all endpoints still work with security middleware
  - [ ] 8.9: Verify performance impact (< 1ms overhead)

- [ ] Task 9: Add frontend CSRF token integration (AC: #3, #4)
  - [ ] 9.1: Create `frontend/src/hooks/useCSRFToken.ts` hook
  - [ ] 9.2: Fetch CSRF token on app initialization
  - [ ] 9.3: Store token in memory (not localStorage, for security)
  - [ ] 9.4: Create axios/fetch interceptor to add `X-CSRF-Token` header
  - [ ] 9.5: Add header to all POST, PUT, DELETE, PATCH requests
  - [ ] 9.6: Handle CSRF token refresh when token expires (401/403 response)
  - [ ] 9.7: Retry request after token refresh
  - [ ] 9.8: Add error handling for CSRF failures
  - [ ] 9.9: Add loading state during token fetch
  - [ ] 9.10: Add unit tests for CSRF token hook

- [ ] Task 10: Update Settings for security configuration (AC: #6)
  - [ ] 10.1: Add security settings to `crates/qa-pms-config/src/settings.rs`
  - [ ] 10.2: Add `allowed_origins: Vec<String>` field (CORS)
  - [ ] 10.3: Add `csrf_token_ttl_seconds: u64` field (default: 3600)
  - [ ] 10.4: Add `csp_report_only: bool` field (default: false, enable CSP report-only mode)
  - [ ] 10.5: Add `enable_hsts: bool` field (default: true, production only)
  - [ ] 10.6: Load from environment variables: `QA_PMS_ALLOWED_ORIGINS`, `QA_PMS_CSRF_TTL_SECS`, etc.
  - [ ] 10.7: Add validation for allowed origins (must be valid URLs)
  - [ ] 10.8: Add unit tests for settings parsing

- [ ] Task 11: Add comprehensive tests (AC: All)
  - [ ] 11.1: Add unit tests for security headers middleware
  - [ ] 11.2: Add unit tests for CSP generation (dev vs prod)
  - [ ] 11.3: Add unit tests for CSRF token generation and validation
  - [ ] 11.4: Add integration tests for CORS configuration
  - [ ] 11.5: Add integration tests for CSRF protection (test 403 on missing/invalid token)
  - [ ] 11.6: Add integration tests for CSP violation reporting
  - [ ] 11.7: Test security headers are present on all responses
  - [ ] 11.8: Test CSRF token endpoint returns valid token
  - [ ] 11.9: Test CSRF middleware skips validation for safe methods
  - [ ] 11.10: Test performance (verify < 1ms overhead)

- [ ] Task 12: Add security documentation (AC: All)
  - [ ] 12.1: Document security headers in API docs
  - [ ] 12.2: Document CSRF protection implementation
  - [ ] 12.3: Document CSP policy and violation reporting
  - [ ] 12.4: Document CORS configuration
  - [ ] 12.5: Add security best practices guide
  - [ ] 12.6: Document environment variables for security configuration
  - [ ] 12.7: Add security testing guidelines (OWASP ZAP, Burp Suite)

## Dev Notes

### Architecture Compliance

**Tech Stack:**
- **Backend:** Rust 1.80+ with Tokio async runtime, Axum 0.7+ web framework
- **Middleware:** Tower ecosystem (`tower-http`) for security headers
- **Cookies:** `tower-cookies` or `axum-extra::extract::cookie` for CSRF token storage
- **Crypto:** `rand` crate for token generation, `constant_time_eq` for token comparison
- **Error Handling:** `anyhow` (internal) + `thiserror` (API boundaries)
- **Logging:** `tracing` + `tracing-subscriber` (never `println!`)

**Code Structure:**
- **Security Middleware:** `crates/qa-pms-api/src/middleware/security.rs` (new module)
- **CSRF Service:** `crates/qa-pms-core/src/security/csrf.rs` (new module)
- **Security Routes:** `crates/qa-pms-api/src/routes/security.rs` (new module)
- **Frontend Hook:** `frontend/src/hooks/useCSRFToken.ts` (new file)
- **Settings:** `crates/qa-pms-config/src/settings.rs` (modify existing file)

**Security Headers Pattern (Tower-HTTP):**
Following Tower-HTTP patterns for security headers:
```rust
use tower_http::set_header::SetResponseHeaderLayer;
use axum::http::HeaderValue;

let security_headers = tower::ServiceBuilder::new()
    .layer(SetResponseHeaderLayer::overriding(
        axum::http::header::X_CONTENT_TYPE_OPTIONS,
        HeaderValue::from_static("nosniff"),
    ))
    .layer(SetResponseHeaderLayer::overriding(
        axum::http::header::X_FRAME_OPTIONS,
        HeaderValue::from_static("DENY"),
    ))
    // ... more headers
```

**CSRF Protection Pattern (Axum Middleware):**
Following Axum middleware pattern for CSRF validation:
```rust
use axum::{
    middleware::{self, Next},
    extract::Request,
    response::Response,
    http::StatusCode,
};

async fn csrf_middleware(
    req: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    // Skip CSRF for safe methods
    if matches!(req.method(), &Method::GET | &Method::HEAD | &Method::OPTIONS) {
        return Ok(next.run(req).await);
    }
    
    // Extract and validate CSRF token
    let csrf_token = req.headers().get("X-CSRF-Token")?;
    let stored_token = extract_from_cookie(&req)?;
    
    if validate_csrf_token(stored_token, csrf_token)? {
        Ok(next.run(req).await)
    } else {
        Err(StatusCode::FORBIDDEN)
    }
}
```

**CORS Configuration Pattern:**
Following Tower-HTTP CORS patterns (update existing CORS config):
```rust
use tower_http::cors::CorsLayer;
use axum::http::Method;

let cors = CorsLayer::new()
    .allow_origin(allowed_origins.parse::<HeaderValue>()?)  // Not `Any`!
    .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE, Method::PATCH, Method::OPTIONS])
    .allow_headers([CONTENT_TYPE, AUTHORIZATION, HeaderName::from_static("x-csrf-token")])
    .allow_credentials(true)
    .max_age(Duration::from_secs(3600));
```

### Context7 Requirements (MANDATORY)

**CRITICAL:** Before implementing any code, use Context7 MCP to:

1. **Resolve library ID**: `/tokio-rs/axum` and `/tower-rs/tower-http`
2. **Query Context7 for**: "How to implement CSRF protection middleware in Axum with Tower"
3. **Query Context7 for**: "How to add security headers in Axum using tower-http SetResponseHeaderLayer"
4. **Verify patterns for**:
   - `tower_http::set_header::SetResponseHeaderLayer` - security headers
   - `tower_http::cors::CorsLayer` - CORS configuration
   - `axum::middleware::from_fn()` - custom middleware
   - Constant-time token comparison (prevent timing attacks)
   - Cookie management for CSRF tokens
5. **Check best practices for**:
   - CSRF token generation (crypto-secure random)
   - CSRF token storage (encrypted cookies vs session)
   - Content Security Policy configuration
   - CORS configuration for production

**Why this is mandatory:**
- Security implementations must follow current best practices
- Prevents vulnerabilities from incorrect CSRF implementation
- Ensures proper use of Tower-HTTP security features
- Reduces risk of security bypasses from incorrect configuration

### Previous Story Intelligence

**From Story 15.1 (JWT Authentication):**
- Already have authentication middleware
- Already have request extensions for user context
- CSRF middleware should run **after** auth middleware
- CSRF tokens are separate from JWT tokens (different purpose)

**From Story 15.6 (Session Management):**
- May have session storage for user sessions
- CSRF tokens can be stored in session (alternative to cookies)
- Session expiration affects CSRF token expiration

**From Story 15.11 (Rate Limiting):**
- CSP violation reporting endpoint should be rate-limited
- CSRF validation should not be rate-limited (security critical)
- Rate limiting middleware should run **before** CSRF middleware

**Key Integration Points:**
- Reuse existing CORS configuration in `app.rs` (make it more restrictive)
- CSRF middleware should skip validation for health checks (already existing endpoints)
- Security headers should apply to all responses (even error responses)
- Frontend needs to fetch CSRF token on app load (integrate with existing auth flow)

**Code Patterns to Follow:**
```rust
// From app.rs - Existing CORS pattern (needs to be more restrictive)
.layer(CorsLayer::new()
    .allow_origin(Any)  // ❌ Too permissive - needs to be restricted
    .allow_methods(Any)
    .allow_headers(Any),
)
```

### Project Structure Notes

**Alignment with unified structure:**
- ✅ Security middleware in `qa-pms-api/src/middleware/` (middleware layer)
- ✅ CSRF service in `qa-pms-core/src/security/` (core domain)
- ✅ Security routes in `qa-pms-api/src/routes/security.rs` (API layer)
- ✅ Frontend hooks in `frontend/src/hooks/` (frontend utilities)

**Files to Create:**
- `crates/qa-pms-api/src/middleware/security.rs` - Security headers and CSRF middleware
- `crates/qa-pms-core/src/security/csrf.rs` - CSRF token service
- `crates/qa-pms-api/src/routes/security.rs` - CSRF token and CSP report endpoints
- `frontend/src/hooks/useCSRFToken.ts` - Frontend CSRF token hook

**Files to Modify:**
- `crates/qa-pms-api/src/app.rs` - Add security middleware, update CORS config
- `crates/qa-pms-api/src/routes/mod.rs` - Add security routes
- `crates/qa-pms-core/src/lib.rs` - Export security module
- `crates/qa-pms-config/src/settings.rs` - Add security configuration
- `crates/qa-pms-api/Cargo.toml` - Add `tower-cookies` or `axum-extra` dependency
- `crates/qa-pms-core/Cargo.toml` - Add `rand` and `constant_time_eq` dependencies
- `frontend/src/App.tsx` - Initialize CSRF token on app load
- `frontend/src/utils/api.ts` or axios config - Add CSRF token interceptor

**Naming Conventions:**
- Functions: `csrf_middleware()`, `generate_csrf_token()`, `validate_csrf_token()`
- Modules: `security.rs`, `csrf.rs`
- API endpoints: `/api/v1/csrf-token`, `/api/v1/security/csp-report`
- Headers: `X-CSRF-Token` (case-sensitive)

### Security Best Practices

**CSRF Token Security:**
- Generate tokens using cryptographically secure RNG (`rand::thread_rng()`)
- Use 32-byte tokens (256 bits of entropy)
- Store tokens in HttpOnly cookies (prevent XSS access)
- Use Secure flag in production (HTTPS only)
- Use SameSite=Strict in production, SameSite=Lax in development
- Use constant-time comparison for validation (prevent timing attacks)
- Regenerate tokens after use (single-use tokens)
- Set appropriate expiration (1 hour default)

**Content Security Policy:**
- Start with restrictive policy, relax only when needed
- Use `report-uri` or `report-to` for violation reporting
- Allow `unsafe-inline` only in development (for hot reload)
- Use nonces or hashes instead of `unsafe-inline` in production (future enhancement)
- Test CSP policy thoroughly (browser console will show violations)

**CORS Configuration:**
- Never use `allow_origin(Any)` in production
- Whitelist specific origins from environment variable
- Allow only necessary methods and headers
- Set appropriate `max_age` for preflight cache
- Allow credentials only for authenticated endpoints

**Security Headers:**
- Apply headers to all responses (even error responses)
- Use appropriate values based on application needs
- Test headers with security scanners (OWASP ZAP, Burp Suite)
- Monitor CSP violation reports for policy adjustments

### Testing Standards

**Unit Tests:**
- Test CSRF token generation (verify randomness, length)
- Test CSRF token validation (valid token, invalid token, expired token)
- Test constant-time comparison (prevent timing attacks)
- Test security headers middleware (verify all headers are set)
- Test CSP generation (dev vs prod policies)

**Integration Tests:**
- Test CSRF protection (403 Forbidden on missing/invalid token)
- Test CSRF token endpoint (returns valid token, sets cookie)
- Test CSP violation reporting (accepts reports, rate limits)
- Test CORS preflight requests (OPTIONS with appropriate headers)
- Test security headers on all endpoints

**Security Tests:**
- Test CSRF token single-use (token invalid after use)
- Test CSRF token expiration (token invalid after TTL)
- Test CSRF protection bypass attempts (header manipulation)
- Test CORS origin validation (reject unauthorized origins)
- Test CSP violation reporting (verify violations are logged)

**Performance Tests:**
- Verify security middleware adds < 1ms overhead per request
- Test CSRF token generation performance (should be < 1ms)
- Test constant-time comparison performance
- Load test with security middleware enabled

**Test Files:**
- `crates/qa-pms-api/src/middleware/security.rs` - `#[cfg(test)]` module
- `crates/qa-pms-core/src/security/csrf.rs` - `#[cfg(test)]` module
- `crates/qa-pms-api/tests/security_integration.rs` - Integration tests
- `frontend/src/hooks/__tests__/useCSRFToken.test.ts` - Frontend tests

**Test Coverage Target:**
- Minimum 90% coverage for security-critical code
- 100% coverage for CSRF token validation logic
- Integration tests for all security endpoints

### References

- **Source: `_bmad-output/planning-artifacts/epics-detailed/epic-15-authentication-authorization.md#story-15.12`** - Story requirements and acceptance criteria
- **Source: `qa-intelligent-pms/crates/qa-pms-api/src/app.rs`** - Current CORS and middleware setup
- **Source: Tower-HTTP docs (`/tower-rs/tower-http`) via Context7** - Security headers and CORS patterns
- **Source: Axum docs (`/tokio-rs/axum`) via Context7** - Middleware patterns, CSRF protection
- **Source: OWASP CSRF Prevention Cheat Sheet** - CSRF protection best practices
- **Source: OWASP Secure Headers Project** - Security headers best practices
- **Source: `_bmad-output/planning-artifacts/project-context.md`** - Rust patterns, error handling, logging

## Dev Agent Record

### Agent Model Used

Claude Sonnet 4.5 (via Cursor)

### Debug Log References

(None yet - story not implemented)

### Completion Notes List

(None yet - story not implemented)

### File List

**Created:**
- `crates/qa-pms-api/src/middleware/security.rs` - Security headers and CSRF middleware
- `crates/qa-pms-core/src/security/csrf.rs` - CSRF token service
- `crates/qa-pms-api/src/routes/security.rs` - CSRF token and CSP report endpoints
- `frontend/src/hooks/useCSRFToken.ts` - Frontend CSRF token hook

**Modified:**
- `crates/qa-pms-api/src/app.rs` - Add security middleware, update CORS configuration
- `crates/qa-pms-api/src/routes/mod.rs` - Add security routes
- `crates/qa-pms-core/src/lib.rs` - Export security module
- `crates/qa-pms-config/src/settings.rs` - Add security configuration fields
- `crates/qa-pms-api/Cargo.toml` - Add `tower-cookies` or `axum-extra` dependency
- `crates/qa-pms-core/Cargo.toml` - Add `rand` and `constant_time_eq` dependencies
- `frontend/src/App.tsx` - Initialize CSRF token on app load
- `frontend/src/utils/api.ts` (or axios config) - Add CSRF token interceptor

### Change Log

**2026-01-10 - Story Created:**
- Initial story file created with complete structure
- Added all required sections: Story, Metadata, Acceptance Criteria (10 ACs), Tasks (12 tasks with subtasks), Dev Notes, Dev Agent Record, File List
- Converted acceptance criteria from epic format to Given/When/Then format
- Added comprehensive dev notes with architecture patterns, security best practices, testing standards
- Added references to previous stories (15.1, 15.6, 15.11) for integration
- Added Context7 requirements for security implementation patterns
- Added file list with all files to create and modify
