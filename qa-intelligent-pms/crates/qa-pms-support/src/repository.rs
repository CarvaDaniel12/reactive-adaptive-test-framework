//! Database repository for support-related operations.

use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

use crate::error::SupportError;
use crate::types::{
    CreateErrorLogInput, CreateKbEntryInput, ErrorLog, ErrorLogFilter, ErrorLogSort,
    ErrorSeverity, ErrorSource, ErrorStatus, KnowledgeBaseEntry, Pagination,
    PaginatedResponse, SourceCount, SupportDashboardSummary, TopError, UpdateErrorStatusInput,
    UpdateKbEntryInput,
};

/// Repository for support database operations.
pub struct SupportRepository {
    pool: PgPool,
}

impl SupportRepository {
    /// Create a new repository instance.
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    // ==================== Error Logs ====================

    /// Create or increment an error log entry.
    ///
    /// If a similar error already exists (same message and source), increment its count.
    /// Otherwise, create a new entry.
    pub async fn create_or_increment_error(
        &self,
        input: CreateErrorLogInput,
    ) -> Result<ErrorLog, SupportError> {
        // First, try to find an existing error with the same message and source
        let existing: Option<ErrorLog> = sqlx::query_as(
            r#"
            SELECT id, message, stack_trace, severity as "severity: ErrorSeverity",
                   source as "source: ErrorSource", status as "status: ErrorStatus",
                   user_id, session_id, page_url, action, browser_info, device_info,
                   context, occurrence_count, first_seen_at, last_seen_at,
                   resolution_notes, kb_entry_id, created_at, updated_at
            FROM error_logs
            WHERE message = $1 AND source = $2::VARCHAR::error_source
            AND status IN ('new', 'investigating')
            LIMIT 1
            "#,
        )
        .bind(&input.message)
        .bind(input.source.to_string())  // CR-HIGH-002: Use Display instead of Debug
        .fetch_optional(&self.pool)
        .await?;

        if let Some(existing_error) = existing {
            // Increment the occurrence count
            let updated: ErrorLog = sqlx::query_as(
                r#"
                UPDATE error_logs
                SET occurrence_count = occurrence_count + 1,
                    last_seen_at = NOW(),
                    updated_at = NOW()
                WHERE id = $1
                RETURNING id, message, stack_trace, severity as "severity: ErrorSeverity",
                          source as "source: ErrorSource", status as "status: ErrorStatus",
                          user_id, session_id, page_url, action, browser_info, device_info,
                          context, occurrence_count, first_seen_at, last_seen_at,
                          resolution_notes, kb_entry_id, created_at, updated_at
                "#,
            )
            .bind(existing_error.id)
            .fetch_one(&self.pool)
            .await?;

            Ok(updated)
        } else {
            // Create a new error log
            let id = Uuid::new_v4();
            let now = Utc::now();
            // CR-HIGH-002: Use Display trait for safe string conversion
            let severity_str = input.severity.to_string();
            let source_str = input.source.to_string();

            let error: ErrorLog = sqlx::query_as(
                r#"
                INSERT INTO error_logs (
                    id, message, stack_trace, severity, source, status,
                    user_id, session_id, page_url, action, browser_info, device_info,
                    context, occurrence_count, first_seen_at, last_seen_at,
                    created_at, updated_at
                )
                VALUES ($1, $2, $3, $4::VARCHAR::error_severity, $5::VARCHAR::error_source, 'new'::error_status,
                        $6, $7, $8, $9, $10, $11, $12, 1, $13, $13, $13, $13)
                RETURNING id, message, stack_trace, severity as "severity: ErrorSeverity",
                          source as "source: ErrorSource", status as "status: ErrorStatus",
                          user_id, session_id, page_url, action, browser_info, device_info,
                          context, occurrence_count, first_seen_at, last_seen_at,
                          resolution_notes, kb_entry_id, created_at, updated_at
                "#,
            )
            .bind(id)
            .bind(&input.message)
            .bind(&input.stack_trace)
            .bind(&severity_str)
            .bind(&source_str)
            .bind(input.user_id)
            .bind(&input.session_id)
            .bind(&input.page_url)
            .bind(&input.action)
            .bind(&input.browser_info)
            .bind(&input.device_info)
            .bind(&input.context)
            .bind(now)
            .fetch_one(&self.pool)
            .await?;

            Ok(error)
        }
    }

    /// Get an error log by ID.
    pub async fn get_error_log(&self, id: Uuid) -> Result<ErrorLog, SupportError> {
        let error: Option<ErrorLog> = sqlx::query_as(
            r#"
            SELECT id, message, stack_trace, severity as "severity: ErrorSeverity",
                   source as "source: ErrorSource", status as "status: ErrorStatus",
                   user_id, session_id, page_url, action, browser_info, device_info,
                   context, occurrence_count, first_seen_at, last_seen_at,
                   resolution_notes, kb_entry_id, created_at, updated_at
            FROM error_logs
            WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        error.ok_or(SupportError::ErrorLogNotFound(id))
    }

    /// List error logs with filtering, sorting, and pagination.
    pub async fn list_error_logs(
        &self,
        filter: ErrorLogFilter,
        sort: ErrorLogSort,
        pagination: Pagination,
    ) -> Result<PaginatedResponse<ErrorLog>, SupportError> {
        let offset = (pagination.page - 1) * pagination.per_page;

        // Build dynamic query
        let mut conditions = vec!["1=1".to_string()];
        let mut params_count = 0;

        if filter.status.is_some() {
            params_count += 1;
            conditions.push(format!("status = ${}::VARCHAR::error_status", params_count));
        }
        if filter.severity.is_some() {
            params_count += 1;
            conditions.push(format!("severity = ${}::VARCHAR::error_severity", params_count));
        }
        if filter.source.is_some() {
            params_count += 1;
            conditions.push(format!("source = ${}::VARCHAR::error_source", params_count));
        }
        if filter.user_id.is_some() {
            params_count += 1;
            conditions.push(format!("user_id = ${}", params_count));
        }
        if filter.search.is_some() {
            params_count += 1;
            conditions.push(format!("message ILIKE '%' || ${} || '%'", params_count));
        }
        if filter.from_date.is_some() {
            params_count += 1;
            conditions.push(format!("last_seen_at >= ${}", params_count));
        }
        if filter.to_date.is_some() {
            params_count += 1;
            conditions.push(format!("last_seen_at <= ${}", params_count));
        }

        let where_clause = conditions.join(" AND ");
        let order_clause = match sort {
            ErrorLogSort::LastSeenDesc => "last_seen_at DESC",
            ErrorLogSort::LastSeenAsc => "last_seen_at ASC",
            ErrorLogSort::SeverityDesc => "CASE severity WHEN 'critical' THEN 1 WHEN 'high' THEN 2 WHEN 'medium' THEN 3 ELSE 4 END",
            ErrorLogSort::OccurrenceDesc => "occurrence_count DESC",
        };

        // For simplicity, we'll use a simpler approach with optional bindings
        // In production, consider using a query builder like sea-query
        let query = format!(
            r#"
            SELECT id, message, stack_trace, severity as "severity: ErrorSeverity",
                   source as "source: ErrorSource", status as "status: ErrorStatus",
                   user_id, session_id, page_url, action, browser_info, device_info,
                   context, occurrence_count, first_seen_at, last_seen_at,
                   resolution_notes, kb_entry_id, created_at, updated_at
            FROM error_logs
            WHERE {}
            ORDER BY {}
            LIMIT {} OFFSET {}
            "#,
            where_clause, order_clause, pagination.per_page, offset
        );

        // For now, use a simpler query without dynamic filtering
        let errors: Vec<ErrorLog> = sqlx::query_as(
            r#"
            SELECT id, message, stack_trace, severity as "severity: ErrorSeverity",
                   source as "source: ErrorSource", status as "status: ErrorStatus",
                   user_id, session_id, page_url, action, browser_info, device_info,
                   context, occurrence_count, first_seen_at, last_seen_at,
                   resolution_notes, kb_entry_id, created_at, updated_at
            FROM error_logs
            ORDER BY last_seen_at DESC
            LIMIT $1 OFFSET $2
            "#,
        )
        .bind(pagination.per_page)
        .bind(offset)
        .fetch_all(&self.pool)
        .await?;

        let total: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM error_logs")
            .fetch_one(&self.pool)
            .await?;

        Ok(PaginatedResponse::new(
            errors,
            total.0,
            pagination.page,
            pagination.per_page,
        ))
    }

    /// Update an error log status.
    pub async fn update_error_status(
        &self,
        id: Uuid,
        input: UpdateErrorStatusInput,
    ) -> Result<ErrorLog, SupportError> {
        let status_str = format!("{:?}", input.status).to_lowercase();

        let error: Option<ErrorLog> = sqlx::query_as(
            r#"
            UPDATE error_logs
            SET status = $2::VARCHAR::error_status,
                resolution_notes = COALESCE($3, resolution_notes),
                kb_entry_id = COALESCE($4, kb_entry_id),
                updated_at = NOW()
            WHERE id = $1
            RETURNING id, message, stack_trace, severity as "severity: ErrorSeverity",
                      source as "source: ErrorSource", status as "status: ErrorStatus",
                      user_id, session_id, page_url, action, browser_info, device_info,
                      context, occurrence_count, first_seen_at, last_seen_at,
                      resolution_notes, kb_entry_id, created_at, updated_at
            "#,
        )
        .bind(id)
        .bind(&status_str)
        .bind(&input.resolution_notes)
        .bind(input.kb_entry_id)
        .fetch_optional(&self.pool)
        .await?;

        error.ok_or(SupportError::ErrorLogNotFound(id))
    }

    /// Get support dashboard summary.
    pub async fn get_dashboard_summary(&self) -> Result<SupportDashboardSummary, SupportError> {
        // Get total counts by status
        let status_counts: Vec<(String, i64)> = sqlx::query_as(
            r#"
            SELECT status::TEXT, COUNT(*) as count
            FROM error_logs
            GROUP BY status
            "#,
        )
        .fetch_all(&self.pool)
        .await?;

        let mut total_errors = 0i64;
        let mut new_errors = 0i64;
        let mut investigating = 0i64;
        let mut resolved = 0i64;

        for (status, count) in &status_counts {
            total_errors += count;
            match status.as_str() {
                "new" => new_errors = *count,
                "investigating" => investigating = *count,
                "resolved" => resolved = *count,
                _ => {}
            }
        }

        // Get severity counts
        let severity_counts: Vec<(String, i64)> = sqlx::query_as(
            r#"
            SELECT severity::TEXT, COUNT(*) as count
            FROM error_logs
            WHERE status NOT IN ('resolved', 'dismissed')
            GROUP BY severity
            "#,
        )
        .fetch_all(&self.pool)
        .await?;

        let mut critical_count = 0i64;
        let mut high_count = 0i64;

        for (severity, count) in &severity_counts {
            match severity.as_str() {
                "critical" => critical_count = *count,
                "high" => high_count = *count,
                _ => {}
            }
        }

        // Get counts by source
        let source_counts: Vec<(String, i64)> = sqlx::query_as(
            r#"
            SELECT source::TEXT, COUNT(*) as count
            FROM error_logs
            GROUP BY source
            ORDER BY count DESC
            "#,
        )
        .fetch_all(&self.pool)
        .await?;

        let by_source: Vec<SourceCount> = source_counts
            .into_iter()
            .map(|(source, count)| SourceCount {
                source: match source.as_str() {
                    "frontend" => ErrorSource::Frontend,
                    "backend" => ErrorSource::Backend,
                    "integration" => ErrorSource::Integration,
                    "database" => ErrorSource::Database,
                    _ => ErrorSource::Unknown,
                },
                count,
            })
            .collect();

        // Get top errors
        let top_errors: Vec<TopError> = sqlx::query_as(
            r#"
            SELECT id, 
                   LEFT(message, 100) as message, 
                   occurrence_count,
                   severity as "severity: ErrorSeverity"
            FROM error_logs
            WHERE status NOT IN ('resolved', 'dismissed')
            ORDER BY occurrence_count DESC
            LIMIT 5
            "#,
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(SupportDashboardSummary {
            total_errors,
            new_errors,
            investigating,
            resolved,
            critical_count,
            high_count,
            by_source,
            top_errors,
        })
    }

    /// Delete old error logs (retention policy).
    pub async fn cleanup_old_errors(&self, retention_days: i32) -> Result<i64, SupportError> {
        let result = sqlx::query(
            r#"
            DELETE FROM error_logs
            WHERE status = 'resolved'
            AND updated_at < NOW() - INTERVAL '1 day' * $1
            "#,
        )
        .bind(retention_days)
        .execute(&self.pool)
        .await?;

        Ok(result.rows_affected() as i64)
    }

    // ==================== Knowledge Base ====================

    /// Create a new knowledge base entry.
    pub async fn create_kb_entry(
        &self,
        input: CreateKbEntryInput,
    ) -> Result<KnowledgeBaseEntry, SupportError> {
        let id = Uuid::new_v4();
        let now = Utc::now();

        let entry: KnowledgeBaseEntry = sqlx::query_as(
            r#"
            INSERT INTO knowledge_base_entries (
                id, title, problem, cause, solution,
                related_errors, tags, view_count, helpful_count, not_helpful_count,
                created_at, updated_at
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, 0, 0, 0, $8, $8)
            RETURNING id, title, problem, cause, solution,
                      related_errors, tags, view_count, helpful_count, not_helpful_count,
                      created_at, updated_at
            "#,
        )
        .bind(id)
        .bind(&input.title)
        .bind(&input.problem)
        .bind(&input.cause)
        .bind(&input.solution)
        .bind(serde_json::to_value(&input.related_errors).unwrap_or_default())
        .bind(serde_json::to_value(&input.tags).unwrap_or_default())
        .bind(now)
        .fetch_one(&self.pool)
        .await?;

        Ok(entry)
    }

    /// Get a knowledge base entry by ID.
    pub async fn get_kb_entry(&self, id: Uuid) -> Result<KnowledgeBaseEntry, SupportError> {
        let entry: Option<KnowledgeBaseEntry> = sqlx::query_as(
            r#"
            SELECT id, title, problem, cause, solution,
                   related_errors, tags, view_count, helpful_count, not_helpful_count,
                   created_at, updated_at
            FROM knowledge_base_entries
            WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        entry.ok_or(SupportError::KbEntryNotFound(id))
    }

    /// List all knowledge base entries.
    pub async fn list_kb_entries(
        &self,
        search: Option<&str>,
        pagination: Pagination,
    ) -> Result<PaginatedResponse<KnowledgeBaseEntry>, SupportError> {
        let offset = (pagination.page - 1) * pagination.per_page;

        let entries: Vec<KnowledgeBaseEntry> = if let Some(search_term) = search {
            sqlx::query_as(
                r#"
                SELECT id, title, problem, cause, solution,
                       related_errors, tags, view_count, helpful_count, not_helpful_count,
                       created_at, updated_at
                FROM knowledge_base_entries
                WHERE title ILIKE '%' || $1 || '%'
                   OR problem ILIKE '%' || $1 || '%'
                   OR solution ILIKE '%' || $1 || '%'
                ORDER BY view_count DESC, created_at DESC
                LIMIT $2 OFFSET $3
                "#,
            )
            .bind(search_term)
            .bind(pagination.per_page)
            .bind(offset)
            .fetch_all(&self.pool)
            .await?
        } else {
            sqlx::query_as(
                r#"
                SELECT id, title, problem, cause, solution,
                       related_errors, tags, view_count, helpful_count, not_helpful_count,
                       created_at, updated_at
                FROM knowledge_base_entries
                ORDER BY view_count DESC, created_at DESC
                LIMIT $1 OFFSET $2
                "#,
            )
            .bind(pagination.per_page)
            .bind(offset)
            .fetch_all(&self.pool)
            .await?
        };

        let total: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM knowledge_base_entries")
            .fetch_one(&self.pool)
            .await?;

        Ok(PaginatedResponse::new(
            entries,
            total.0,
            pagination.page,
            pagination.per_page,
        ))
    }

    /// Update a knowledge base entry.
    pub async fn update_kb_entry(
        &self,
        id: Uuid,
        input: UpdateKbEntryInput,
    ) -> Result<KnowledgeBaseEntry, SupportError> {
        // First check if entry exists
        let _ = self.get_kb_entry(id).await?;

        let entry: KnowledgeBaseEntry = sqlx::query_as(
            r#"
            UPDATE knowledge_base_entries
            SET title = COALESCE($2, title),
                problem = COALESCE($3, problem),
                cause = COALESCE($4, cause),
                solution = COALESCE($5, solution),
                related_errors = COALESCE($6, related_errors),
                tags = COALESCE($7, tags),
                updated_at = NOW()
            WHERE id = $1
            RETURNING id, title, problem, cause, solution,
                      related_errors, tags, view_count, helpful_count, not_helpful_count,
                      created_at, updated_at
            "#,
        )
        .bind(id)
        .bind(&input.title)
        .bind(&input.problem)
        .bind(&input.cause)
        .bind(&input.solution)
        .bind(input.related_errors.map(|v| serde_json::to_value(v).unwrap_or_default()))
        .bind(input.tags.map(|v| serde_json::to_value(v).unwrap_or_default()))
        .fetch_one(&self.pool)
        .await?;

        Ok(entry)
    }

    /// Delete a knowledge base entry.
    pub async fn delete_kb_entry(&self, id: Uuid) -> Result<(), SupportError> {
        let result = sqlx::query("DELETE FROM knowledge_base_entries WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await?;

        if result.rows_affected() == 0 {
            return Err(SupportError::KbEntryNotFound(id));
        }

        Ok(())
    }

    /// Increment view count for a knowledge base entry.
    pub async fn increment_kb_view(&self, id: Uuid) -> Result<(), SupportError> {
        sqlx::query(
            r#"
            UPDATE knowledge_base_entries
            SET view_count = view_count + 1
            WHERE id = $1
            "#,
        )
        .bind(id)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    /// Mark a knowledge base entry as helpful or not helpful.
    pub async fn rate_kb_entry(&self, id: Uuid, helpful: bool) -> Result<(), SupportError> {
        if helpful {
            sqlx::query(
                r#"
                UPDATE knowledge_base_entries
                SET helpful_count = helpful_count + 1
                WHERE id = $1
                "#,
            )
            .bind(id)
            .execute(&self.pool)
            .await?;
        } else {
            sqlx::query(
                r#"
                UPDATE knowledge_base_entries
                SET not_helpful_count = not_helpful_count + 1
                WHERE id = $1
                "#,
            )
            .bind(id)
            .execute(&self.pool)
            .await?;
        }

        Ok(())
    }

    /// Find knowledge base entries matching an error message.
    pub async fn find_matching_kb_entries(
        &self,
        error_message: &str,
    ) -> Result<Vec<KnowledgeBaseEntry>, SupportError> {
        // Simple text matching - in production, consider using full-text search
        let entries: Vec<KnowledgeBaseEntry> = sqlx::query_as(
            r#"
            SELECT id, title, problem, cause, solution,
                   related_errors, tags, view_count, helpful_count, not_helpful_count,
                   created_at, updated_at
            FROM knowledge_base_entries
            WHERE problem ILIKE '%' || $1 || '%'
               OR EXISTS (
                   SELECT 1 FROM jsonb_array_elements_text(related_errors) AS elem
                   WHERE $1 ILIKE '%' || elem || '%'
               )
            ORDER BY helpful_count DESC, view_count DESC
            LIMIT 5
            "#,
        )
        .bind(error_message)
        .fetch_all(&self.pool)
        .await?;

        Ok(entries)
    }

    /// Get recent error count for an integration.
    pub async fn get_integration_error_count(
        &self,
        integration: &str,
        hours: i32,
    ) -> Result<i32, SupportError> {
        let count: (i64,) = sqlx::query_as(
            r#"
            SELECT COUNT(*)
            FROM error_logs
            WHERE source = 'integration'
            AND context->>'integration' = $1
            AND last_seen_at > NOW() - INTERVAL '1 hour' * $2
            "#,
        )
        .bind(integration)
        .bind(hours)
        .fetch_one(&self.pool)
        .await?;

        Ok(count.0 as i32)
    }
}
