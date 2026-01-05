# Story 15.6: Session Management & Refresh Tokens

Status: ready-for-dev

## Story

As a QA Engineer,
I want to have seamless session management with automatic token refresh,
So that I don't have to repeatedly log in during long testing sessions and my sessions remain secure.

## Acceptance Criteria

1. **Given** a user successfully authenticates
   **When** tokens are issued
   **Then** system generates short-lived access token (15 minutes)
   **And** generates long-lived refresh token (7 days)
   **And** stores refresh token in HTTP-only, Secure cookie
   **And** access token is returned to client
   **And** refresh token is never exposed in API responses

2. **Given** an access token is about to expire
   **When** client detects approaching expiry
   **Then** system automatically refreshes tokens
   **And** issues new access token without user action
   **And** issues new refresh token (token rotation)
   **And** invalidates old refresh token

3. **Given** a user wants to view active sessions
   **When** they access security settings
   **Then** they see list of all active sessions
   **And** each session shows device, location, last activity
   **And** they can revoke individual sessions
   **And** they can revoke all other sessions

4. **Given** a user logs out
   **When** logout is requested
   **Then** refresh token is invalidated
   **And** access token is added to blacklist
   **And** all active sessions for user are terminated
   **And** auth cookies are cleared
   **And** frontend redirects to login page

5. **Given** an admin revokes a user's access
   **When** revocation is triggered
   **Then** all tokens for user are invalidated
   **And** all refresh tokens are deleted
   **And** user is immediately logged out
   **And** user must re-authenticate to access system

6. **Given** a refresh token is used
   **When** token exchange is requested
   **Then** system validates refresh token
   **And** checks if token is not blacklisted
   **And** checks if token hasn't expired
   **And** generates new access token
   **And** rotates refresh token

7. **Given** suspicious activity is detected
   **When** session needs to be terminated
   **Then** system can revoke specific session
   **And** can revoke all sessions for user
   **And** logs revocation event with reason
   **And** notifies user via email if configured

8. **Given** multiple concurrent sessions exist
   **When** limit is exceeded (e.g., 5 concurrent sessions)
   **Then** oldest session is terminated
   **And** user is notified of session termination
   **And** session limit is configurable

9. **Given** user changes password
   **When** password change completes
   **Then** all existing sessions are terminated
   **And** all refresh tokens are invalidated
   **And** user must re-login on all devices

10. **Given** session data needs to be displayed
   **When** viewing session details
   **Then** shows session creation time
   **And** shows last activity time
   **And** shows device/browser info
   **And** shows IP address and location (if available)
   **And** shows "current session" indicator

## Tasks / Subtasks

- [ ] Task 1: Setup refresh token database schema (AC: #1, #5, #6)
  - [ ] 1.1: Update `refresh_tokens` table with additional columns
  - [ ] 1.2: Add device_info (JSONB) for browser, OS, device name
  - [ ] 1.3: Add last_used_at timestamp
  - [ ] 1.4: Add is_revoked boolean column
  - [ ] 1.5: Create migration file
  - [ ] 1.6: Add indexes for performance queries
  - [ ] 1.7: Add unit tests for schema changes

- [ ] Task 2: Implement token generation service (AC: #1)
  - [ ] 2.1: Create `generate_tokens()` function in AuthService
  - [ ] 2.2: Generate JWT access token with 15-minute expiry
  - [ ] 2.3: Generate cryptographically secure refresh token
  - [ ] 2.4: Store refresh token hash in database
  - [ ] 2.5: Add unit tests for token generation

- [ ] Task 3: Implement token refresh mechanism (AC: #2, #6)
  - [ ] 3.1: Create `refresh_access_token()` endpoint
  - [ ] 3.2: Validate refresh token from cookie
  - [ ] 3.3: Check if refresh token is not revoked
  - [ ] 3.4: Check if refresh token hasn't expired
  - [ ] 3.5: Generate new access token
  - [ ] 3.6: Generate new refresh token (rotation)
  - [ ] 3.7: Update database with new refresh token
  - [ ] 3.8: Invalidate old refresh token
  - [ ] 3.9: Set new refresh token in HTTP-only cookie
  - [ ] 3.10: Add integration tests for refresh flow

- [ ] Task 4: Implement session tracking (AC: #3, #8, #10)
  - [ ] 4.1: Create `Session` struct with device info, IP, user agent
  - [ ] 4.2: Implement `track_session_activity()` function
  - [ ] 4.3: Update last_used_at on each request
  - [ ] 4.4: Store user agent and IP on session creation
  - [ ] 4.5: Add unit tests for session tracking

- [ ] Task 5: Implement logout functionality (AC: #4)
  - [ ] 5.1: Create `logout()` endpoint
  - [ ] 5.2: Get refresh token from cookie
  - [ ] 5.3: Mark refresh token as revoked in database
  - [ ] 5.4: Add access token to blacklist
  - [ ] 5.5: Clear auth cookies (access and refresh)
  - [ ] 5.6: Return 204 No Content
  - [ ] 5.7: Add integration tests for logout

- [ ] Task 6: Implement session revocation (AC: #3, #5, #7)
  - [ ] 6.1: Create `revoke_session()` endpoint
  - [ ] 6.2: Accept session_id parameter
  - [ ] 6.3: Mark refresh token as revoked
  - [ ] 6.4: Add access token to blacklist
  - [ ] 6.5: Log revocation event
  - [ ] 6.6: Send email notification if enabled
  - [ ] 6.7: Add integration tests for revocation

- [ ] Task 7: Implement session list endpoint (AC: #3, #10)
  - [ ] 7.1: Create `GET /api/v1/auth/sessions` endpoint
  - [ ] 7.2: Fetch all active sessions for user
  - [ ] 7.3: Mark current session in response
  - [ ] 7.4: Include device info, IP, last activity
  - [ ] 7.5: Add pagination support
  - [ ] 7.6: Add integration tests for sessions list

- [ ] Task 8: Implement revoke all sessions (AC: #3, #5)
  - [ ] 8.1: Create `revoke_all_sessions()` endpoint
  - [ ] 8.2: Revoke all sessions except current (optional)
  - [ ] 8.3: Invalidate all refresh tokens
  - [ ] 8.4: Add all access tokens to blacklist
  - [ ] 8.5: Clear all cookies for user
  - [ ] 8.6: Send email notification about security action
  - [ ] 8.7: Add integration tests for revoke all

- [ ] Task 9: Implement concurrent session limit (AC: #8)
  - [ ] 9.1: Add concurrent_sessions_limit to config
  - [ ] 9.2: Check session count on new session creation
  - [ ] 9.3: Terminate oldest session if limit exceeded
  - [ ] 9.4: Log session termination event
  - [ ] 9.5: Notify user of session termination
  - [ ] 9.6: Add integration tests for session limit

- [ ] Task 10: Implement password change session termination (AC: #9)
  - [ ] 10.1: Hook into password change service
  - [ ] 10.2: Call revoke all sessions on password update
  - [ ] 10.3: Invalidate all refresh tokens
  - [ ] 10.4: Add integration tests for password change flow

- [ ] Task 11: Implement automatic token refresh on frontend (AC: #2)
  - [ ] 11.1: Create `useAuth` hook enhancement
  - [ ] 11.2: Implement token expiry detection (5 min before)
  - [ ] 11.3: Implement automatic refresh before expiry
  - [ ] 11.4: Handle refresh failure gracefully
  - [ ] 11.5: Show refresh indicator to user
  - [ ] 11.6: Add unit tests for refresh logic

- [ ] Task 12: Create session management UI (AC: #3, #10)
  - [ ] 12.1: Create `SessionsList` component
  - [ ] 12.2: Create `SessionCard` component with device info
  - [ ] 12.3: Add "Revoke" button for each session
  - [ ] 12.4: Add "Revoke All Other Sessions" button
  - [ ] 12.5: Show "Current Session" indicator
  - [ ] 12.6: Add session time remaining display
  - [ ] 12.7: Add loading states for actions

## Dev Notes

### Architecture Alignment

This story implements **Session Management & Refresh Tokens** per Epic 15 requirements:

- **Backend Location**: `crates/qa-pms-auth/src/sessions/`
- **Token Strategy**: Short-lived access (15min), long-lived refresh (7 days)
- **Security**: HTTP-only, Secure cookies, token rotation, token blacklisting
- **Performance**: Session caching, indexed queries

### Technical Implementation Details

#### Dependencies to Add
```toml
# crates/qa-pms-auth/Cargo.toml
[dependencies]
# Existing
jsonwebtoken = "9"
sqlx = { version = "0.7", features = ["postgres", "runtime-tokio"] }
chrono = "0.4"

# New for session management
user-agent-parser = "0.3"
maxminddb = { version = "0.23", optional = true } # For IP geolocation
```

#### Database Schema
```sql
-- Refresh tokens table (updated from 15.1)
ALTER TABLE refresh_tokens ADD COLUMN device_info JSONB;
ALTER TABLE refresh_tokens ADD COLUMN last_used_at TIMESTAMP DEFAULT NOW();
ALTER TABLE refresh_tokens ADD COLUMN is_revoked BOOLEAN DEFAULT FALSE;
ALTER TABLE refresh_tokens ADD COLUMN revoked_at TIMESTAMP;
ALTER TABLE refresh_tokens ADD COLUMN revoked_reason TEXT;

-- Create index for efficient queries
CREATE INDEX idx_refresh_tokens_user_id_not_revoked 
    ON refresh_tokens(user_id, is_revoked) 
    WHERE is_revoked = FALSE;

CREATE INDEX idx_refresh_tokens_expires_at 
    ON refresh_tokens(expires_at) 
    WHERE is_revoked = FALSE;

-- Token blacklist (from 15.1)
CREATE TABLE token_blacklist (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    jti TEXT NOT NULL UNIQUE,
    token_type TEXT NOT NULL, -- 'access' or 'refresh'
    user_id UUID REFERENCES users(id),
    expires_at TIMESTAMP NOT NULL,
    created_at TIMESTAMP DEFAULT NOW()
);

CREATE INDEX idx_token_blacklist_jti ON token_blacklist(jti);
CREATE INDEX idx_token_blacklist_expires_at ON token_blacklist(expires_at);
```

#### Token Generation Service
```rust
use chrono::{Duration, Utc};
use jsonwebtoken::{encode, EncodingKey, Header};
use rand::{thread_rng, Rng};

pub struct TokenPair {
    pub access_token: String,
    pub refresh_token: String,
    pub expires_in: u64, // Seconds until access token expires
}

pub async fn generate_token_pair(
    user_id: &Uuid,
    jwt_secret: &str,
) -> Result<TokenPair, AuthError> {
    // Generate JTI (JWT ID) for both tokens
    let access_jti = generate_jti();
    let refresh_jti = generate_jti();
    
    // Generate access token (15 minutes expiry)
    let access_expiry = Utc::now() + Duration::minutes(15);
    let access_claims = Claims {
        sub: user_id.to_string(),
        exp: access_expiry.timestamp(),
        iat: Utc::now().timestamp(),
        jti: access_jti.clone(),
        token_type: "access",
    };
    
    let access_token = encode(
        &Header::new(jwt_secret),
        &access_claims,
    )?;
    
    // Generate refresh token (7 days expiry)
    let refresh_expiry = Utc::now() + Duration::days(7);
    let refresh_claims = Claims {
        sub: user_id.to_string(),
        exp: refresh_expiry.timestamp(),
        iat: Utc::now().timestamp(),
        jti: refresh_jti.clone(),
        token_type: "refresh",
    };
    
    let refresh_token = generate_refresh_token_string();
    let refresh_token_hash = hash_token(&refresh_token)?;
    
    Ok(TokenPair {
        access_token,
        refresh_token,
        expires_in: 60 * 15, // 900 seconds (15 minutes)
    })
}

fn generate_jti() -> String {
    uuid::Uuid::new_v4().to_string()
}

pub fn hash_token(token: &str) -> Result<String, AuthError> {
    use sha2::{Sha256, Digest};
    let mut hasher = Sha256::new();
    hasher.update(token.as_bytes());
    Ok(hex::encode(hasher.finalize()))
}
```

#### Token Refresh Endpoint
```rust
use axum::{
    extract::{State, TypedHeader},
    http::StatusCode,
    Json, Request,
};
use tower_cookies::{Cookie, Cookies};

#[derive(Deserialize)]
pub struct RefreshRequest {
    // No body needed, token from cookie
}

#[derive(Serialize)]
pub struct RefreshResponse {
    pub access_token: String,
    pub expires_in: u64,
}

#[utoipa::path(
    request_body = RefreshRequest,
    responses(
        (status = 200, description = "Token refreshed successfully", body = RefreshResponse),
        (status = 401, description = "Invalid or expired refresh token"),
        (status = 403, description = "Refresh token revoked"),
    ),
    tag = "Authentication"
)]
pub async fn refresh_token(
    State(auth_service): State<Arc<AuthService>>,
    State(refresh_service): State<Arc<RefreshService>>,
    cookies: Cookies,
) -> Result<Json<RefreshResponse>, ApiError> {
    // Get refresh token from cookie
    let refresh_token = cookies
        .get("refresh_token")
        .and_then(|c| c.value())
        .ok_or(ApiError::MissingRefreshToken)?;
    
    // Validate refresh token
    let session = refresh_service
        .validate_refresh_token(&refresh_token)
        .await?;
    
    if !session.is_valid {
        // Delete invalid refresh token from cookie
        let mut clear_cookie = Cookie::from(("refresh_token", ""));
        clear_cookie.set_max_age(time::Duration::ZERO);
        cookies.add(clear_cookie);
        
        return Err(ApiError::InvalidRefreshToken);
    }
    
    // Generate new tokens
    let new_tokens = auth_service
        .generate_token_pair(&session.user_id)
        .await?;
    
    // Store new refresh token in database
    refresh_service
        .store_refresh_token(&session.user_id, &new_tokens.refresh_token)
        .await?;
    
    // Invalidate old refresh token
    refresh_service
        .invalidate_refresh_token(&refresh_token)
        .await?;
    
    // Set new refresh token in HTTP-only, Secure cookie
    let cookie = Cookie::build(("refresh_token", new_tokens.refresh_token))
        .http_only(true)
        .secure(true)
        .same_site(tower_cookies::cookie::SameSite::Lax)
        .max_age(time::Duration::days(7))
        .path("/");
    
    cookies.add(cookie);
    
    Ok(Json(RefreshResponse {
        access_token: new_tokens.access_token,
        expires_in: new_tokens.expires_in,
    }))
}
```

#### Session Service
```rust
use sqlx::PgPool;
use std::sync::Arc;
use user_agent_parser::UserAgentParser;

#[derive(Debug, Clone)]
pub struct SessionInfo {
    pub id: Uuid,
    pub user_id: Uuid,
    pub token_hash: String,
    pub device_info: DeviceInfo,
    pub ip_address: Option<String>,
    pub user_agent: String,
    pub created_at: DateTime<Utc>,
    pub last_used_at: DateTime<Utc>,
    pub is_current: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceInfo {
    pub browser: String,
    pub os: String,
    pub device_type: String, // 'desktop', 'mobile', 'tablet'
    pub name: Option<String>, // Custom device name
}

pub struct SessionService {
    db: Arc<PgPool>,
    user_agent_parser: UserAgentParser,
}

impl SessionService {
    pub async fn create_session(
        &self,
        user_id: &Uuid,
        refresh_token: &str,
        ip_address: Option<String>,
        user_agent: String,
    ) -> Result<SessionInfo, DbError> {
        // Parse user agent
        let parsed = self.user_agent_parser.parse(&user_agent);
        
        let device_info = DeviceInfo {
            browser: parsed.browser,
            os: parsed.os,
            device_type: parsed.device_type,
            name: None,
        };
        
        let token_hash = hash_token(refresh_token)?;
        
        let session = sqlx::query!(
            r#"
            INSERT INTO refresh_tokens 
                (user_id, token_hash, device_info, ip_address, user_agent, created_at, last_used_at)
            VALUES ($1, $2, $3, $4, $5, $6, NOW(), NOW())
            RETURNING id, user_id, token_hash, device_info, ip_address, 
                      user_agent, created_at, last_used_at
            "#
        )
        .bind(user_id, &token_hash, &device_info, &ip_address, &user_agent)
        .fetch_one(&*self.db)
        .await?;
        
        Ok(session)
    }
    
    pub async fn get_active_sessions(
        &self,
        user_id: &Uuid,
    ) -> Result<Vec<SessionInfo>, DbError> {
        sqlx::query!(
            r#"
            SELECT id, user_id, token_hash, device_info, ip_address, 
                   user_agent, created_at, last_used_at
            FROM refresh_tokens
            WHERE user_id = $1 
              AND is_revoked = FALSE 
              AND expires_at > NOW()
            ORDER BY last_used_at DESC
            "#
        )
        .bind(user_id)
        .fetch_all(&*self.db)
        .await
    }
    
    pub async fn revoke_session(
        &self,
        user_id: &Uuid,
        session_id: &Uuid,
        reason: Option<&str>,
    ) -> Result<(), DbError> {
        sqlx::query!(
            r#"
            UPDATE refresh_tokens
            SET is_revoked = TRUE,
                revoked_at = NOW(),
                revoked_reason = $1
            WHERE id = $2 AND user_id = $3
            "#
        )
        .bind(reason, session_id, user_id)
        .execute(&*self.db)
        .await?;
        
        Ok(())
    }
    
    pub async fn revoke_all_sessions(
        &self,
        user_id: &Uuid,
        current_session_id: Option<&Uuid>,
    ) -> Result<usize, DbError> {
        let result = sqlx::query!(
            r#"
            UPDATE refresh_tokens
            SET is_revoked = TRUE,
                revoked_at = NOW(),
                revoked_reason = 'user_requested'
            WHERE user_id = $1
              AND is_revoked = FALSE
              AND id != $2
            RETURNING id
            "#
        )
        .bind(user_id, current_session_id)
        .fetch_all(&*self.db)
        .await?;
        
        Ok(result.len())
    }
}
```

#### Frontend Auth Hook with Auto-Refresh
```typescript
// frontend/src/hooks/useAuth.ts
import { useState, useEffect } from 'react';
import { api } from '@/api';

export const useAuth = () => {
  const [accessToken, setAccessToken] = useState<string | null>(null);
  const [refreshing, setRefreshing] = useState(false);
  const [expiresAt, setExpiresAt] = useState<Date | null>(null);

  // Check token expiry every minute
  useEffect(() => {
    const checkExpiry = () => {
      if (!expiresAt || !accessToken || refreshing) return;
      
      const now = new Date();
      const expiresIn = expiresAt.getTime() - now.getTime();
      
      // Refresh 5 minutes before expiry
      if (expiresIn < 5 * 60 * 1000) {
        autoRefresh();
      }
    };
    
    const interval = setInterval(checkExpiry, 60 * 1000); // Every minute
    return () => clearInterval(interval);
  }, [expiresAt, accessToken, refreshing]);

  const autoRefresh = async () => {
    setRefreshing(true);
    try {
      const response = await api.post('/api/v1/auth/refresh');
      setAccessToken(response.data.access_token);
      setExpiresAt(new Date(Date.now() + response.data.expires_in * 1000));
    } catch (error) {
      // If refresh fails, redirect to login
      window.location.href = '/auth/login';
    } finally {
      setRefreshing(false);
    }
  };

  const logout = async () => {
    await api.post('/api/v1/auth/logout');
    setAccessToken(null);
    setExpiresAt(null);
    window.location.href = '/auth/login';
  };

  return {
    accessToken,
    refreshing,
    logout,
    setTokens: (tokens: AuthTokens) => {
      setAccessToken(tokens.access_token);
      setExpiresAt(new Date(Date.now() + tokens.expires_in * 1000));
    },
  };
};
```

#### Sessions List Component
```typescript
// frontend/src/components/auth/SessionsList.tsx
import { useState, useEffect } from 'react';
import { api } from '@/api';
import { Trash2, Smartphone, Monitor, Shield } from 'lucide-react';

interface Session {
  id: string;
  device_info: {
    browser: string;
    os: string;
    device_type: string;
    name?: string;
  };
  ip_address?: string;
  last_used_at: string;
  created_at: string;
  is_current: boolean;
}

export const SessionsList: React.FC = () => {
  const [sessions, setSessions] = useState<Session[]>([]);
  const [loading, setLoading] = useState(false);
  const [revoking, setRevoking] = useState<Set<string>>(new Set());

  useEffect(() => {
    loadSessions();
  }, []);

  const loadSessions = async () => {
    setLoading(true);
    try {
      const response = await api.get('/api/v1/auth/sessions');
      setSessions(response.data);
    } catch (error) {
      console.error('Failed to load sessions:', error);
    } finally {
      setLoading(false);
    }
  };

  const revokeSession = async (sessionId: string) => {
    setRevoking(prev => new Set(prev).add(sessionId));
    try {
      await api.post(`/api/v1/auth/sessions/${sessionId}/revoke`);
      setSessions(prev => prev.filter(s => s.id !== sessionId));
    } catch (error) {
      console.error('Failed to revoke session:', error);
    } finally {
      setRevoking(prev => {
        const next = new Set(prev);
        next.delete(sessionId);
        return next;
      });
    }
  };

  const revokeAllOtherSessions = async () => {
    if (!confirm('Are you sure you want to revoke all other sessions?')) return;
    
    try {
      const response = await api.post('/api/v1/auth/sessions/revoke-all');
      // Keep current session, remove others
      setSessions(prev => prev.filter(s => s.is_current));
      toast.success(`Revoked ${response.data.count} other sessions`);
    } catch (error) {
      console.error('Failed to revoke sessions:', error);
    }
  };

  const getDeviceIcon = (deviceType: string) => {
    switch (deviceType) {
      case 'desktop': return <Monitor className="icon" />;
      case 'mobile': return <Smartphone className="icon" />;
      case 'tablet': return <Shield className="icon" />;
      default: return <Monitor className="icon" />;
    }
  };

  if (loading) {
    return <div className="sessions-loading">Loading sessions...</div>;
  }

  return (
    <div className="sessions-list">
      <div className="header">
        <h2>Active Sessions</h2>
        <button 
          onClick={revokeAllOtherSessions}
          className="revoke-all-button"
        >
          <Shield className="icon" />
          Revoke All Other Sessions
        </button>
      </div>

      <div className="sessions-grid">
        {sessions.map(session => (
          <div key={session.id} className={`session-card ${session.is_current ? 'current' : ''}`}>
            <div className="session-header">
              <div className="device-info">
                {getDeviceIcon(session.device_info.device_type)}
                <div className="device-details">
                  <div className="browser">{session.device_info.browser}</div>
                  <div className="os">{session.device_info.os}</div>
                </div>
              </div>
              {session.is_current && (
                <span className="current-badge">Current Session</span>
              )}
            </div>
              
              <div className="session-meta">
                <div className="last-used">
                  <span className="label">Last used:</span>
                  <span className="value">
                    {new Date(session.last_used_at).toLocaleString()}
                  </span>
                </div>
                {session.ip_address && (
                  <div className="ip-address">
                    <span className="label">IP:</span>
                    <span className="value">{session.ip_address}</span>
                  </div>
                )}
              </div>

              {!session.is_current && (
                <button
                  onClick={() => revokeSession(session.id)}
                  disabled={revoking.has(session.id)}
                  className="revoke-button"
                  title="Revoke this session"
                >
                  {revoking.has(session.id) ? (
                    <span className="revoking">Revoking...</span>
                  ) : (
                    <Trash2 className="icon" />
                  )}
                </button>
              )}
            </div>

            <div className="session-footer">
              <div className="created-at">
                <span className="label">Created:</span>
                <span className="value">
                  {new Date(session.created_at).toLocaleString()}
                </span>
              </div>
              {session.device_info.name && (
                <div className="device-name">
                  {session.device_info.name}
                </div>
              )}
            </div>
          </div>
        ))}
      </div>
    </div>
  );
};
```

### Testing Strategy

#### Unit Tests
- Token generation logic
- Token hashing
- Token validation
- Session creation
- Session revocation
- Device info parsing

#### Integration Tests
- Complete login flow with session creation
- Token refresh flow
- Session list retrieval
- Session revocation
- Revoke all sessions
- Password change session termination
- Concurrent session limit enforcement

#### End-to-End Tests
- User logs in and sees session created
- Token refreshes automatically before expiry
- User views active sessions
- User revokes specific session
- User revokes all other sessions
- Multiple concurrent sessions handled correctly

#### Security Tests
- Attempt to use revoked refresh token
- Attempt to use expired refresh token
- Attempt to use refresh token from different IP (if configured)
- Concurrent session limit exceeded
- Session hijacking attempts
- Token replay prevention

### File List

**Files to be created:**
- `crates/qa-pms-auth/src/sessions/mod.rs`
- `crates/qa-pms-auth/src/sessions/service.rs`
- `crates/qa-pms-auth/src/sessions/types.rs`
- `migrations/update_refresh_tokens_table.sql`
- `migrations/create_token_blacklist_table.sql`
- `frontend/src/hooks/useAuth.ts`
- `frontend/src/components/auth/SessionsList.tsx`

**Files to be modified:**
- `crates/qa-pms-auth/Cargo.toml` (add user-agent-parser dependency)
- `crates/qa-pms-auth/src/lib.rs` (add session module)
- `crates/qa-pms-api/src/main.rs` (add session routes)
- `frontend/src/api/auth.ts` (add session endpoints)