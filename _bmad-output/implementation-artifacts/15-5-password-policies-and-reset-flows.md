# crates/qa-pms-auth/Cargo.toml
[dependencies]
# Existing
argon2 = "0.5"
rand = "0.8"
chrono = "0.4"

# New for password policies
validator = "0.18"
regex = "1.10"

# Email service
lettre = "0.11"
askama = "0.12"  # Email templates
handlebars = "5.0"  # Alternative template engine
```

#### Database Schema
```sql
-- Password reset tokens table
CREATE TABLE password_reset_tokens (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id),
    token_hash TEXT NOT NULL UNIQUE,
    expires_at TIMESTAMP NOT NULL,
    used_at TIMESTAMP,
    ip_address INET,
    user_agent TEXT,
    created_at TIMESTAMP DEFAULT NOW()
);

-- Password history table (if not created in 15.1)
CREATE TABLE password_history (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id),
    password_hash TEXT NOT NULL,
    created_at TIMESTAMP DEFAULT NOW(),
    UNIQUE(user_id, password_hash) -- Prevent exact duplicates
);

-- Indexes
CREATE INDEX idx_password_reset_tokens_user_id ON password_reset_tokens(user_id);
CREATE INDEX idx_password_reset_tokens_expires_at ON password_reset_tokens(expires_at);
CREATE INDEX idx_password_history_user_id ON password_history(user_id);
CREATE INDEX idx_password_history_created_at ON password_history(created_at);

-- Cleanup constraint
ALTER TABLE password_history ADD CONSTRAINT chk_password_history_limit 
    CHECK (user_id IN (
        SELECT user_id FROM password_history ph
        WHERE ph.user_id = password_history.user_id
        ORDER BY ph.created_at DESC
        LIMIT 1 OFFSET 19
    ));
```

#### Password Policy Validation
```rust
use regex::Regex;
use validator::ValidateLength;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PasswordPolicy {
    pub min_length: usize,
    pub require_uppercase: bool,
    pub require_lowercase: bool,
    pub require_number: bool,
    pub require_special_char: bool,
    pub max_history: usize,
}

impl Default for PasswordPolicy {
    fn default() -> Self {
        Self {
            min_length: 12,
            require_uppercase: true,
            require_lowercase: true,
            require_number: true,
            require_special_char: true,
            max_history: 5,
        }
    }
}

#[derive(Debug)]
pub enum PasswordValidationError {
    TooShort,
    MissingUppercase,
    MissingLowercase,
    MissingNumber,
    MissingSpecialChar,
    ContainsEmail,
    ReusedPassword,
    SameAsCurrent,
}

impl std::fmt::Display for PasswordValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            PasswordValidationError::TooShort => write!(f, "Password must be at least 12 characters"),
            PasswordValidationError::MissingUppercase => write!(f, "Password must contain at least one uppercase letter"),
            PasswordValidationError::MissingLowercase => write!(f, "Password must contain at least one lowercase letter"),
            PasswordValidationError::MissingNumber => write!(f, "Password must contain at least one number"),
            PasswordValidationError::MissingSpecialChar => write!(f, "Password must contain at least one special character (!@#$%^&*"),
            PasswordValidationError::ContainsEmail => write!(f, "Password cannot contain parts of your email address"),
            PasswordValidationError::ReusedPassword => write!(f, "This password was used recently. Please choose a different one"),
            PasswordValidationError::SameAsCurrent => write!(f, "New password must be different from your current password"),
        }
    }
}

pub fn validate_password_with_policy(
    password: &str,
    email: &str,
    policy: &PasswordPolicy,
    password_history: Option<&[String]>,
) -> Result<(), Vec<PasswordValidationError>> {
    let mut errors = Vec::new();
    
    // Length check
    if password.len() < policy.min_length {
        errors.push(PasswordValidationError::TooShort);
    }
    
    // Uppercase check
    if policy.require_uppercase && !password.chars().any(|c| c.is_uppercase()) {
        errors.push(PasswordValidationError::MissingUppercase);
    }
    
    // Lowercase check
    if policy.require_lowercase && !password.chars().any(|c| c.is_lowercase()) {
        errors.push(PasswordValidationError::MissingLowercase);
    }
    
    // Number check
    if policy.require_number && !password.chars().any(|c| c.is_numeric()) {
        errors.push(PasswordValidationError::MissingNumber);
    }
    
    // Special character check
    let special_regex = Regex::new(r"[!@#$%^&*]").unwrap();
    if policy.require_special_char && !special_regex.is_match(password) {
        errors.push(PasswordValidationError::MissingSpecialChar);
    }
    
    // Email parts check
    let email_parts: Vec<&str> = email.split('@').collect();
    if let Some(username) = email_parts.get(0) {
        if password.to_lowercase().contains(&username.to_lowercase()) {
            errors.push(PasswordValidationError::ContainsEmail);
        }
    }
    
    // Password history check
    if let Some(history) = password_history {
        let new_hash = hash_password(password)?;
        for old_hash in history {
            if old_hash == &new_hash {
                errors.push(PasswordValidationError::ReusedPassword);
                break;
            }
        }
    }
    
    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}
```

#### Password Reset Token Generation
```rust
use rand::{thread_rng, Rng};
use sha2::{Sha256, Digest};
use base64::{Engine, engine::general_purpose::URL_SAFE_NO_PAD};

pub fn generate_reset_token() -> Result<String, anyhow::Error> {
    // Generate 32-byte cryptographically secure random token
    let mut rng = thread_rng();
    let token: String = (0..32)
        .map(|_| {
            const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
            CHARSET[rng.gen_range(0..CHARSET.len())] as char
        })
        .collect();
    
    Ok(token)
}

pub fn hash_token(token: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(token.as_bytes());
    let hash = hasher.finalize();
    
    URL_SAFE_NO_PAD.encode(hash)
}
```

#### Password Reset Request Endpoint
```rust
use axum::{
    extract::State,
    http::StatusCode,
    Json, Request,
};
use serde::Deserialize;
use validator::Validate;
use tower_governor::{Governor, GovernorConfigBuilder};

#[derive(Deserialize, Validate)]
pub struct ForgotPasswordRequest {
    #[validate(email)]
    pub email: String,
}

#[utoipa::path(
    request_body = ForgotPasswordRequest,
    responses(
        (status = 200, description = "If email exists, reset link sent"),
        (status = 429, description = "Too many requests"),
    ),
    tag = "Password"
)]
pub async fn request_password_reset(
    State(password_service): State<Arc<PasswordService>>,
    State(governor_conf): State<Arc<Governor_conf>>,
    Json(request): Json<ForgotPasswordRequest>,
) -> Result<StatusCode, ApiError> {
    // Rate limiting check
    let rate_limit_key = format!("password_reset:{}", request.email);
    
    // Generate reset token
    let token = generate_reset_token()?;
    let token_hash = hash_token(&token);
    
    // Store token in database
    let expires_at = chrono::Utc::now() + chrono::Duration::hours(1);
    password_service.store_reset_token(request.email.clone(), token_hash, expires_at).await?;
    
    // Send email
    let reset_link = format!(
        "http://localhost:5173/auth/reset-password?token={}",
        token
    );
    
    email_service.send_password_reset_email(&request.email, &reset_link).await?;
    
    Ok(StatusCode::OK)
}
```

#### Password Reset Completion Endpoint
```rust
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct ResetPasswordRequest {
    pub token: String,
    pub new_password: String,
    pub confirm_password: String,
}

#[utoipa::path(
    request_body = ResetPasswordRequest,
    params(
        ("token", description = "Password reset token"),
    ),
    responses(
        (status = 200, description = "Password reset successfully"),
        (status = 400, description = "Invalid token or password mismatch"),
    ),
    tag = "Password"
)]
pub async fn reset_password(
    State(password_service): State<Arc<PasswordService>>,
    Json(request): Json<ResetPasswordRequest>,
) -> Result<StatusCode, ApiError> {
    // Validate token
    let token_hash = hash_token(&request.token);
    
    if !password_service.validate_reset_token(&token_hash).await? {
        return Err(ApiError::InvalidOrExpiredToken);
    }
    
    // Validate passwords match
    if request.new_password != request.confirm_password {
        return Err(ApiError::PasswordMismatch);
    }
    
    // Validate password policy
    let user = password_service.get_user_by_email(&email).await?;
    let password_history = password_service.get_password_history(&user.id).await?;
    let policy = PasswordPolicy::default();
    
    if let Err(errors) = validate_password_with_policy(
        &request.new_password,
        &user.email,
        &policy,
        Some(&password_history),
    ) {
        return Err(ApiError::PasswordValidationErrors(errors));
    }
    
    // Hash new password
    let new_hash = argon2::hash_encoded(&request.new_password, &salt, &config)?;
    
    // Update user password
    password_service.update_password(&user.id, new_hash).await?;
    
    // Record in password history
    password_service.record_password_in_history(&user.id, new_hash).await?;
    
    // Mark token as used
    password_service.mark_reset_token_used(&token_hash).await?;
    
    // Terminate all user sessions
    password_service.terminate_all_sessions(&user.id).await?;
    
    // Clear failed login attempts
    password_service.reset_failed_attempts(&user.id).await?;
    
    Ok(StatusCode::OK)
}
```

#### Change Password Endpoint
```rust
use axum::{
    extract::State,
    http::StatusCode,
    Json,
};

#[derive(Deserialize)]
pub struct ChangePasswordRequest {
    pub current_password: String,
    pub new_password: String,
    pub confirm_password: String,
}

#[utoipa::path(
    request_body = ChangePasswordRequest,
    responses(
        (status = 200, description = "Password changed successfully"),
        (status = 400, description = "Validation errors"),
        (status = 401, description = "Current password incorrect"),
    ),
    tag = "Password"
)]
pub async fn change_password(
    State(auth_service): State<Arc<AuthService>>,
    State(password_service): State<Arc<PasswordService>>,
    Extension(user_context): Extension<UserContext>,
    Json(request): Json<ChangePasswordRequest>,
) -> Result<StatusCode, ApiError> {
    // Verify current password
    if !auth_service.verify_password(&user_context.user_id, &request.current_password).await? {
        return Err(ApiError::InvalidCurrentPassword);
    }
    
    // Validate passwords match
    if request.new_password != request.confirm_password {
        return Err(ApiError::PasswordMismatch);
    }
    
    // Get user and password history
    let user = password_service.get_user(&user_context.user_id).await?;
    let password_history = password_service.get_password_history(&user.id).await?;
    let current_hash = password_service.get_password_hash(&user.id).await?;
    let policy = PasswordPolicy::default();
    
    // Validate new password
    if let Err(mut errors) = validate_password_with_policy(
        &request.new_password,
        &user.email,
        &policy,
        Some(&password_history),
    ) {
        // Check if new password is same as current
        let new_hash = argon2::hash_encoded(&request.new_password, &salt, &config)?;
        if new_hash == current_hash {
            errors.push(PasswordValidationError::SameAsCurrent);
        }
        return Err(ApiError::PasswordValidationErrors(errors));
    }
    
    // Hash new password
    let new_hash = argon2::hash_encoded(&request.new_password, &salt, &config)?;
    
    // Update password
    password_service.update_password(&user.id, new_hash).await?;
    
    // Record in history
    password_service.record_password_in_history(&user.id, new_hash).await?;
    
    // Terminate all other sessions
    password_service.terminate_all_sessions_except(&user.id, &user_context.session_id).await?;
    
    // Log password change
    audit_service.log_password_change(&user.id, "password_changed").await?;
    
    Ok(StatusCode::OK)
}
```

#### Password Reset Email Template
```handlebars
<!DOCTYPE html>
<html>
<head>
    <meta charset="UTF-8">
    <style>
        body { font-family: Arial, sans-serif; line-height: 1.6; color: #333; }
        .container { max-width: 600px; margin: 0 auto; padding: 20px; }
        .header { text-align: center; margin-bottom: 30px; }
        .logo { font-size: 24px; color: #2563eb; }
        .content { background: #f9f9f9; padding: 30px; border-radius: 8px; }
        .button { background: #2563eb; color: white; padding: 12px 24px; 
                text-decoration: none; border-radius: 4px; display: inline-block; 
                margin-top: 20px; }
        .security-tips { margin-top: 30px; padding: 20px; background: #e8f4f8; 
                         border-radius: 8px; }
        .tip { margin-bottom: 10px; padding-left: 20px; }
    </style>
</head>
<body>
    <div class="container">
        <div class="header">
            <div class="logo">🔐 QA Intelligent PMS</div>
            <h1>Password Reset Request</h1>
        </div>
        
        <div class="content">
            <p>Hello,</p>
            
            <p>We received a request to reset your password for your QA Intelligent PMS account.</p>
            
            <p>If you didn't make this request, you can safely ignore this email.</p>
            
            <p style="text-align: center; margin: 30px 0;">
                <a href="{{reset_link}}" class="button">Reset My Password</a>
            </p>
            
            <p><strong>Important:</strong> This link will expire in 1 hour for your security.</p>
        </div>
        
        <div class="security-tips">
            <h3>🔒 Security Tips</h3>
            <div class="tip">✓ Choose a password with at least 12 characters</div>
            <div class="tip">✓ Include uppercase and lowercase letters</div>
            <div class="tip">✓ Add numbers and special characters (!@#$%^&*)</div>
            <div class="tip">✓ Don't reuse previous passwords</div>
            <div class="tip">✓ Don't include parts of your email address</div>
        </div>
        
        <div style="text-align: center; margin-top: 40px; color: #666;">
            <p>© 2025 QA Intelligent PMS. All rights reserved.</p>
        </div>
    </div>
</body>
</html>
```

#### Frontend Forgot Password Component
```typescript
// frontend/src/components/auth/ForgotPassword.tsx
import { useState } from 'react';
import { api } from '@/api';

const ForgotPassword: React.FC = () => {
  const [email, setEmail] = useState('');
  const [loading, setLoading] = useState(false);
  const [sent, setSent] = useState(false);
  const [error, setError] = useState<string | null>(null);
  
  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    
    if (!email) {
      setError('Please enter your email address');
      return;
    }
    
    setLoading(true);
    setError(null);
    
    try {
      await api.post('/api/v1/auth/password/reset-request', { email });
      setSent(true);
    } catch (err) {
      if (err.status === 429) {
        setError('Too many requests. Please try again later.');
      } else {
        setError('Failed to send reset link. Please try again.');
      }
    } finally {
      setLoading(false);
    }
  };
  
  if (sent) {
    return (
      <div className="forgot-password-success">
        <CheckCircle className="icon success" size={64} />
        <h2>Reset Link Sent</h2>
        <p>
          If an account exists with {email}, you will receive a password reset link shortly.
        </p>
        <p>The link will expire in 1 hour.</p>
        <button onClick={() => setSent(false)}>
          Send Another Request
        </button>
      </div>
    );
  }
  
  return (
    <div className="forgot-password">
      <h1>Forgot Password</h1>
      <p>Enter your email address and we'll send you a link to reset your password.</p>
      
      <form onSubmit={handleSubmit}>
        <div className="form-group">
          <label htmlFor="email">Email Address</label>
          <input
            type="email"
            id="email"
            value={email}
            onChange={(e) => setEmail(e.target.value)}
            placeholder="you@example.com"
            required
            disabled={loading}
          />
        </div>
        
        {error && (
          <Alert variant="error" className="error-message">
            {error}
          </Alert>
        )}
        
        <button type="submit" disabled={loading}>
          {loading ? 'Sending...' : 'Send Reset Link'}
        </button>
      </form>
      
      <div className="back-link">
        <a href="/auth/login">
          <ArrowLeft className="icon" />
          Back to Login
        </a>
      </div>
    </div>
  );
};
```

#### Frontend Reset Password Component
```typescript
// frontend/src/components/auth/ResetPassword.tsx
import { useState } from 'react';
import { useParams } from 'react-router-dom';
import { api } from '@/api';

const ResetPassword: React.FC = () => {
  const { token } = useParams<{ token: string }>();
  const [newPassword, setNewPassword] = useState('');
  const [confirmPassword, setConfirmPassword] = useState('');
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [success, setSuccess] = useState(false);
  
  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    
    setError(null);
    
    // Validate passwords match
    if (newPassword !== confirmPassword) {
      setError('Passwords do not match');
      return;
    }
    
    setLoading(true);
    
    try {
      await api.post('/api/v1/auth/password/reset', {
        token,
        new_password: newPassword,
        confirm_password: confirmPassword,
      });
      
      setSuccess(true);
    } catch (err) {
      setError(err.message || 'Failed to reset password');
    } finally {
      setLoading(false);
    }
  };
  
  if (success) {
    return (
      <div className="reset-password-success">
        <CheckCircle className="icon success" size={64} />
        <h2>Password Reset Successfully</h2>
        <p>You can now log in with your new password.</p>
        <button onClick={() => window.location.href = '/auth/login'}>
          Go to Login
        </button>
      </div>
    );
  }
  
  return (
    <div className="reset-password">
      <h1>Reset Password</h1>
      <p>Enter your new password below.</p>
      
      <form onSubmit={handleSubmit}>
        <div className="form-group">
          <label htmlFor="new_password">New Password</label>
          <input
            type="password"
            id="new_password"
            value={newPassword}
            onChange={(e) => setNewPassword(e.target.value)}
            placeholder="Enter new password"
            required
            disabled={loading}
            minLength={12}
          />
          <PasswordStrengthMeter password={newPassword} />
        </div>
        
        <div className="form-group">
          <label htmlFor="confirm_password">Confirm Password</label>
          <input
            type="password"
            id="confirm_password"
            value={confirmPassword}
            onChange={(e) => setConfirmPassword(e.target.value)}
            placeholder="Confirm new password"
            required
            disabled={loading}
            minLength={12}
          />
        </div>
        
        {error && (
          <Alert variant="error" className="error-message">
            {error}
          </Alert>
        )}
        
        <button type="submit" disabled={loading || !newPassword || !confirmPassword}>
          {loading ? 'Resetting...' : 'Reset Password'}
        </button>
      </form>
      
      <div className="password-requirements">
        <h3>Password Requirements:</h3>
        <ul>
          <li>✓ At least 12 characters</li>
          <li>✓ At least one uppercase letter</li>
          <li>✓ At least one lowercase letter</li>
          <li>✓ At least one number</li>
          <li>✓ At least one special character (!@#$%^&*)</li>
          <li>✓ Not contain parts of your email</li>
        </ul>
      </div>
    </div>
  );
};
```

### Testing Strategy

#### Unit Tests
- Password policy validation rules
- Token generation and hashing
- Token validation logic
- Password history checking
- Email address validation

#### Integration Tests
- Complete password reset request flow
- Password reset completion with valid token
- Password reset with expired token
- Password reset with used token
- Password change flow
- Rate limiting enforcement
- Token cleanup job

#### End-to-End Tests
- User requests password reset via forgot password page
- User receives email with reset link
- User clicks reset link
- User enters new password
- User is redirected to login page
- User logs in with new password
- Rate limiting prevents abuse
- Token cleanup removes expired tokens

#### Security Tests
- Attempt reset with invalid token
- Attempt reset with expired token
- Attempt reset with already used token
- Attempt reset with weak password
- Attempt reset with password reuse
- Attempt to reset password too quickly (rate limiting)
- Attempt change password without current password
- Attempt change password with same password

### File List

**Files to be created:**
- `crates/qa-pms-auth/src/password/mod.rs`
- `crates/qa-pms-auth/src/password/policy.rs`
- `crates/qa-pms-auth/src/password/service.rs`
- `crates/qa-pms-auth/src/password/reset.rs`
- `migrations/create_password_reset_tokens_table.sql`
- `migrations/create_password_history_table.sql`
- `frontend/src/components/auth/ForgotPassword.tsx`
- `frontend/src/components/auth/ResetPassword.tsx`
- `frontend/src/components/auth/PasswordStrengthMeter.tsx`
- `frontend/templates/password_reset.html`

**Files to be modified:**
- `crates/qa-pms-auth/Cargo.toml` (add validator, regex dependencies)
- `crates/qa-pms-api/src/main.rs` (add password reset routes)
- `frontend/src/api/auth.ts` (add password reset methods)
- `crates/qa-pms-auth/src/middleware/rate_limiting.rs` (extend for password endpoints)