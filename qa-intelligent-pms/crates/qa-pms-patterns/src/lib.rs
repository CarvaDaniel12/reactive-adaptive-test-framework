//! Pattern Detection & Proactive Alerts
//!
//! Epic 9: Detects patterns in workflow data and generates alerts.
//!
//! Pattern Types:
//! - Time Excess: Steps/tickets taking >50% longer than estimated
//! - Consecutive Problem: 3+ tickets with same component/issue
//! - Spike: Sudden increase in tickets for an area

pub mod alerts;
pub mod detector;
pub mod repository;
pub mod types;

pub use alerts::AlertService;
pub use detector::PatternDetector;
pub use repository::PatternRepository;
pub use types::*;
