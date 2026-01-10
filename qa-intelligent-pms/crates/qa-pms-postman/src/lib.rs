//! # QA PMS Postman
//!
//! Postman API integration.
//!
//! This crate provides:
//! - API key authentication
//! - Workspace and collection listing
//! - Collection search by keywords
//! - Test case retrieval
//! - Health check for integration monitoring

mod client;
mod error;
pub mod health;
mod types;

pub use client::PostmanClient;
pub use error::PostmanError;
pub use health::PostmanHealthCheck;
pub use types::{
    Collection, CollectionInfo, CollectionItem, CollectionSummary, RequestInfo, RequestUrl,
    SearchResult, Workspace,
};
