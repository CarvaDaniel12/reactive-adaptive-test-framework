//! # QA PMS Dashboard
//!
//! Dashboard logic, shared types, and metric utilities for QA Intelligent PMS.
//!
//! This crate provides shared building blocks used by both QA Individual Dashboard
//! and PM Observability Dashboard:
//!
//! - **Error handling**: [`SqlxResultExt`] trait for consistent SQLx error mapping
//! - **Period utilities**: [`parse_period`], [`period_boundaries`] for date range handling
//! - **Metric calculations**: [`calculate_change`], [`calculate_trend`] for KPI computations
//! - **Shared types**: [`KPIMetric`], [`TrendDataPoint`], [`ActivityItem`] for API responses
//!
//! ## Example Usage
//!
//! ```ignore
//! use qa_pms_dashboard::{SqlxResultExt, parse_period, calculate_change, KPIMetric};
//!
//! // Parse period from query parameter
//! let days = parse_period("30d"); // Returns 30
//!
//! // Map SQLx errors consistently
//! let result = sqlx::query_as!(...)
//!     .fetch_optional(&pool)
//!     .await
//!     .map_internal("Failed to fetch data")?;
//!
//! // Calculate KPI with automatic trend
//! let kpi = KPIMetric::from_values(current_value, previous_value);
//! ```

pub mod error;
pub mod metrics;
pub mod period;
pub mod types;

// Re-export commonly used items at crate root
pub use error::SqlxResultExt;
pub use metrics::{calculate_change, calculate_trend, Trend};
pub use period::{default_period, parse_period, period_boundaries, Period};
pub use types::{ActivityItem, KPIMetric, TrendDataPoint};
