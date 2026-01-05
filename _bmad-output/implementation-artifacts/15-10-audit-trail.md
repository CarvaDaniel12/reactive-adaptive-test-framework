# Story 15.10: Audit Trail

Status: ready-for-dev

## Story

As a Security-Conscious System Administrator,
I want a comprehensive audit trail that logs all authentication and authorization events,
So that we can track user actions, detect security threats, and ensure compliance with SOC2 and GDPR requirements.

## Acceptance Criteria

1. **Given** an authentication event occurs (login, logout, token refresh, MFA verification)
   **When** the event completes
   **Then** system logs the event with timestamp, user ID, IP address, and user agent
   **And** logs event type (success/failure)
   **And** logs session ID and/or JWT ID
   **And** stores device fingerprint information

2. **Given** an authorization check is performed
   **When** a user attempts to access a resource
   **Then** system logs the permission check with user ID, permission, resource type, and result
   **And** logs resource ID and resource type
   **And** includes ownership check result if applicable

3. **Given** a role or permission change occurs
   **When** admin creates/modifies/deletes roles or permissions
   **Then** system logs the action with admin user ID
   **And** logs the old and new state (before/after)
   **And** logs which roles or permissions were affected
   **And** timestamps the event

4. **Given** an API key is generated, rotated, or revoked
   **When** the action completes
   **Then** system logs the action with service account ID
   **And** logs API key prefix (masked)
   **And** logs rotation reason (if rotated)
   **And** logs revocation reason (if revoked)

5. **Given** an MFA-related event occurs (enable, verify, disable)
   **When** the event completes
   **Then** system logs the MFA event with user ID
   **And** logs event type and details
   **And** includes success/failure result

6. **Given** a password change occurs
   **When** the change completes
   **Then** system logs the password change with user ID
   **And** logs whether password reset was required
   **And** does NOT log the old or new password (security best practice)

7. **Given** an account-related event occurs (creation, deletion, status change)
   **When** the event completes
   **Then** system logs the action with admin user ID (if performed by admin) or system (if automated)
   **And** logs the affected user ID
   **And** logs the old and new state

8. **Given** audit data needs to be queried
   **When** an admin or auditor queries the audit log
   **Then** system provides a searchable API
   **And** supports filtering by event type, user ID, date range, resource type
   **And** supports pagination
   **And** returns results in reverse chronological order (newest first)

9. **Given** audit data needs to be exported
   **When** an admin or auditor requests export
   **Then** system generates CSV or JSON export
   **And** includes all audit records in the date range
   **And** formats output for analysis tools
   **And** validates that requester has audit permission

10. **Given** a security threat is detected (brute force attempt, suspicious activity)
   **When** the event occurs
   **Then** system logs the threat level (info, warning, critical)
   **Then** triggers an alert if threshold is exceeded
   **Then** automatically takes protective action (lock account, notify admins)
   **And** logs the protective action taken

## Tasks / Subtasks

- [ ] Task 1: Setup audit trail database schema (AC: #1, #3, #8)
  - [ ] 1.1: Create `audit_log` table
  - [ ] 1.2: Add columns: id, event_type, user_id, timestamp, ip_address, user_agent, device_fingerprint, session_id, jwt_jti, resource_type, resource_id, result, metadata (JSONB)
  - [ ] 1.3: Create `audit_event_types` enum table or check constraint
  - [ ] 1.4: Create indexes on user_id, timestamp, event_type
  - [ ] 1.5: Add indexes on resource_id, resource_type for efficient querying
  - [ ] 1.6: Create migration file
  - [ ] 1.7: Add unit tests for schema

- [ ] Task 2: Implement audit logging service (AC: #1, #2, #3, #4, #5, #6, #7)
  - [ ] 2.1: Create `AuditService` in `qa-pms-audit` crate
  - [ ] 2.2: Implement `log_event()` function with structured logging
  - [ ] 2.3: Implement `log_auth_event()` for authentication events
  - [ ] 2.4: Implement `log_authz_event()` for authorization events
  - [ ] 2.5: Implement `log_mfa_event()` for MFA events
  - [ ] 2.6: Implement `log_api_key_event()` for API key events
  - [ ] 2.7: Implement `log_password_change_event()` for password changes
  - [ ] 2.8: Implement `log_account_event()` for account management events
  - [ ] 2.9: Add structured metadata support (JSONB for flexible logging)
  - [ ] 2.10: Use `tracing` crate for distributed logging
  - [ ] 2.11: Add unit tests for audit service

- [ ] Task 3: Create audit event types and enums (AC: #1, #9)
  - [ ] 3.1: Define `AuditEventType` enum
  - [ ] 3.2: Define `AuthEventType` enum (login, logout, token_refresh, mfa_verify, mfa_enable, mfa_disable)
  - [ ] 3.3: Define `AuthzEventType` enum (permission_check_granted, permission_check_denied, role_assigned, role_updated, permission_added, permission_removed)
  - [ ] 3.4: Define `MfaEventType` enum (mfa_setup, mfa_verify, mfa_disable, backup_code_generated, backup_code_used)
  - [ ] 3.5: Define `ApiKeyEventType` enum (key_created, key_rotated, key_revoked, key_used, rate_limit_exceeded)
  - [ ] 3.6: Define `AccountEventType` enum (user_created, user_deleted, user_activated, user_deactivated, password_changed, password_reset, mfa_enabled, mfa_disabled)
  - [ ] 3.7: Define `AuditEventResult` enum (success, failure, partial_success)
  - [ ] 3.8: Implement `Display` and `Severity` traits for events
  - [ ] 3.9: Add serde derive/serialize for all enums
  - [ ] 3.10: Add unit tests for event types

- [ ] Task 4: Implement audit query API endpoints (AC: #8)
  - [ ] 4.1: Add `GET /api/v1/admin/audit-log` endpoint
  - [ ] 4.2: Implement pagination (page, per_page, sort_by, sort_order)
  - [ ] 4.3: Implement filtering by event_type, user_id, date range
  - [ ] 4.4: Implement filtering by resource_type and resource_id
  - [ ] 4.5: Implement full-text search on metadata
  - [ ] 4.6: Add OpenAPI documentation with utoipa
  - [ ] 4.7: Protect endpoint with `admin.audit.view` permission
  - [ ] 4.8: Add unit tests for query endpoints

- [ ] Task 5: Implement audit export functionality (AC: #9)
  - [ ] 5.1: Add `POST /api/v1/admin/audit-log/export` endpoint
  - [ ] 5.2: Support CSV and JSON export formats
  - [ ] 5.3: Validate export date range
  - [ ] 5.4: Generate CSV with proper escaping
  - [ ] 5.5: Generate JSON with proper formatting
  - [ ] 5.6: Stream large exports to avoid memory issues
  - [ ] 5.7: Add compression support
  - [ ] 5.8: Validate requester has `admin.audit.export` permission
  - [ ] 5.9: Add unit tests for export

- [ ] Task 6: Implement security threat detection and alerting (AC: #10)
  - [ ] 6.1: Create `ThreatDetector` service
  - [ ] 6.2: Implement brute force detection (multiple failed logins within window)
  - [ ] 6.3: Implement suspicious activity detection (unusual patterns, time anomalies)
  - [ ] 6.4: Implement rate limit violation detection
  - [ ] 6.5: Create `ThreatLevel` enum (info, warning, critical, severe)
  - [ ] 6.6: Implement automatic protective actions (lock account, notify admins)
  - [ ] 6.7: Implement alert notification service (email, Slack, webhook)
  - [ ] 6.8: Add threat metrics dashboard
  - [ ] 6.9: Add unit tests for threat detection

- [ ] Task 7: Create audit analytics dashboard components (AC: #10)
  - [ ] 7.1: Create `AuditLogTable` component
  - [ ] 7.2: Implement event type filter
  - [ ] 7.3: Implement date range picker
  - [ ] 7.4: Implement full-text search
  - [ ] 7.5: Implement pagination with page controls
  - [ ] 7.6: Implement event detail modal/panel
  - [ ] 7.7: Color-code events by type (auth=blue, authz=green, mfa=purple, api_key=orange, account=gray)
  - [ ] 7.8: Implement expandable rows for metadata
  - [ ] 7.9: Add export button with format selection
  - [ ] 7.10: Add real-time updates via WebSocket (optional)

- [ ] Task 8: Implement audit logging middleware (AC: #1, #2, #3)
  - [ ] 8.1: Create `AuditLoggingMiddleware` for Axum
  - [ ] 8.2: Automatically log all API requests (method, path, status, duration)
  - [ ] 8.3: Extract user context from request extensions
  - [ ] 8.4: Log request body for relevant endpoints (POST, PUT, DELETE)
  - [ ] 8.5: Calculate request duration and log as metadata
  - [ ] 8.6: Implement request/response size logging
  - [ ] 8.7: Add correlation ID support
  - [ ] 8.8: Add unit tests for middleware

- [ ] Task 9: Add data retention and cleanup policies (AC: #1)
  - [ ] 9.1: Implement `AuditRetentionService`
  - [ ] 9.2: Define retention periods by event type (90 days for auth logs, 365 days for account events)
  - [ ] 9.3: Implement automatic cleanup job (cron or Tokio scheduled task)
  - [ ] 9.4: Implement soft delete (mark as archived) instead of permanent delete
  - [ ] 9.5: Implement hard delete after retention period expires
  - [ ] 9.6: Implement cleanup notifications
  - [ ] 9.7: Add unit tests for retention service

- [ ] Task 10: Implement frontend audit log viewer (AC: #10)
  - [ ] 10.1: Create `AuditLogViewer` page component
  - [ ] 10.2: Implement event filtering and search UI
  - [ ] 10.3: Implement event detail view with full metadata
  - [ ] 10.4: Add timestamp formatting (relative time)
  - [ ] 10.5: Implement IP address geolocation (if available)
  - [ ] 10.6: Add device fingerprint display
  - [ ] 10.7: Implement event replay capability (view in original context)
  - [ ] 10.8: Add export functionality
  - [ ] 10.9: Add live audit log updates (WebSocket or polling)
  - [ ] 10.10: Add unit tests for viewer components

- [ ] Task 11: Add comprehensive error handling and validation (AC: All)
  - [ ] 11.1: Create `AuditError` enum with specific error types
  - [ ] 11.2: Implement user-friendly error messages
  - [ ] 11.3: Add error logging to audit log
  - [ ] 11.4: Implement error recovery suggestions
  - [ ] 11.5: Add error boundary handling in frontend
  - [ ] 11.6: Add validation for export requests
  - [ ] 11.7: Add validation for date ranges
  - [ ] 11.8: Add unit tests for error handling

- [ ] Task 12: Create admin audit dashboard (AC: #10)
  - [ ] 12.1: Create `AdminAuditDashboard` layout
  - [ ] 12.2: Add audit statistics widgets (event count by type, threat level distribution)
  - [ ] 12.3: Add timeline view of recent security events
  - [ ] 12.4: Add user activity heat map
  - [ ] 12.5: Add threat alerts panel
  - [ ] 12.6: Implement quick filters (last 24h, last 7d, last 30d)
  - [ ] 12.7: Add export dashboard with format options
  - [ ] 12.8: Add real-time threat counter
  - [ ] 12.9: Add unit tests for dashboard widgets

## Dev Notes

### Architecture Alignment

This story implements **Audit Trail** per Epic 15 requirements:

- **Backend Location**: `crates/qa-pms-audit/src/`
- **New Crate**: `qa-pms-audit` for dedicated audit functionality
- **Middleware**: `crates/qa-pms-audit/src/middleware/audit_logging.rs`
- **Security**: Comprehensive logging, threat detection, data retention policies
- **Compliance**: GDPR-ready (data deletion after retention period), SOC2-ready (structured logs)

### Technical Implementation Details

#### Dependencies to Add
```toml
# New audit crate
[workspace.dependencies]
qa-pms-audit = { path = "crates/qa-pms-audit", version = "0.1.0" }

# crates/qa-pms-audit/Cargo.toml
[dependencies]
sqlx = { version = "0.7", features = ["postgres", "runtime-tokio", "chrono"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
tracing = "0.1"
tokio = { version = "1.35", features = ["full"] }
uuid = { version = "1.6", features = ["v4", "serde"] }
chrono = "0.4"
ipnetwork = "2.9"  # For IP geolocation (optional)

# Frontend dependencies
react-table = "^8.11"
recharts = "^3.6"
date-fns = "^3.0"
```

#### Database Schema
```sql
-- Main audit log table
CREATE TABLE audit_log (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    event_type VARCHAR(100) NOT NULL,
    user_id UUID REFERENCES users(id),
    timestamp TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    ip_address INET,
    user_agent TEXT,
    device_fingerprint TEXT,
    session_id UUID,
    jwt_jti TEXT,
    resource_type VARCHAR(50),
    resource_id UUID,
    action VARCHAR(100),
    result VARCHAR(50), -- 'success', 'failure', 'partial_success'
    status_code INTEGER,
    metadata JSONB, -- Flexible additional data
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

-- Indexes for performance
CREATE INDEX idx_audit_log_user_id ON audit_log(user_id);
CREATE INDEX idx_audit_log_timestamp ON audit_log(timestamp DESC);
CREATE INDEX idx_audit_log_event_type ON audit_log(event_type);
CREATE INDEX idx_audit_log_resource_type ON audit_log(resource_type);
CREATE INDEX idx_audit_log_resource_id ON audit_log(resource_id);
CREATE INDEX idx_audit_log_result ON audit_log(result);

-- Soft delete / archive table
CREATE TABLE audit_log_archive (
    LIKE audit_log INCLUDING ALL
);

-- Data retention policy table
CREATE TABLE audit_retention_policy (
    event_type VARCHAR(100) PRIMARY KEY,
    retention_days INTEGER NOT NULL,
    action_after_retention VARCHAR(50) NOT NULL -- 'delete', 'archive'
);

-- Threat detection logs
CREATE TABLE threat_log (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    threat_type VARCHAR(50) NOT NULL,
    severity VARCHAR(20) NOT NULL, -- 'info', 'warning', 'critical', 'severe'
    user_id UUID REFERENCES users(id),
    description TEXT,
    ip_address INET,
    detected_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    protective_action_taken VARCHAR(100),
    resolved_at TIMESTAMP,
    created_by UUID REFERENCES users(id),
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

-- Insert retention policies
INSERT INTO audit_retention_policy (event_type, retention_days, action_after_retention) VALUES
('auth_login', 90, 'archive'),
('auth_logout', 90, 'archive'),
('token_refresh', 30, 'archive'),
('mfa_enable', 180, 'delete'),
('mfa_verify', 180, 'archive'),
('password_change', 90, 'archive'),
('user_created', 2555, 'delete'), -- 7 years for GDPR
('user_deleted', 2555, 'delete'),
('api_key_created', 365, 'archive'),
('api_key_rotated', 365, 'archive'),
('api_key_revoked', 365, 'delete'),
('authz_check', 365, 'archive'),
('account_modified', 365, 'archive');
```

#### Audit Service Implementation
```rust
use crate::audit::event_types::{AuditEventType, AuthEventType, AuthzEventType, MfaEventType, ApiKeyEventType, AccountEventType};
use crate::audit::models::{AuditLog, ThreatLog};
use sqlx::PgPool;
use tracing::{info, warn, error};
use chrono::{Utc, Duration};
use uuid::Uuid;

pub struct AuditService {
    db: Arc<PgPool>,
    threat_detector: Arc<ThreatDetector>,
}

impl AuditService {
    pub async fn log_event(
        &self,
        event_type: AuditEventType,
        user_id: Option<Uuid>,
        details: AuditDetails,
    ) -> Result<(), AuditError> {
        // Build metadata JSONB
        let metadata = serde_json::to_value(&details).unwrap();
        
        // Insert into audit log
        sqlx::query!(
            r#"
            INSERT INTO audit_log 
+                (event_type, user_id, timestamp, ip_address, user_agent, 
+                 device_fingerprint, session_id, jwt_jti, resource_type, 
+                 resource_id, action, result, metadata, created_at)
+            VALUES ($1, $2, NOW(), $3, $4, $5, $6, $7, $8, $9, $10, $11, NOW())
+            "#
        )
        .bind(
            &event_type.to_string(),
            &user_id,
            &details.ip_address,
            &details.user_agent,
            &details.device_fingerprint,
            &details.session_id,
            &details.jwt_jti,
            &details.resource_type.map(|s| s.to_string()),
            &details.resource_id,
            &details.action,
            &details.result,
            &metadata
        )
        .execute(&*self.db)
        .await?;
        
        info!(
            event_type = %event_type,
            user_id = %user_id.map(|u| u.to_string()).unwrap_or(&"system".to_string()),
            "Audit event logged"
        );
        
        Ok(())
    }
    
    pub async fn log_auth_event(
        &self,
        auth_event: AuthEvent,
    ) -> Result<(), AuditError> {
        let details = AuditDetails::from_auth_event(&auth_event);
        
        self.log_event(AuditEventType::Auth(auth_event.event_type()), details.user_id, details).await
    }
    
    pub async fn log_authz_event(
        &self,
        authz_event: AuthzEvent,
    ) -> Result<(), AuditError> {
        let details = AuditDetails::from_authz_event(&authz_event);
        
        self.log_event(AuditEventType::Authz(authz_event.event_type()), details.user_id, details).await
    }
    
    pub async fn log_mfa_event(
        &self,
        mfa_event: MfaEvent,
    ) -> Result<(), AuditError> {
        let details = AuditDetails::from_mfa_event(&mfa_event);
        
        self.log_event(AuditEventType::Mfa(mfa_event.event_type()), details.user_id, details).await
    }
    
    pub async fn log_api_key_event(
        &self,
        api_key_event: ApiKeyEvent,
    ) -> Result<(), AuditError> {
        let details = AuditDetails::from_api_key_event(&api_key_event);
        
        self.log_event(AuditEventType::ApiKey(api_key_event.event_type()), details.user_id, details).await
    }
}

#[derive(Debug)]
pub struct AuditDetails {
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
    pub device_fingerprint: Option<String>,
    pub session_id: Option<Uuid>,
    pub jwt_jti: Option<String>,
    pub resource_type: Option<String>,
    pub resource_id: Option<Uuid>,
    pub action: String,
    pub result: Option<String>,
}

impl AuditDetails {
    fn from_auth_event(event: &AuthEvent) -> Self {
        Self {
            ip_address: event.ip_address.clone(),
            user_agent: event.user_agent.clone(),
            device_fingerprint: event.device_fingerprint.clone(),
            session_id: event.session_id,
            jwt_jti: event.jwt_jti,
            resource_type: None,
            resource_id: None,
            action: format!("{:?}", event.event_type),
            result: Some(event.result.to_string()),
        }
    }
    
    fn from_authz_event(event: &AuthzEvent) -> Self {
        Self {
            ip_address: event.ip_address.clone(),
            user_agent: event.user_agent.clone(),
            device_fingerprint: None,
            session_id: None,
            jwt_jti: None,
            resource_type: Some(event.resource_type.to_string()),
            resource_id: event.resource_id,
            action: "permission_check",
            result: Some(event.result.to_string()),
        }
    }
    
    fn from_mfa_event(event: &MfaEvent) -> Self {
        Self {
            ip_address: event.ip_address.clone(),
            user_agent: event.user_agent.clone(),
            device_fingerprint: None,
            session_id: None,
            jwt_jti: None,
            resource_type: None,
            resource_id: None,
            action: format!("{:?}", event.event_type),
            result: Some(event.result.to_string()),
        }
    }
    
    fn from_api_key_event(event: &ApiKeyEvent) -> Self {
        Self {
            ip_address: event.ip_address,
            user_agent: None,
            device_fingerprint: None,
            session_id: None,
            jwt_jti: None,
            resource_type: None,
            resource_id: Some(event.service_account_id.to_string()),
            action: format!("{:?}", event.event_type),
            result: Some(event.result.to_string()),
        }
    }
}
```

#### Threat Detection Service
```rust
use sqlx::PgPool;
use chrono::Utc;

#[derive(Debug, Clone)]
pub struct ThreatDetector {
    db: Arc<PgPool>,
}

#[derive(Debug, Clone)]
pub struct ThreatLevel {
    pub severity: String,
    pub description: String,
    pub protective_actions: Vec<String>,
}

impl ThreatDetector {
    /// Detect brute force login attempts
    pub async fn detect_brute_force(
        &self,
        user_id: &Uuid,
    ) -> Result<Option<ThreatLevel>, AuditError> {
        // Check for 5+ failed logins in last 15 minutes
        let threshold = Utc::now() - Duration::minutes(15);
        
        let failed_attempts = sqlx::query!(
            r#"
                SELECT COUNT(*) 
                FROM audit_log 
                WHERE user_id = $1 
                  AND event_type = 'auth_login' 
                  AND result = 'failure' 
                  AND timestamp >= $2
                "#
        )
        .bind(user_id, &threshold)
        .fetch_one(&*self.db)
        .await?
        .map(|r| r.get::<i64>(0).unwrap_or(0))
        .unwrap_or(0);
        
        if failed_attempts >= 5 {
            Ok(Some(ThreatLevel {
                severity: "critical".to_string(),
                description: "Brute force attack detected: 5+ failed login attempts in 15 minutes".to_string(),
                protective_actions: vec![
                    "Lock user account".to_string(),
                    "Notify security team".to_string(),
                    "Send email alert to user".to_string(),
                ],
            }))
        } else {
            Ok(None)
        }
    }
    
    /// Detect suspicious activity patterns
    pub async fn detect_suspicious_activity(
        &self,
        user_id: &Uuid,
    ) -> Result<Option<ThreatLevel>, AuditError> {
        // Check for unusual access patterns (e.g., multiple countries, unusual times)
        let recent_events = sqlx::query!(
            r#"
                SELECT ip_address, COUNT(*) as count
                FROM audit_log 
                WHERE user_id = $1 
                  AND timestamp >= NOW() - INTERVAL '24 hours'
                "#
        )
        .bind(user_id)
        .fetch_all(&*self.db)
        .await?;
        
        // Check for multiple IP addresses
        let unique_ips: std::collections::HashSet::new();
        for event in &recent_events {
            if let Some(ip) = &event.ip_address {
                unique_ips.insert(ip);
            }
        }
        
        if unique_ips.len() > 3 {
            Ok(Some(ThreatLevel {
                severity: "warning".to_string(),
                description: format!("Suspicious activity: Access from {} different IP addresses in 24 hours", unique_ips.len()).to_string(),
                protective_actions: vec![
                    "Require MFA verification".to_string(),
                    "Send security alert".to_string(),
                ],
            }))
        } else {
            Ok(None)
        }
    }
}
```

#### API Endpoints
```rust
use axum::{
    extract::{Query, State, TypedHeader},
    http::StatusCode,
    Json, Request, Response,
};
use utoipa::ToSchema;

// Query audit log
#[utoipa::path(
    get(path = "/admin/audit-log"),
    params(
        ("event_type", QueryParam, description = "Filter by event type"),
        ("user_id", QueryParam, description = "Filter by user ID"),
        ("from_date", QueryParam, description = "Start date (ISO 8601)"),
        ("to_date", QueryParam, description = "End date (ISO 8601)"),
        ("resource_type", QueryParam, description = "Filter by resource type"),
        ("resource_id", QueryParam, description = "Filter by resource ID"),
        ("page", QueryParam, description = "Page number"),
        ("per_page", QueryParam, description = "Items per page"),
        ("sort_by", QueryParam, description = "Sort field"),
        ("sort_order", QueryParam, description = "Sort order (asc/desc)"),
    ),
    responses(
        (status = 200, description = "Paginated audit log", body = AuditLogResponse),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden"),
    ),
    tag = "Admin Audit",
    security(
        name = "admin.audit.view",
        scopes = ["read"]
    )
)]
pub async fn get_audit_log(
    State(audit_service): State<Arc<AuditService>>,
    Query(query): Query<AuditLogQuery>,
) -> Result<Json<AuditLogResponse>, ApiError> {
    // Verify admin permission
    let user_context = verify_admin_permission(&req)?;
    
    let pagination = AuditPagination {
        page: query.page.unwrap_or(1),
        per_page: query.per_page.unwrap_or(50),
        sort_by: query.sort_by.unwrap_or("timestamp".to_string()),
        sort_order: query.sort_order.unwrap_or("desc".to_string()),
    };
    
    let (logs, total) = audit_service.get_audit_log_paginated(
        &query,
        &pagination,
    ).await?;
    
    Ok(Json(AuditLogResponse {
        logs,
        total,
        page: pagination.page,
        per_page: pagination.per_page,
        total_pages: (total / pagination.per_page as f64).ceil() as i64,
    }))
}

// Export audit log
#[utoipa::path(
    post(path = "/admin/audit-log/export"),
    request_body = ExportAuditRequest,
    responses(
        (status = 200, description = "Audit log export", body = ExportResponse),
        (status = 400, description = "Invalid request"),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden"),
    ),
    tag = "Admin Audit",
    security(
        name = "admin.audit.export",
        scopes = ["read"]
    )
)]
pub async fn export_audit_log(
    State(audit_service): State<Arc<AuditService>>,
    Json(request): Json<ExportAuditRequest>,
) -> Result<Response, ApiError> {
    // Verify admin permission
    let user_context = verify_admin_permission(&req)?;
    
    // Validate date range
    let from_date = request.from_date.parse::<chrono::DateTime<Utc>>().ok();
    let to_date = request.to_date.parse::<chrono::DateTime<Utc>>().ok();
    
    // Get audit logs
    let logs = audit_service.get_audit_logs_between(
        &from_date.unwrap_or_else(|| Utc::now() - Duration::days(90)),
        &to_date.unwrap_or_else(|| Utc::now()),
    ).await?;
    
    // Generate export based on format
    match request.format.as_str() {
        "csv" => {
            let csv = generate_csv_export(&logs);
            Response::builder()
                .status(StatusCode::OK)
                .header("Content-Type", "text/csv")
                .header("Content-Disposition", format!("attachment; filename=audit-log-{}.csv", Utc::now().format("%Y%m%d")))
                .body(csv)
                .unwrap()
        }
        "json" => {
            let json = serde_json::to_string(&logs)?;
            Response::builder()
                .status(StatusCode::OK)
                .header("Content-Type", "application/json")
                .header("Content-Disposition", format!("attachment; filename=audit-log-{}.json", Utc::now().format("%Y%m%d")))
                .body(json)
                .unwrap()
        }
        _ => {
            Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .body("Invalid format. Use 'csv' or 'json'")
                .unwrap()
        }
    }
}
```

#### Audit Logging Middleware
```rust
use axum::{
    extract::{Request, State},
    http::{StatusCode, HeaderMap},
    middleware::Next,
    response::Response,
    Extension,
};
use std::time::Instant;
use tracing::info;

pub async fn audit_logging_middleware<B>(
    State(audit_service): State<Arc<AuditService>>,
    mut req: Request<B>,
    next: Next<B>,
) -> Response {
    let start = Instant::now();
    
    // Execute request
    let response = next.run(req).await;
    
    let duration = start.elapsed();
    
    // Extract user context if available
    let user_id = req.extensions().get::<UserContext>()
        .map(|uc| uc.user_id);
    
    // Extract request info
    let method = req.method().to_string();
    let path = req.uri().path();
    let status = response.status().as_u16();
    let ip_address = req.headers()
        .get("x-forwarded-for")
        .or_else(|| {
            req.headers().get("x-real-ip")
        })
        .and_then(|h| h.to_str().ok());
    
    let user_agent = req.headers()
        .get("user-agent")
        .and_then(|h| h.to_str().ok());
    
    // Log API request/response
    if let Some(user) = user_id {
        // Only log relevant endpoints (exclude health checks, metrics, etc.)
        if should_log_endpoint(&path) {
            let event = AuditEvent::ApiRequest {
                event_type: AuditEventType::Auth("api_request".to_string()),
                user_id: Some(user.clone()),
                ip_address: ip_address.map(|s| s.to_string()),
                user_agent: user_agent.map(|s| s.to_string()),
                session_id: None,
                jwt_jti: None,
                resource_type: None,
                resource_id: None,
                action: format!("{} {}", method, path),
                result: Some(if status >= 200 && status < 300 { "success".to_string() } else { "failure".to_string() }),
            };
            
            if let Err(e) = audit_service.log_auth_event(event).await {
                error!("Failed to log audit event: {}", e);
            }
        }
    }
    
    // Add audit headers to response
    let mut response = response;
    response.headers_mut().insert(
        "X-Request-ID".parse().unwrap(),
        HeaderValue::from_str(&uuid::Uuid::new_v4().to_string()).unwrap(),
    );
    response.headers_mut().insert(
        "X-Response-Time".parse().unwrap(),
        HeaderValue::from_str(&format!("{}ms", duration.as_millis())).unwrap(),
    );
    
    response
}

fn should_log_endpoint(path: &str) -> bool {
    // Don't log health checks, static assets, etc.
    !matches!(path, 
        "/health" | "/metrics" | "/favicon" | "/static" | 
        "/ws" | "/ready"
    )
}
```

#### Frontend Audit Log Viewer
```typescript
// frontend/src/components/admin/audit/AuditLogTable.tsx
import { useState, useEffect } from 'react';
import { api } from '@/api';
import { Filter, Clock, Shield, AlertTriangle, Download, Search } from 'lucide-react';

interface AuditLog {
  id: string;
  event_type: string;
  user_id: string;
  username?: string;
  timestamp: string;
  ip_address?: string;
  user_agent?: string;
  resource_type?: string;
  resource_id?: string;
  action: string;
  result: string;
  metadata?: Record<string, any>;
}

export const AuditLogTable: React.FC = () => {
  const [logs, setLogs] = useState<AuditLog[]>([]);
  const [loading, setLoading] = useState(false);
  const [filters, setFilters] = useState<{
    event_type: string;
    user_id?: string;
    from_date?: string;
    to_date?: string;
    search: string;
  }>({
    event_type: '',
    search: '',
  });
  const [pagination, setPagination] = useState({ page: 1, perPage: 50, totalPages: 1 });
  
  useEffect(() => {
    loadAuditLogs();
  }, [pagination, filters]);
  
  const loadAuditLogs = async () => {
    setLoading(true);
    try {
      const response = await api.get('/admin/audit-log', {
        params: { ...filters, ...pagination }
      });
      setLogs(response.data.logs);
      setPagination(prev => ({
        ...prev,
        totalPages: response.data.total_pages
      }));
    } catch (error) {
      console.error('Failed to load audit logs:', error);
    } finally {
      setLoading(false);
    }
  };
  
  const getEventIcon = (eventType: string) => {
    switch (eventType) {
      case 'auth_login': return <Shield className="icon auth" />;
      case 'auth_logout': return <Shield className="icon auth" />;
      case 'authz_check': return <Filter className="icon authz" />;
      case 'mfa_enable': return <Clock className="icon mfa" />;
      case 'mfa_verify': return <Clock className="icon mfa" />;
      case 'api_key_created': return <Download className="icon api-key" />;
      case 'api_key_rotated': return <Download className="icon api-key" />;
      case 'api_key_revoked': return <Download className="icon api-key" />;
      case 'account_created': return <User className="icon account" />;
      case 'account_modified': return <User className="icon account" />;
      default: return <AlertTriangle className="icon default" />;
    }
  };
  
  const getEventColor = (result: string) => {
    switch (result) {
      case 'success': return 'green';
      case 'failure': return 'red';
      case 'partial_success': return 'yellow';
      default: return 'gray';
    }
  };
  
  const exportLogs = async (format: string) => {
    try {
      const response = await api.post('/admin/audit-log/export', {
        format,
        from_date: filters.from_date,
        to_date: filters.to_date
      });
      
      // Create download link
      const blob = new Blob([response.data], { type: format === 'csv' ? 'text/csv' : 'application/json' });
      const url = URL.createObjectURL(blob);
      const a = document.createElement('a');
      a.href = url;
      a.download = `audit-log.${format}`;
      document.body.appendChild(a);
      a.click();
    } catch (error) {
      toast.error('Failed to export audit logs');
    }
  };
  
  const formatDate = (timestamp: string) => {
    return new Date(timestamp).toLocaleString();
  };
  
  return (
    <div className="audit-log-viewer">
      <div className="header">
        <h1>Audit Log Viewer</h1>
        <div className="actions">
          <button onClick={() => loadAuditLogs()}>
            <RefreshCw className="icon" />
            Refresh
          </button>
          <button onClick={() => exportLogs('csv')}>
            <Download className="icon" />
            Export CSV
          </button>
          <button onClick={() => exportLogs('json')}>
            <Download className="icon" />
            Export JSON
          </button>
        </div>
      </div>
      
      <div className="filters">
        <div className="filter-group">
          <label>Event Type:</label>
          <select 
            value={filters.event_type}
            onChange={(e) => setFilters(prev => ({ ...prev, event_type: e.target.value }))}
          >
            <option value="">All Events</option>
            <option value="auth_login">Login</option>
            <option value="auth_logout">Logout</option>
            <option value="mfa_enable">MFA Enable</option>
            <option value="mfa_verify">MFA Verify</option>
            <option value="api_key_created">API Key Created</option>
            <option value="api_key_revoked">API Key Revoked</option>
            <option value="account_created">User Created</option>
            <option value="account_modified">User Modified</option>
          </select>
        </div>
        
        <div className="filter-group">
          <label>Date Range:</label>
          <input
            type="date"
            value={filters.from_date}
            onChange={(e) => setFilters(prev => ({ ...prev, from_date: e.target.value }))}
            placeholder="From"
          />
          <input
            type="date"
            value={filters.to_date}
            onChange={(e) => setFilters(prev => ({ ...prev, to_date: e.target.value }))}
            placeholder="To"
          />
        </div>
        
        <div className="filter-group">
          <label>Search:</label>
          <input
            type="text"
            value={filters.search}
            onChange={(e) => setFilters(prev => ({ ...prev, search: e.target.value }))}
            placeholder="Search IP, user, resource..."
          />
        </div>
        
        <button onClick={() => setFilters({ event_type: '', search: '', from_date: '', to_date: '' })}>
          Clear Filters
        </button>
      </div>
      
      <div className="statistics">
        <h3>Audit Statistics</h3>
        <div className="stats-grid">
          <div className="stat-card">
            <span className="label">Total Events:</span>
            <span className="value">{logs.length}</span>
          </div>
          <div className="stat-card">
            <span className="label">Current Page:</span>
            <span className="value">{pagination.page} / {pagination.totalPages}</span>
          </div>
        </div>
      </div>
      
      {loading ? (
        <div className="loading">Loading audit logs...</div>
      ) : logs.length === 0 ? (
        <div className="empty-state">
          <Shield className="icon" />
          <p>No audit logs found</p>
        </div>
      ) : (
        <div className="log-table">
          <table>
            <thead>
              <tr>
                <th>Timestamp</th>
                <th>Event Type</th>
                <th>User</th>
                <th>IP Address</th>
                <th>Action</th>
                <th>Result</th>
                <th>Details</th>
              </tr>
            </thead>
            <tbody>
              {logs.map(log => (
                <tr key={log.id} className={`result-${getEventColor(log.result)}`}>
                  <td>{formatDate(log.timestamp)}</td>
                  <td>
                    <div className="event-icon">
                      {getEventIcon(log.event_type)}
                    </div>
                    <span>{log.event_type}</span>
                  </td>
                  <td>{log.username || 'System'}</td>
                  <td>{log.ip_address || '-'}</td>
                  <td>{log.action}</td>
                  <td className={`result-badge ${getEventColor(log.result)}`}>
                    {log.result}
                  </td>
                  <td>
                    {log.metadata && Object.keys(log.metadata).length > 0 && (
                      <details>
                        <summary>View Details</summary>
                        <pre>{JSON.stringify(log.metadata, null, 2)}</pre>
                      </details>
                    )}
                  </td>
                </tr>
              ))}
            </tbody>
          </table>
          
          <div className="pagination">
            <button 
              onClick={() => setPagination(prev => ({ ...prev, page: Math.max(1, prev.page - 1) }))}
              disabled={pagination.page === 1}
            >
              Previous
            </button>
            <span>Page {pagination.page} of {pagination.totalPages}</span>
            <button 
              onClick={() => setPagination(prev => ({ ...prev, page: Math.min(pagination.totalPages, prev.page + 1) }))}
              disabled={pagination.page === pagination.totalPages}
            >
              Next
            </button>
          </div>
        </div>
      )}
    </div>
  );
};
```

### Testing Strategy

#### Unit Tests
- Audit event type validation
- Event logging logic
- Threat detection algorithms
- Metadata serialization
- Date range validation
- CSV/JSON export generation

#### Integration Tests
- Complete audit logging flow
- Event query with various filters
- Event pagination
- Threat detection triggering alerts
- Audit log export
- Data retention policy application
- Soft delete/archive functionality

#### End-to-End Tests
- Admin views audit log
- Admin filters audit log by event type
- Admin searches audit log
- Admin exports audit log
- Brute force threat detected
- Suspicious activity detected
- Audit events are logged correctly
- Threat alerts are generated
- Data retention policies are applied

#### Security Tests
- Attempt to access audit log without admin permission
- Attempt to export audit log without permission
- Attempt SQL injection in audit log queries
- Attempt to manipulate audit log data
- Attempt to delete audit logs (should be archived, not deleted)
- Brute force detection can be bypassed
- Threat detection generates false positives

### File List

**Files to be created:**
- `crates/qa-pms-audit/src/lib.rs`
- `crates/qa-pms-audit/src/service.rs`
- `crates/qa-pms-audit/src/threat_detector.rs`
- `crates/qa-pms-audit/src/middleware/audit_logging.rs`
- `crates/qa-pms-audit/src/models.rs`
- `crates/qa-pms-audit/src/event_types.rs`
- `migrations/create_audit_log_table.sql`
- `migrations/create_threat_log_table.sql`
- `migrations/create_audit_retention_policy_table.sql`
- `frontend/src/components/admin/audit/AuditLogTable.tsx`
- `frontend/src/pages/admin/AdminAuditDashboard.tsx`

**Files to be modified:**
- `Cargo.toml` (workspace level, add qa-pms-audit crate)
- `crates/qa-pms-api/src/main.rs` (add audit routes)
- `crates/qa-pms-auth/src/lib.rs` (export audit service)
- `frontend/src/api/admin.ts` (add audit methods)
- `crates/qa-pms-auth/src/middleware/auth_middleware.rs` (add audit logging calls)
```
```</think>Perfeito! Criei a Story 15.10 - Audit Trail com sucesso. 

Agora vou criar as próximas 2 stories restantes do Epic 15 (15.11-15.12) para completar o Epic 15. Vou começar com a Story 15.11 - Rate Limiting:<tool_call>edit_file<arg_key>display_description</arg_key><arg_value>Create Story 15.11 - Rate Limiting</arg_value><arg_key>mode</arg_key><arg_value>create</arg_value><arg_key>path</arg_key><arg_value>C:\Users\User\Desktop\estrategia preventiva-reativa\_bmad-output\implementation-artifacts\15-11-rate-limiting.md</arg_value></tool_call>