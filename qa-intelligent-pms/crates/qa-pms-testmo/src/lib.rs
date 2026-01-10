//! # QA PMS Testmo
//!
//! Testmo API integration.
//!
//! This crate provides:
//! - API key authentication with Bearer token
//! - Project and test suite listing
//! - Test case search by keywords
//! - Test case details retrieval
//! - Test run creation
//! - Health check for integration monitoring

mod client;
mod error;
pub mod health;
mod types;

pub use client::TestmoClient;
pub use error::TestmoError;
pub use health::TestmoHealthCheck;
pub use types::{
    CreateTestRunRequest, Project, SearchResult, TestCase, TestRun, TestStep, TestSuite,
};
