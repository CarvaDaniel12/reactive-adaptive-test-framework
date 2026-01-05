//! AI companion API endpoints.
//!
//! Epic 13: AI Companion (BYOK)
//!
//! Security features:
//! - API keys encrypted with AES-256-GCM before storage
//! - Input validation for API keys
//!
//! TODO: Add rate limiting when tower_governor/axum version compatibility is resolved

use axum::{
    extract::State,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use tracing::{info, warn};
use utoipa::ToSchema;
use uuid::Uuid;

use qa_pms_ai::{
    AIClient, ChatContext, ChatInput, ChatMessage, ChatService,
    ConnectionTestResult, GherkinAnalyzer, GherkinInput,
    ProviderModels, ProviderType, SemanticSearchInput, SemanticSearchService,
};
use qa_pms_config::Encryptor;
use qa_pms_core::ApiError;
use secrecy::ExposeSecret;

use crate::app::AppState;

type ApiResult<T> = Result<T, ApiError>;

/// Minimum API key length for validation
const MIN_API_KEY_LENGTH: usize = 20;

/// Create the AI router.
///
/// TODO: Add rate limiting when tower_governor/axum version compatibility is resolved
pub fn router() -> Router<AppState> {
    Router::new()
        // Configuration
        .route("/status", get(get_ai_status))
        .route("/providers", get(get_providers))
        .route("/configure", post(configure_ai))
        .route("/test", post(test_connection))
        .route("/disable", post(disable_ai))
        // Chat
        .route("/chat", post(chat))
        .route("/chat/suggestions", post(get_chat_suggestions))
        // Semantic search
        .route("/semantic-search", post(semantic_search))
        // Gherkin analysis
        .route("/gherkin", post(analyze_gherkin))
}

// ==================== Request/Response Types ====================

/// Request to configure AI.
#[derive(Debug, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ConfigureAIRequest {
    /// Provider type
    pub provider: String,
    /// API key
    pub api_key: String,
    /// Model ID
    pub model_id: String,
    /// Custom base URL (for custom provider)
    pub custom_base_url: Option<String>,
}

/// Response for AI status.
#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct AIStatusResponse {
    /// Whether AI is available
    pub available: bool,
    /// Current provider
    pub provider: Option<String>,
    /// Current model
    pub model: Option<String>,
    /// Status message
    pub message: String,
}

/// Response for providers list.
#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ProvidersResponse {
    /// Available providers with models
    pub providers: Vec<ProviderModels>,
}

/// Request for chat.
#[derive(Debug, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ChatRequest {
    /// User message
    pub message: String,
    /// Chat history
    #[serde(default)]
    pub history: Vec<ChatMessageDto>,
    /// Current context
    pub context: Option<ChatContextDto>,
}

/// Chat message DTO.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ChatMessageDto {
    /// Message ID
    pub id: String,
    /// Role
    pub role: String,
    /// Content
    pub content: String,
    /// Timestamp
    pub timestamp: String,
}

/// Chat context DTO.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ChatContextDto {
    /// Current page
    pub current_page: String,
    /// Current ticket
    pub current_ticket: Option<TicketContextDto>,
    /// Workflow step
    pub workflow_step: Option<WorkflowStepContextDto>,
    /// Recent actions
    #[serde(default)]
    pub recent_actions: Vec<String>,
}

/// Ticket context DTO.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct TicketContextDto {
    /// Ticket key
    pub key: String,
    /// Title
    pub title: String,
    /// Description
    pub description: Option<String>,
    /// Type
    pub ticket_type: String,
    /// Status
    pub status: String,
}

/// Workflow step context DTO.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct WorkflowStepContextDto {
    /// Workflow name
    pub workflow_name: String,
    /// Step name
    pub step_name: String,
    /// Step number
    pub step_number: i32,
    /// Total steps
    pub total_steps: i32,
}

/// Response for chat.
#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ChatResponseDto {
    /// Response message
    pub message: ChatMessageDto,
    /// Token usage
    pub usage: Option<TokenUsageDto>,
}

/// Token usage DTO.
#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct TokenUsageDto {
    /// Prompt tokens
    pub prompt_tokens: u32,
    /// Completion tokens
    pub completion_tokens: u32,
    /// Total tokens
    pub total_tokens: u32,
}

/// Request for chat suggestions.
#[derive(Debug, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct SuggestionsRequest {
    /// Current context
    pub context: Option<ChatContextDto>,
}

/// Response for chat suggestions.
#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct SuggestionsResponse {
    /// Suggested questions
    pub suggestions: Vec<String>,
}

/// Request for semantic search.
#[derive(Debug, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct SemanticSearchRequest {
    /// Ticket title
    pub title: String,
    /// Description
    pub description: Option<String>,
    /// Acceptance criteria
    pub acceptance_criteria: Option<String>,
}

/// Response for semantic search.
#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct SemanticSearchResponse {
    /// Generated queries
    pub queries: Vec<String>,
    /// Key concepts
    pub key_concepts: Vec<String>,
    /// Test areas
    pub test_areas: Vec<String>,
    /// Whether AI was used
    pub ai_enhanced: bool,
}

/// Request for Gherkin analysis.
#[derive(Debug, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct GherkinRequest {
    /// Acceptance criteria
    pub acceptance_criteria: String,
    /// Ticket context
    pub ticket_context: Option<TicketContextDto>,
}

/// Response for Gherkin analysis.
#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct GherkinResponse {
    /// Parsed scenarios
    pub scenarios: Vec<GherkinScenarioDto>,
    /// Edge cases
    pub edge_cases: Vec<String>,
    /// Negative tests
    pub negative_tests: Vec<String>,
    /// Whether AI was used
    pub ai_enhanced: bool,
}

/// Gherkin scenario DTO.
#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct GherkinScenarioDto {
    /// Scenario name
    pub name: String,
    /// Given steps
    pub given: Vec<String>,
    /// When steps
    pub when: Vec<String>,
    /// Then steps
    pub then: Vec<String>,
    /// Suggested test steps
    pub suggested_test_steps: Vec<String>,
}

/// Simple success response.
#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct SuccessResponse {
    /// Success message
    pub message: String,
}

// ==================== Handlers ====================

/// Get AI status.
#[utoipa::path(
    get,
    path = "/api/v1/ai/status",
    responses(
        (status = 200, description = "AI status", body = AIStatusResponse)
    ),
    tag = "AI"
)]
pub async fn get_ai_status(State(state): State<AppState>) -> ApiResult<Json<AIStatusResponse>> {
    // Check if AI is configured in database
    let config: Option<(bool, String, String)> = sqlx::query_as(
        "SELECT enabled, provider, model_id FROM ai_configs WHERE user_id IS NULL LIMIT 1",
    )
    .fetch_optional(&state.db)
    .await
    .map_err(|e| ApiError::Internal(e.into()))?;

    let status = if let Some((enabled, provider, model)) = config {
        if enabled {
            AIStatusResponse {
                available: true,
                provider: Some(provider),
                model: Some(model),
                message: "AI is configured and ready".to_string(),
            }
        } else {
            AIStatusResponse {
                available: false,
                provider: None,
                model: None,
                message: "AI is disabled. Enable it in Settings.".to_string(),
            }
        }
    } else {
        AIStatusResponse {
            available: false,
            provider: None,
            model: None,
            message: "AI not configured. Add your API key in Settings to enable AI features."
                .to_string(),
        }
    };

    Ok(Json(status))
}

/// Get available AI providers.
#[utoipa::path(
    get,
    path = "/api/v1/ai/providers",
    responses(
        (status = 200, description = "Available providers", body = ProvidersResponse)
    ),
    tag = "AI"
)]
pub async fn get_providers() -> ApiResult<Json<ProvidersResponse>> {
    let providers = qa_pms_ai::provider::get_all_provider_models();
    Ok(Json(ProvidersResponse { providers }))
}

/// Validate API key format.
fn validate_api_key(api_key: &str, provider: ProviderType) -> Result<(), ApiError> {
    // Check minimum length
    if api_key.len() < MIN_API_KEY_LENGTH {
        return Err(ApiError::Validation(format!(
            "API key too short (minimum {} characters)",
            MIN_API_KEY_LENGTH
        )));
    }

    // Check provider-specific patterns
    match provider {
        ProviderType::OpenAi => {
            if !api_key.starts_with("sk-") {
                warn!("OpenAI API key doesn't start with 'sk-' prefix");
                // Don't reject - some keys may have different formats
            }
        }
        ProviderType::Anthropic => {
            if !api_key.starts_with("sk-ant-") {
                warn!("Anthropic API key doesn't start with 'sk-ant-' prefix");
            }
        }
        _ => {
            // Other providers - just check it's not empty/whitespace
            if api_key.trim().is_empty() {
                return Err(ApiError::Validation("API key cannot be empty".into()));
            }
        }
    }

    Ok(())
}

/// Get encryption key from settings.
fn get_encryption_key(state: &AppState) -> Result<Encryptor, ApiError> {
    let key = state.settings.encryption_key.expose_secret();
    Encryptor::from_hex_key(key).map_err(|e| ApiError::Internal(e))
}

/// Configure AI provider.
#[utoipa::path(
    post,
    path = "/api/v1/ai/configure",
    request_body = ConfigureAIRequest,
    responses(
        (status = 200, description = "AI configured", body = SuccessResponse),
        (status = 400, description = "Invalid configuration")
    ),
    tag = "AI"
)]
pub async fn configure_ai(
    State(state): State<AppState>,
    Json(req): Json<ConfigureAIRequest>,
) -> ApiResult<Json<SuccessResponse>> {
    let provider = parse_provider(&req.provider)?;

    // CR-HIGH-003: Validate API key format
    validate_api_key(&req.api_key, provider)?;

    // Validate by testing connection
    let client = create_client(provider, &req.api_key, &req.model_id, req.custom_base_url.clone())?;
    let test_result = client.test_connection().await.map_err(|e| {
        ApiError::Validation(format!("Connection test failed: {}", e))
    })?;

    if !test_result.success {
        return Err(ApiError::Validation(format!(
            "Connection test failed: {}",
            test_result.message
        )));
    }

    // CR-HIGH-001: Encrypt API key before storage
    let encryptor = get_encryption_key(&state)?;
    let encrypted_key = encryptor.encrypt(&req.api_key).map_err(|e| {
        ApiError::Internal(anyhow::anyhow!("Failed to encrypt API key: {}", e))
    })?;

    info!(provider = %req.provider, model = %req.model_id, "Storing encrypted AI configuration");

    // Store configuration with encrypted API key
    sqlx::query(
        r#"
        INSERT INTO ai_configs (user_id, enabled, provider, model_id, api_key_encrypted, custom_base_url, validated_at)
        VALUES (NULL, TRUE, $1, $2, $3, $4, NOW())
        ON CONFLICT (user_id) DO UPDATE SET
            enabled = TRUE,
            provider = $1,
            model_id = $2,
            api_key_encrypted = $3,
            custom_base_url = $4,
            validated_at = NOW(),
            updated_at = NOW()
        "#,
    )
    .bind(&req.provider)
    .bind(&req.model_id)
    .bind(&encrypted_key)
    .bind(&req.custom_base_url)
    .execute(&state.db)
    .await
    .map_err(|e| ApiError::Internal(e.into()))?;

    Ok(Json(SuccessResponse {
        message: "AI configured successfully".to_string(),
    }))
}

/// Test AI connection.
#[utoipa::path(
    post,
    path = "/api/v1/ai/test",
    request_body = ConfigureAIRequest,
    responses(
        (status = 200, description = "Connection test result", body = ConnectionTestResult)
    ),
    tag = "AI"
)]
pub async fn test_connection(
    Json(req): Json<ConfigureAIRequest>,
) -> ApiResult<Json<ConnectionTestResult>> {
    let provider = parse_provider(&req.provider)?;
    let client = create_client(provider, &req.api_key, &req.model_id, req.custom_base_url)?;

    let result = client.test_connection().await.map_err(|e| {
        ApiError::Validation(format!("Connection test failed: {}", e))
    })?;

    Ok(Json(result))
}

/// Disable AI.
#[utoipa::path(
    post,
    path = "/api/v1/ai/disable",
    responses(
        (status = 200, description = "AI disabled", body = SuccessResponse)
    ),
    tag = "AI"
)]
pub async fn disable_ai(State(state): State<AppState>) -> ApiResult<Json<SuccessResponse>> {
    sqlx::query("UPDATE ai_configs SET enabled = FALSE, updated_at = NOW() WHERE user_id IS NULL")
        .execute(&state.db)
        .await
        .map_err(|e| ApiError::Internal(e.into()))?;

    Ok(Json(SuccessResponse {
        message: "AI disabled".to_string(),
    }))
}

/// Get decrypted API key from database.
async fn get_decrypted_api_key(state: &AppState) -> Result<(String, String, String, Option<String>), ApiError> {
    // Get AI configuration including encrypted key
    let config: Option<(String, String, Option<String>, Option<String>)> = sqlx::query_as(
        "SELECT provider, model_id, api_key_encrypted, custom_base_url FROM ai_configs WHERE user_id IS NULL AND enabled = TRUE LIMIT 1",
    )
    .fetch_optional(&state.db)
    .await
    .map_err(|e| ApiError::Internal(e.into()))?;

    let (provider_str, model_id, encrypted_key, custom_url) = config.ok_or_else(|| {
        ApiError::ServiceUnavailable("AI not configured. Please configure AI in Settings.".into())
    })?;

    // Decrypt API key
    let api_key = if let Some(encrypted) = encrypted_key {
        let encryptor = get_encryption_key(state)?;
        encryptor
            .decrypt(&encrypted)
            .map_err(|e| ApiError::Internal(anyhow::anyhow!("Failed to decrypt API key: {}", e)))?
            .expose_secret()
            .to_string()
    } else {
        // Fallback to env var for backwards compatibility
        std::env::var("AI_API_KEY").unwrap_or_default()
    };

    if api_key.is_empty() {
        return Err(ApiError::ServiceUnavailable(
            "AI API key not configured".into(),
        ));
    }

    Ok((provider_str, model_id, api_key, custom_url))
}

/// Chat with AI.
#[utoipa::path(
    post,
    path = "/api/v1/ai/chat",
    request_body = ChatRequest,
    responses(
        (status = 200, description = "Chat response", body = ChatResponseDto),
        (status = 503, description = "AI not available")
    ),
    tag = "AI"
)]
pub async fn chat(
    State(state): State<AppState>,
    Json(req): Json<ChatRequest>,
) -> ApiResult<Json<ChatResponseDto>> {
    // Get decrypted AI configuration
    let (provider_str, model_id, api_key, custom_url) = get_decrypted_api_key(&state).await?;

    let provider = parse_provider(&provider_str)?;

    let custom_base_url = custom_url.filter(|s| !s.is_empty());

    let client = create_client(provider, &api_key, &model_id, custom_base_url)?;
    let chat_service = ChatService::new(client);

    // Convert DTOs to domain types
    let history: Vec<ChatMessage> = req
        .history
        .into_iter()
        .map(|m| ChatMessage {
            id: Uuid::parse_str(&m.id).unwrap_or_else(|_| Uuid::new_v4()),
            role: match m.role.as_str() {
                "system" => qa_pms_ai::MessageRole::System,
                "assistant" => qa_pms_ai::MessageRole::Assistant,
                _ => qa_pms_ai::MessageRole::User,
            },
            content: m.content,
            timestamp: chrono::DateTime::parse_from_rfc3339(&m.timestamp)
                .map(|dt| dt.with_timezone(&chrono::Utc))
                .unwrap_or_else(|_| chrono::Utc::now()),
        })
        .collect();

    let context = req.context.map(|c| ChatContext {
        current_page: c.current_page,
        current_ticket: c.current_ticket.map(|t| qa_pms_ai::TicketContext {
            key: t.key,
            title: t.title,
            description: t.description,
            ticket_type: t.ticket_type,
            status: t.status,
        }),
        workflow_step: c.workflow_step.map(|w| qa_pms_ai::WorkflowStepContext {
            workflow_name: w.workflow_name,
            step_name: w.step_name,
            step_number: w.step_number,
            total_steps: w.total_steps,
        }),
        recent_actions: c.recent_actions,
    });

    let input = ChatInput {
        message: req.message,
        history,
        context,
        stream: false,
    };

    let response = chat_service.chat(input).await.map_err(|e| {
        ApiError::Internal(anyhow::anyhow!("Chat failed: {}", e))
    })?;

    Ok(Json(ChatResponseDto {
        message: ChatMessageDto {
            id: response.message.id.to_string(),
            role: format!("{:?}", response.message.role).to_lowercase(),
            content: response.message.content,
            timestamp: response.message.timestamp.to_rfc3339(),
        },
        usage: response.usage.map(|u| TokenUsageDto {
            prompt_tokens: u.prompt_tokens,
            completion_tokens: u.completion_tokens,
            total_tokens: u.total_tokens,
        }),
    }))
}

/// Get chat suggestions based on context.
#[utoipa::path(
    post,
    path = "/api/v1/ai/chat/suggestions",
    request_body = SuggestionsRequest,
    responses(
        (status = 200, description = "Suggested questions", body = SuggestionsResponse)
    ),
    tag = "AI"
)]
pub async fn get_chat_suggestions(
    Json(req): Json<SuggestionsRequest>,
) -> ApiResult<Json<SuggestionsResponse>> {
    let context = req.context.map(|c| ChatContext {
        current_page: c.current_page,
        current_ticket: c.current_ticket.map(|t| qa_pms_ai::TicketContext {
            key: t.key,
            title: t.title,
            description: t.description,
            ticket_type: t.ticket_type,
            status: t.status,
        }),
        workflow_step: c.workflow_step.map(|w| qa_pms_ai::WorkflowStepContext {
            workflow_name: w.workflow_name,
            step_name: w.step_name,
            step_number: w.step_number,
            total_steps: w.total_steps,
        }),
        recent_actions: c.recent_actions,
    });

    let suggestions = ChatService::get_suggested_questions(&context);

    Ok(Json(SuggestionsResponse { suggestions }))
}

/// Perform semantic search analysis.
#[utoipa::path(
    post,
    path = "/api/v1/ai/semantic-search",
    request_body = SemanticSearchRequest,
    responses(
        (status = 200, description = "Semantic search result", body = SemanticSearchResponse)
    ),
    tag = "AI"
)]
pub async fn semantic_search(
    State(state): State<AppState>,
    Json(req): Json<SemanticSearchRequest>,
) -> ApiResult<Json<SemanticSearchResponse>> {
    let input = SemanticSearchInput {
        title: req.title,
        description: req.description,
        acceptance_criteria: req.acceptance_criteria,
    };

    // Try to use AI if configured (with encrypted key)
    if let Ok((provider_str, model_id, api_key, custom_url)) = get_decrypted_api_key(&state).await {
        if let Ok(provider) = parse_provider(&provider_str) {
            let custom_base_url = custom_url.filter(|s| !s.is_empty());

            if let Ok(client) = create_client(provider, &api_key, &model_id, custom_base_url) {
                let service = SemanticSearchService::new(client);
                if let Ok(result) = service.analyze(input.clone()).await {
                    return Ok(Json(SemanticSearchResponse {
                        queries: result.queries,
                        key_concepts: result.key_concepts,
                        test_areas: result.test_areas,
                        ai_enhanced: true,
                    }));
                }
            }
        }
    }

    // Fallback to keyword-based search
    let result = SemanticSearchService::fallback_search(&input);
    Ok(Json(SemanticSearchResponse {
        queries: result.queries,
        key_concepts: result.key_concepts,
        test_areas: result.test_areas,
        ai_enhanced: false,
    }))
}

/// Analyze Gherkin acceptance criteria.
#[utoipa::path(
    post,
    path = "/api/v1/ai/gherkin",
    request_body = GherkinRequest,
    responses(
        (status = 200, description = "Gherkin analysis result", body = GherkinResponse)
    ),
    tag = "AI"
)]
pub async fn analyze_gherkin(
    State(state): State<AppState>,
    Json(req): Json<GherkinRequest>,
) -> ApiResult<Json<GherkinResponse>> {
    let input = GherkinInput {
        acceptance_criteria: req.acceptance_criteria,
        ticket_context: req.ticket_context.map(|t| qa_pms_ai::TicketContext {
            key: t.key,
            title: t.title,
            description: t.description,
            ticket_type: t.ticket_type,
            status: t.status,
        }),
    };

    // Try to use AI if configured (with encrypted key)
    if let Ok((provider_str, model_id, api_key, custom_url)) = get_decrypted_api_key(&state).await {
        if let Ok(provider) = parse_provider(&provider_str) {
            let custom_base_url = custom_url.filter(|s| !s.is_empty());

            if let Ok(client) = create_client(provider, &api_key, &model_id, custom_base_url) {
                let analyzer = GherkinAnalyzer::new(client);
                if let Ok(result) = analyzer.analyze(input.clone()).await {
                    return Ok(Json(GherkinResponse {
                        scenarios: result
                            .scenarios
                            .into_iter()
                            .map(|s| GherkinScenarioDto {
                                name: s.name,
                                given: s.given,
                                when: s.when,
                                then: s.then,
                                suggested_test_steps: s.suggested_test_steps,
                            })
                            .collect(),
                        edge_cases: result.edge_cases,
                        negative_tests: result.negative_tests,
                        ai_enhanced: true,
                    }));
                }
            }
        }
    }

    // Fallback to basic parsing
    let result = GherkinAnalyzer::fallback_analysis(&input);
    Ok(Json(GherkinResponse {
        scenarios: result
            .scenarios
            .into_iter()
            .map(|s| GherkinScenarioDto {
                name: s.name,
                given: s.given,
                when: s.when,
                then: s.then,
                suggested_test_steps: s.suggested_test_steps,
            })
            .collect(),
        edge_cases: result.edge_cases,
        negative_tests: result.negative_tests,
        ai_enhanced: false,
    }))
}

// ==================== Helper Functions ====================

fn parse_provider(s: &str) -> Result<ProviderType, ApiError> {
    match s.to_lowercase().as_str() {
        "anthropic" => Ok(ProviderType::Anthropic),
        "openai" => Ok(ProviderType::OpenAi),
        "deepseek" => Ok(ProviderType::Deepseek),
        "zai" | "z.ai" => Ok(ProviderType::Zai),
        "custom" => Ok(ProviderType::Custom),
        _ => Err(ApiError::Validation(format!("Unknown provider: {}", s))),
    }
}

fn create_client(
    provider: ProviderType,
    api_key: &str,
    model: &str,
    custom_base_url: Option<String>,
) -> Result<AIClient, ApiError> {
    let secret_key = secrecy::SecretString::new(api_key.to_string().into());
    AIClient::from_config(provider, secret_key, model.to_string(), custom_base_url)
        .map_err(|e| ApiError::Validation(format!("Failed to create AI client: {}", e)))
}
