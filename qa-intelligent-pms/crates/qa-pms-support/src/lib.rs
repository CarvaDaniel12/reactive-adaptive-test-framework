//! Support Portal & Troubleshooting module for QA Intelligent PMS.
//!
//! This crate provides:
//! - Error log capture and storage
//! - Support dashboard functionality
//! - Integration diagnostics
//! - Knowledge base for common issues
//! - Troubleshooting suggestions

pub mod types;
pub mod error;
pub mod repository;
pub mod diagnostics;
pub mod knowledge_base;

pub use types::*;
pub use error::SupportError;
pub use repository::SupportRepository;
pub use diagnostics::DiagnosticsService;
pub use knowledge_base::KnowledgeBaseService;
