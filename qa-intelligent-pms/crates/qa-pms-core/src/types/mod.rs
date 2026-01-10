//! Common types used across the QA PMS framework.
//!
//! All types follow the naming conventions from the architecture document:
//! - Rust structs use `PascalCase`
//! - JSON serialization uses `camelCase` via `#[serde(rename_all = "camelCase")]`

mod ids;
mod integration;
mod pagination;
mod test_case;

pub use ids::{TestCaseId, TicketId, UserId, WorkflowId, WorkflowInstanceId, WorkflowStepId};
pub use integration::{Integration, IntegrationHealth, IntegrationStatus};
pub use pagination::{PageInfo, Paginated};
pub use test_case::{TestCase, TestCaseStatus, TestCaseType, TestPriority, TestRepository};