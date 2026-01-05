//! # QA PMS AI
//!
//! AI companion with BYOK (Bring Your Own Key) support.
//!
//! This crate provides:
//! - AI provider abstraction (Anthropic, `OpenAI`, Deepseek, z.ai, Custom)
//! - Semantic search enhancement
//! - Gherkin test suggestions
//! - Mini-chatbot functionality
//!
//! ## Features
//!
//! - **BYOK**: Users bring their own API keys
//! - **Multi-Provider**: Support for multiple AI providers
//! - **Graceful Fallback**: Works without AI configured
//! - **Streaming**: Real-time response streaming

pub mod types;
pub mod error;
pub mod provider;
pub mod chat;
pub mod semantic;
pub mod gherkin;

pub use types::*;
pub use error::AIError;
pub use provider::{AIProvider, AIClient};
pub use chat::ChatService;
pub use semantic::SemanticSearchService;
pub use gherkin::GherkinAnalyzer;
