//! Splunk types and data structures.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

/// A Splunk query template.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QueryTemplate {
    /// Unique template ID.
    pub id: Uuid,
    /// Template name.
    pub name: String,
    /// Template description.
    pub description: Option<String>,
    /// SPL query with placeholders (e.g., {`TICKET_KEY`}, {`USER_ID`}).
    pub query: String,
    /// Category for grouping templates.
    pub category: TemplateCategory,
    /// Whether this is a system template (read-only).
    pub is_system: bool,
    /// User who created the template (None for system templates).
    pub created_by: Option<Uuid>,
    /// Creation timestamp.
    pub created_at: DateTime<Utc>,
    /// Last update timestamp.
    pub updated_at: DateTime<Utc>,
}

/// Template category for organization.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type, ToSchema)]
#[sqlx(type_name = "VARCHAR", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum TemplateCategory {
    /// Error and exception logs.
    Errors,
    /// Performance metrics.
    Performance,
    /// User activity logs.
    UserActivity,
    /// Security-related logs.
    Security,
    /// Custom user templates.
    Custom,
}

impl std::fmt::Display for TemplateCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Errors => write!(f, "errors"),
            Self::Performance => write!(f, "performance"),
            Self::UserActivity => write!(f, "user_activity"),
            Self::Security => write!(f, "security"),
            Self::Custom => write!(f, "custom"),
        }
    }
}

/// Input for creating a new template.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateTemplateInput {
    /// Template name.
    pub name: String,
    /// Template description.
    pub description: Option<String>,
    /// SPL query with placeholders.
    pub query: String,
    /// Category for grouping.
    pub category: TemplateCategory,
}

/// Input for updating a template.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateTemplateInput {
    /// Updated name (optional).
    pub name: Option<String>,
    /// Updated description (optional).
    pub description: Option<String>,
    /// Updated query (optional).
    pub query: Option<String>,
    /// Updated category (optional).
    pub category: Option<TemplateCategory>,
}

/// A prepared query ready for execution.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PreparedQuery {
    /// The original template ID.
    pub template_id: Option<Uuid>,
    /// The final SPL query with placeholders filled.
    pub query: String,
    /// Time range start.
    pub time_start: DateTime<Utc>,
    /// Time range end.
    pub time_end: DateTime<Utc>,
    /// Index to search (optional).
    pub index: Option<String>,
}

/// Placeholder definition for query templates.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Placeholder {
    /// Placeholder key (e.g., "`TICKET_KEY`").
    pub key: String,
    /// Human-readable label.
    pub label: String,
    /// Default value (optional).
    pub default_value: Option<String>,
    /// Whether this placeholder is required.
    pub required: bool,
}

impl Placeholder {
    /// Get common placeholders.
    #[must_use]
    pub fn common_placeholders() -> Vec<Self> {
        vec![
            Self {
                key: "TICKET_KEY".to_string(),
                label: "Ticket Key".to_string(),
                default_value: None,
                required: false,
            },
            Self {
                key: "USER_ID".to_string(),
                label: "User ID".to_string(),
                default_value: None,
                required: false,
            },
            Self {
                key: "ERROR_TYPE".to_string(),
                label: "Error Type".to_string(),
                default_value: None,
                required: false,
            },
            Self {
                key: "ENDPOINT".to_string(),
                label: "API Endpoint".to_string(),
                default_value: None,
                required: false,
            },
        ]
    }
}

/// A log entry from Splunk (for display purposes).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LogEntry {
    /// Timestamp of the log entry.
    pub timestamp: DateTime<Utc>,
    /// Log level (INFO, WARN, ERROR, etc.).
    pub level: String,
    /// Log message.
    pub message: String,
    /// Source of the log.
    pub source: Option<String>,
    /// Host that generated the log.
    pub host: Option<String>,
    /// Additional fields as JSON.
    #[serde(default)]
    pub fields: serde_json::Value,
}

/// Query execution result.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QueryResult {
    /// The query that was executed.
    pub query: String,
    /// Log entries returned.
    pub entries: Vec<LogEntry>,
    /// Total count of results.
    pub total_count: i64,
    /// Whether results were truncated.
    pub truncated: bool,
    /// Execution time in milliseconds.
    pub execution_time_ms: i64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_template_category_display() {
        assert_eq!(TemplateCategory::Errors.to_string(), "errors");
        assert_eq!(TemplateCategory::Performance.to_string(), "performance");
        assert_eq!(TemplateCategory::UserActivity.to_string(), "user_activity");
        assert_eq!(TemplateCategory::Security.to_string(), "security");
        assert_eq!(TemplateCategory::Custom.to_string(), "custom");
    }

    #[test]
    fn test_common_placeholders() {
        let placeholders = Placeholder::common_placeholders();
        assert_eq!(placeholders.len(), 4);
        assert_eq!(placeholders[0].key, "TICKET_KEY");
    }
}
