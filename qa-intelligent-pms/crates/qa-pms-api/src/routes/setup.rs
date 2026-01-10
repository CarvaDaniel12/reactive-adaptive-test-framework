//! Setup wizard API endpoints.
//!
//! Provides endpoints for the initial configuration wizard:
//! - Profile configuration
//! - Integration testing (Jira, Postman, Testmo)
//! - Setup completion and status

use std::sync::Arc;

use axum::{
    extract::{Json, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;
use tracing::{info, warn};
use utoipa::ToSchema;

use crate::app::AppState;
use crate::startup::{StartupValidationReport, StartupValidator};
use qa_pms_core::error::ApiError;
use qa_pms_core::health::HealthCheck;
use qa_pms_core::health::HealthStatus;
use qa_pms_jira::JiraHealthCheck;
use qa_pms_postman::PostmanHealthCheck;
use qa_pms_testmo::TestmoHealthCheck;

// ============================================================================
// Router
// ============================================================================

/// Create the setup wizard router.
///
/// All routes are prefixed with `/api/v1/setup`.
pub fn router() -> Router<AppState> {
    Router::new()
        .route("/api/v1/setup/profile", post(save_profile))
        .route("/api/v1/setup/integrations/jira/test", post(test_jira))
        .route(
            "/api/v1/setup/integrations/postman/test",
            post(test_postman),
        )
        .route("/api/v1/setup/integrations/testmo/test", post(test_testmo))
        .route("/api/v1/setup/complete", post(complete_setup))
        .route("/api/v1/setup/status", get(get_status))
}

// ============================================================================
// Request/Response Types
// ============================================================================

/// User profile configuration request.
#[derive(Debug, Clone, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ProfileRequest {
    /// User's display name
    pub display_name: String,
    /// Jira email/username
    pub jira_email: String,
    /// Ticket states to show (e.g., "Ready for QA", "In Progress")
    pub ticket_states: Vec<String>,
}

/// Jira connection test request.
#[derive(Debug, Clone, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct JiraTestRequest {
    /// Jira instance URL (e.g., `https://company.atlassian.net`)
    pub instance_url: String,

    // === API Token Authentication (recommended) ===
    /// User email for API Token auth
    #[serde(default)]
    pub email: Option<String>,
    /// API Token from <https://id.atlassian.com/manage-profile/security/api-tokens>
    #[serde(default)]
    pub api_token: Option<String>,

    // === OAuth 2.0 Authentication (alternative) ===
    /// OAuth 2.0 Client ID
    #[serde(default)]
    pub client_id: Option<String>,
    /// OAuth 2.0 Client Secret
    #[serde(default)]
    pub client_secret: Option<String>,
    /// Jira Cloud ID (obtained after OAuth callback)
    #[serde(default)]
    pub cloud_id: Option<String>,
    /// Access token (obtained after OAuth callback)
    #[serde(default)]
    pub access_token: Option<String>,
}

impl JiraTestRequest {
    /// Check if API Token auth is configured.
    pub const fn has_api_token(&self) -> bool {
        self.email.is_some() && self.api_token.is_some()
    }

    /// Check if OAuth auth is configured.
    #[allow(dead_code)]
    pub const fn has_oauth(&self) -> bool {
        self.cloud_id.is_some() && self.access_token.is_some()
    }
}

/// Postman connection test request.
#[derive(Debug, Clone, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct PostmanTestRequest {
    /// Postman API key
    pub api_key: String,
    /// Optional workspace ID to validate
    #[serde(default)]
    pub workspace_id: Option<String>,
}

/// Minimum length for Postman API keys (basic validation).
const MIN_POSTMAN_API_KEY_LENGTH: usize = 32;

/// Minimum length for OAuth client credentials (basic validation).
const MIN_OAUTH_CREDENTIAL_LENGTH: usize = 10;

/// Testmo connection test request.
#[derive(Debug, Clone, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct TestmoTestRequest {
    /// Testmo instance URL
    pub instance_url: String,
    /// Testmo API key
    pub api_key: String,
}

/// Splunk configuration request (manual, no test).
#[derive(Debug, Clone, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct SplunkConfigRequest {
    /// Splunk base URL
    pub base_url: String,
    /// Default index to search
    #[serde(default)]
    pub default_index: Option<String>,
}

/// Connection test response for all integrations.
#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ConnectionTestResponse {
    /// Whether the connection was successful
    pub success: bool,
    /// Human-readable message (success or error details)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// Number of accessible workspaces (Postman)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace_count: Option<u32>,
    /// Number of accessible projects (Jira, Testmo)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project_count: Option<u32>,
}

impl ConnectionTestResponse {
    /// Create a successful response.
    fn success(message: impl Into<String>) -> Self {
        Self {
            success: true,
            message: Some(message.into()),
            workspace_count: None,
            project_count: None,
        }
    }

    /// Create a failed response.
    fn failure(message: impl Into<String>) -> Self {
        Self {
            success: false,
            message: Some(message.into()),
            workspace_count: None,
            project_count: None,
        }
    }

    /// Add workspace count to response.
    #[cfg(test)]
    const fn with_workspaces(mut self, count: u32) -> Self {
        self.workspace_count = Some(count);
        self
    }

    /// Add project count to response.
    const fn with_projects(mut self, count: u32) -> Self {
        self.project_count = Some(count);
        self
    }
}

/// Setup completion request.
#[derive(Debug, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CompleteSetupRequest {
    /// Profile configuration
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile: Option<ProfileRequest>,
    /// Jira configuration
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jira: Option<JiraTestRequest>,
    /// Postman configuration
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postman: Option<PostmanTestRequest>,
    /// Testmo configuration
    #[serde(skip_serializing_if = "Option::is_none")]
    pub testmo: Option<TestmoTestRequest>,
    /// Splunk configuration (manual)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub splunk: Option<SplunkConfigRequest>,
}

/// Setup validation error returned by the complete setup flow.
#[derive(Debug, Clone, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct SetupValidationError {
    pub field: String,
    pub message: String,
    pub step: String,
    pub fix_path: String,
}

impl SetupValidationError {
    fn new(
        field: impl Into<String>,
        message: impl Into<String>,
        step: impl Into<String>,
        fix_path: impl Into<String>,
    ) -> Self {
        Self {
            field: field.into(),
            message: message.into(),
            step: step.into(),
            fix_path: fix_path.into(),
        }
    }
}

/// Setup completion response.
#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CompleteSetupResponse {
    /// Whether setup completed successfully
    pub success: bool,
    /// Validation errors (if any)
    pub errors: Vec<SetupValidationError>,
    /// Configured integrations
    pub configured_integrations: Vec<String>,
    /// Path where the YAML config was written (on success)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config_path: Option<String>,
    /// Optional startup validation report (health checks) run during completion
    #[serde(skip_serializing_if = "Option::is_none")]
    pub startup_validation: Option<StartupValidationReport>,
}

/// Setup status response.
#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct SetupStatusResponse {
    /// Whether initial setup is complete
    pub complete: bool,
    /// List of configured integrations
    pub configured_integrations: Vec<String>,
    /// Whether profile is configured
    pub profile_configured: bool,
    /// Server address for reference
    pub server_address: String,
}

/// Simple success response.
#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct SuccessResponse {
    /// Success status
    pub success: bool,
    /// Optional message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

// ============================================================================
// Setup State (temporary storage during wizard)
// ============================================================================

/// Temporary setup state stored during the wizard flow.
///
/// This is stored in memory until setup is complete, then persisted to config.
#[derive(Debug, Default, Clone)]
pub struct SetupState {
    /// User profile configuration
    pub profile: Option<ProfileRequest>,
    /// Jira credentials (if tested successfully)
    pub jira: Option<JiraTestRequest>,
    /// Postman credentials (if tested successfully)
    pub postman: Option<PostmanTestRequest>,
    /// Testmo credentials (if tested successfully)
    pub testmo: Option<TestmoTestRequest>,
    /// Splunk configuration (manual, no test)
    pub splunk: Option<SplunkConfigRequest>,
}

impl SetupState {
    /// Get list of configured integrations.
    pub fn configured_integrations(&self) -> Vec<String> {
        let mut integrations = Vec::new();
        if self.jira.is_some() {
            integrations.push("jira".to_string());
        }
        if self.postman.is_some() {
            integrations.push("postman".to_string());
        }
        if self.testmo.is_some() {
            integrations.push("testmo".to_string());
        }
        if self.splunk.is_some() {
            integrations.push("splunk".to_string());
        }
        integrations
    }

    /// Check if profile is configured.
    pub const fn is_profile_configured(&self) -> bool {
        self.profile.is_some()
    }

    /// Check if setup is complete (profile + at least Jira).
    pub const fn is_complete(&self) -> bool {
        self.profile.is_some() && self.jira.is_some()
    }
}

/// Thread-safe setup state store.
pub type SetupStore = Arc<Mutex<SetupState>>;

/// Create a new setup store.
pub fn create_setup_store() -> SetupStore {
    Arc::new(Mutex::new(SetupState::default()))
}

// ============================================================================
// Handlers
// ============================================================================

/// Save user profile configuration.
///
/// Stores the profile temporarily until setup is complete.
#[utoipa::path(
    post,
    path = "/api/v1/setup/profile",
    request_body = ProfileRequest,
    responses(
        (status = 200, description = "Profile saved successfully", body = SuccessResponse),
        (status = 400, description = "Invalid request body", body = qa_pms_core::error::ErrorResponse)
    ),
    tag = "Setup"
)]
pub async fn save_profile(
    State(state): State<AppState>,
    Json(req): Json<ProfileRequest>,
) -> Result<impl IntoResponse, ApiError> {
    // Validate required fields
    if req.display_name.trim().is_empty() {
        return Err(ApiError::Validation("Display name is required".into()));
    }
    if req.jira_email.trim().is_empty() {
        return Err(ApiError::Validation("Jira email is required".into()));
    }
    if req.ticket_states.is_empty() {
        return Err(ApiError::Validation(
            "At least one ticket state is required".into(),
        ));
    }

    // Store in setup state
    {
        let mut setup = state.setup_store.lock().await;
        setup.profile = Some(req.clone());
    }

    info!(
        display_name = %req.display_name,
        jira_email_prefix = %req.jira_email.split('@').next().unwrap_or("unknown"),
        ticket_states_count = req.ticket_states.len(),
        "Profile saved"
    );

    Ok((
        StatusCode::OK,
        Json(SuccessResponse {
            success: true,
            message: Some("Profile saved successfully".into()),
        }),
    ))
}

/// Test Jira connection.
///
/// Test Jira connection with API Token (Basic) or OAuth 2.0 credentials.
///
/// For API Token auth: validates and tests connection immediately.
/// For OAuth auth: validates credential format and stores for later OAuth flow completion.
#[utoipa::path(
    post,
    path = "/api/v1/setup/integrations/jira/test",
    request_body = JiraTestRequest,
    responses(
        (status = 200, description = "Connection test result", body = ConnectionTestResponse),
        (status = 400, description = "Invalid request", body = qa_pms_core::error::ErrorResponse)
    ),
    tag = "Setup"
)]
pub async fn test_jira(
    State(state): State<AppState>,
    Json(req): Json<JiraTestRequest>,
) -> Result<Json<ConnectionTestResponse>, ApiError> {
    // Validate URL format (basic validation without external dependencies)
    let url_lower = req.instance_url.to_lowercase();
    if !url_lower.starts_with("https://") {
        return Ok(Json(ConnectionTestResponse::failure(
            "Jira URL must use HTTPS",
        )));
    }

    // Basic URL structure validation
    if url_lower.len() < 12 || !url_lower.contains('.') {
        return Ok(Json(ConnectionTestResponse::failure(
            "Invalid Jira URL format",
        )));
    }

    // Extract host for validation
    let host_part = url_lower
        .strip_prefix("https://")
        .and_then(|s| s.split('/').next())
        .unwrap_or("");
    
    if host_part.is_empty() || !host_part.contains('.') {
        return Ok(Json(ConnectionTestResponse::failure(
            "Jira URL must have a valid host",
        )));
    }

    // Warn if URL doesn't look like a Jira instance
    if !host_part.contains(".atlassian.net") && !host_part.contains("jira") {
        warn!(url = %req.instance_url, host = %host_part, "Jira URL might be invalid (unexpected host)");
    }

    // Check for API Token auth (preferred)
    let has_api_token = req.has_api_token();
    // Check for OAuth auth (alternative)
    let has_oauth_creds = req.client_id.as_ref().is_some_and(|s| !s.trim().is_empty())
        && req
            .client_secret
            .as_ref()
            .is_some_and(|s| !s.trim().is_empty());

    if !has_api_token && !has_oauth_creds {
        return Ok(Json(ConnectionTestResponse::failure(
            "Either API Token (email + api_token) or OAuth credentials (client_id + client_secret) are required",
        )));
    }

    // Test the connection based on auth method
    if let (Some(email), Some(api_token)) = (&req.email, &req.api_token) {
        let email_prefix = email.split('@').next().unwrap_or("unknown");
        info!(
            url = %req.instance_url,
            email_prefix = %email_prefix,
            "Testing Jira connection with API Token"
        );

        // Actually test the connection
        let client = qa_pms_jira::JiraHealthCheck::with_api_token(
            req.instance_url.clone(),
            email.clone(),
            api_token.clone(),
        );

        let result = client.check().await;

        if result.status != qa_pms_core::health::HealthStatus::Online
            && result.status != qa_pms_core::health::HealthStatus::Degraded
        {
            return Ok(Json(ConnectionTestResponse::failure(
                result
                    .error_message
                    .unwrap_or_else(|| "Connection failed".to_string()),
            )));
        }

        // Store successful test in setup state
        {
            let mut setup = state.setup_store.lock().await;
            setup.jira = Some(req);
        }

        Ok(Json(
            ConnectionTestResponse::success(format!(
                "Connected to Jira successfully (response time: {}ms)",
                result.response_time_ms.unwrap_or(0)
            ))
            .with_projects(1),
        ))
    } else {
        // OAuth flow - validate credentials format before storing
        if let (Some(client_id), Some(client_secret)) = (&req.client_id, &req.client_secret) {
            // Basic validation: client_id and client_secret should not be empty
            if client_id.trim().is_empty() || client_secret.trim().is_empty() {
                return Ok(Json(ConnectionTestResponse::failure(
                    "OAuth client_id and client_secret cannot be empty",
                )));
            }

        // Client IDs typically are UUIDs or base64 strings (basic format check)
        if client_id.len() < MIN_OAUTH_CREDENTIAL_LENGTH
            || client_secret.len() < MIN_OAUTH_CREDENTIAL_LENGTH
        {
            return Ok(Json(ConnectionTestResponse::failure(format!(
                "OAuth credentials appear to be invalid (minimum length: {} characters)",
                MIN_OAUTH_CREDENTIAL_LENGTH
            ))));
        }

            info!(
                url = %req.instance_url,
                "Storing Jira OAuth credentials (OAuth flow requires access token exchange)"
            );

            // Store credentials for OAuth flow (will be validated during complete_setup)
            {
                let mut setup = state.setup_store.lock().await;
                setup.jira = Some(req);
            }

            Ok(Json(ConnectionTestResponse::success(
                "OAuth credentials validated. Complete OAuth flow to connect.",
            )))
        } else {
            Ok(Json(ConnectionTestResponse::failure(
                "OAuth credentials incomplete: both client_id and client_secret are required",
            )))
        }
    }
}

/// Test Postman connection.
///
/// Validates the Postman API key and returns workspace info.
#[utoipa::path(
    post,
    path = "/api/v1/setup/integrations/postman/test",
    request_body = PostmanTestRequest,
    responses(
        (status = 200, description = "Connection test result", body = ConnectionTestResponse)
    ),
    tag = "Setup"
)]
pub async fn test_postman(
    State(state): State<AppState>,
    Json(req): Json<PostmanTestRequest>,
) -> Result<Json<ConnectionTestResponse>, ApiError> {
    // Validate API key format (Postman API keys are typically 64 chars)
    if req.api_key.trim().is_empty() {
        return Ok(Json(ConnectionTestResponse::failure("API key is required")));
    }

    if req.api_key.len() < MIN_POSTMAN_API_KEY_LENGTH {
        return Ok(Json(ConnectionTestResponse::failure(format!(
            "API key appears to be invalid (minimum length: {} characters)",
            MIN_POSTMAN_API_KEY_LENGTH
        ))));
    }

    info!(
        workspace_id = ?req.workspace_id,
        "Testing Postman connection"
    );

    let check = PostmanHealthCheck::new(req.api_key.clone());
    let health = check.check().await;

    let success = matches!(health.status, HealthStatus::Online | HealthStatus::Degraded);
    if !success {
        return Ok(Json(ConnectionTestResponse::failure(
            health
                .error_message
                .unwrap_or_else(|| "Postman validation failed".to_string()),
        )));
    }

    // Store successful test in setup state
    {
        let mut setup = state.setup_store.lock().await;
        setup.postman = Some(req);
    }

    Ok(Json(ConnectionTestResponse::success(format!(
        "Connected to Postman successfully (response time: {}ms)",
        health.response_time_ms.unwrap_or(0)
    ))))
}

/// Test Testmo connection.
///
/// Validates the Testmo credentials and returns project info.
#[utoipa::path(
    post,
    path = "/api/v1/setup/integrations/testmo/test",
    request_body = TestmoTestRequest,
    responses(
        (status = 200, description = "Connection test result", body = ConnectionTestResponse)
    ),
    tag = "Setup"
)]
pub async fn test_testmo(
    State(state): State<AppState>,
    Json(req): Json<TestmoTestRequest>,
) -> Result<Json<ConnectionTestResponse>, ApiError> {
    // Validate URL format (consistent with Jira validation)
    let url_lower = req.instance_url.to_lowercase();
    if !url_lower.starts_with("https://") {
        return Ok(Json(ConnectionTestResponse::failure(
            "Testmo URL must use HTTPS",
        )));
    }

    // Basic URL structure validation
    if url_lower.len() < 12 || !url_lower.contains('.') {
        return Ok(Json(ConnectionTestResponse::failure(
            "Invalid Testmo URL format",
        )));
    }

    // Extract host for validation
    let host_part = url_lower
        .strip_prefix("https://")
        .and_then(|s| s.split('/').next())
        .unwrap_or("");
    
    if host_part.is_empty() || !host_part.contains('.') {
        return Ok(Json(ConnectionTestResponse::failure(
            "Testmo URL must have a valid host",
        )));
    }

    if req.api_key.trim().is_empty() {
        return Ok(Json(ConnectionTestResponse::failure("API key is required")));
    }

    info!(url = %req.instance_url, "Testing Testmo connection");

    let check = TestmoHealthCheck::new(req.instance_url.clone(), req.api_key.clone());
    let health = check.check().await;

    let success = matches!(health.status, HealthStatus::Online | HealthStatus::Degraded);
    if !success {
        return Ok(Json(ConnectionTestResponse::failure(
            health
                .error_message
                .unwrap_or_else(|| "Testmo validation failed".to_string()),
        )));
    }

    // Store successful test in setup state
    {
        let mut setup = state.setup_store.lock().await;
        setup.testmo = Some(req);
    }

    Ok(Json(ConnectionTestResponse::success(format!(
        "Connected to Testmo successfully (response time: {}ms)",
        health.response_time_ms.unwrap_or(0)
    ))))
}

// ============================================================================
// Helper Functions for Setup Completion
// ============================================================================

/// Consolidate setup state from request and store (atomic snapshot to prevent race conditions).
///
/// Updates the setup store with any new configuration from the request,
/// then returns an atomic snapshot of the complete state.
async fn consolidate_setup_state(
    setup_store: &SetupStore,
    req: &CompleteSetupRequest,
) -> SetupState {
    // Single atomic operation: update and snapshot in one lock
    let mut setup = setup_store.lock().await;
    
    // Update with profile if provided
    if let Some(profile) = &req.profile {
        setup.profile = Some(profile.clone());
    }
    
    // Update with integrations if provided
    if let Some(jira) = &req.jira {
        info!(url = %jira.instance_url, "Saving Jira configuration");
        setup.jira = Some(jira.clone());
    }
    if let Some(postman) = &req.postman {
        info!("Saving Postman configuration");
        setup.postman = Some(postman.clone());
    }
    if let Some(testmo) = &req.testmo {
        info!(url = %testmo.instance_url, "Saving Testmo configuration");
        setup.testmo = Some(testmo.clone());
    }
    if let Some(splunk) = &req.splunk {
        info!(
            base_url = %splunk.base_url,
            default_index = ?splunk.default_index,
            "Saving Splunk configuration"
        );
        setup.splunk = Some(splunk.clone());
    }
    
    // Return snapshot (clone while holding lock ensures atomicity)
    setup.clone()
}

/// Validate basic setup requirements.
fn validate_setup_requirements(setup: &SetupState) -> Vec<SetupValidationError> {
    let mut errors = Vec::new();
    
    if setup.profile.is_none() {
        errors.push(SetupValidationError::new(
            "profile",
            "Profile must be configured before completing setup",
            "profile",
            "/setup/profile",
        ));
    }
    
    if setup.jira.is_none() {
        errors.push(SetupValidationError::new(
            "jira",
            "Jira integration is required",
            "jira",
            "/setup/jira",
        ));
    }
    
    errors
}

/// Build SetupWizardInput from consolidated setup state.
fn build_wizard_input(
    setup: &SetupState,
) -> Result<qa_pms_config::SetupWizardInput, ApiError> {
    use qa_pms_config::{
        JiraAuthInput, JiraInput, PostmanInput, ProfileInput, SplunkInput, TestmoInput,
    };
    use secrecy::SecretString;
    
    let profile = setup
        .profile
        .as_ref()
        .ok_or_else(|| ApiError::Validation("Profile is required".into()))?;
    
    let jira = setup
        .jira
        .as_ref()
        .ok_or_else(|| ApiError::Validation("Jira configuration is required".into()))?;
    
    // Determine Jira auth type and validate OAuth if needed
    let jira_auth = if let (Some(email), Some(api_token)) = (&jira.email, &jira.api_token) {
        JiraAuthInput::ApiToken {
            email: email.clone(),
            api_token: SecretString::from(api_token.clone()),
        }
    } else if let (Some(client_id), Some(client_secret)) = (&jira.client_id, &jira.client_secret) {
        // Validate OAuth credentials format
        if client_id.trim().is_empty() || client_secret.trim().is_empty() {
            return Err(ApiError::Validation(
                "OAuth client_id and client_secret cannot be empty".into(),
            ));
        }
        if client_id.len() < MIN_OAUTH_CREDENTIAL_LENGTH
            || client_secret.len() < MIN_OAUTH_CREDENTIAL_LENGTH
        {
            return Err(ApiError::Validation(format!(
                "OAuth credentials appear to be invalid (minimum length: {} characters)",
                MIN_OAUTH_CREDENTIAL_LENGTH
            )));
        }
        
        // Block OAuth completion until access token is obtained
        return Err(ApiError::Validation(
            "OAuth authentication requires access token exchange. Use API Token authentication or complete OAuth flow first.".into(),
        ));
    } else {
        return Err(ApiError::Validation(
            "Jira requires either API Token (email + token) or OAuth (client_id + secret)".into(),
        ));
    };
    
    Ok(qa_pms_config::SetupWizardInput {
        profile: ProfileInput {
            display_name: profile.display_name.clone(),
            jira_email: profile.jira_email.clone(),
            ticket_states: profile.ticket_states.clone(),
        },
        jira: JiraInput {
            instance_url: jira.instance_url.clone(),
            auth: jira_auth,
        },
        postman: setup.postman.as_ref().map(|p| PostmanInput {
            api_key: SecretString::from(p.api_key.clone()),
            workspace_id: p.workspace_id.clone(),
        }),
        testmo: setup.testmo.as_ref().map(|t| TestmoInput {
            instance_url: t.instance_url.clone(),
            api_key: SecretString::from(t.api_key.clone()),
        }),
        splunk: setup.splunk.as_ref().map(|s| SplunkInput {
            base_url: s.base_url.clone(),
            default_index: s.default_index.clone(),
        }),
    })
}

/// Validate and generate UserConfig from wizard input.
fn validate_and_generate_config(
    wizard_input: qa_pms_config::SetupWizardInput,
    encryptor: &qa_pms_config::Encryptor,
) -> Result<(qa_pms_config::UserConfig, Vec<SetupValidationError>), ApiError> {
    use qa_pms_config::UserConfig;
    
    let user_config = UserConfig::from_wizard_input(wizard_input, encryptor)
        .map_err(ApiError::Internal)?;
    
    let mut validation_errors: Vec<qa_pms_config::ValidationError> = user_config.validate().errors;
    validation_errors.extend(user_config.validate_decryption(encryptor).errors);
    
    let setup_errors: Vec<SetupValidationError> = validation_errors
        .into_iter()
        .map(|e| SetupValidationError::new(e.field, e.message, e.step, e.fix_path))
        .collect();
    
    Ok((user_config, setup_errors))
}

/// Build startup validator for configured integrations.
async fn build_startup_validator(
    setup: &SetupState,
) -> StartupValidator {
    let mut validator = StartupValidator::new();
    
    // Jira is critical: validate only if API token auth is configured
    if let Some(jira) = setup.jira.as_ref() {
        if let (Some(email), Some(api_token)) = (&jira.email, &jira.api_token) {
            let check = JiraHealthCheck::with_api_token(
                jira.instance_url.clone(),
                email.clone(),
                api_token.clone(),
            );
            validator = validator.add_critical(Arc::new(check));
        }
        // OAuth not validated here (requires access token exchange)
    }
    
    // Postman + Testmo are optional
    if let Some(p) = setup.postman.as_ref() {
        validator = validator.add_optional(Arc::new(PostmanHealthCheck::new(p.api_key.clone())));
    }
    if let Some(t) = setup.testmo.as_ref() {
        validator = validator.add_optional(Arc::new(TestmoHealthCheck::new(
            t.instance_url.clone(),
            t.api_key.clone(),
        )));
    }
    
    validator
}

/// Complete setup wizard.
///
/// Validates all configuration and persists to YAML config file.
#[utoipa::path(
    post,
    path = "/api/v1/setup/complete",
    request_body = CompleteSetupRequest,
    responses(
        (status = 200, description = "Setup completion result", body = CompleteSetupResponse),
        (status = 400, description = "Validation failed", body = qa_pms_core::error::ErrorResponse)
    ),
    tag = "Setup"
)]
pub async fn complete_setup(
    State(state): State<AppState>,
    Json(req): Json<CompleteSetupRequest>,
) -> Result<Json<CompleteSetupResponse>, ApiError> {
    use qa_pms_config::UserConfig;
    use secrecy::ExposeSecret;
    
    // Atomic snapshot: consolidate request + store in single operation
    let setup = consolidate_setup_state(&state.setup_store, &req).await;
    
    // Validate basic requirements
    let mut errors = validate_setup_requirements(&setup);
    if !errors.is_empty() {
        return Ok(Json(CompleteSetupResponse {
            success: false,
            errors,
            configured_integrations: setup.configured_integrations(),
            config_path: None,
            startup_validation: None,
        }));
    }
    
    // Build wizard input (includes OAuth validation/blocking)
    let wizard_input = match build_wizard_input(&setup) {
        Ok(input) => input,
        Err(e) => {
            return Ok(Json(CompleteSetupResponse {
                success: false,
                errors: vec![SetupValidationError::new(
                    "configuration",
                    e.to_string(),
                    "complete",
                    "/setup",
                )],
                configured_integrations: setup.configured_integrations(),
                config_path: None,
                startup_validation: None,
            }));
        }
    };
    
    // Create encryptor and validate/generate config
    let encryptor = qa_pms_config::Encryptor::from_hex_key(
        state.settings.encryption_key.expose_secret(),
    )
    .map_err(ApiError::Internal)?;
    
    let (user_config, config_errors) = match validate_and_generate_config(wizard_input, &encryptor) {
        Ok((config, errs)) => (config, errs),
        Err(e) => {
            return Err(ApiError::Internal(anyhow::anyhow!(
                "Failed to generate config: {}",
                e
            )));
        }
    };
    
    errors.extend(config_errors);
    if !errors.is_empty() {
        return Ok(Json(CompleteSetupResponse {
            success: false,
            errors,
            configured_integrations: setup.configured_integrations(),
            config_path: None,
            startup_validation: None,
        }));
    }
    
    // Run startup validation
    let validator = build_startup_validator(&setup).await;
    let startup_validation = if validator.check_count() > 0 {
        Some(validator.validate().await)
    } else {
        None
    };
    
    // Check for critical validation failures
    if let Some(ref report) = startup_validation {
        if report.has_critical_failure {
            for r in &report.results {
                if r.is_critical && !r.success {
                    errors.push(SetupValidationError::new(
                        format!("{}.credentials", r.integration),
                        r.error_message
                            .clone()
                            .unwrap_or_else(|| "Integration validation failed".to_string()),
                        r.integration.clone(),
                        format!("/setup/{}", r.integration),
                    ));
                }
            }
            
            return Ok(Json(CompleteSetupResponse {
                success: false,
                errors,
                configured_integrations: setup.configured_integrations(),
                config_path: None,
                startup_validation,
            }));
        }
    }
    
    // Write config to file
    let config_path = UserConfig::default_path().map_err(ApiError::Internal)?;
    user_config
        .write_to_file(&config_path)
        .map_err(ApiError::Internal)?;
    
    info!(
        path = %config_path.display(),
        integrations = ?setup.configured_integrations(),
        "Setup completed - config saved"
    );
    
    Ok(Json(CompleteSetupResponse {
        success: true,
        errors: vec![],
        configured_integrations: setup.configured_integrations(),
        config_path: Some(config_path.display().to_string()),
        startup_validation,
    }))
}

/// Get setup wizard status.
///
/// Returns whether setup is complete and which integrations are configured.
#[utoipa::path(
    get,
    path = "/api/v1/setup/status",
    responses(
        (status = 200, description = "Setup status", body = SetupStatusResponse)
    ),
    tag = "Setup"
)]
pub async fn get_status(State(state): State<AppState>) -> Json<SetupStatusResponse> {
    let setup = state.setup_store.lock().await;

    Json(SetupStatusResponse {
        complete: setup.is_complete(),
        configured_integrations: setup.configured_integrations(),
        profile_configured: setup.is_profile_configured(),
        server_address: state.settings.server_addr(),
    })
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_connection_response_success() {
        let response = ConnectionTestResponse::success("Connected")
            .with_projects(5)
            .with_workspaces(3);

        assert!(response.success);
        assert_eq!(response.message, Some("Connected".to_string()));
        assert_eq!(response.project_count, Some(5));
        assert_eq!(response.workspace_count, Some(3));
    }

    #[test]
    fn test_connection_response_failure() {
        let response = ConnectionTestResponse::failure("Connection failed");

        assert!(!response.success);
        assert_eq!(response.message, Some("Connection failed".to_string()));
        assert!(response.project_count.is_none());
        assert!(response.workspace_count.is_none());
    }

    #[test]
    fn test_setup_state_configured_integrations() {
        let mut state = SetupState::default();
        assert!(state.configured_integrations().is_empty());

        state.jira = Some(JiraTestRequest {
            instance_url: "https://test.atlassian.net".to_string(),
            email: Some("test@example.com".to_string()),
            api_token: Some("test-token".to_string()),
            client_id: None,
            client_secret: None,
            cloud_id: None,
            access_token: None,
        });

        let integrations = state.configured_integrations();
        assert_eq!(integrations.len(), 1);
        assert!(integrations.contains(&"jira".to_string()));
    }

    #[test]
    fn test_setup_state_is_complete() {
        let mut state = SetupState::default();
        assert!(!state.is_complete());

        // Profile alone is not complete
        state.profile = Some(ProfileRequest {
            display_name: "Test User".to_string(),
            jira_email: "test@example.com".to_string(),
            ticket_states: vec!["Ready for QA".to_string()],
        });
        assert!(!state.is_complete());

        // Profile + Jira is complete
        state.jira = Some(JiraTestRequest {
            instance_url: "https://test.atlassian.net".to_string(),
            email: Some("test@example.com".to_string()),
            api_token: Some("test-token".to_string()),
            client_id: None,
            client_secret: None,
            cloud_id: None,
            access_token: None,
        });
        assert!(state.is_complete());
    }

    #[test]
    fn test_validate_setup_requirements_empty() {
        let state = SetupState::default();
        let errors = validate_setup_requirements(&state);
        assert_eq!(errors.len(), 2);
        assert!(errors.iter().any(|e| e.field == "profile"));
        assert!(errors.iter().any(|e| e.field == "jira"));
    }

    #[test]
    fn test_validate_setup_requirements_with_profile() {
        let mut state = SetupState::default();
        state.profile = Some(ProfileRequest {
            display_name: "Test User".to_string(),
            jira_email: "test@example.com".to_string(),
            ticket_states: vec!["Ready for QA".to_string()],
        });
        let errors = validate_setup_requirements(&state);
        assert_eq!(errors.len(), 1);
        assert!(errors.iter().any(|e| e.field == "jira"));
    }

    #[test]
    fn test_validate_setup_requirements_complete() {
        let mut state = SetupState::default();
        state.profile = Some(ProfileRequest {
            display_name: "Test User".to_string(),
            jira_email: "test@example.com".to_string(),
            ticket_states: vec!["Ready for QA".to_string()],
        });
        state.jira = Some(JiraTestRequest {
            instance_url: "https://test.atlassian.net".to_string(),
            email: Some("test@example.com".to_string()),
            api_token: Some("test-token".to_string()),
            client_id: None,
            client_secret: None,
            cloud_id: None,
            access_token: None,
        });
        let errors = validate_setup_requirements(&state);
        assert!(errors.is_empty());
    }

    #[tokio::test]
    async fn test_consolidate_setup_state_atomic() {
        let store = create_setup_store();
        let req = CompleteSetupRequest {
            profile: Some(ProfileRequest {
                display_name: "Test User".to_string(),
                jira_email: "test@example.com".to_string(),
                ticket_states: vec!["Ready for QA".to_string()],
            }),
            jira: Some(JiraTestRequest {
                instance_url: "https://test.atlassian.net".to_string(),
                email: Some("test@example.com".to_string()),
                api_token: Some("test-token".to_string()),
                client_id: None,
                client_secret: None,
                cloud_id: None,
                access_token: None,
            }),
            postman: None,
            testmo: None,
            splunk: None,
        };

        let snapshot = consolidate_setup_state(&store, &req).await;
        assert!(snapshot.profile.is_some());
        assert!(snapshot.jira.is_some());
        assert_eq!(snapshot.profile.as_ref().unwrap().display_name, "Test User");
    }

    #[test]
    fn test_build_wizard_input_with_api_token() {
        let mut setup = SetupState::default();
        setup.profile = Some(ProfileRequest {
            display_name: "Test User".to_string(),
            jira_email: "test@example.com".to_string(),
            ticket_states: vec!["Ready for QA".to_string()],
        });
        setup.jira = Some(JiraTestRequest {
            instance_url: "https://test.atlassian.net".to_string(),
            email: Some("test@example.com".to_string()),
            api_token: Some("test-token".to_string()),
            client_id: None,
            client_secret: None,
            cloud_id: None,
            access_token: None,
        });

        let result = build_wizard_input(&setup);
        assert!(result.is_ok());
        let input = result.unwrap();
        assert_eq!(input.profile.display_name, "Test User");
        assert_eq!(input.jira.instance_url, "https://test.atlassian.net");
    }

    #[test]
    fn test_build_wizard_input_blocks_oauth() {
        let mut setup = SetupState::default();
        setup.profile = Some(ProfileRequest {
            display_name: "Test User".to_string(),
            jira_email: "test@example.com".to_string(),
            ticket_states: vec!["Ready for QA".to_string()],
        });
        setup.jira = Some(JiraTestRequest {
            instance_url: "https://test.atlassian.net".to_string(),
            email: None,
            api_token: None,
            client_id: Some("test-client-id-12345".to_string()),
            client_secret: Some("test-client-secret-12345".to_string()),
            cloud_id: None,
            access_token: None,
        });

        let result = build_wizard_input(&setup);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.to_string().contains("OAuth authentication requires access token exchange"));
    }

    #[test]
    fn test_build_wizard_input_rejects_invalid_oauth() {
        let mut setup = SetupState::default();
        setup.profile = Some(ProfileRequest {
            display_name: "Test User".to_string(),
            jira_email: "test@example.com".to_string(),
            ticket_states: vec!["Ready for QA".to_string()],
        });
        setup.jira = Some(JiraTestRequest {
            instance_url: "https://test.atlassian.net".to_string(),
            email: None,
            api_token: None,
            client_id: Some("short".to_string()), // Too short
            client_secret: Some("secret".to_string()), // Too short
            cloud_id: None,
            access_token: None,
        });

        let result = build_wizard_input(&setup);
        assert!(result.is_err());
        let err = result.unwrap_err();
        // OAuth is blocked entirely (requires access token), but validation still checks format
        // The error should mention OAuth or access token
        assert!(
            err.to_string().contains("OAuth") || err.to_string().contains("access token"),
            "Expected OAuth-related error, got: {}",
            err
        );
    }

    #[test]
    fn test_build_wizard_input_missing_profile() {
        let mut setup = SetupState::default();
        setup.jira = Some(JiraTestRequest {
            instance_url: "https://test.atlassian.net".to_string(),
            email: Some("test@example.com".to_string()),
            api_token: Some("test-token".to_string()),
            client_id: None,
            client_secret: None,
            cloud_id: None,
            access_token: None,
        });

        let result = build_wizard_input(&setup);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Profile is required"));
    }

    #[test]
    fn test_build_wizard_input_missing_jira() {
        let mut setup = SetupState::default();
        setup.profile = Some(ProfileRequest {
            display_name: "Test User".to_string(),
            jira_email: "test@example.com".to_string(),
            ticket_states: vec!["Ready for QA".to_string()],
        });

        let result = build_wizard_input(&setup);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Jira configuration is required"));
    }
}
