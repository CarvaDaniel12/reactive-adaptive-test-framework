//! Support Portal & Troubleshooting module for QA Intelligent PMS.
//!
//! This crate provides:
//! - Error log capture and storage
//! - Support dashboard functionality
//! - Integration diagnostics
//! - Knowledge base for common issues
//! - Troubleshooting suggestions

pub mod diagnostics;
pub mod error;
pub mod knowledge_base;
pub mod repository;
pub mod types;

pub use diagnostics::DiagnosticsService;
pub use error::SupportError;
pub use knowledge_base::KnowledgeBaseService;
pub use repository::SupportRepository;
pub use types::*;
