//! AI provider abstraction.
//!
//! Supports multiple AI providers with a unified interface.

use std::time::{Duration, Instant};

use async_trait::async_trait;
use reqwest::Client;
use secrecy::{ExposeSecret, SecretString};
use serde::{Deserialize, Serialize};
use tracing::{debug, info, warn};

use crate::error::AIError;
use crate::types::{
    ChatMessage, ConnectionTestResult, MessageRole, ModelInfo, ProviderModels, ProviderType,
    TokenUsage,
};

/// Trait for AI providers.
#[async_trait]
pub trait AIProvider: Send + Sync {
    /// Get the provider type.
    fn provider_type(&self) -> ProviderType;

    /// Get available models for this provider.
    fn available_models(&self) -> Vec<ModelInfo>;

    /// Test the connection with the API key.
    async fn test_connection(&self) -> Result<ConnectionTestResult, AIError>;

    /// Send a chat completion request.
    async fn chat_completion(
        &self,
        messages: Vec<ChatMessage>,
        model: &str,
    ) -> Result<(ChatMessage, Option<TokenUsage>), AIError>;
}

/// AI client that wraps a provider.
pub struct AIClient {
    provider: Box<dyn AIProvider>,
    model: String,
}

impl AIClient {
    /// Create a new AI client.
    pub fn new(provider: Box<dyn AIProvider>, model: String) -> Self {
        Self { provider, model }
    }

    /// Create a client from configuration.
    pub fn from_config(
        provider_type: ProviderType,
        api_key: SecretString,
        model: String,
        custom_base_url: Option<String>,
    ) -> Result<Self, AIError> {
        let provider: Box<dyn AIProvider> = match provider_type {
            ProviderType::OpenAi => Box::new(OpenAIProvider::new(api_key)),
            ProviderType::Anthropic => Box::new(AnthropicProvider::new(api_key)),
            ProviderType::Deepseek => Box::new(DeepseekProvider::new(api_key)),
            ProviderType::Zai => Box::new(ZaiProvider::new(api_key)),
            ProviderType::Custom => {
                let base_url = custom_base_url
                    .ok_or_else(|| AIError::InvalidApiKey("Custom provider requires base URL".into()))?;
                Box::new(CustomProvider::new(api_key, base_url))
            }
        };

        Ok(Self::new(provider, model))
    }

    /// Test the connection.
    pub async fn test_connection(&self) -> Result<ConnectionTestResult, AIError> {
        self.provider.test_connection().await
    }

    /// Send a chat completion.
    pub async fn chat(
        &self,
        messages: Vec<ChatMessage>,
    ) -> Result<(ChatMessage, Option<TokenUsage>), AIError> {
        self.provider.chat_completion(messages, &self.model).await
    }

    /// Get the provider type.
    pub fn provider_type(&self) -> ProviderType {
        self.provider.provider_type()
    }

    /// Get the model.
    pub fn model(&self) -> &str {
        &self.model
    }
}

/// Get available models for all providers.
pub fn get_all_provider_models() -> Vec<ProviderModels> {
    vec![
        ProviderModels {
            provider: ProviderType::OpenAi,
            models: OpenAIProvider::default_models(),
        },
        ProviderModels {
            provider: ProviderType::Anthropic,
            models: AnthropicProvider::default_models(),
        },
        ProviderModels {
            provider: ProviderType::Deepseek,
            models: DeepseekProvider::default_models(),
        },
        ProviderModels {
            provider: ProviderType::Zai,
            models: ZaiProvider::default_models(),
        },
        ProviderModels {
            provider: ProviderType::Custom,
            models: vec![ModelInfo {
                id: "custom".to_string(),
                name: "Custom Model".to_string(),
                context_window: 8192,
                supports_streaming: true,
            }],
        },
    ]
}

// ==================== OpenAI Provider ====================

/// OpenAI API provider.
pub struct OpenAIProvider {
    client: Client,
    api_key: SecretString,
    base_url: String,
}

impl OpenAIProvider {
    /// Create a new OpenAI provider.
    pub fn new(api_key: SecretString) -> Self {
        Self {
            client: Client::new(),
            api_key,
            base_url: "https://api.openai.com/v1".to_string(),
        }
    }

    /// Get default models.
    pub fn default_models() -> Vec<ModelInfo> {
        vec![
            ModelInfo {
                id: "gpt-4o".to_string(),
                name: "GPT-4o".to_string(),
                context_window: 128000,
                supports_streaming: true,
            },
            ModelInfo {
                id: "gpt-4o-mini".to_string(),
                name: "GPT-4o Mini".to_string(),
                context_window: 128000,
                supports_streaming: true,
            },
            ModelInfo {
                id: "gpt-4-turbo".to_string(),
                name: "GPT-4 Turbo".to_string(),
                context_window: 128000,
                supports_streaming: true,
            },
            ModelInfo {
                id: "gpt-3.5-turbo".to_string(),
                name: "GPT-3.5 Turbo".to_string(),
                context_window: 16385,
                supports_streaming: true,
            },
        ]
    }
}

#[derive(Serialize)]
struct OpenAIChatRequest {
    model: String,
    messages: Vec<OpenAIMessage>,
    max_tokens: u32,
}

#[derive(Serialize, Deserialize)]
struct OpenAIMessage {
    role: String,
    content: String,
}

#[derive(Deserialize)]
struct OpenAIChatResponse {
    choices: Vec<OpenAIChoice>,
    usage: Option<OpenAIUsage>,
}

#[derive(Deserialize)]
struct OpenAIChoice {
    message: OpenAIMessage,
}

#[derive(Deserialize)]
struct OpenAIUsage {
    prompt_tokens: u32,
    completion_tokens: u32,
    total_tokens: u32,
}

#[async_trait]
impl AIProvider for OpenAIProvider {
    fn provider_type(&self) -> ProviderType {
        ProviderType::OpenAi
    }

    fn available_models(&self) -> Vec<ModelInfo> {
        Self::default_models()
    }

    async fn test_connection(&self) -> Result<ConnectionTestResult, AIError> {
        let start = Instant::now();

        let request = OpenAIChatRequest {
            model: "gpt-4o-mini".to_string(),
            messages: vec![OpenAIMessage {
                role: "user".to_string(),
                content: "Say 'OK' if you can hear me.".to_string(),
            }],
            max_tokens: 10,
        };

        let response = self
            .client
            .post(format!("{}/chat/completions", self.base_url))
            .header("Authorization", format!("Bearer {}", self.api_key.expose_secret()))
            .header("Content-Type", "application/json")
            .json(&request)
            .timeout(Duration::from_secs(30))
            .send()
            .await?;

        let elapsed = start.elapsed().as_millis() as u64;

        if response.status().is_success() {
            info!("OpenAI connection test successful");
            Ok(ConnectionTestResult {
                success: true,
                message: "Connection successful".to_string(),
                response_time_ms: Some(elapsed),
                model: Some("gpt-4o-mini".to_string()),
            })
        } else {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            warn!("OpenAI connection test failed: {} - {}", status, error_text);

            if status.as_u16() == 401 {
                return Err(AIError::InvalidApiKey("Invalid OpenAI API key".into()));
            }
            if status.as_u16() == 429 {
                return Err(AIError::RateLimited);
            }

            Ok(ConnectionTestResult {
                success: false,
                message: format!("Connection failed: {}", status),
                response_time_ms: Some(elapsed),
                model: None,
            })
        }
    }

    async fn chat_completion(
        &self,
        messages: Vec<ChatMessage>,
        model: &str,
    ) -> Result<(ChatMessage, Option<TokenUsage>), AIError> {
        let openai_messages: Vec<OpenAIMessage> = messages
            .iter()
            .map(|m| OpenAIMessage {
                role: match m.role {
                    MessageRole::System => "system".to_string(),
                    MessageRole::User => "user".to_string(),
                    MessageRole::Assistant => "assistant".to_string(),
                },
                content: m.content.clone(),
            })
            .collect();

        let request = OpenAIChatRequest {
            model: model.to_string(),
            messages: openai_messages,
            max_tokens: 2048,
        };

        debug!("Sending chat completion request to OpenAI");

        let response = self
            .client
            .post(format!("{}/chat/completions", self.base_url))
            .header("Authorization", format!("Bearer {}", self.api_key.expose_secret()))
            .header("Content-Type", "application/json")
            .json(&request)
            .timeout(Duration::from_secs(60))
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();

            if status.as_u16() == 401 {
                return Err(AIError::InvalidApiKey("Invalid API key".into()));
            }
            if status.as_u16() == 429 {
                return Err(AIError::RateLimited);
            }

            return Err(AIError::RequestFailed(format!("{}: {}", status, error_text)));
        }

        let chat_response: OpenAIChatResponse = response.json().await?;

        let assistant_message = chat_response
            .choices
            .first()
            .ok_or_else(|| AIError::ParseError("No choices in response".into()))?;

        let message = ChatMessage {
            id: uuid::Uuid::new_v4(),
            role: MessageRole::Assistant,
            content: assistant_message.message.content.clone(),
            timestamp: chrono::Utc::now(),
        };

        let usage = chat_response.usage.map(|u| TokenUsage {
            prompt_tokens: u.prompt_tokens,
            completion_tokens: u.completion_tokens,
            total_tokens: u.total_tokens,
        });

        Ok((message, usage))
    }
}

// ==================== Anthropic Provider ====================

/// Anthropic (Claude) API provider.
pub struct AnthropicProvider {
    client: Client,
    api_key: SecretString,
    base_url: String,
}

impl AnthropicProvider {
    /// Create a new Anthropic provider.
    pub fn new(api_key: SecretString) -> Self {
        Self {
            client: Client::new(),
            api_key,
            base_url: "https://api.anthropic.com/v1".to_string(),
        }
    }

    /// Get default models.
    pub fn default_models() -> Vec<ModelInfo> {
        vec![
            ModelInfo {
                id: "claude-sonnet-4-20250514".to_string(),
                name: "Claude Sonnet 4".to_string(),
                context_window: 200000,
                supports_streaming: true,
            },
            ModelInfo {
                id: "claude-3-5-sonnet-20241022".to_string(),
                name: "Claude 3.5 Sonnet".to_string(),
                context_window: 200000,
                supports_streaming: true,
            },
            ModelInfo {
                id: "claude-3-haiku-20240307".to_string(),
                name: "Claude 3 Haiku".to_string(),
                context_window: 200000,
                supports_streaming: true,
            },
        ]
    }
}

#[derive(Serialize)]
struct AnthropicChatRequest {
    model: String,
    messages: Vec<AnthropicMessage>,
    max_tokens: u32,
}

#[derive(Serialize, Deserialize)]
struct AnthropicMessage {
    role: String,
    content: String,
}

#[derive(Deserialize)]
struct AnthropicChatResponse {
    content: Vec<AnthropicContent>,
    usage: Option<AnthropicUsage>,
}

#[derive(Deserialize)]
struct AnthropicContent {
    text: String,
}

#[derive(Deserialize)]
struct AnthropicUsage {
    input_tokens: u32,
    output_tokens: u32,
}

#[async_trait]
impl AIProvider for AnthropicProvider {
    fn provider_type(&self) -> ProviderType {
        ProviderType::Anthropic
    }

    fn available_models(&self) -> Vec<ModelInfo> {
        Self::default_models()
    }

    async fn test_connection(&self) -> Result<ConnectionTestResult, AIError> {
        let start = Instant::now();

        let request = AnthropicChatRequest {
            model: "claude-3-haiku-20240307".to_string(),
            messages: vec![AnthropicMessage {
                role: "user".to_string(),
                content: "Say 'OK' if you can hear me.".to_string(),
            }],
            max_tokens: 10,
        };

        let response = self
            .client
            .post(format!("{}/messages", self.base_url))
            .header("x-api-key", self.api_key.expose_secret())
            .header("anthropic-version", "2023-06-01")
            .header("Content-Type", "application/json")
            .json(&request)
            .timeout(Duration::from_secs(30))
            .send()
            .await?;

        let elapsed = start.elapsed().as_millis() as u64;

        if response.status().is_success() {
            info!("Anthropic connection test successful");
            Ok(ConnectionTestResult {
                success: true,
                message: "Connection successful".to_string(),
                response_time_ms: Some(elapsed),
                model: Some("claude-3-haiku".to_string()),
            })
        } else {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            warn!("Anthropic connection test failed: {} - {}", status, error_text);

            if status.as_u16() == 401 {
                return Err(AIError::InvalidApiKey("Invalid Anthropic API key".into()));
            }
            if status.as_u16() == 429 {
                return Err(AIError::RateLimited);
            }

            Ok(ConnectionTestResult {
                success: false,
                message: format!("Connection failed: {}", status),
                response_time_ms: Some(elapsed),
                model: None,
            })
        }
    }

    async fn chat_completion(
        &self,
        messages: Vec<ChatMessage>,
        model: &str,
    ) -> Result<(ChatMessage, Option<TokenUsage>), AIError> {
        // Anthropic doesn't support system messages in the messages array
        // We need to handle them separately
        let anthropic_messages: Vec<AnthropicMessage> = messages
            .iter()
            .filter(|m| m.role != MessageRole::System)
            .map(|m| AnthropicMessage {
                role: match m.role {
                    MessageRole::User => "user".to_string(),
                    MessageRole::Assistant => "assistant".to_string(),
                    MessageRole::System => "user".to_string(), // Won't reach here
                },
                content: m.content.clone(),
            })
            .collect();

        let request = AnthropicChatRequest {
            model: model.to_string(),
            messages: anthropic_messages,
            max_tokens: 2048,
        };

        debug!("Sending chat completion request to Anthropic");

        let response = self
            .client
            .post(format!("{}/messages", self.base_url))
            .header("x-api-key", self.api_key.expose_secret())
            .header("anthropic-version", "2023-06-01")
            .header("Content-Type", "application/json")
            .json(&request)
            .timeout(Duration::from_secs(60))
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();

            if status.as_u16() == 401 {
                return Err(AIError::InvalidApiKey("Invalid API key".into()));
            }
            if status.as_u16() == 429 {
                return Err(AIError::RateLimited);
            }

            return Err(AIError::RequestFailed(format!("{}: {}", status, error_text)));
        }

        let chat_response: AnthropicChatResponse = response.json().await?;

        let content = chat_response
            .content
            .first()
            .ok_or_else(|| AIError::ParseError("No content in response".into()))?;

        let message = ChatMessage {
            id: uuid::Uuid::new_v4(),
            role: MessageRole::Assistant,
            content: content.text.clone(),
            timestamp: chrono::Utc::now(),
        };

        let usage = chat_response.usage.map(|u| TokenUsage {
            prompt_tokens: u.input_tokens,
            completion_tokens: u.output_tokens,
            total_tokens: u.input_tokens + u.output_tokens,
        });

        Ok((message, usage))
    }
}

// ==================== Deepseek Provider ====================

/// Deepseek API provider.
pub struct DeepseekProvider {
    inner: OpenAIProvider,
}

impl DeepseekProvider {
    /// Create a new Deepseek provider.
    pub fn new(api_key: SecretString) -> Self {
        let mut inner = OpenAIProvider::new(api_key);
        inner.base_url = "https://api.deepseek.com".to_string();
        Self { inner }
    }

    /// Get default models.
    pub fn default_models() -> Vec<ModelInfo> {
        vec![
            ModelInfo {
                id: "deepseek-chat".to_string(),
                name: "Deepseek Chat".to_string(),
                context_window: 64000,
                supports_streaming: true,
            },
            ModelInfo {
                id: "deepseek-coder".to_string(),
                name: "Deepseek Coder".to_string(),
                context_window: 64000,
                supports_streaming: true,
            },
        ]
    }
}

#[async_trait]
impl AIProvider for DeepseekProvider {
    fn provider_type(&self) -> ProviderType {
        ProviderType::Deepseek
    }

    fn available_models(&self) -> Vec<ModelInfo> {
        Self::default_models()
    }

    async fn test_connection(&self) -> Result<ConnectionTestResult, AIError> {
        self.inner.test_connection().await
    }

    async fn chat_completion(
        &self,
        messages: Vec<ChatMessage>,
        model: &str,
    ) -> Result<(ChatMessage, Option<TokenUsage>), AIError> {
        self.inner.chat_completion(messages, model).await
    }
}

// ==================== z.ai Provider ====================

/// z.ai API provider.
pub struct ZaiProvider {
    inner: OpenAIProvider,
}

impl ZaiProvider {
    /// Create a new z.ai provider.
    pub fn new(api_key: SecretString) -> Self {
        let mut inner = OpenAIProvider::new(api_key);
        inner.base_url = "https://api.z.ai/v1".to_string();
        Self { inner }
    }

    /// Get default models.
    pub fn default_models() -> Vec<ModelInfo> {
        vec![ModelInfo {
            id: "z-1".to_string(),
            name: "Z-1".to_string(),
            context_window: 32000,
            supports_streaming: true,
        }]
    }
}

#[async_trait]
impl AIProvider for ZaiProvider {
    fn provider_type(&self) -> ProviderType {
        ProviderType::Zai
    }

    fn available_models(&self) -> Vec<ModelInfo> {
        Self::default_models()
    }

    async fn test_connection(&self) -> Result<ConnectionTestResult, AIError> {
        self.inner.test_connection().await
    }

    async fn chat_completion(
        &self,
        messages: Vec<ChatMessage>,
        model: &str,
    ) -> Result<(ChatMessage, Option<TokenUsage>), AIError> {
        self.inner.chat_completion(messages, model).await
    }
}

// ==================== Custom Provider ====================

/// Custom OpenAI-compatible API provider.
pub struct CustomProvider {
    inner: OpenAIProvider,
}

impl CustomProvider {
    /// Create a new custom provider.
    pub fn new(api_key: SecretString, base_url: String) -> Self {
        let mut inner = OpenAIProvider::new(api_key);
        inner.base_url = base_url;
        Self { inner }
    }
}

#[async_trait]
impl AIProvider for CustomProvider {
    fn provider_type(&self) -> ProviderType {
        ProviderType::Custom
    }

    fn available_models(&self) -> Vec<ModelInfo> {
        vec![ModelInfo {
            id: "custom".to_string(),
            name: "Custom Model".to_string(),
            context_window: 8192,
            supports_streaming: true,
        }]
    }

    async fn test_connection(&self) -> Result<ConnectionTestResult, AIError> {
        self.inner.test_connection().await
    }

    async fn chat_completion(
        &self,
        messages: Vec<ChatMessage>,
        model: &str,
    ) -> Result<(ChatMessage, Option<TokenUsage>), AIError> {
        self.inner.chat_completion(messages, model).await
    }
}
