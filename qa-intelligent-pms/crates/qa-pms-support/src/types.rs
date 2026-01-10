//! Type definitions for the support module.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;
use uuid::Uuid;

/// Status of an error log entry.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type, ToSchema)]
#[sqlx(type_name = "VARCHAR", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
#[derive(Default)]
pub enum ErrorStatus {
    /// New error, not yet investigated
    #[default]
    New,
    /// Currently being investigated
    Investigating,
    /// Error has been resolved
    Resolved,
    /// Error was dismissed (not actionable)
    Dismissed,
}

impl std::fmt::Display for ErrorStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::New => write!(f, "new"),
            Self::Investigating => write!(f, "investigating"),
            Self::Resolved => write!(f, "resolved"),
            Self::Dismissed => write!(f, "dismissed"),
        }
    }
}

/// Severity level of an error.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type, ToSchema)]
#[sqlx(type_name = "VARCHAR", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
#[derive(Default)]
pub enum ErrorSeverity {
    /// Low severity - informational
    Low,
    /// Medium severity - needs attention
    #[default]
    Medium,
    /// High severity - urgent
    High,
    /// Critical severity - immediate action required
    Critical,
}

impl std::fmt::Display for ErrorSeverity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Low => write!(f, "low"),
            Self::Medium => write!(f, "medium"),
            Self::High => write!(f, "high"),
            Self::Critical => write!(f, "critical"),
        }
    }
}

/// Type of error source.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type, ToSchema)]
#[sqlx(type_name = "VARCHAR", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
#[derive(Default)]
pub enum ErrorSource {
    /// Frontend error (browser)
    Frontend,
    /// Backend API error
    Backend,
    /// Integration error (Jira, Postman, etc.)
    Integration,
    /// Database error
    Database,
    /// Unknown source
    #[default]
    Unknown,
}

impl std::fmt::Display for ErrorSource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Frontend => write!(f, "frontend"),
            Self::Backend => write!(f, "backend"),
            Self::Integration => write!(f, "integration"),
            Self::Database => write!(f, "database"),
            Self::Unknown => write!(f, "unknown"),
        }
    }
}

/// An error log entry captured by the system.
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ErrorLog {
    /// Unique identifier
    pub id: Uuid,
    /// Error message
    pub message: String,
    /// Stack trace (if available)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack_trace: Option<String>,
    /// Error severity
    pub severity: ErrorSeverity,
    /// Error source
    pub source: ErrorSource,
    /// Current status
    pub status: ErrorStatus,
    /// User ID who encountered the error (if authenticated)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<Uuid>,
    /// User's session ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_id: Option<String>,
    /// Page/route where error occurred
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_url: Option<String>,
    /// Action being performed when error occurred
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
    /// Browser information
    #[serde(skip_serializing_if = "Option::is_none")]
    pub browser_info: Option<String>,
    /// Device information
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_info: Option<String>,
    /// Additional context as JSON
    #[sqlx(json)]
    pub context: serde_json::Value,
    /// Number of times this error occurred
    pub occurrence_count: i32,
    /// First occurrence timestamp
    pub first_seen_at: DateTime<Utc>,
    /// Last occurrence timestamp
    pub last_seen_at: DateTime<Utc>,
    /// Resolution notes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolution_notes: Option<String>,
    /// Linked knowledge base entry ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kb_entry_id: Option<Uuid>,
    /// Created timestamp
    pub created_at: DateTime<Utc>,
    /// Updated timestamp
    pub updated_at: DateTime<Utc>,
}

/// Input for creating a new error log.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CreateErrorLogInput {
    /// Error message
    pub message: String,
    /// Stack trace (if available)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack_trace: Option<String>,
    /// Error severity
    #[serde(default)]
    pub severity: ErrorSeverity,
    /// Error source
    #[serde(default)]
    pub source: ErrorSource,
    /// User ID who encountered the error
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<Uuid>,
    /// User's session ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_id: Option<String>,
    /// Page/route where error occurred
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_url: Option<String>,
    /// Action being performed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
    /// Browser information
    #[serde(skip_serializing_if = "Option::is_none")]
    pub browser_info: Option<String>,
    /// Device information
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_info: Option<String>,
    /// Additional context
    #[serde(default)]
    pub context: serde_json::Value,
}

/// Input for updating an error log status.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct UpdateErrorStatusInput {
    /// New status
    pub status: ErrorStatus,
    /// Resolution notes (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolution_notes: Option<String>,
    /// Link to knowledge base entry
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kb_entry_id: Option<Uuid>,
}

/// A knowledge base entry for common issues.
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct KnowledgeBaseEntry {
    /// Unique identifier
    pub id: Uuid,
    /// Title of the entry
    pub title: String,
    /// Problem description
    pub problem: String,
    /// Root cause explanation
    pub cause: String,
    /// Solution steps
    pub solution: String,
    /// Related error messages (for matching)
    #[sqlx(json)]
    pub related_errors: Vec<String>,
    /// Tags for categorization
    #[sqlx(json)]
    pub tags: Vec<String>,
    /// Number of times this entry was viewed
    pub view_count: i32,
    /// Number of times marked as helpful
    pub helpful_count: i32,
    /// Number of times marked as not helpful
    pub not_helpful_count: i32,
    /// Created timestamp
    pub created_at: DateTime<Utc>,
    /// Updated timestamp
    pub updated_at: DateTime<Utc>,
}

/// Input for creating a knowledge base entry.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CreateKbEntryInput {
    /// Title of the entry
    pub title: String,
    /// Problem description
    pub problem: String,
    /// Root cause explanation
    pub cause: String,
    /// Solution steps
    pub solution: String,
    /// Related error messages
    #[serde(default)]
    pub related_errors: Vec<String>,
    /// Tags for categorization
    #[serde(default)]
    pub tags: Vec<String>,
}

/// Input for updating a knowledge base entry.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct UpdateKbEntryInput {
    /// Title of the entry
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// Problem description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub problem: Option<String>,
    /// Root cause explanation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cause: Option<String>,
    /// Solution steps
    #[serde(skip_serializing_if = "Option::is_none")]
    pub solution: Option<String>,
    /// Related error messages
    #[serde(skip_serializing_if = "Option::is_none")]
    pub related_errors: Option<Vec<String>>,
    /// Tags for categorization
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
}

/// Result of an integration diagnostic check.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct DiagnosticResult {
    /// Integration name
    pub integration: String,
    /// Whether the check passed
    pub passed: bool,
    /// Status message
    pub message: String,
    /// Latency in milliseconds (if applicable)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latency_ms: Option<u64>,
    /// Recent error count
    pub recent_error_count: i32,
    /// Suggested fixes (if check failed)
    #[serde(default)]
    pub suggestions: Vec<String>,
    /// Timestamp of the check
    pub checked_at: DateTime<Utc>,
}

/// Full diagnostics report for all integrations.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct DiagnosticsReport {
    /// Overall health status
    pub overall_healthy: bool,
    /// Individual integration results
    pub results: Vec<DiagnosticResult>,
    /// Summary message
    pub summary: String,
    /// Report generation timestamp
    pub generated_at: DateTime<Utc>,
}

/// A troubleshooting suggestion based on error analysis.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct TroubleshootingSuggestion {
    /// Suggestion ID
    pub id: Uuid,
    /// Source of the suggestion
    pub source: SuggestionSource,
    /// Title of the suggestion
    pub title: String,
    /// Description/steps
    pub description: String,
    /// Relevance score (0-100)
    pub relevance_score: i32,
    /// Linked knowledge base entry (if applicable)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kb_entry_id: Option<Uuid>,
}

/// Source of a troubleshooting suggestion.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum SuggestionSource {
    /// From knowledge base
    KnowledgeBase,
    /// From similar past issues
    SimilarIssues,
    /// System-generated diagnostic step
    DiagnosticStep,
}

/// Filter options for querying error logs.
#[derive(Debug, Clone, Default, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ErrorLogFilter {
    /// Filter by status
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<ErrorStatus>,
    /// Filter by severity
    #[serde(skip_serializing_if = "Option::is_none")]
    pub severity: Option<ErrorSeverity>,
    /// Filter by source
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<ErrorSource>,
    /// Filter by user ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<Uuid>,
    /// Search in error message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search: Option<String>,
    /// Filter by date range start
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_date: Option<DateTime<Utc>>,
    /// Filter by date range end
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_date: Option<DateTime<Utc>>,
}

/// Pagination parameters.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct Pagination {
    /// Page number (1-indexed)
    #[serde(default = "default_page")]
    pub page: i32,
    /// Items per page
    #[serde(default = "default_per_page")]
    pub per_page: i32,
}

const fn default_page() -> i32 {
    1
}

const fn default_per_page() -> i32 {
    20
}

impl Default for Pagination {
    fn default() -> Self {
        Self {
            page: 1,
            per_page: 20,
        }
    }
}

/// Sort options for error logs.
#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum ErrorLogSort {
    /// Sort by last seen (most recent first)
    #[default]
    LastSeenDesc,
    /// Sort by last seen (oldest first)
    LastSeenAsc,
    /// Sort by severity (highest first)
    SeverityDesc,
    /// Sort by occurrence count (highest first)
    OccurrenceDesc,
}

/// Paginated response wrapper.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct PaginatedResponse<T: Serialize> {
    /// Items in current page
    pub items: Vec<T>,
    /// Total number of items
    pub total: i64,
    /// Current page
    pub page: i32,
    /// Items per page
    pub per_page: i32,
    /// Total number of pages
    pub total_pages: i32,
}

impl<T: Serialize> PaginatedResponse<T> {
    /// Create a new paginated response.
    #[must_use]
    pub fn new(items: Vec<T>, total: i64, page: i32, per_page: i32) -> Self {
        let total_pages = ((total as f64) / f64::from(per_page)).ceil() as i32;
        Self {
            items,
            total,
            page,
            per_page,
            total_pages,
        }
    }
}

/// Support dashboard summary statistics.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct SupportDashboardSummary {
    /// Total error count
    pub total_errors: i64,
    /// New errors (not investigated)
    pub new_errors: i64,
    /// Errors being investigated
    pub investigating: i64,
    /// Resolved errors
    pub resolved: i64,
    /// Critical severity count
    pub critical_count: i64,
    /// High severity count
    pub high_count: i64,
    /// Errors by source
    pub by_source: Vec<SourceCount>,
    /// Most frequent errors (top 5)
    pub top_errors: Vec<TopError>,
}

/// Error count by source.
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct SourceCount {
    /// Source type
    pub source: ErrorSource,
    /// Count
    pub count: i64,
}

/// Top error entry.
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct TopError {
    /// Error ID
    pub id: Uuid,
    /// Error message (truncated)
    pub message: String,
    /// Occurrence count
    pub occurrence_count: i32,
    /// Severity
    pub severity: ErrorSeverity,
}
