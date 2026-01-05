# Story 15.4: Multi-Factor Authentication (MFA)

Status: ready-for-dev

## Story

As a Security-Conscious QA Lead,
I want to enable MFA for my account,
So that my account remains secure even if my password is compromised.

## Acceptance Criteria

1. **Given** a user with account enabled
   **When** they navigate to Security Settings
   **Then** they can enable MFA using TOTP (Time-based One-Time Password)
   **And** they see a QR code to scan with authenticator app
   **And** they must verify a code to confirm setup

2. **Given** MFA is enabled on user account
   **When** they attempt to login
   **Then** after successful password/OAuth authentication
   **And** they are prompted for 6-digit TOTP code
   **And** system validates code with 30-second tolerance
   **And** grants access only if code is correct

3. **Given** user loses access to authenticator app
   **When** they use backup codes
   **Then** they can login with one-time backup code
   **And** backup code is consumed after use
   **And** they are prompted to regenerate backup codes

4. **Given** MFA is being enabled
   **When** setup is initiated
   **Then** system generates TOTP secret
   **And** generates 10 backup codes
   **And** displays QR code for scanning
   **And** shows backup codes (only once)
   **And** stores TOTP secret encrypted in database

5. **Given** user completes MFA setup
   **When** they enter correct verification code
   **Then** MFA is marked as enabled on their account
   **And** secret is stored encrypted
   **And** backup codes are stored encrypted
   **And** user is returned to security settings

6. **Given** user has MFA enabled
   **When** they want to disable MFA
   **Then** they must re-authenticate with password and MFA code
   **And** system confirms disable action
   **And** MFA secret and backup codes are deleted from database
   **And** user is returned to security settings with MFA disabled

7. **Given** MFA is optional per role
   **When** checking user's role requirements
   **Then** admin role requires MFA
   **And** qa_lead role can optionally enable MFA
   **And** qa_engineer role can optionally enable MFA
   **And** viewer role cannot enable MFA

8. **Given** MFA is required but not enabled for user
   **When** user attempts to access protected resource
   **Then** system redirects user to MFA setup page
   **And** shows clear message about MFA requirement
   **And** prevents access until MFA is enabled

9. **Given** TOTP code is entered during login
   **When** code is invalid
   **Then** system shows error message
   **And** allows 3 retry attempts
   **And** after 3 failed attempts, login flow restarts

10. **Given** backup codes are generated
   **When** user wants to regenerate them
   **Then** old backup codes are invalidated
   **And** 10 new backup codes are generated
   **And** old codes cannot be used anymore
   **And** new codes are displayed only once

## Tasks / Subtasks

- [ ] Task 1: Setup MFA database schema (AC: #4, #5, #10)
  - [ ] 1.1: Add mfa_enabled, mfa_secret_encrypted columns to users table
  - [ ] 1.2: Add backup_codes column to users table (TEXT[] array, encrypted)
  - [ ] 1.3: Add mfa_enabled_at column to users table
  - [ ] 1.4: Create migration file
  - [ ] 1.5: Add indexes for MFA queries
  - [ ] 1.6: Add unit tests for schema changes

- [ ] Task 2: Implement TOTP generation and validation (AC: #1, #4, #9)
  - [ ] 2.1: Create `mfa.rs` module in `qa-pms-auth`
  - [ ] 2.2: Implement `generate_totp_secret()` using crypto-secure RNG
  - [ ] 2.3: Implement `generate_backup_codes()` (10 codes, 8 chars each)
  - [ ] 2.4: Implement `generate_qr_code()` for authenticator apps
  - [ ] 2.5: Implement `validate_totp_code()` with 30-second tolerance
  - [ ] 2.6: Add unit tests for TOTP operations

- [ ] Task 3: Implement MFA setup flow (AC: #1, #4, #5)
  - [ ] 3.1: Create `setup_mfa()` function
  - [ ] 3.2: Generate TOTP secret and backup codes
  - [ ] 3.3: Encrypt and store secret and backup codes
  - [ ] 3.4: Generate QR code using `qrcode` crate
  - [ ] 3.5: Return setup data to frontend
  - [ ] 3.6: Add integration tests for setup flow

- [ ] Task 4: Implement MFA verification (AC: #1, #5)
  - [ ] 4.1: Create `verify_mfa_setup()` endpoint
  - [ ] 4.2: Validate TOTP code during setup
  - [ ] 4.3: Mark user as MFA enabled on success
  - [ ] 4.4: Handle invalid codes with retry count
  - [ ] 4.5: Add rate limiting for verification attempts
  - [ ] 4.6: Add integration tests for verification

- [ ] Task 5: Implement MFA check during login (AC: #2, #9)
  - [ ] 5.1: Extend `login()` endpoint to check MFA status
  - [ ] 5.2: If MFA enabled, return MFA required flag
  - [ ] 5.3: Create `verify_mfa_login()` endpoint
  - [ ] 5.4: Validate TOTP code for login
  - [ ] 5.5: Return JWT tokens only on successful MFA verification
  - [ ] 5.6: Implement retry logic with 3 attempt limit
  - [ ] 5.7: Add integration tests for MFA login flow

- [ ] Task 6: Implement MFA disable flow (AC: #6)
  - [ ] 6.1: Create `disable_mfa()` endpoint
  - [ ] 6.2: Require password and MFA code verification
  - [ ] 6.3: Delete MFA secret from database
  - [ ] 6.4: Delete backup codes from database
  - [ ] 6.5: Set mfa_enabled to false
  - [ ] 6.6: Log MFA disable event
  - [ ] 6.7: Add integration tests for disable flow

- [ ] Task 7: Implement backup code regeneration (AC: #3, #10)
  - [ ] 7.1: Create `regenerate_backup_codes()` endpoint
  - [ ] 7.2: Invalidate existing backup codes
  - [ ] 7.3: Generate new 10 backup codes
  - [ ] 7.4: Encrypt and store new codes
  - [ ] 7.5: Return codes to user (display only once)
  - [ ] 7.6: Add rate limiting to prevent abuse
  - [ ] 7.7: Add integration tests for regeneration

- [ ] Task 8: Implement MFA role requirements (AC: #7, #8)
  - [ ] 8.1: Create MFA requirement policy configuration
  - [ ] 8.2: Implement `check_mfa_required()` function
  - [ ] 8.3: Check user role against policy
  - [ ] 8.4: Redirect to MFA setup if required but not enabled
  - [ ] 8.5: Add admin config for required roles
  - [ ] 8.6: Add unit tests for requirement logic

- [ ] Task 9: Implement trusted devices (AC: #2, #8)
  - [ ] 9.1: Create `trusted_devices` table
  - [ ] 9.2: Implement "remember this device" checkbox on login
  - [ ] 9.3: Store device fingerprint and expiration
  - [ ] 9.4: Skip MFA for trusted devices within 30 days
  - [ ] 9.5: Add device management UI
  - [ ] 9.6: Implement device revocation
  - [ ] 9.7: Add integration tests for device trust

- [ ] Task 10: Create MFA management UI (AC: #1, #4, #6, #10)
  - [ ] 10.1: Create `MFASecuritySettings` page component
  - [ ] 10.2: Create `MFASetupWizard` component with QR code display
  - [ ] 10.3: Create `BackupCodesDisplay` component
  - [ ] 10.4: Implement TOTP code input during login
  - [ ] 10.5: Show MFA status in profile settings
  - [ ] 10.6: Add enable/disable MFA buttons
  - [ ] 10.7: Implement backup code regeneration flow
  - [ ] 10.8: Add QR code rendering using `qrcode.react`
  - [ ] 10.9: Add countdown timer for code validity
  - [ ] 10.10: Add MFA requirement messaging

- [ ] Task 11: Add MFA audit logging (AC: #2, #6)
  - [ ] 11.1: Log MFA enable events with IP and user agent
  - [ ] 11.2: Log MFA disable events with reason
  - [ ] 11.3: Log failed MFA verification attempts
  - [ ] 11.4: Log backup code usage
  - [ ] 11.5: Create MFA audit query endpoint
  - [ ] 11.6: Add security alerting for suspicious patterns
  - [ ] 11.7: Add unit tests for audit logging

- [ ] Task 12: Implement MFA for OAuth flows (AC: #1, #2)
  - [ ] 12.1: Extend OAuth callback to check MFA status
  - [ ] 12.2: If MFA enabled, require TOTP verification before tokens
  - [ ] 12.3: Create separate MFA verification endpoint for OAuth
  - [ ] 12.4: Store MFA completion token temporarily
  - [ ] 12.5: Generate JWT tokens after MFA verification
  - [ ] 12.6: Add integration tests for OAuth + MFA flow

## Dev Notes

### Architecture Alignment

This story implements **Multi-Factor Authentication (MFA)** per Epic 15 requirements:

- **Backend Location**: `crates/qa-pms-auth/src/mfa/`
- **TOTP Library**: `totp-lite` crate for code generation/validation
- **QR Code**: `qrcode` crate for QR generation
- **Security**: AES-256-GCM encryption for MFA secrets, TOTP with 30-second tolerance

### Technical Implementation Details

#### Dependencies to Add
```toml
# crates/qa-pms-auth/Cargo.toml
[dependencies]
totp-lite = "2.0"
qrcode = "0.14"
base64 = "0.22"
rand = "0.8"
serde = { version = "1.0", features = ["derive"] }

# Frontend dependencies
qrcode.react = "^4.0"
```

#### Database Schema
```sql
-- Add to existing users table
ALTER TABLE users ADD COLUMN mfa_enabled BOOLEAN DEFAULT FALSE;
ALTER TABLE users ADD COLUMN mfa_secret_encrypted TEXT;
ALTER TABLE users ADD COLUMN backup_codes TEXT[]; -- Encrypted
ALTER TABLE users ADD COLUMN mfa_enabled_at TIMESTAMP;
ALTER TABLE users ADD COLUMN mfa_verification_attempts INTEGER DEFAULT 0;
ALTER TABLE users ADD COLUMN mfa_locked_until TIMESTAMP;

-- Trusted devices table
CREATE TABLE trusted_devices (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id),
    device_fingerprint TEXT NOT NULL,
    device_name TEXT,
    ip_address INET,
    user_agent TEXT,
    expires_at TIMESTAMP NOT NULL,
    created_at TIMESTAMP DEFAULT NOW(),
    UNIQUE(user_id, device_fingerprint)
);

-- Indexes
CREATE INDEX idx_trusted_devices_user_id ON trusted_devices(user_id);
CREATE INDEX idx_trusted_devices_expires_at ON trusted_devices(expires_at);
```

#### TOTP Implementation
```rust
use totp_lite::{totp_custom, Sha1, Totp};
use std::time::{SystemTime, UNIX_EPOCH};
use base64::{Engine, engine::general_purpose::URL_SAFE_NO_PAD};

pub struct TotpSecret {
    pub secret: String,
    pub qr_code: String,
    pub backup_codes: Vec<String>,
}

pub fn generate_totp_secret() -> TotpSecret {
    // Generate 32-byte secret
    let secret_bytes: Vec<u8> = (0..32)
        .map(|_| rand::random::<u8>())
        .collect();
    
    let secret = URL_SAFE_NO_PAD.encode(&secret_bytes);
    
    // Generate backup codes (10 codes, 8 chars each)
    let backup_codes: Vec<String> = (0..10)
        .map(|_| generate_backup_code())
        .collect();
    
    // Generate QR code URL
    let qr_url = format!(
        "otpauth://totp/QA PMS:({})?secret={}&issuer=QA+PMS",
        "user@example.com",
        secret
    );
    
    let qr_code = qrcode::QrCode::new(qr_url.as_str())
        .unwrap()
        .render::<qrcode::render::svg::Color>()
        .unwrap();
    
    TotpSecret {
        secret,
        qr_code,
        backup_codes,
    }
}

fn generate_backup_code() -> String {
    let chars: Vec<char> = "ABCDEFGHJKLMNPQRSTUVWXYZ23456789"
        .chars()
        .collect();
    
    (0..8)
        .map(|_| chars[rand::random::<usize>() % chars.len()])
        .collect()
}

pub fn validate_totp_code(secret: &str, code: &str) -> bool {
    let secret_bytes = URL_SAFE_NO_PAD
        .decode(secret)
        .expect("Invalid base64");
    
    let totp = Totp::new(secret_bytes);
    
    // Generate valid TOTP codes for current time (current, previous 30s, next 30s)
    let time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() / 30;
    
    let valid_codes: Vec<String> = vec![
        totp.generate(time - 1),
        totp.generate(time),
        totp.generate(time + 1),
    ];
    
    valid_codes.contains(&code.to_string())
}
```

#### MFA Service
```rust
use sqlx::PgPool;
use std::sync::Arc;

pub struct MfaService {
    db: Arc<PgPool>,
    encryption: Arc<EncryptionService>,
}

impl MfaService {
    pub async fn setup_mfa(
        &self,
        user_id: &Uuid,
    ) -> Result<MfaSetupData, MfaError> {
        // Generate TOTP secret and backup codes
        let totp_data = generate_totp_secret();
        
        // Encrypt secret and backup codes
        let secret_encrypted = self.encryption.encrypt(&totp_data.secret)?;
        let backup_codes_encrypted: Vec<String> = totp_data.backup_codes
            .into_iter()
            .map(|code| self.encryption.encrypt(code))
            .collect::<Result<Vec<_>, _>>()?;
        
        // Store in database
        sqlx::query!(
            r#"
            UPDATE users
            SET mfa_secret_encrypted = $1,
                backup_codes = $2,
                mfa_enabled = TRUE,
                mfa_enabled_at = NOW()
            WHERE id = $3
            "#
        )
        .bind(&secret_encrypted, &backup_codes_encrypted, user_id)
        .execute(&*self.db)
        .await?;
        
        Ok(MfaSetupData {
            qr_code: totp_data.qr_code,
            backup_codes: totp_data.backup_codes,
            secret: totp_data.secret,
        })
    }
    
    pub async fn verify_mfa(
        &self,
        user_id: &Uuid,
        code: &str,
    ) -> Result<bool, MfaError> {
        // Get user's MFA secret
        let user = sqlx::query!(
            r#"SELECT mfa_secret_encrypted, mfa_verification_attempts, mfa_locked_until FROM users WHERE id = $1"#
        )
        .bind(user_id)
        .fetch_one(&*self.db)
        .await?
        .ok_or(MfaError::UserNotFound)?;
        
        // Check if MFA is locked
        if let Some(locked_until) = user.mfa_locked_until {
            if locked_until > chrono::Utc::now() {
                return Err(MfaError::LockedUntil(locked_until));
            }
        }
        
        // Decrypt secret
        let secret = self.encryption.decrypt(&user.mfa_secret_encrypted)?;
        
        // Validate code
        let is_valid = validate_totp_code(&secret, code);
        
        if is_valid {
            // Reset verification attempts
            sqlx::query!(
                r#"
                UPDATE users
                SET mfa_verification_attempts = 0, mfa_locked_until = NULL
                WHERE id = $1
                "#
            )
            .bind(user_id)
            .execute(&*self.db)
            .await?;
            
            Ok(true)
        } else {
            // Increment verification attempts
            let new_attempts = user.mfa_verification_attempts + 1;
            
            sqlx::query!(
                r#"
                UPDATE users
                SET mfa_verification_attempts = $1
                WHERE id = $2
                "#
            )
            .bind(new_attempts, user_id)
            .execute(&*self.db)
            .await?;
            
            // Lock account after 3 failed attempts
            if new_attempts >= 3 {
                let lock_until = chrono::Utc::now() + chrono::Duration::minutes(15);
                sqlx::query!(
                    r#"
                    UPDATE users
                    SET mfa_locked_until = $1
                    WHERE id = $2
                    "#
                )
                .bind(lock_until, user_id)
                .execute(&*self.db)
                .await?;
                
                return Err(MfaError::LockedUntil(lock_until));
            }
            
            Ok(false)
        }
    }
    
    pub async fn disable_mfa(
        &self,
        user_id: &Uuid,
        password: &str,
        mfa_code: &str,
    ) -> Result<(), MfaError> {
        // Verify password and MFA code first
        let user = self.get_user_with_mfa(user_id).await?;
        
        if !self.verify_password(user_id, password).await? {
            return Err(MfaError::InvalidPassword);
        }
        
        if !self.verify_mfa(user_id, mfa_code).await? {
            return Err(MfaError::InvalidMfaCode);
        }
        
        // Delete MFA secret and backup codes
        sqlx::query!(
            r#"
            UPDATE users
            SET mfa_secret_encrypted = NULL,
                backup_codes = NULL,
                mfa_enabled = FALSE,
                mfa_enabled_at = NULL
            WHERE id = $1
            "#
        )
        .bind(user_id)
        .execute(&*self.db)
        .await?;
        
        // Log disable event
        self.log_mfa_event(user_id, "mfa_disabled").await;
        
        Ok(())
    }
}
```

#### API Endpoints
```rust
// POST /api/v1/auth/mfa/setup
#[utoipa::path(
    responses(
        (status = 200, description = "MFA setup data returned", body = MfaSetupData),
        (status = 401, description = "Not authenticated")
    ),
    tag = "MFA"
)]
pub async fn setup_mfa(
    State(mfa_service): State<Arc<MfaService>>,
    Extension(user_context): Extension<UserContext>,
) -> Result<Json<MfaSetupData>, ApiError>

// POST /api/v1/auth/mfa/verify-setup
#[utoipa::path(
    request_body = VerifyMfaRequest,
    responses(
        (status = 200, description = "MFA enabled successfully"),
        (status = 400, description = "Invalid code")
    ),
    tag = "MFA"
)]
pub async fn verify_mfa_setup(
    State(mfa_service): State<Arc<MfaService>>,
    Extension(user_context): Extension<UserContext>,
    Json(request): Json<VerifyMfaRequest>,
) -> Result<StatusCode, ApiError>

// POST /api/v1/auth/mfa/verify-login
#[utoipa::path(
    request_body = VerifyMfaRequest,
    responses(
        (status = 200, description = "MFA verified, tokens returned", body = AuthResponse),
        (status = 400, description = "Invalid code or too many attempts"),
        (status = 423, description = "Account locked")
    ),
    tag = "MFA"
)]
pub async fn verify_mfa_login(
    State(mfa_service): State<Arc<MfaService>>,
    State(auth_service): State<Arc<AuthService>>,
    Json(request): Json<VerifyMfaRequest>,
) -> Result<Json<AuthResponse>, ApiError>

// POST /api/v1/auth/mfa/disable
#[utoipa::path(
    request_body = DisableMfaRequest,
    responses(
        (status = 204, description = "MFA disabled successfully"),
        (status = 401, description = "Invalid credentials")
    ),
    tag = "MFA"
)]
pub async fn disable_mfa(
    State(mfa_service): State<Arc<MfaService>>,
    State(auth_service): State<Arc<AuthService>>,
    Json(request): Json<DisableMfaRequest>,
) -> Result<StatusCode, ApiError>

// POST /api/v1/auth/mfa/regenerate-backup-codes
#[utoipa::path(
    responses(
        (status = 200, description = "New backup codes generated", body = BackupCodesResponse),
        (status = 401, description = "Not authenticated or MFA not enabled")
    ),
    tag = "MFA"
)]
pub async fn regenerate_backup_codes(
    State(mfa_service): State<Arc<MfaService>>,
    Extension(user_context): Extension<UserContext>,
) -> Result<Json<BackupCodesResponse>, ApiError>
```

#### Frontend Components
```typescript
// MFA Setup Wizard
const MFASetupWizard: React.FC = () => {
  const [step, setStep] = useState<'scan' | 'verify' | 'complete'>('scan');
  const [qrCode, setQrCode] = useState<string>('');
  const [backupCodes, setBackupCodes] = useState<string[]>([]);
  const [code, setCode] = useState<string>('');
  const [loading, setLoading] = useState(false);
  
  const setupMfa = async () => {
    setLoading(true);
    try {
      const response = await api.post('/api/v1/auth/mfa/setup');
      setQrCode(response.data.qr_code);
      setBackupCodes(response.data.backup_codes);
      setStep('scan');
    } catch (error) {
      toast.error('Failed to setup MFA');
    } finally {
      setLoading(false);
    }
  };
  
  const verifySetup = async () => {
    setLoading(true);
    try {
      await api.post('/api/v1/auth/mfa/verify-setup', { code });
      setStep('complete');
      toast.success('MFA enabled successfully!');
    } catch (error) {
      toast.error('Invalid code. Please try again.');
    } finally {
      setLoading(false);
    }
  };
  
  return (
    <div className="mfa-setup">
      {step === 'scan' && (
        <>
          <h2>Step 1: Scan QR Code</h2>
          <p>Use your authenticator app (Google Authenticator, Authy, Microsoft Authenticator) to scan this QR code.</p>
          <div className="qr-code">
            <QRCode value={qrCode} />
          </div>
          <button onClick={() => setStep('verify')} disabled={!qrCode}>
            Next
          </button>
        </>
      )}
      
      {step === 'verify' && (
        <>
          <h2>Step 2: Verify Setup</h2>
          <p>Enter the 6-digit code from your authenticator app.</p>
          <input
            type="text"
            maxLength={6}
            value={code}
            onChange={(e) => setCode(e.target.value)}
            placeholder="123456"
            autoFocus
          />
          <button onClick={verifySetup} disabled={code.length !== 6 || loading}>
            {loading ? 'Verifying...' : 'Verify'}
          </button>
          <button onClick={() => setStep('scan')}>
            Back
          </button>
        </>
      )}
      
      {step === 'complete' && (
        <>
          <CheckCircle className="icon success" />
          <h2>MFA Enabled!</h2>
          <p>Your account is now protected with two-factor authentication.</p>
          <div className="backup-codes">
            <h3>Backup Codes</h3>
            <Alert variant="warning">
              Save these codes in a secure location. Each code can only be used once!
            </Alert>
            <div className="codes-grid">
              {backupCodes.map((code, index) => (
                <div key={index} className="backup-code">
                  <CopyToClipboard text={code} />
                  <code>{code}</code>
                </div>
              ))}
            </div>
          </div>
        </>
      )}
    </div>
  );
};

// MFA Login Component
const MfaLogin: React.FC<{ onSuccess: (tokens: AuthTokens) => void }> = ({ onSuccess }) => {
  const [code, setCode] = useState<string>('');
  const [attempts, setAttempts] = useState(0);
  const [lockedUntil, setLockedUntil] = useState<Date | null>(null);
  const [loading, setLoading] = useState(false);
  
  const verifyMfa = async () => {
    setLoading(true);
    try {
      const response = await api.post('/api/v1/auth/mfa/verify-login', { code });
      onSuccess(response.data);
    } catch (error) {
      setAttempts(prev => prev + 1);
      if (error.status === 423) {
        setLockedUntil(new Date(error.data.locked_until));
      } else {
        toast.error('Invalid code. Please try again.');
      }
    } finally {
      setLoading(false);
    }
  };
  
  return (
    <div className="mfa-login">
      <h2>Enter Authentication Code</h2>
      <p>Open your authenticator app and enter the 6-digit code.</p>
      
      {lockedUntil ? (
        <Alert variant="error">
          Account locked until {lockedUntil.toLocaleString()}
        </Alert>
      ) : (
        <input
          type="text"
          maxLength={6}
          value={code}
          onChange={(e) => setCode(e.target.value)}
          placeholder="123456"
          autoFocus
        />
      )}
      
      <div className="attempts">
        Attempts remaining: {3 - attempts}
      </div>
      
      <button onClick={verifyMfa} disabled={code.length !== 6 || loading || !!lockedUntil}>
        {loading ? 'Verifying...' : 'Verify'}
      </button>
    </div>
  );
};
```

### Testing Strategy

#### Unit Tests
- TOTP code generation
- TOTP code validation (including time window)
- Backup code generation
- Secret encryption/decryption
- QR code generation

#### Integration Tests
- Complete MFA setup flow
- MFA verification during setup
- MFA verification during login
- Backup code usage
- MFA disable flow
- Backup code regeneration

#### End-to-End Tests
- User enables MFA through setup wizard
- User logs in with MFA code
- User uses backup code
- User disables MFA
- User is locked after 3 failed attempts
- Trusted device skip MFA

#### Security Tests
- Attempt login with wrong MFA code
- Attempt login with expired MFA code
- Attempt to disable MFA without password
- Attempt to disable MFA without MFA code
- Brute force attempts (rate limiting)
- Backup code reuse prevention

### File List

**Files to be created:**
- `crates/qa-pms-auth/src/mfa/mod.rs`
- `crates/qa-pms-auth/src/mfa/service.rs`
- `crates/qa-pms-auth/src/mfa/totp.rs`
- `migrations/add_mfa_columns_to_users.sql`
- `migrations/create_trusted_devices_table.sql`
- `frontend/src/components/auth/MFASetupWizard.tsx`
- `frontend/src/components/auth/MfaLogin.tsx`
- `frontend/src/pages/auth/SecuritySettings.tsx`

**Files to be modified:**
- `crates/qa-pms-auth/Cargo.toml` (add totp-lite, qrcode dependencies)
- `crates/qa-pms-api/src/main.rs` (add MFA routes)
- `frontend/src/api/auth.ts` (add MFA API calls)
- `crates/qa-pms-auth/src/middleware/auth_middleware.rs` (extend to check MFA)