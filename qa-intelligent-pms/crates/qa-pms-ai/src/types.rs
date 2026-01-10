//! Type definitions for the AI module.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

/// Supported AI providers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum ProviderType {
    /// Anthropic (Claude)
    Anthropic,
    /// `OpenAI` (GPT)
    OpenAi,
    /// Deepseek
    Deepseek,
    /// z.ai
    Zai,
    /// Custom OpenAI-compatible endpoint
    Custom,
}

impl std::fmt::Display for ProviderType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Anthropic => write!(f, "Anthropic"),
            Self::OpenAi => write!(f, "OpenAI"),
            Self::Deepseek => write!(f, "Deepseek"),
            Self::Zai => write!(f, "z.ai"),
            Self::Custom => write!(f, "Custom"),
        }
    }
}

/// Available models per provider.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ProviderModels {
    /// Provider type
    pub provider: ProviderType,
    /// Available models
    pub models: Vec<ModelInfo>,
}

/// Information about an AI model.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ModelInfo {
    /// Model ID (used in API calls)
    pub id: String,
    /// Display name
    pub name: String,
    /// Context window size
    pub context_window: u32,
    /// Whether model supports streaming
    pub supports_streaming: bool,
}

/// AI configuration for a user.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct AIConfig {
    /// Whether AI is enabled
    pub enabled: bool,
    /// Selected provider
    pub provider: ProviderType,
    /// Selected model ID
    pub model_id: String,
    /// Custom base URL (for Custom provider)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_base_url: Option<String>,
    /// Last validated timestamp
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validated_at: Option<DateTime<Utc>>,
}

impl Default for AIConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            provider: ProviderType::OpenAi,
            model_id: "gpt-4o-mini".to_string(),
            custom_base_url: None,
            validated_at: None,
        }
    }
}

/// Input for configuring AI.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ConfigureAIInput {
    /// Provider to use
    pub provider: ProviderType,
    /// API key (will be encrypted)
    pub api_key: String,
    /// Model to use
    pub model_id: String,
    /// Custom base URL (for Custom provider)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_base_url: Option<String>,
}

/// Result of testing AI connection.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ConnectionTestResult {
    /// Whether connection succeeded
    pub success: bool,
    /// Message (success or error)
    pub message: String,
    /// Response time in milliseconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_time_ms: Option<u64>,
    /// Model used for test
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
}

/// A chat message.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ChatMessage {
    /// Message ID
    pub id: Uuid,
    /// Role (user, assistant, system)
    pub role: MessageRole,
    /// Message content
    pub content: String,
    /// Timestamp
    pub timestamp: DateTime<Utc>,
}

/// Role of a chat message.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum MessageRole {
    /// System message (context/instructions)
    System,
    /// User message
    User,
    /// Assistant (AI) message
    Assistant,
}

/// Chat context for contextual awareness.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ChatContext {
    /// Current page/view
    pub current_page: String,
    /// Current ticket (if viewing one)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_ticket: Option<TicketContext>,
    /// Current workflow step (if in workflow)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workflow_step: Option<WorkflowStepContext>,
    /// Recent user actions
    #[serde(default)]
    pub recent_actions: Vec<String>,
}

/// Ticket context for AI.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct TicketContext {
    /// Ticket key (e.g., "PROJ-123")
    pub key: String,
    /// Ticket title
    pub title: String,
    /// Ticket description (truncated)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Ticket type
    pub ticket_type: String,
    /// Ticket status
    pub status: String,
}

/// Workflow step context for AI.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct WorkflowStepContext {
    /// Workflow template name
    pub workflow_name: String,
    /// Current step name
    pub step_name: String,
    /// Step number
    pub step_number: i32,
    /// Total steps
    pub total_steps: i32,
}

/// Input for chat completion.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ChatInput {
    /// User message
    pub message: String,
    /// Chat history (for context)
    #[serde(default)]
    pub history: Vec<ChatMessage>,
    /// Current context
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<ChatContext>,
    /// Whether to stream response
    #[serde(default)]
    pub stream: bool,
}

/// Response from chat completion.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ChatResponse {
    /// Response message
    pub message: ChatMessage,
    /// Token usage
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage: Option<TokenUsage>,
}

/// Token usage information.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct TokenUsage {
    /// Prompt tokens
    pub prompt_tokens: u32,
    /// Completion tokens
    pub completion_tokens: u32,
    /// Total tokens
    pub total_tokens: u32,
}

/// Input for semantic search.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct SemanticSearchInput {
    /// Ticket title
    pub title: String,
    /// Ticket description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Acceptance criteria
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acceptance_criteria: Option<String>,
}

/// Result of semantic search analysis.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct SemanticSearchResult {
    /// Generated search queries
    pub queries: Vec<String>,
    /// Key concepts extracted
    pub key_concepts: Vec<String>,
    /// Suggested test areas
    pub test_areas: Vec<String>,
}

/// Input for Gherkin analysis.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct GherkinInput {
    /// Acceptance criteria text
    pub acceptance_criteria: String,
    /// Ticket context for better suggestions
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ticket_context: Option<TicketContext>,
}

/// Result of Gherkin analysis.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct GherkinAnalysisResult {
    /// Parsed scenarios
    pub scenarios: Vec<GherkinScenario>,
    /// Suggested edge cases
    pub edge_cases: Vec<String>,
    /// Suggested negative test cases
    pub negative_tests: Vec<String>,
}

/// A parsed Gherkin scenario.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct GherkinScenario {
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

/// AI feature availability status.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct AIStatus {
    /// Whether AI is configured and available
    pub available: bool,
    /// Current provider (if configured)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider: Option<ProviderType>,
    /// Current model (if configured)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    /// Status message
    pub message: String,
}

impl AIStatus {
    /// Create a status indicating AI is not configured.
    #[must_use]
    pub fn not_configured() -> Self {
        Self {
            available: false,
            provider: None,
            model: None,
            message: "AI not configured. Add your API key in Settings to enable AI features."
                .to_string(),
        }
    }

    /// Create a status indicating AI is available.
    #[must_use]
    pub fn available(provider: ProviderType, model: String) -> Self {
        let message = format!("AI enabled with {} ({})", provider, &model);
        Self {
            available: true,
            provider: Some(provider),
            model: Some(model),
            message,
        }
    }
}
