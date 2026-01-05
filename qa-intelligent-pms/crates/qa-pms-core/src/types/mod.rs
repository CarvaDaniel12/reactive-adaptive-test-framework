//! Common types used across the QA PMS framework.
//!
//! All types follow the naming conventions from the architecture document:
//! - Rust structs use `PascalCase`
//! - JSON serialization uses `camelCase` via `#[serde(rename_all = "camelCase")]`

mod ids;
mod integration;
mod pagination;

pub use ids::{TicketId, UserId, WorkflowId, WorkflowInstanceId, WorkflowStepId};
pub use integration::{Integration, IntegrationHealth, IntegrationStatus};
pub use pagination::{PageInfo, Paginated};
