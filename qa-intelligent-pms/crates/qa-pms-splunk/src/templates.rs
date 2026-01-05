//! Query template service for Splunk.
//!
//! Manages CRUD operations for SPL query templates.

use chrono::Utc;
use sqlx::{FromRow, PgPool};
use std::collections::HashMap;
use tracing::{info, instrument};
use uuid::Uuid;

use crate::error::SplunkError;
use crate::types::{
    CreateTemplateInput, PreparedQuery, QueryTemplate, TemplateCategory, UpdateTemplateInput,
};

/// Database row for query template.
#[derive(Debug, FromRow)]
struct QueryTemplateRow {
    id: Uuid,
    name: String,
    description: Option<String>,
    query: String,
    category: String,
    is_system: bool,
    created_by: Option<Uuid>,
    created_at: chrono::DateTime<Utc>,
    updated_at: chrono::DateTime<Utc>,
}

impl From<QueryTemplateRow> for QueryTemplate {
    fn from(row: QueryTemplateRow) -> Self {
        let category = match row.category.as_str() {
            "errors" => TemplateCategory::Errors,
            "performance" => TemplateCategory::Performance,
            "user_activity" => TemplateCategory::UserActivity,
            "security" => TemplateCategory::Security,
            _ => TemplateCategory::Custom,
        };

        Self {
            id: row.id,
            name: row.name,
            description: row.description,
            query: row.query,
            category,
            is_system: row.is_system,
            created_by: row.created_by,
            created_at: row.created_at,
            updated_at: row.updated_at,
        }
    }
}

/// Service for managing Splunk query templates.
#[derive(Debug, Clone)]
pub struct QueryTemplateService {
    pool: PgPool,
}

impl QueryTemplateService {
    /// Create a new template service.
    #[must_use]
    pub const fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// List all templates, optionally filtered by category.
    #[instrument(skip(self))]
    pub async fn list_templates(
        &self,
        category: Option<TemplateCategory>,
        user_id: Option<Uuid>,
    ) -> Result<Vec<QueryTemplate>, SplunkError> {
        let rows: Vec<QueryTemplateRow> = if let Some(cat) = category {
            sqlx::query_as(
                r"
                SELECT id, name, description, query, category, is_system, created_by, created_at, updated_at
                FROM splunk_query_templates
                WHERE category = $1
                  AND (is_system = true OR created_by = $2 OR $2 IS NULL)
                ORDER BY is_system DESC, name ASC
                ",
            )
            .bind(cat.to_string())
            .bind(user_id)
            .fetch_all(&self.pool)
            .await?
        } else {
            sqlx::query_as(
                r"
                SELECT id, name, description, query, category, is_system, created_by, created_at, updated_at
                FROM splunk_query_templates
                WHERE is_system = true OR created_by = $1 OR $1 IS NULL
                ORDER BY is_system DESC, category, name ASC
                ",
            )
            .bind(user_id)
            .fetch_all(&self.pool)
            .await?
        };

        Ok(rows.into_iter().map(Into::into).collect())
    }

    /// Get a template by ID.
    #[instrument(skip(self))]
    pub async fn get_template(&self, id: Uuid) -> Result<QueryTemplate, SplunkError> {
        let row: Option<QueryTemplateRow> = sqlx::query_as(
            r"
            SELECT id, name, description, query, category, is_system, created_by, created_at, updated_at
            FROM splunk_query_templates
            WHERE id = $1
            ",
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        row.map(Into::into)
            .ok_or_else(|| SplunkError::TemplateNotFound(id.to_string()))
    }

    /// Create a new template.
    #[instrument(skip(self))]
    pub async fn create_template(
        &self,
        input: CreateTemplateInput,
        user_id: Uuid,
    ) -> Result<QueryTemplate, SplunkError> {
        // Validate the query has valid syntax (basic check)
        if input.query.trim().is_empty() {
            return Err(SplunkError::InvalidTemplate(
                "Query cannot be empty".to_string(),
            ));
        }

        let now = Utc::now();
        let id = Uuid::new_v4();

        let row: QueryTemplateRow = sqlx::query_as(
            r"
            INSERT INTO splunk_query_templates (id, name, description, query, category, is_system, created_by, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, false, $6, $7, $7)
            RETURNING id, name, description, query, category, is_system, created_by, created_at, updated_at
            ",
        )
        .bind(id)
        .bind(&input.name)
        .bind(&input.description)
        .bind(&input.query)
        .bind(input.category.to_string())
        .bind(user_id)
        .bind(now)
        .fetch_one(&self.pool)
        .await?;

        info!(template_id = %id, name = %input.name, "Created new Splunk query template");

        Ok(row.into())
    }

    /// Update an existing template.
    #[instrument(skip(self))]
    pub async fn update_template(
        &self,
        id: Uuid,
        input: UpdateTemplateInput,
        user_id: Uuid,
    ) -> Result<QueryTemplate, SplunkError> {
        // Check if template exists and is not a system template
        let existing = self.get_template(id).await?;

        if existing.is_system {
            return Err(SplunkError::InvalidTemplate(
                "Cannot modify system templates".to_string(),
            ));
        }

        // Check ownership
        if existing.created_by != Some(user_id) {
            return Err(SplunkError::InvalidTemplate(
                "Cannot modify templates created by other users".to_string(),
            ));
        }

        let now = Utc::now();
        let name = input.name.unwrap_or(existing.name);
        let description = input.description.or(existing.description);
        let query = input.query.unwrap_or(existing.query);
        let category = input.category.unwrap_or(existing.category);

        let row: QueryTemplateRow = sqlx::query_as(
            r"
            UPDATE splunk_query_templates
            SET name = $2, description = $3, query = $4, category = $5, updated_at = $6
            WHERE id = $1
            RETURNING id, name, description, query, category, is_system, created_by, created_at, updated_at
            ",
        )
        .bind(id)
        .bind(&name)
        .bind(&description)
        .bind(&query)
        .bind(category.to_string())
        .bind(now)
        .fetch_one(&self.pool)
        .await?;

        info!(template_id = %id, "Updated Splunk query template");

        Ok(row.into())
    }

    /// Delete a template.
    #[instrument(skip(self))]
    pub async fn delete_template(&self, id: Uuid, user_id: Uuid) -> Result<(), SplunkError> {
        // Check if template exists and is not a system template
        let existing = self.get_template(id).await?;

        if existing.is_system {
            return Err(SplunkError::InvalidTemplate(
                "Cannot delete system templates".to_string(),
            ));
        }

        // Check ownership
        if existing.created_by != Some(user_id) {
            return Err(SplunkError::InvalidTemplate(
                "Cannot delete templates created by other users".to_string(),
            ));
        }

        sqlx::query("DELETE FROM splunk_query_templates WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await?;

        info!(template_id = %id, "Deleted Splunk query template");

        Ok(())
    }

    /// Prepare a query by filling in placeholders.
    pub fn prepare_query(
        &self,
        template: &QueryTemplate,
        placeholders: &HashMap<String, String>,
        time_start: chrono::DateTime<Utc>,
        time_end: chrono::DateTime<Utc>,
        index: Option<String>,
    ) -> Result<PreparedQuery, SplunkError> {
        let mut query = template.query.clone();

        // Replace placeholders in the format {PLACEHOLDER_NAME}
        for (key, value) in placeholders {
            let placeholder = format!("{{{key}}}");
            query = query.replace(&placeholder, value);
        }

        // Check for any remaining unfilled required placeholders
        if query.contains('{') && query.contains('}') {
            // Extract the first unfilled placeholder for error message
            if let Some(start) = query.find('{') {
                if let Some(end) = query[start..].find('}') {
                    let placeholder = &query[start + 1..start + end];
                    return Err(SplunkError::MissingPlaceholder(placeholder.to_string()));
                }
            }
        }

        Ok(PreparedQuery {
            template_id: Some(template.id),
            query,
            time_start,
            time_end,
            index,
        })
    }

    /// Extract placeholders from a query template.
    #[must_use]
    pub fn extract_placeholders(query: &str) -> Vec<String> {
        let mut placeholders = Vec::new();
        let mut in_placeholder = false;
        let mut current = String::new();

        for ch in query.chars() {
            if ch == '{' {
                in_placeholder = true;
                current.clear();
            } else if ch == '}' && in_placeholder {
                if !current.is_empty() {
                    placeholders.push(current.clone());
                }
                in_placeholder = false;
            } else if in_placeholder {
                current.push(ch);
            }
        }

        // Remove duplicates while preserving order
        let mut seen = std::collections::HashSet::new();
        placeholders.retain(|p| seen.insert(p.clone()));

        placeholders
    }

    /// Seed default system templates.
    #[instrument(skip(self))]
    pub async fn seed_default_templates(&self) -> Result<(), SplunkError> {
        let now = Utc::now();

        let default_templates = vec![
            (
                "Error Logs by Date Range",
                "Search for error logs within a time range",
                r"index=* level=ERROR earliest=-24h@h latest=now
| table _time, host, source, message
| sort -_time",
                TemplateCategory::Errors,
            ),
            (
                "Errors for Ticket",
                "Search for errors related to a specific ticket",
                r#"index=* level=ERROR "{TICKET_KEY}"
| table _time, host, source, message
| sort -_time"#,
                TemplateCategory::Errors,
            ),
            (
                "Logs by User ID",
                "Search for all logs for a specific user",
                r#"index=* user_id="{USER_ID}"
| table _time, level, host, source, message
| sort -_time"#,
                TemplateCategory::UserActivity,
            ),
            (
                "API Endpoint Performance",
                "Analyze response times for an API endpoint",
                r#"index=* sourcetype=access_combined "{ENDPOINT}"
| stats avg(response_time) as avg_time, max(response_time) as max_time, count by endpoint
| sort -count"#,
                TemplateCategory::Performance,
            ),
            (
                "Slow Requests",
                "Find requests that took longer than expected",
                r"index=* sourcetype=access_combined response_time>1000
| table _time, endpoint, response_time, status_code
| sort -response_time",
                TemplateCategory::Performance,
            ),
            (
                "Authentication Failures",
                "Search for failed authentication attempts",
                r#"index=* (level=ERROR OR level=WARN) (auth* OR login* OR "401" OR "403")
| table _time, host, user_id, source, message
| sort -_time"#,
                TemplateCategory::Security,
            ),
            (
                "Exception Stack Traces",
                "Find exceptions with stack traces",
                r#"index=* (exception OR stacktrace OR "Error:" OR "Exception:")
| table _time, host, source, message
| sort -_time"#,
                TemplateCategory::Errors,
            ),
        ];

        for (name, description, query, category) in default_templates {
            // Check if template already exists
            let existing: Option<(i64,)> = sqlx::query_as(
                "SELECT COUNT(*) FROM splunk_query_templates WHERE name = $1 AND is_system = true",
            )
            .bind(name)
            .fetch_optional(&self.pool)
            .await?;

            if existing.map_or(true, |(count,)| count == 0) {
                sqlx::query(
                    r"
                    INSERT INTO splunk_query_templates (id, name, description, query, category, is_system, created_at, updated_at)
                    VALUES ($1, $2, $3, $4, $5, true, $6, $6)
                    ON CONFLICT DO NOTHING
                    ",
                )
                .bind(Uuid::new_v4())
                .bind(name)
                .bind(description)
                .bind(query)
                .bind(category.to_string())
                .bind(now)
                .execute(&self.pool)
                .await?;

                info!(name = %name, "Seeded Splunk query template");
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_placeholders() {
        let query = r#"index=* "{TICKET_KEY}" user="{USER_ID}" endpoint="{ENDPOINT}""#;
        let placeholders = QueryTemplateService::extract_placeholders(query);

        assert_eq!(placeholders.len(), 3);
        assert!(placeholders.contains(&"TICKET_KEY".to_string()));
        assert!(placeholders.contains(&"USER_ID".to_string()));
        assert!(placeholders.contains(&"ENDPOINT".to_string()));
    }

    #[test]
    fn test_extract_placeholders_duplicates() {
        let query = r#"index=* "{TICKET_KEY}" OR "{TICKET_KEY}""#;
        let placeholders = QueryTemplateService::extract_placeholders(query);

        assert_eq!(placeholders.len(), 1);
        assert_eq!(placeholders[0], "TICKET_KEY");
    }

    #[test]
    fn test_extract_placeholders_empty() {
        let query = "index=* level=ERROR";
        let placeholders = QueryTemplateService::extract_placeholders(query);

        assert!(placeholders.is_empty());
    }
}
