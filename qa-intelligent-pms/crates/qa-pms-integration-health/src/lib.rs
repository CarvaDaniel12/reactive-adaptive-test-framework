//! # QA PMS Integration Health
//!
//! Integration health monitoring for PMS integrations (Booking.com, Airbnb, Vrbo, HMBN).
//!
//! This crate provides:
//! - Types for integration health status and events
//! - Error handling for integration health operations
//!
//! ## Example Usage
//!
//! ```ignore
//! use qa_pms_integration_health::{IntegrationId, HealthStatus, IntegrationHealth, IntegrationHealthError};
//!
//! let integration = IntegrationId::BookingCom;
//! let status = HealthStatus::Healthy;
//! ```

pub mod error;
pub mod repository;
pub mod service;
pub mod types;

// Re-export commonly used items at crate root
pub use error::IntegrationHealthError;
pub use repository::IntegrationHealthRepository;
pub use service::IntegrationHealthService;
pub use types::{HealthStatus, IntegrationEvent, IntegrationHealth, IntegrationId};
