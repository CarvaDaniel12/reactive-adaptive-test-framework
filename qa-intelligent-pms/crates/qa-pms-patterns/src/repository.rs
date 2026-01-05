//! Pattern repository for database operations.

use chrono::{DateTime, Utc};
use sqlx::PgPool;
use uuid::Uuid;

use crate::types::{NewPattern, DetectedPattern, PatternType, NewAlert, Alert, Severity};

/// Repository for pattern and alert data.
pub struct PatternRepository {
    pool: PgPool,
}

impl PatternRepository {
    /// Create a new repository.
    pub const fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// Create a new detected pattern.
    pub async fn create_pattern(&self, pattern: NewPattern) -> anyhow::Result<DetectedPattern> {
        let id = Uuid::new_v4();
        let now = Utc::now();

        sqlx::query(
            r"
            INSERT INTO detected_patterns (
                id, pattern_type, severity, title, description,
                affected_tickets, common_factor, average_excess_percent,
                confidence_score, suggested_actions, metadata, detected_at, created_at
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13)
            ",
        )
        .bind(id)
        .bind(pattern.pattern_type.to_string())
        .bind(pattern.severity.to_string())
        .bind(&pattern.title)
        .bind(&pattern.description)
        .bind(&pattern.affected_tickets)
        .bind(&pattern.common_factor)
        .bind(pattern.average_excess_percent)
        .bind(pattern.confidence_score)
        .bind(&pattern.suggested_actions)
        .bind(&pattern.metadata)
        .bind(now)
        .bind(now)
        .execute(&self.pool)
        .await?;

        Ok(DetectedPattern {
            id,
            pattern_type: pattern.pattern_type,
            severity: pattern.severity,
            title: pattern.title,
            description: pattern.description,
            affected_tickets: pattern.affected_tickets,
            common_factor: pattern.common_factor,
            average_excess_percent: pattern.average_excess_percent,
            confidence_score: pattern.confidence_score,
            suggested_actions: pattern.suggested_actions,
            metadata: pattern.metadata,
            detected_at: now,
            created_at: now,
        })
    }

    /// Get patterns by type.
    pub async fn get_patterns_by_type(
        &self,
        pattern_type: PatternType,
        limit: i32,
    ) -> anyhow::Result<Vec<DetectedPattern>> {
        let rows: Vec<PatternRow> = sqlx::query_as(
            r"
            SELECT 
                id, pattern_type, severity, title, description,
                affected_tickets, common_factor, average_excess_percent,
                confidence_score, suggested_actions, metadata, detected_at, created_at
            FROM detected_patterns
            WHERE pattern_type = $1
            ORDER BY detected_at DESC
            LIMIT $2
            ",
        )
        .bind(pattern_type.to_string())
        .bind(limit)
        .fetch_all(&self.pool)
        .await?;

        Ok(rows.into_iter().map(Into::into).collect())
    }

    /// Get recent patterns.
    pub async fn get_recent_patterns(&self, limit: i32) -> anyhow::Result<Vec<DetectedPattern>> {
        let rows: Vec<PatternRow> = sqlx::query_as(
            r"
            SELECT 
                id, pattern_type, severity, title, description,
                affected_tickets, common_factor, average_excess_percent,
                confidence_score, suggested_actions, metadata, detected_at, created_at
            FROM detected_patterns
            ORDER BY detected_at DESC
            LIMIT $1
            ",
        )
        .bind(limit)
        .fetch_all(&self.pool)
        .await?;

        Ok(rows.into_iter().map(Into::into).collect())
    }

    /// Get pattern by ID.
    pub async fn get_pattern(&self, id: Uuid) -> anyhow::Result<Option<DetectedPattern>> {
        let row: Option<PatternRow> = sqlx::query_as(
            r"
            SELECT 
                id, pattern_type, severity, title, description,
                affected_tickets, common_factor, average_excess_percent,
                confidence_score, suggested_actions, metadata, detected_at, created_at
            FROM detected_patterns
            WHERE id = $1
            ",
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(Into::into))
    }

    /// Create an alert.
    pub async fn create_alert(&self, alert: NewAlert) -> anyhow::Result<Alert> {
        let id = Uuid::new_v4();
        let now = Utc::now();

        sqlx::query(
            r"
            INSERT INTO alerts (
                id, pattern_id, alert_type, severity, title, message,
                affected_tickets, suggested_actions, created_at
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
            ",
        )
        .bind(id)
        .bind(alert.pattern_id)
        .bind(alert.alert_type.to_string())
        .bind(alert.severity.to_string())
        .bind(&alert.title)
        .bind(&alert.message)
        .bind(&alert.affected_tickets)
        .bind(&alert.suggested_actions)
        .bind(now)
        .execute(&self.pool)
        .await?;

        Ok(Alert {
            id,
            pattern_id: alert.pattern_id,
            alert_type: alert.alert_type,
            severity: alert.severity,
            title: alert.title,
            message: alert.message,
            affected_tickets: alert.affected_tickets,
            suggested_actions: alert.suggested_actions,
            is_read: false,
            is_dismissed: false,
            dismissed_at: None,
            dismissed_by: None,
            created_at: now,
        })
    }

    /// Get unread alerts.
    pub async fn get_unread_alerts(&self) -> anyhow::Result<Vec<Alert>> {
        let rows: Vec<AlertRow> = sqlx::query_as(
            r"
            SELECT 
                id, pattern_id, alert_type, severity, title, message,
                affected_tickets, suggested_actions, is_read, is_dismissed,
                dismissed_at, dismissed_by, created_at
            FROM alerts
            WHERE NOT is_read AND NOT is_dismissed
            ORDER BY 
                CASE severity 
                    WHEN 'critical' THEN 1 
                    WHEN 'warning' THEN 2 
                    ELSE 3 
                END,
                created_at DESC
            ",
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(rows.into_iter().map(Into::into).collect())
    }

    /// Get unread alert count.
    pub async fn get_unread_count(&self) -> anyhow::Result<i64> {
        let (count,): (i64,) = sqlx::query_as(
            r"
            SELECT COUNT(*) FROM alerts
            WHERE NOT is_read AND NOT is_dismissed
            ",
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(count)
    }

    /// Mark alert as read.
    pub async fn mark_alert_read(&self, id: Uuid) -> anyhow::Result<()> {
        sqlx::query("UPDATE alerts SET is_read = TRUE WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    /// Dismiss an alert.
    pub async fn dismiss_alert(&self, id: Uuid, dismissed_by: Option<&str>) -> anyhow::Result<()> {
        sqlx::query(
            r"
            UPDATE alerts 
            SET is_dismissed = TRUE, dismissed_at = NOW(), dismissed_by = $2
            WHERE id = $1
            ",
        )
        .bind(id)
        .bind(dismissed_by)
        .execute(&self.pool)
        .await?;
        Ok(())
    }
}

// Internal row types for sqlx
#[derive(sqlx::FromRow)]
struct PatternRow {
    id: Uuid,
    pattern_type: String,
    severity: String,
    title: String,
    description: Option<String>,
    affected_tickets: Vec<String>,
    common_factor: Option<String>,
    average_excess_percent: Option<f64>,
    confidence_score: f64,
    suggested_actions: Vec<String>,
    metadata: serde_json::Value,
    detected_at: DateTime<Utc>,
    created_at: DateTime<Utc>,
}

impl From<PatternRow> for DetectedPattern {
    fn from(row: PatternRow) -> Self {
        Self {
            id: row.id,
            pattern_type: match row.pattern_type.as_str() {
                "time_excess" => PatternType::TimeExcess,
                "consecutive_problem" => PatternType::ConsecutiveProblem,
                "spike" => PatternType::Spike,
                _ => PatternType::TimeExcess,
            },
            severity: match row.severity.as_str() {
                "critical" => Severity::Critical,
                "warning" => Severity::Warning,
                _ => Severity::Info,
            },
            title: row.title,
            description: row.description,
            affected_tickets: row.affected_tickets,
            common_factor: row.common_factor,
            average_excess_percent: row.average_excess_percent,
            confidence_score: row.confidence_score,
            suggested_actions: row.suggested_actions,
            metadata: row.metadata,
            detected_at: row.detected_at,
            created_at: row.created_at,
        }
    }
}

#[derive(sqlx::FromRow)]
struct AlertRow {
    id: Uuid,
    pattern_id: Option<Uuid>,
    alert_type: String,
    severity: String,
    title: String,
    message: Option<String>,
    affected_tickets: Vec<String>,
    suggested_actions: Vec<String>,
    is_read: bool,
    is_dismissed: bool,
    dismissed_at: Option<DateTime<Utc>>,
    dismissed_by: Option<String>,
    created_at: DateTime<Utc>,
}

impl From<AlertRow> for Alert {
    fn from(row: AlertRow) -> Self {
        Self {
            id: row.id,
            pattern_id: row.pattern_id,
            alert_type: match row.alert_type.as_str() {
                "time_excess" => PatternType::TimeExcess,
                "consecutive_problem" => PatternType::ConsecutiveProblem,
                "spike" => PatternType::Spike,
                _ => PatternType::TimeExcess,
            },
            severity: match row.severity.as_str() {
                "critical" => Severity::Critical,
                "warning" => Severity::Warning,
                _ => Severity::Info,
            },
            title: row.title,
            message: row.message,
            affected_tickets: row.affected_tickets,
            suggested_actions: row.suggested_actions,
            is_read: row.is_read,
            is_dismissed: row.is_dismissed,
            dismissed_at: row.dismissed_at,
            dismissed_by: row.dismissed_by,
            created_at: row.created_at,
        }
    }
}
