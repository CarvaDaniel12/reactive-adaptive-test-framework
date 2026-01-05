//! # QA PMS Splunk
//!
//! Splunk manual query integration for QA Intelligent PMS.
//!
//! Epic 11: Provides:
//! - Query template management (CRUD)
//! - SPL query building with placeholders
//! - Log display formatting
//!
//! Note: Splunk Cloud does not support direct API integration for log queries.
//! This module provides a manual query interface with pre-built templates.

pub mod error;
pub mod templates;
pub mod types;

pub use error::SplunkError;
pub use templates::QueryTemplateService;
pub use types::*;
