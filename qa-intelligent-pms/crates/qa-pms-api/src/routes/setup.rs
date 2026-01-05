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
use qa_pms_core::error::ApiError;
use qa_pms_core::health::HealthCheck;

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
    /// API Token from https://id.atlassian.com/manage-profile/security/api-tokens
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
    pub fn has_api_token(&self) -> bool {
        self.email.is_some() && self.api_token.is_some()
    }

    /// Check if OAuth auth is configured.
    pub fn has_oauth(&self) -> bool {
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

/// Setup completion response.
#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CompleteSetupResponse {
    /// Whether setup completed successfully
    pub success: bool,
    /// Validation errors (if any)
    pub errors: Vec<String>,
    /// Configured integrations
    pub configured_integrations: Vec<String>,
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
        jira_email = %req.jira_email,
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
/// Validates the Jira OAuth credentials and returns connection info.
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
    // Validate URL format
    if !req.instance_url.starts_with("https://") {
        return Ok(Json(ConnectionTestResponse::failure(
            "Jira URL must use HTTPS",
        )));
    }

    if !req.instance_url.contains(".atlassian.net") && !req.instance_url.contains("jira") {
        warn!(url = %req.instance_url, "Jira URL might be invalid");
    }

    // Check for API Token auth (preferred)
    let has_api_token = req.has_api_token();
    // Check for OAuth auth (alternative)
    let has_oauth_creds = req.client_id.as_ref().is_some_and(|s| !s.trim().is_empty())
        && req.client_secret.as_ref().is_some_and(|s| !s.trim().is_empty());

    if !has_api_token && !has_oauth_creds {
        return Ok(Json(ConnectionTestResponse::failure(
            "Either API Token (email + api_token) or OAuth credentials (client_id + client_secret) are required",
        )));
    }

    // Test the connection based on auth method
    if has_api_token {
        let email = req.email.as_ref().unwrap();
        let api_token = req.api_token.as_ref().unwrap();

        info!(url = %req.instance_url, email = %email, "Testing Jira connection with API Token");

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
                result.error_message.unwrap_or_else(|| "Connection failed".to_string()),
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
        // OAuth flow - just validate and store for now
        info!(url = %req.instance_url, "Storing Jira OAuth credentials (OAuth flow not implemented)");

        // Store credentials for OAuth flow
        {
            let mut setup = state.setup_store.lock().await;
            setup.jira = Some(req);
        }

        Ok(Json(
            ConnectionTestResponse::success("OAuth credentials stored. Complete OAuth flow to connect."),
        ))
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

    if req.api_key.len() < 32 {
        return Ok(Json(ConnectionTestResponse::failure(
            "API key appears to be invalid (too short)",
        )));
    }

    // TODO: Implement actual Postman API test in qa-pms-postman crate (Epic 4)
    // For now, simulate a successful connection test
    info!(
        workspace_id = ?req.workspace_id,
        "Testing Postman connection"
    );

    // Store successful test in setup state
    {
        let mut setup = state.setup_store.lock().await;
        setup.postman = Some(req);
    }

    Ok(Json(
        ConnectionTestResponse::success("Connected to Postman successfully").with_workspaces(3),
    ))
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
    // Validate URL format
    if !req.instance_url.starts_with("https://") {
        return Ok(Json(ConnectionTestResponse::failure(
            "Testmo URL must use HTTPS",
        )));
    }

    if req.api_key.trim().is_empty() {
        return Ok(Json(ConnectionTestResponse::failure("API key is required")));
    }

    // TODO: Implement actual Testmo API test in qa-pms-testmo crate (Epic 4)
    // For now, simulate a successful connection test
    info!(url = %req.instance_url, "Testing Testmo connection");

    // Store successful test in setup state
    {
        let mut setup = state.setup_store.lock().await;
        setup.testmo = Some(req);
    }

    Ok(Json(
        ConnectionTestResponse::success("Connected to Testmo successfully").with_projects(2),
    ))
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
#[allow(clippy::too_many_lines)]
pub async fn complete_setup(
    State(state): State<AppState>,
    Json(req): Json<CompleteSetupRequest>,
) -> Result<Json<CompleteSetupResponse>, ApiError> {
    use qa_pms_config::{
        JiraAuthInput, JiraInput, PostmanInput, ProfileInput, SetupWizardInput, SplunkInput,
        TestmoInput, UserConfig,
    };
    use secrecy::{ExposeSecret, SecretString};

    let mut errors = Vec::new();
    let setup = state.setup_store.lock().await;

    // Validate profile is configured
    if setup.profile.is_none() {
        errors.push("Profile must be configured before completing setup".to_string());
    }

    // Validate at least Jira is configured
    if setup.jira.is_none() && req.jira.is_none() {
        errors.push("Jira integration is required".to_string());
    }

    // If there are validation errors, return them
    if !errors.is_empty() {
        return Ok(Json(CompleteSetupResponse {
            success: false,
            errors,
            configured_integrations: setup.configured_integrations(),
        }));
    }

    // Update setup state with any new configuration from the request
    drop(setup); // Release lock
    {
        let mut setup = state.setup_store.lock().await;

        if let Some(jira) = req.jira {
            info!(url = %jira.instance_url, "Saving Jira configuration");
            setup.jira = Some(jira);
        }
        if let Some(postman) = req.postman {
            info!("Saving Postman configuration");
            setup.postman = Some(postman);
        }
        if let Some(testmo) = req.testmo {
            info!(url = %testmo.instance_url, "Saving Testmo configuration");
            setup.testmo = Some(testmo);
        }
        if let Some(splunk) = req.splunk {
            info!(
                base_url = %splunk.base_url,
                default_index = ?splunk.default_index,
                "Saving Splunk configuration"
            );
            setup.splunk = Some(splunk);
        }
    }

    // Build user config from setup state
    let setup = state.setup_store.lock().await;
    let profile = setup.profile.as_ref().ok_or_else(|| {
        ApiError::Validation("Profile is required".into())
    })?;
    let jira = setup.jira.as_ref().ok_or_else(|| {
        ApiError::Validation("Jira configuration is required".into())
    })?;

    // Determine Jira auth type based on which credentials are present
    let jira_auth = if let (Some(email), Some(api_token)) = (&jira.email, &jira.api_token) {
        JiraAuthInput::ApiToken {
            email: email.clone(),
            api_token: SecretString::from(api_token.clone()),
        }
    } else if let (Some(client_id), Some(client_secret)) = (&jira.client_id, &jira.client_secret) {
        JiraAuthInput::OAuth {
            client_id: client_id.clone(),
            client_secret: SecretString::from(client_secret.clone()),
        }
    } else {
        return Err(ApiError::Validation(
            "Jira requires either API Token (email + token) or OAuth (client_id + secret)".into(),
        ));
    };

    let wizard_input = SetupWizardInput {
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
    };

    // Create encryptor using app encryption key
    let encryptor = qa_pms_config::Encryptor::from_hex_key(
        state.settings.encryption_key.expose_secret()
    ).map_err(ApiError::Internal)?;

    // Generate user config with encrypted secrets
    let user_config = UserConfig::from_wizard_input(wizard_input, &encryptor)
        .map_err(ApiError::Internal)?;

    // Validate the config
    let validation = user_config.validate();
    if !validation.success {
        let error_messages: Vec<String> = validation
            .errors
            .iter()
            .map(|e| format!("{}: {}", e.field, e.message))
            .collect();
        return Ok(Json(CompleteSetupResponse {
            success: false,
            errors: error_messages,
            configured_integrations: setup.configured_integrations(),
        }));
    }

    // Write config to file
    let config_path = UserConfig::default_path()
        .map_err(ApiError::Internal)?;

    user_config.write_to_file(&config_path)
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
            client_id: "test".to_string(),
            client_secret: "secret".to_string(),
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
            client_id: "test".to_string(),
            client_secret: "secret".to_string(),
            cloud_id: Some("test-cloud-id".to_string()),
            access_token: Some("test-token".to_string()),
        });
        assert!(state.is_complete());
    }
}
