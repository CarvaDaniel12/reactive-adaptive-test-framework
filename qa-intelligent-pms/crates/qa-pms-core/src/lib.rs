//! # QA PMS Core
//!
//! Shared types, traits, and utilities for the QA Intelligent PMS framework.
//!
//! This crate provides:
//! - Common types used across all crates (`UserId`, `WorkflowId`, `TicketId`, etc.)
//! - Error types for API boundaries using `thiserror`
//! - Shared traits for integrations
//! - Authentication types and token storage traits
//! - Health check types and traits for integration monitoring
//! - Keyword extraction for contextual search
//! - Result type aliases using `anyhow` for internal operations

pub mod auth;
pub mod error;
pub mod health;
pub mod health_store;
pub mod keywords;
pub mod types;

// Re-export commonly used types at crate root
pub use auth::{AuthStateStore, StoredTokens, TokenStore};
pub use error::{ApiError, ErrorResponse};
pub use health::{HealthCheck, HealthCheckResult, HealthStatus, IntegrationHealth};
pub use health_store::HealthStore;
pub use keywords::KeywordExtractor;
pub use types::{TicketId, UserId, WorkflowId};

/// Result type alias for internal operations using `anyhow`
pub type Result<T> = anyhow::Result<T>;
