### Story 15.1: JWT Authentication with Refresh Tokens

As a **user**,
I want to **login with email/password or access token**,
So that **I can access the system securely**.

**Priority:** P0 (Critical)  
**Estimated Effort:** 2 days  
**Sprint:** 1

### Acceptance Criteria

**Given** a user has valid credentials
**When** they submit login request with email and password
**Then** the system validates credentials using argon2 hash comparison
**And** returns JWT access token (15 min expiration)
**And** sets secure HTTP-only refresh token cookie (7 days expiration)
**And** tokens are signed with RS256 asymmetric key
**And** response includes user info (name, email, roles)

**Given** a user has a valid refresh token
**When** their access token expires
**Then** they can refresh using the refresh token
**And** receive a new access token
**And** receive a new refresh token (token rotation)
**And** the old refresh token is invalidated
**And** invalid refresh tokens return 401 Unauthorized

**Given** a user is logged in
**When** they logout
**Then** their refresh token is invalidated (blacklisted)
**And** all active sessions are terminated
**And** they receive 204 No Content response

**Given** a user tries to login with invalid credentials
**When** they submit login request
**Then** the system returns 401 Unauthorized
**And** does not reveal if user exists (security)
**And** logs the failed attempt

### Technical Requirements
- JWT access tokens with 15-minute expiration
- Secure HTTP-only, SameSite=Strict refresh token cookies
- Token rotation on refresh to prevent replay attacks
- Password hashing using argon2id with memory cost 256 MB
- Token blacklisting for forced logout
- RS256 asymmetric signing (better than HS256 for production)
- JTI (JWT ID) claim for token identification
- Rate limiting on login endpoint (5 attempts per 15 min)

### API Endpoints
POST /api/auth/login
{
  "email": "user@example.com",
  "password": "securePassword123"
}
Response:
{
  "access_token": "eyJhbGciOiJSUzI1NiIs...",
  "expires_in": 900,
  "token_type": "Bearer",
  "user": {
    "id": "uuid",
    "name": "John Doe",
    "email": "user@example.com",
    "roles": ["qa_engineer"]
  }
}

POST /api/auth/refresh
Headers: Cookie: refresh_token=...
Response: 200 OK
{
  "access_token": "eyJhbGciOiJSUzI1NiIs...",
  "expires_in": 900,
  "refresh_token": "new_token_value" // Token rotation
}

POST /api/auth/logout
Headers: Authorization: Bearer <token>
Response: 204 No Content
```

### Implementation Notes
- Database table: `refresh_tokens` (id, user_id, token_hash, expires_at, created_at)
- Middleware: `auth_middleware.rs` to validate JWT on protected routes
- Use `argon2` for password hashing
- Implement token blacklisting on logout
- Handle token rotation to prevent replay attacks

#### Files to Create
- `crates/qa-pms-auth/src/lib.rs` - New auth crate
- `crates/qa-pms-auth/src/jwt.rs` - JWT token generation/validation
- `crates/qa-pms-auth/src/middleware.rs` - Auth middleware
- `crates/qa-pms-auth/src/models.rs` - Auth-related database models
- `migrations/015_add_auth_tables.sql` - Database migrations for auth

#### Files to Modify
- `Cargo.toml` - Add `qa-pms-auth` crate and auth dependencies
- `crates/qa-pms-api/src/app.rs` - Add auth middleware layer
- `crates/qa-pms-api/src/routes/mod.rs` - Add auth routes
- `crates/qa-pms-api/src/routes/auth.rs` - Create auth endpoint handlers
- `crates/qa-pms-config/src/settings.rs` - Add JWT configuration

---

## Story 15.2: OAuth 2.0 + PKCE (OpenID Connect)

As an **Enterprise QA Lead**,
I want to **log in using my corporate SSO (Okta, Azure AD, Google)**,
So that **I don't need to remember another password and leverage existing security policies**.

### Acceptance Criteria

**Given** OAuth 2.0 provider is configured (e.g., Okta)  
**When** user clicks "Login with [Provider]" button  
**Then** the system initiates PKCE flow  
**And** generates code_verifier and code_challenge  
**And** redirects user to provider's consent screen  
**And** includes state parameter for CSRF protection  

**Given** user approves consent on provider  
**When** provider redirects back with authorization code  
**Then** the system validates state parameter  
**And** exchanges code for tokens using code_verifier  
**And** retrieves user info from OIDC userinfo endpoint  
**And** creates or updates user account  
**And** generates JWT tokens for the application  
**And** redirects user to dashboard  

**Given** user doesn't exist yet  
**When** OAuth login succeeds  
**Then** the system creates new user account  
**And** assigns default role based on email domain rules  
**And** sends welcome email  

### Technical Requirements
- Support multiple OAuth providers (Okta, Azure AD, Google, GitHub)
- Implement PKCE (Proof Key for Code Exchange) for mobile/SPA security
- Use OIDC (OpenID Connect) for standardized user info
- Store OAuth tokens in encrypted database fields
- Implement token auto-refresh for providers

### Supported Providers
| Provider | Documentation |
|----------|---------------|
| Okta | Native OIDC support |
| Azure AD | Azure AD B2C support |
| Google Workspace | OAuth 2.0 + OIDC |
| GitHub | OAuth 2.0 + user endpoints |

### API Endpoints
```rust
GET /api/auth/oauth/{provider}
Response: Redirect to provider's auth URL

GET /api/auth/oauth/{provider}/callback
Query Params: code, state
Response: Set auth cookies, redirect to frontend

GET /api/auth/oauth/providers
Response: [
  {
    "id": "okta",
    "name": "Okta",
    "enabled": true,
    "login_url": "/api/auth/oauth/okta"
  }
]
```

### Implementation Notes
- Use `oauth2` crate with PKCE extension
- Database table: `oauth_accounts` (id, user_id, provider, provider_user_id, access_token, refresh_token, expires_at)
- Implement email domain -> role mapping rules
- Add provider configuration in settings YAML
- Handle provider errors gracefully with user-friendly messages
- Support account linking (multiple providers per user)

---

## Story 15.3: Role-Based Access Control (RBAC)

As an **System Administrator**,
I want to **define roles and permissions**,
So that **users have appropriate access based on their responsibilities**.

### Acceptance Criteria

**Given** RBAC system is configured with roles and permissions  
**When** a user is created or updated  
**Then** the user is assigned a role  
**And** inherits all permissions from that role  

**Given** a user with role "QA Engineer" attempts to access admin endpoint  
**When** the request is made  
**Then** the system checks if role has required permission  
**And** returns 403 Forbidden if not authorized  
**And** logs unauthorized access attempt  

**Given** an admin user  
**When** they want to create custom role  
**Then** they can select from available permissions  
**And** save role with descriptive name  
**And** assign users to the role  

### Predefined Roles

| Role | Description | Permissions |
|------|-------------|-------------|
| **admin** | Full system access | All permissions |
| **qa_lead** | Team management, reports | qa.*, reports.*, users.view |
| **qa_engineer** | Execute workflows, view tickets | workflows.*, tickets.*, time.* |
| **pm_po** | View-only dashboards, metrics | dashboards.view, reports.view, patterns.view |
| **viewer** | Read-only access | dashboards.view, tickets.view |
| **service_account** | API access only | api.read, api.write |

### Permission Structure
```
- tickets.view, tickets.update, tickets.create
- workflows.view, workflows.execute, workflows.manage
- reports.view, reports.generate, reports.export
- users.view, users.manage
- system.config, system.audit
- api.read, api.write
- integrations.manage
```

### Technical Requirements
- Database tables:
  - `roles` (id, name, description, permissions, is_system, created_at)
  - `user_roles` (user_id, role_id, assigned_at, assigned_by)
- Permission check middleware: `require_permission("tickets.update")`
- Hierarchical roles (inheritance)
- Role assignment history for audit

### API Endpoints
```rust
GET /api/auth/roles
Response: [Role objects]

POST /api/auth/roles
Request: {
  "name": "custom_role",
  "description": "...",
  "permissions": ["tickets.view", "workflows.execute"]
}

POST /api/users/{user_id}/roles
Request: { "role_id": 3 }

DELETE /api/users/{user_id}/roles/{role_id}
```

### Implementation Notes
- Store permissions as JSONB array in database
- Implement permission caching for performance
- Add permission check decorators for Axum handlers
- Use macro for permission checks: `#[require_permission("tickets.update")]`
- Support dynamic permission creation (future-proof)

---

## Story 15.4: Multi-Factor Authentication (MFA)

As a **Security-Conscious QA Lead**,
I want to **enable MFA for my account**,
So that **my account remains secure even if my password is compromised**.

### Acceptance Criteria

**Given** a user with account enabled  
**When** they navigate to Security Settings  
**Then** they can enable MFA using TOTP (Time-based One-Time Password)  
**And** they see a QR code to scan with authenticator app  
**And** they must verify a code to confirm setup  

**Given** MFA is enabled on user account  
**When** they attempt to login  
**Then** after successful password/OAuth authentication  
**And** they are prompted for 6-digit TOTP code  
**And** system validates code with 30-second tolerance  
**And** grants access only if code is correct  

**Given** user loses access to authenticator app  
**When** they use backup codes  
**Then** they can login with one-time backup code  
**And** backup code is consumed after use  
**And** they are prompted to regenerate backup codes  

### Technical Requirements
- TOTP implementation using `totp-lite` crate
- QR code generation for authenticator apps (Google Authenticator, Authy, Microsoft Authenticator)
- Generate 10 backup codes during MFA setup
- Store TOTP secret encrypted in database
- Remember trusted device for 30 days (optional)

### MFA Flow
```
1. User enables MFA in settings
2. System generates TOTP secret
3. System shows QR code
4. User scans with authenticator app
5. User enters code to verify
6. System generates 10 backup codes (display once)
7. User saves backup codes securely
```

### Database Schema
```sql
-- MFA settings
ALTER TABLE users ADD COLUMN mfa_enabled BOOLEAN DEFAULT FALSE;
ALTER TABLE users ADD COLUMN mfa_secret_encrypted TEXT;
ALTER TABLE users ADD COLUMN backup_codes TEXT[]; -- Encrypted

-- Trusted devices
CREATE TABLE trusted_devices (
    id UUID PRIMARY KEY,
    user_id UUID NOT NULL REFERENCES users(id),
    device_fingerprint TEXT,
    device_name TEXT,
    expires_at TIMESTAMP NOT NULL,
    created_at TIMESTAMP DEFAULT NOW()
);
```

### API Endpoints
```rust
POST /api/auth/mfa/setup
Response: {
  "qr_code": "data:image/png;base64,...",
  "backup_codes": ["abc123", "def456", ...],
  "secret": "JBSWY3DPEHPK3PXP" // Store this!
}

POST /api/auth/mfa/verify
Request: { "code": "123456" }
Response: { "success": true }

POST /api/auth/login (with MFA)
Request: { "email": "...", "password": "...", "mfa_code": "123456" }
Response: { "access_token": "...", "requires_mfa": false }
```

### Implementation Notes
- Use `totp-lite` for TOTP generation/validation
- Generate QR codes with `qrcode` crate
- Encrypt MFA secrets with AES-256-GCM
- Allow MFA to be optional per role (admin required)
- Implement MFA bypass with admin approval (emergency recovery)
- Log all MFA setup/verification attempts

---

## Story 15.5: Password Policies & Reset Flows

As a **QA Engineer**,
I want to **reset my password securely if forgotten**,
So that **I can regain access to my account without contacting support**.

### Acceptance Criteria

**Given** a user forgets their password  
**When** they click "Forgot Password"  
**Then** they enter their email address  
**And** system sends password reset link to email  
**And** link expires after 1 hour  
**And** link can only be used once  

**Given** user clicks password reset link  
**When** they enter new password  
**Then** password must meet security policy  
**And** cannot reuse last 5 passwords  
**And** password is hashed and saved  
**And** all active sessions are terminated (except current)  

### Password Policy Requirements
- Minimum length: 12 characters
- Must include: uppercase, lowercase, number, special character
- Cannot contain email address parts
- Cannot use common passwords (haveibeenpwned check optional)
- Password expiration: 90 days (configurable)
- Prevent password reuse: last 5 passwords

### Technical Requirements
- Database tables:
  - `password_reset_tokens` (id, user_id, token_hash, expires_at, used_at)
  - `password_history` (id, user_id, password_hash, created_at)
- Email templates for reset flow
- Rate limiting on password reset requests (5 per hour)
- CAPTCHA optional for brute force prevention

### API Endpoints
```rust
POST /api/auth/password/reset-request
Request: { "email": "user@example.com" }
Response: { "message": "If email exists, reset link sent" }

POST /api/auth/password/reset
Request: {
  "token": "reset_token_here",
  "new_password": "SecurePass123!"
}
Response: { "success": true }

POST /api/auth/password/change
Headers: Authorization: Bearer <token>
Request: {
  "current_password": "oldPass123",
  "new_password": "SecurePass123!"
}
Response: { "success": true }
```

### Password Validation
```rust
pub fn validate_password(password: &str, email: &str) -> Result<(), PasswordError> {
    // Length check
    if password.len() < 12 {
        return Err(PasswordError::TooShort);
    }
    
    // Complexity check
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
    
    // No email parts
    let email_parts: Vec<&str> = email.split('@').collect();
    let username = email_parts.get(0).unwrap_or(&"");
    if password.to_lowercase().contains(&username.to_lowercase()) {
        return Err(PasswordError::ContainsEmail);
    }
    
    Ok(())
}
```

### Implementation Notes
- Send password reset emails using background jobs
- Include security tips in reset email
- Log all password reset attempts for security
- Allow password change without email if user is logged in
- Implement account lockout after 5 failed attempts (15 min)
- Support password expiration reminders (email at day 80, 85, 90)

---

## Story 15.6: Session Management & Refresh Tokens

As a **QA Engineer**,
I want to **maintain my session seamlessly**,
So that **I don't have to re-login frequently while working on long testing sessions**.

### Acceptance Criteria

**Given** a user is logged in  
**When** their access token expires  
**Then** the system automatically refreshes using refresh token  
**And** new access token is issued  
**And** user experience is uninterrupted  

**Given** a user logs out from one device  
**When** they logout  
**Then** all refresh tokens for that user are invalidated  
**And** user is logged out from all devices  

**Given** an admin user  
**When** they revoke a user's access  
**Then** all tokens for that user are blacklisted  
**And** user is forced to login again  

### Technical Requirements
- Short-lived access tokens (15 min)
- Long-lived refresh tokens (7 days)
- Token rotation on refresh (issue new refresh token)
- Token blacklisting for forced logout
- Session timeout configuration
- Remember me option (30-day refresh tokens)

### Database Schema
```sql
-- Refresh tokens
CREATE TABLE refresh_tokens (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id),
    token_hash TEXT NOT NULL UNIQUE,
    device_info JSONB,
    ip_address INET,
    expires_at TIMESTAMP NOT NULL,
    created_at TIMESTAMP DEFAULT NOW(),
    last_used_at TIMESTAMP DEFAULT NOW()
);

-- Token blacklist (for forced logout)
CREATE TABLE token_blacklist (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    jti TEXT NOT NULL UNIQUE, -- JWT ID claim
    expires_at TIMESTAMP NOT NULL,
    created_at TIMESTAMP DEFAULT NOW()
);

-- Active sessions view
CREATE VIEW active_sessions AS
SELECT 
    rt.user_id,
    u.email,
    u.name,
    rt.device_info->>'browser' AS browser,
    rt.device_info->>'os' AS os,
    rt.ip_address,
    rt.created_at,
    rt.last_used_at
FROM refresh_tokens rt
JOIN users u ON rt.user_id = u.id
WHERE rt.expires_at > NOW();
```

### API Endpoints
```rust
POST /api/auth/refresh
Headers: Cookie: refresh_token=...
Response: {
  "access_token": "eyJhbGci...",
  "expires_in": 900
}

GET /api/auth/sessions
Response: [
  {
    "id": "...",
    "device": "Chrome on Windows",
    "ip": "192.168.1.1",
    "created_at": "2024-01-15T10:00:00Z",
    "last_used": "2024-01-15T14:30:00Z"
  }
]

DELETE /api/auth/sessions/{session_id}
Response: 204 No Content (logout specific session)

DELETE /api/auth/sessions
Response: 204 No Content (logout all sessions)
```

### Implementation Notes
- Store device info (user agent, OS, browser) on token creation
- Implement token refresh middleware in frontend (axios interceptors)
- Use JTI (JWT ID) claim for blacklisting
- Set `SameSite=Strict` and `Secure` flags on cookies
- Implement refresh token rotation (issue new on refresh)
- Clean up expired refresh tokens with scheduled job
- Show active sessions in user profile for security visibility

---

## Story 15.7: API Key Authentication for Service Accounts

As a **DevOps Engineer**,
I want to **use API keys for automated scripts**,
So that **I can integrate the platform with CI/CD pipelines without user authentication**.

### Acceptance Criteria

**Given** an admin user  
**When** they create a service account  
**Then** they specify name, description, and permissions  
**And** system generates API key (only shown once)  
**And** API key can be scoped to specific permissions  

**Given** an automated script  
**When** it makes API request with API key  
**Then** system validates API key  
**And** checks key permissions  
**And** allows or denies request based on scope  

**Given** API key is compromised  
**When** admin revokes the key  
**Then** all requests with that key fail immediately  
**And** revocation is logged  

### Technical Requirements
- API keys stored as hash (SHA-256) with salt
- Prefix format: `qapms_sk_` (followed by 32 chars)
- Scoping: limit API keys to specific endpoints
- Rate limiting per API key
- Expiration date optional
- Last used tracking

### API Key Format
```
Prefix: qapms_sk_
Example: qapms_sk_7f8e9d1a2b3c4d5e6f7a8b9c0d1e2f3a
```

### Database Schema
```sql
-- Service accounts
CREATE TABLE service_accounts (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name TEXT NOT NULL,
    description TEXT,
    permissions TEXT[] NOT NULL,
    expires_at TIMESTAMP,
    created_by UUID REFERENCES users(id),
    created_at TIMESTAMP DEFAULT NOW(),
    is_active BOOLEAN DEFAULT TRUE
);

-- API keys
CREATE TABLE api_keys (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    service_account_id UUID NOT NULL REFERENCES service_accounts(id),
    key_hash TEXT NOT NULL UNIQUE,
    key_prefix TEXT NOT NULL,
    last_used_at TIMESTAMP,
    expires_at TIMESTAMP,
    created_at TIMESTAMP DEFAULT NOW(),
    is_active BOOLEAN DEFAULT TRUE
);
```

### API Endpoints
```rust
POST /api/admin/service-accounts
Request: {
  "name": "CI/CD Pipeline",
  "description": "Used by GitHub Actions",
  "permissions": ["workflows.execute", "tickets.create"],
  "expires_at": "2025-01-01T00:00:00Z"
}
Response: {
  "id": "...",
  "api_key": "qapms_sk_7f8e9d1a...", // Only shown once!
  "name": "CI/CD Pipeline",
  "permissions": [...],
  "expires_at": "2025-01-01T00:00:00Z"
}

GET /api/admin/service-accounts
Response: [ServiceAccount objects without api_key]

DELETE /api/admin/service-accounts/{id}
Response: 204 No Content

// Authentication with API key
GET /api/workflows
Headers: X-API-Key: qapms_sk_7f8e9d1a...
Response: [...]
```

### Implementation Notes
- Generate API keys using `rand` crate with crypto-secure RNG
- Hash keys with SHA-256 + salt before storage
- Implement API key middleware: `api_key_middleware`
- Differentiate user vs service account in logs
- Support key rotation (revoke old, issue new)
- Limit number of API keys per service account
- Implement key usage analytics (last used, request count)

---

## Story 15.8: Permission System with Granular Controls

As a **System Administrator**,
I want to **define fine-grained permissions**,
So that **users have access only to what they need**.

### Acceptance Criteria

**Given** the permission system  
**When** I define a permission like `tickets.update.own`  
**Then** users can only update their own tickets  
**And** users with `tickets.update.all` can update any ticket  

**Given** a permission check  
**When** checking if user can perform action  
**Then** system checks user's role permissions  
**And** supports resource ownership checks  
**And** supports custom permission predicates  

### Permission Hierarchy
```
tickets
├── tickets.view
│   ├── tickets.view.own
│   └── tickets.view.all
├── tickets.update
│   ├── tickets.update.own
│   └── tickets.update.all
└── tickets.delete
    ├── tickets.delete.own
    └── tickets.delete.all

workflows
├── workflows.execute
├── workflows.manage
│   ├── workflows.manage.own
│   └── workflows.manage.all
└── workflows.delete
```

### Technical Implementation
```rust
// Permission enum
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Permission {
    TicketsView,
    TicketsUpdate,
    TicketsUpdateOwn,
    TicketsUpdateAll,
    WorkflowsExecute,
    WorkflowsManage,
    WorkflowsManageOwn,
    WorkflowsManageAll,
    // ... more permissions
}

// Permission check
pub trait PermissionChecker {
    fn has_permission(&self, permission: &Permission) -> bool;
    fn can_update_ticket(&self, ticket: &Ticket) -> bool {
        self.has_permission(&Permission::TicketsUpdateAll) ||
        (self.has_permission(&Permission::TicketsUpdateOwn) && self.id == ticket.created_by)
    }
}

// Middleware example
pub async fn require_permission(
    permission: Permission,
) -> impl Fn(Request, Next) -> impl Future<Output = Response> {
    move |req: Request, next: Next| async move {
        let user = req.extensions().get::<User>().unwrap();
        
        if user.has_permission(&permission) {
            next.run(req).await
        } else {
            Response::builder()
                .status(StatusCode::FORBIDDEN)
                .body(Body::from("Insufficient permissions"))
                .unwrap()
        }
    }
}
```

### Usage in Routes
```rust
// Only update own tickets
router.route(
    "/api/tickets/:id",
    patch(update_ticket_handler)
        .layer(require_permission(Permission::TicketsUpdateOwn)),
)

// Update any ticket
router.route(
    "/api/admin/tickets/:id",
    patch(update_ticket_handler_admin)
        .layer(require_permission(Permission::TicketsUpdateAll)),
)
```

### Implementation Notes
- Store permissions as enum or string-based system
- Implement permission inheritance (e.g., `tickets.update.all` includes `tickets.update.own`)
- Add permission predicates for complex checks (e.g., user is assigned to ticket)
- Create permission matrix documentation
- Implement permission caching (Redis) for performance
- Support dynamic permissions (future: user-defined permissions)

---

## Story 15.9: Admin Dashboard for User Management

As a **System Administrator**,
I want to **manage users, roles, and permissions via UI**,
So that **I can maintain system security without writing SQL or API calls**.

### Acceptance Criteria

**Given** admin access  
**When** I access User Management Dashboard  
**Then** I see list of all users with filters  
**And** I can view user details, roles, permissions  
**And** I can create new users  
**And** I can assign/modify roles  
**And** I can reset passwords  
**And** I can revoke access  

**Given** I'm viewing user list  
**When** I filter by role, status, or search by email  
**Then** list updates in real-time  
**And** I can export user list as CSV  

**Given** I need to audit user changes  
**When** I view user history  
**Then** I see all changes (role assignments, password resets, etc.)  

### Dashboard Features

**User List View:**
- Columns: Name, Email, Role, Status, Last Login, Created At, Actions
- Filters: Role, Status (Active/Inactive), Date Range
- Search: Email, Name
- Bulk actions: Export, Deactivate, Assign Role

**User Detail View:**
- User profile information
- Assigned roles with permissions
- Active sessions (view/revoke)
- Login history (last 10)
- Audit trail (all changes)
- Actions: Edit Profile, Change Password, Assign Role, Deactivate, Delete

**Role Management:**
- List all roles with description
- View permissions per role
- Create custom roles
- Edit role permissions
- Assign users to role

### UI Components
```typescript
// User Management Page
interface User {
  id: string;
  name: string;
  email: string;
  roles: Role[];
  status: 'active' | 'inactive' | 'suspended';
  lastLogin: Date;
  createdAt: Date;
}

// User List Component
const UserList = () => {
  const [users, setUsers] = useState<User[]>([]);
  const [filters, setFilters] = useState<Filters>({});
  const [selectedUsers, setSelectedUsers] = useState<string[]>([]);

  // Fetch users with filters
  const fetchUsers = async () => {
    const response = await api.get('/admin/users', { params: filters });
    setUsers(response.data);
  };

  // Bulk actions
  const bulkAssignRole = async (roleId: string) => {
    await api.post('/admin/users/bulk-assign-role', {
      user_ids: selectedUsers,
      role_id: roleId
    });
    fetchUsers();
  };

  return (
    <div className="user-management">
      <UserFilters filters={filters} onChange={setFilters} />
      <UserTable 
        users={users}
        selected={selectedUsers}
        onSelect={setSelectedUsers}
        onAction={(userId, action) => handleUserAction(userId, action)}
      />
      <BulkActions 
        selectedCount={selectedUsers.length}
        onAssignRole={bulkAssignRole}
        onDeactivate={() => handleBulkDeactivate()}
      />
    </div>
  );
};
```

### API Endpoints
```rust
GET /api/admin/users
Query: ?role=qa_engineer&status=active&search=john
Response: PaginatedUserList

GET /api/admin/users/{user_id}
Response: { user, roles, sessions, audit_log }

POST /api/admin/users
Request: {
  "name": "John Doe",
  "email": "john@example.com",
  "role_ids": [2, 3]
}

PUT /api/admin/users/{user_id}
Request: { "name": "John Smith", "role_ids": [2] }

POST /api/admin/users/{user_id}/reset-password
Response: { "success": true, "email_sent": true }

DELETE /api/admin/users/{user_id}
Response: 204 No Content

GET /api/admin/users/{user_id}/sessions
Response: [Session objects]

DELETE /api/admin/users/{user_id}/sessions
Response: 204 No Content (revoke all sessions)
```

### Implementation Notes
- Use React Table for user list with sorting, filtering
- Implement optimistic updates for better UX
- Add confirmation modals for destructive actions
- Show audit trail with timestamps and who made changes
- Support CSV export with all user fields
- Implement real-time updates (WebSocket) for admin dashboard
- Add permission checks in frontend (security defense in depth)
- Create admin audit log for all admin actions

---

## Story 15.10: Audit Trail for Auth Events

As a **Security Auditor**,
I want to **view all authentication and authorization events**,
So that **I can detect security issues and ensure compliance**.

### Acceptance Criteria

**Given** audit logging is enabled  
**When** any auth/authz event occurs  
**Then** event is logged with timestamp  
**And** includes user ID, IP address, user agent  
**And** includes event type (login, logout, permission check)  
**And** includes result (success/failure)  

**Given** I need to investigate security incident  
**When** I query audit logs  
**Then** I can filter by user, date range, event type  
**And** I can export logs as CSV/JSON  
**And** I can see detailed event context  

### Event Types Logged

| Event Type | Description | Logged Fields |
|------------|-------------|---------------|
| `login.success` | User logged in | user_id, method (password/OAuth), ip, user_agent |
| `login.failed` | Login failed | email, ip, user_agent, failure_reason |
| `logout` | User logged out | user_id, ip |
| `mfa.enabled` | MFA enabled | user_id |
| `mfa.disabled` | MFA disabled | user_id, disabled_by |
| `mfa.verify.failed` | MFA verification failed | user_id, ip |
| `password.changed` | Password changed | user_id, changed_by |
| `password.reset` | Password reset requested | user_id, ip |
| `role.assigned` | Role assigned to user | user_id, role_id, assigned_by |
| `permission.denied` | Permission check failed | user_id, permission, endpoint |
| `api_key.created` | API key created | service_account_id, created_by |
| `session.revoked` | Session revoked | user_id, session_id, revoked_by |
| `account.locked` | Account locked | user_id, lock_reason |

### Database Schema
```sql
-- Audit log
CREATE TABLE audit_log (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    event_type TEXT NOT NULL,
    user_id UUID REFERENCES users(id), -- NULL if not applicable
    ip_address INET,
    user_agent TEXT,
    metadata JSONB,
    success BOOLEAN,
    created_at TIMESTAMP DEFAULT NOW()
);

-- Indexes for performance
CREATE INDEX idx_audit_log_event_type ON audit_log(event_type);
CREATE INDEX idx_audit_log_user_id ON audit_log(user_id);
CREATE INDEX idx_audit_log_created_at ON audit_log(created_at);
CREATE INDEX idx_audit_log_ip_address ON audit_log(ip_address);
```

### API Endpoints
```rust
GET /api/admin/audit-log
Query: ?event_type=login.success&user_id=...&from=2024-01-01&to=2024-01-31
Response: PaginatedAuditLog

POST /api/admin/audit-log/export
Request: { filters: {...}, format: "csv" }
Response: { download_url: "..." }

GET /api/admin/audit-log/stats
Response: {
  "total_events": 1234,
  "failed_logins": 56,
  "unique_users": 78,
  "by_event_type": {
    "login.success": 890,
    "login.failed": 56,
    "logout": 288
  }
}
```

### Audit Log Entry Structure
```json
{
  "id": "uuid",
  "event_type": "login.success",
  "user_id": "user-uuid",
  "ip_address": "192.168.1.100",
  "user_agent": "Mozilla/5.0 ...",
  "metadata": {
    "login_method": "password",
    "mfa_enabled": true,
    "session_id": "session-uuid"
  },
  "success": true,
  "created_at": "2024-01-15T10:30:00Z"
}
```

### Implementation Notes
- Use Axum middleware to capture request context
- Log all auth events in a dedicated auth crate
- Implement log retention policy (default: 90 days)
- Add PII masking if needed for compliance
- Support log export for external SIEM systems
- Create dashboard with security metrics (failed login trends)
- Implement alerting for suspicious activity (e.g., 5 failed logins)
- Ensure audit log is tamper-proof (append-only, no updates)

---

## Story 15.11: Rate Limiting for Auth Endpoints

As a **Security Engineer**,
I want to **rate limit authentication endpoints**,
So that **brute force attacks are mitigated**.

### Acceptance Criteria

**Given** rate limiting is configured  
**When** multiple login attempts are made from same IP  
**Then** after 5 failed attempts, rate limit is enforced  
**And** subsequent requests return 429 Too Many Requests  
**And** retry-after header indicates wait time  

**Given** an API key  
**When** it makes too many requests  
**Then** rate limit is enforced per API key  
**And** limit is configurable (default: 1000 req/hour)  

### Rate Limit Configuration

| Endpoint | Rate Limit | Window |
|----------|-------------|--------|
| `POST /api/auth/login` | 5 requests | 15 minutes |
| `POST /api/auth/password/reset` | 3 requests | 1 hour |
| `POST /api/auth/mfa/verify` | 10 requests | 5 minutes |
| `POST /api/auth/oauth/*/callback` | 10 requests | 5 minutes |
| API endpoints (authenticated) | 1000 requests | 1 hour |
| API endpoints (unauthenticated) | 100 requests | 1 hour |

### Technical Requirements
- Use `tower-governor` crate
- Redis-based rate limiting (for distributed deployment)
- Per-IP and per-user rate limiting
- Rate limit bypass for admin accounts (optional)
- Custom rate limits per role/plan

### Implementation
```rust
use tower_governor::{Governor, GovernorConfigBuilder};

// Login rate limiter (5 requests per 15 min)
let login_governor_conf = Box::new(
    GovernorConfigBuilder::default()
        .per_second(15 * 60) // 15 minutes
        .burst_size(5)
        .finish()
        .unwrap(),
);

// Apply to login endpoint
router.route(
    "/api/auth/login",
    post(login_handler).layer(Governor::new(&login_governor_conf))
);

// API rate limiter (1000 req/hour)
let api_governor_conf = Box::new(
    GovernorConfigBuilder::default()
        .per_second(3600) // 1 hour
        .burst_size(1000)
        .finish()
        .unwrap(),
);
```

### API Response with Rate Limit
```http
HTTP/1.1 429 Too Many Requests
Content-Type: application/json
X-RateLimit-Limit: 5
X-RateLimit-Remaining: 0
X-RateLimit-Reset: 1705401000
Retry-After: 900

{
  "error": "Too many requests",
  "message": "Rate limit exceeded. Please try again later.",
  "retry_after": 900
}
```

### Implementation Notes
- Store rate limit counters in Redis (distributed)
- Implement rate limit dashboard (view limits per user/IP)
- Add rate limit bypass for health checks
- Log rate limit violations for security monitoring
- Configure different limits for different plans (future)
- Implement IP-based rate limiting for unauthenticated requests
- Support rate limit whitelist for trusted IPs

---

## Story 15.12: Security Headers & CSRF Protection

As a **Security Engineer**,
I want to **implement security headers and CSRF protection**,
So that **the application is protected against common web vulnerabilities**.

### Acceptance Criteria

**Given** security middleware is configured  
**When** any response is returned  
**Then** security headers are set  
**And** CORS is properly configured  
**And** CSRF tokens are validated for state-changing requests  

### Security Headers

| Header | Value | Purpose |
|--------|-------|---------|
| `X-Content-Type-Options` | `nosniff` | Prevent MIME type sniffing |
| `X-Frame-Options` | `DENY` | Prevent clickjacking |
| `X-XSS-Protection` | `1; mode=block` | XSS protection |
| `Strict-Transport-Security` | `max-age=31536000; includeSubDomains` | Force HTTPS |
| `Content-Security-Policy` | Custom policy | XSS prevention |
| `Referrer-Policy` | `strict-origin-when-cross-origin` | Control referrer info |
| `Permissions-Policy` | Custom policy | Control browser features |

### Content Security Policy (CSP)
```http
Content-Security-Policy: 
  default-src 'self';
  script-src 'self' 'unsafe-inline' 'unsafe-eval' https://*.google.com;
  style-src 'self' 'unsafe-inline';
  img-src 'self' data: https:;
  connect-src 'self' https://*.openai.com https://*.anthropic.com;
  font-src 'self';
  object-src 'none';
  base-uri 'self';
  form-action 'self';
  frame-ancestors 'none';
  upgrade-insecure-requests;
```

### CSRF Protection

For state-changing requests (POST, PUT, DELETE, PATCH):
- Frontend must include CSRF token in header: `X-CSRF-Token`
- Token is generated on initial page load
- Token is validated for each state-changing request
- Token is single-use (regenerated after use)

### Technical Implementation
```rust
use tower_cookies::CookieManagerLayer;
use tower_http::cors::CorsLayer;
use tower_http::set_header::SetResponseHeaderLayer;

// Security headers middleware
let security_headers = SetResponseHeaderLayer::overriding(
    axum::http::header::STRICT_TRANSPORT_SECURITY,
    HeaderValue::from_static("max-age=31536000; includeSubDomains"),
);

// CORS configuration
let cors = CorsLayer::new()
    .allow_origin(allowed_origins)
    .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
    .allow_headers([axum::http::header::CONTENT_TYPE, axum::http::header::AUTHORIZATION])
    .allow_credentials(true)
    .max_age(Duration::from_secs(3600));

// Apply to router
let app = Router::new()
    .route("/api/*", api_routes())
    .layer(security_headers)
    .layer(cors)
    .layer(CookieManagerLayer::new());

// CSRF middleware
async fn csrf_middleware(
    req: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let method = req.method().clone();
    
    // Skip CSRF for GET requests
    if method == Method::GET {
        return Ok(next.run(req).await);
    }
    
    // Validate CSRF token for POST, PUT, DELETE
    let csrf_token = req.headers()
        .get("X-CSRF-Token")
        .and_then(|h| h.to_str().ok());
    
    match validate_csrf_token(csrf_token).await {
        Ok(_) => Ok(next.run(req).await),
        Err(_) => Err(StatusCode::FORBIDDEN),
    }
}
```

### Frontend CSRF Implementation
```typescript
// Get CSRF token on app load
const getCSRFToken = async () => {
  const response = await api.get('/csrf-token');
  return response.data.token;
};

// Include in requests
api.interceptors.request.use((config) => {
  if (['post', 'put', 'delete', 'patch'].includes(config.method?.toLowerCase())) {
    config.headers['X-CSRF-Token'] = csrfToken;
  }
  return config;
});
```

### Implementation Notes
- Use `tower-http` for security headers middleware
- Generate secure random CSRF tokens (32 bytes)
- Store CSRF tokens in encrypted cookies
- Implement CSP violation reporting endpoint
- Use helmet equivalent for Rust (`tower-http::security`)
- Configure CSP to allow inline scripts/styles for development
- Test with security scanners (OWASP ZAP, Burp Suite)
- Document security headers in API docs

---

## Dependencies

### New Crate Dependencies

```toml
[workspace.dependencies]
# Authentication
jsonwebtoken = "9"
oauth2 = "4.4"
totp-lite = "2"
argonautica = "2"
argon2 = "0.5"

# Security
tower-governor = "0.4"
tower-http = "0.5"
aes-gcm = "0.10"

# Session & Tokens
redis = "0.24"
rand = "0.8"

# Validation
validator = "0.18"

# Email (for password reset)
lettre = "0.11"
```

### Database Migrations

```sql
-- Users table
CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    email TEXT NOT NULL UNIQUE,
    password_hash TEXT,
    name TEXT,
    role_id UUID REFERENCES roles(id),
    mfa_enabled BOOLEAN DEFAULT FALSE,
    mfa_secret_encrypted TEXT,
    backup_codes TEXT[],
    is_active BOOLEAN DEFAULT TRUE,
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW(),
    last_login_at TIMESTAMP,
    failed_login_attempts INTEGER DEFAULT 0,
    locked_until TIMESTAMP
);

-- Roles table
CREATE TABLE roles (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name TEXT NOT NULL UNIQUE,
    description TEXT,
    permissions TEXT[] NOT NULL,
    is_system BOOLEAN DEFAULT FALSE,
    created_at TIMESTAMP DEFAULT NOW()
);

-- Refresh tokens
CREATE TABLE refresh_tokens (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id),
    token_hash TEXT NOT NULL UNIQUE,
    device_info JSONB,
    ip_address INET,
    expires_at TIMESTAMP NOT NULL,
    created_at TIMESTAMP DEFAULT NOW(),
    last_used_at TIMESTAMP DEFAULT NOW()
);

-- Audit log
CREATE TABLE audit_log (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    event_type TEXT NOT NULL,
    user_id UUID REFERENCES users(id),
    ip_address INET,
    user_agent TEXT,
    metadata JSONB,
    success BOOLEAN,
    created_at TIMESTAMP DEFAULT NOW()
);

-- Indexes
CREATE INDEX idx_users_email ON users(email);
CREATE INDEX idx_users_role_id ON users(role_id);
CREATE INDEX idx_refresh_tokens_user_id ON refresh_tokens(user_id);
CREATE INDEX idx_audit_log_event_type ON audit_log(event_type);
CREATE INDEX idx_audit_log_user_id ON audit_log(user_id);
CREATE INDEX idx_audit_log_created_at ON audit_log(created_at);
```

---

## Risk Assessment

| Risk | Impact | Probability | Mitigation |
|------|--------|-------------|------------|
| JWT token compromise | High | Low | Short access token duration, refresh token rotation |
| OAuth provider outage | Medium | Low | Multiple provider support, fallback to password |
| Rate limiting false positives | Medium | Medium | Configurable limits, admin bypass |
| CSRF token theft | High | Low | SameSite cookies, short-lived tokens |
| Password database breach | High | Low | Slow hashing (argon2), salt, separate database |
| Audit log tampering | High | Low | Append-only logs, separate database, encryption |

---

## Testing Strategy

### Unit Tests
- Password validation logic
- Permission checking logic
- Token generation and validation
- TOTP code generation and validation

### Integration Tests
- Login flow with password
- OAuth flow end-to-end
- Permission checks on protected endpoints
- Rate limiting enforcement
- Session management (refresh, logout)

### Security Tests
- Brute force attack simulation
- CSRF token validation
- SQL injection attempts (parameterized queries)
- XSS attempts (CSP validation)
- OAuth state parameter validation

### End-to-End Tests
- User signup with email verification
- Complete login flow (password + MFA)
- Password reset flow
- Admin user management
- API key authentication

---

## Success Metrics

| Metric | Target | Measurement |
|--------|--------|-------------|
| Login success rate | > 99% | Monitor auth logs |
| Average login time | < 3 seconds | Frontend timing |
| Password reset completion | > 90% | Track reset flow |
| MFA adoption rate | > 80% (for admins) | User settings |
| Unauthorized access attempts | < 0.1% of total requests | Audit log analysis |
| Rate limit violations | < 100/day | Rate limit logs |

---

## Timeline

| Week | Stories | Deliverables |
|------|---------|--------------|
| 1-2 | 15.1, 15.2, 15.3 | JWT auth, OAuth, RBAC foundation |
| 3 | 15.4, 15.5 | MFA, password policies |
| 4 | 15.6, 15.7 | Session management, API keys |
| 5 | 15.8, 15.9 | Granular permissions, admin dashboard |
| 6 | 15.10, 15.11, 15.12 | Audit logging, rate limiting, security headers |

---

## Next Steps

1. **Architecture Review**: Review security architecture with team
2. **Database Setup**: Run migrations to create auth tables
3. **OAuth Providers**: Configure test OAuth providers (Google, Okta)
4. **Email Service**: Set up email service for password reset
5. **Redis Setup**: Configure Redis for rate limiting and session caching
6. **Frontend Integration**: Build login pages and auth components
7. **Testing**: Write comprehensive tests for all auth flows
8. **Documentation**: Update API documentation with auth endpoints

---

## Notes

- **Priority**: This epic must be completed before any production deployment
- **Security First**: All implementations should follow OWASP security guidelines
- **UX Focus**: Balance security with user experience - minimize friction
- **Extensibility**: Design system to support future auth methods (WebAuthn, SAML)
- **Compliance**: Ensure implementation meets SOC2 and GDPR requirements

---

*Last Updated: 2025-01-16*  
*Author: AI Assistant*  
*Version: 1.0*