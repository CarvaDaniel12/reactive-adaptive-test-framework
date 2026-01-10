//! # QA PMS Tracking
//!
//! High-level time tracking system for workflow execution.
//!
//! This crate provides a convenient service layer over the low-level
//! `qa-pms-time` crate, offering:
//!
//! - **[`TrackingService`]**: Main service for managing time tracking sessions
//! - **[`TrackingError`]**: Comprehensive error types for tracking operations
//!
//! ## Architecture
//!
//! ```text
//! ┌─────────────────────────────────────────┐
//! │          qa-pms-tracking                │
//! │  (High-level service & error handling)  │
//! └────────────────────┬────────────────────┘
//!                      │
//!                      ▼
//! ┌─────────────────────────────────────────┐
//! │            qa-pms-time                  │
//! │  (Low-level repository & types)         │
//! └─────────────────────────────────────────┘
//! ```
//!
//! ## Example Usage
//!
//! ```ignore
//! use qa_pms_tracking::TrackingService;
//! use uuid::Uuid;
//!
//! async fn track_workflow(pool: PgPool, workflow_id: Uuid) -> Result<(), TrackingError> {
//!     let tracker = TrackingService::new(pool);
//!
//!     // Start tracking step 0
//!     tracker.start_step(workflow_id, 0).await?;
//!
//!     // ... user works on step ...
//!
//!     // Pause if needed
//!     tracker.pause_current(workflow_id).await?;
//!
//!     // Resume work
//!     tracker.resume_current(workflow_id).await?;
//!
//!     // End the step
//!     let session = tracker.end_step(workflow_id, 0).await?;
//!     println!("Step took {} seconds", session.total_seconds);
//!
//!     // Get summary for the workflow
//!     let summary = tracker.calculate_summary(workflow_id).await?;
//!     println!("Total workflow time: {} seconds", summary.total_seconds);
//!
//!     Ok(())
//! }
//! ```
//!
//! ## Re-exports
//!
//! This crate re-exports types from `qa-pms-time` for convenience:
//! - [`TimeSession`]: A time tracking session for a workflow step
//! - [`TimeSummary`]: Summary of time spent on a workflow
//! - [`StepTime`]: Time spent on a single step

pub mod error;
pub mod service;

pub use error::TrackingError;
pub use service::TrackingService;

// Re-export commonly used types from qa-pms-time
pub use qa_pms_time::{StepTime, TimeSession, TimeSummary};
