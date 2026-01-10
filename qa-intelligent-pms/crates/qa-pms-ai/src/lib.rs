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

pub mod anomaly_detector;
pub mod anomaly_repository;
pub mod chat;
pub mod error;
pub mod gherkin;
pub mod provider;
pub mod semantic;
pub mod test_generator;
pub mod types;

pub use anomaly_detector::{
    Anomaly, AnomalyDetector, AnomalyMetrics, AnomalySeverity, AnomalyType, BaselineMetrics,
    WorkflowExecution,
};
pub use anomaly_repository::{
    AnomalyCountByDate, AnomalyRepository, SeverityDistribution, WorkflowExecutionData,
};
pub use chat::ChatService;
pub use error::AIError;
pub use gherkin::GherkinAnalyzer;
pub use provider::{AIClient, AIProvider};
pub use semantic::SemanticSearchService;
pub use test_generator::{GeneratedTestCase, TestGenerator, TicketDetails};
pub use types::*;
