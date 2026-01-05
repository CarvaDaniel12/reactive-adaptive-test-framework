//! # QA PMS Jira
//!
//! Jira integration using OAuth 2.0 + PKCE.
//!
//! This crate provides:
//! - OAuth 2.0 + PKCE authentication flow
//! - Secure token storage with encryption
//! - Automatic token refresh
//! - Ticket listing and filtering
//! - Ticket detail retrieval with comments and attachments
//! - Ticket status transitions with retry logic
//! - Health check for integration monitoring

pub mod error;
pub mod health;
pub mod oauth;
pub mod pkce;
pub mod tickets;
pub mod token_refresh;
pub mod token_store;

// Re-export main types
pub use error::{JiraApiError, JiraAuthError};
pub use health::JiraHealthCheck;
pub use oauth::{AuthorizationState, JiraOAuthClient, JiraOAuthConfig, TokenResponse};
pub use tickets::{
    Attachment, Comment, CommentContainer, JiraTicket, JiraTicketsClient, SearchResponse,
    TicketDetail, TicketDetailFields, TicketFields, TicketFilters, Transition, TransitionTarget,
};
pub use token_refresh::spawn_token_refresh_task;
pub use token_store::{FileTokenStore, InMemoryAuthStateStore};
