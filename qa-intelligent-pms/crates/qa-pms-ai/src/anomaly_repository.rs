//! Anomaly repository for database operations.

use chrono::{DateTime, Utc};
use sqlx::PgPool;
use uuid::Uuid;

use crate::anomaly_detector::{Anomaly, AnomalyMetrics, AnomalySeverity, AnomalyType, WorkflowExecution};

/// Repository for anomaly data.
pub struct AnomalyRepository {
    pool: PgPool,
}

impl AnomalyRepository {
    /// Create a new repository.
    pub const fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// Store a detected anomaly.
    pub async fn create_anomaly(&self, anomaly: Anomaly) -> anyhow::Result<Anomaly> {
        let workflow_instance_id: Option<Uuid> = anomaly
            .affected_entities
            .iter()
            .find_map(|e| Uuid::parse_str(e).ok());

        sqlx::query(
            r"
            INSERT INTO anomalies (
                id, anomaly_type, severity, description, metrics,
                affected_entities, investigation_steps, workflow_instance_id,
                detected_at, created_at
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
            ",
        )
        .bind(anomaly.id)
        .bind(anomaly.anomaly_type.as_str())
        .bind(anomaly.severity.as_str())
        .bind(&anomaly.description)
        .bind(serde_json::to_value(&anomaly.metrics)?)
        .bind(&anomaly.affected_entities)
        .bind(&anomaly.investigation_steps)
        .bind(workflow_instance_id)
        .bind(anomaly.detected_at)
        .bind(anomaly.detected_at)
        .execute(&self.pool)
        .await?;

        Ok(anomaly)
    }

    /// Get anomaly by ID.
    pub async fn get_anomaly(&self, id: Uuid) -> anyhow::Result<Option<Anomaly>> {
        let row: Option<AnomalyRow> = sqlx::query_as(
            r"
            SELECT 
                id, anomaly_type, severity, description, metrics,
                affected_entities, investigation_steps, workflow_instance_id,
                detected_at, created_at
            FROM anomalies
            WHERE id = $1
            ",
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(|r| r.into()))
    }

    /// Get anomalies by date range.
    pub async fn get_by_date_range(
        &self,
        start: DateTime<Utc>,
        end: DateTime<Utc>,
    ) -> anyhow::Result<Vec<Anomaly>> {
        let rows: Vec<AnomalyRow> = sqlx::query_as(
            r"
            SELECT 
                id, anomaly_type, severity, description, metrics,
                affected_entities, investigation_steps, workflow_instance_id,
                detected_at, created_at
            FROM anomalies
            WHERE detected_at >= $1 AND detected_at <= $2
            ORDER BY detected_at DESC
            ",
        )
        .bind(start)
        .bind(end)
        .fetch_all(&self.pool)
        .await?;

        Ok(rows.into_iter().map(Into::into).collect())
    }

    /// Get anomalies by type.
    pub async fn get_by_type(
        &self,
        anomaly_type: AnomalyType,
        limit: i32,
    ) -> anyhow::Result<Vec<Anomaly>> {
        let rows: Vec<AnomalyRow> = sqlx::query_as(
            r"
            SELECT 
                id, anomaly_type, severity, description, metrics,
                affected_entities, investigation_steps, workflow_instance_id,
                detected_at, created_at
            FROM anomalies
            WHERE anomaly_type = $1
            ORDER BY detected_at DESC
            LIMIT $2
            ",
        )
        .bind(anomaly_type.as_str())
        .bind(limit)
        .fetch_all(&self.pool)
        .await?;

        Ok(rows.into_iter().map(Into::into).collect())
    }

    /// Get anomalies by severity.
    pub async fn get_by_severity(
        &self,
        severity: AnomalySeverity,
        limit: i32,
    ) -> anyhow::Result<Vec<Anomaly>> {
        let rows: Vec<AnomalyRow> = sqlx::query_as(
            r"
            SELECT 
                id, anomaly_type, severity, description, metrics,
                affected_entities, investigation_steps, workflow_instance_id,
                detected_at, created_at
            FROM anomalies
            WHERE severity = $1
            ORDER BY detected_at DESC
            LIMIT $2
            ",
        )
        .bind(severity.as_str())
        .bind(limit)
        .fetch_all(&self.pool)
        .await?;

        Ok(rows.into_iter().map(Into::into).collect())
    }

    /// Get recent anomalies.
    pub async fn get_recent(&self, limit: i32) -> anyhow::Result<Vec<Anomaly>> {
        let rows: Vec<AnomalyRow> = sqlx::query_as(
            r"
            SELECT 
                id, anomaly_type, severity, description, metrics,
                affected_entities, investigation_steps, workflow_instance_id,
                detected_at, created_at
            FROM anomalies
            ORDER BY detected_at DESC
            LIMIT $1
            ",
        )
        .bind(limit)
        .fetch_all(&self.pool)
        .await?;

        Ok(rows.into_iter().map(Into::into).collect())
    }

    /// Get anomaly count by date (for trend analysis).
    pub async fn get_count_by_date(
        &self,
        start: DateTime<Utc>,
        end: DateTime<Utc>,
    ) -> anyhow::Result<Vec<AnomalyCountByDate>> {
        #[derive(sqlx::FromRow)]
        struct CountRow {
            date: chrono::NaiveDate,
            count: i64,
        }

        let rows: Vec<CountRow> = sqlx::query_as(
            r"
            SELECT DATE(detected_at) as date, COUNT(*)::bigint as count
            FROM anomalies
            WHERE detected_at >= $1 AND detected_at <= $2
            GROUP BY DATE(detected_at)
            ORDER BY date ASC
            ",
        )
        .bind(start)
        .bind(end)
        .fetch_all(&self.pool)
        .await?;

        Ok(rows
            .into_iter()
            .map(|row| AnomalyCountByDate {
                date: row
                    .date
                    .and_hms_opt(0, 0, 0)
                    .unwrap()
                    .and_utc()
                    .into(),
                count: row.count,
            })
            .collect())
    }

    /// Get severity distribution (for trend analysis).
    pub async fn get_severity_distribution(
        &self,
        start: DateTime<Utc>,
        end: DateTime<Utc>,
    ) -> anyhow::Result<Vec<SeverityDistribution>> {
        let rows: Vec<(String, i64)> = sqlx::query_as(
            r"
            SELECT severity, COUNT(*) as count
            FROM anomalies
            WHERE detected_at >= $1 AND detected_at <= $2
            GROUP BY severity
            ",
        )
        .bind(start)
        .bind(end)
        .fetch_all(&self.pool)
        .await?;

        Ok(rows
            .into_iter()
            .map(|(severity, count)| SeverityDistribution {
                severity: AnomalySeverity::from_str(&severity),
                count,
            })
            .collect())
    }

    /// Get historical workflow executions for baseline calculation.
    /// 
    /// Returns the last N completed workflow executions with their execution times
    /// and success status, ordered by completion time descending.
    pub async fn get_historical_executions(
        &self,
        limit: i32,
        template_id: Option<Uuid>,
    ) -> anyhow::Result<Vec<WorkflowExecutionData>> {
        let rows = if let Some(template) = template_id {
            sqlx::query_as::<_, HistoricalExecutionRow>(
                r"
                SELECT 
                    wi.id as instance_id,
                    wi.ticket_id,
                    wi.user_id,
                    wi.template_id,
                    COALESCE(SUM(ts.total_seconds), 0) as execution_time_seconds,
                    CASE WHEN wi.status = 'completed' THEN true ELSE false END as succeeded,
                    COALESCE(wi.completed_at, wi.updated_at) as completed_at
                FROM workflow_instances wi
                LEFT JOIN time_sessions ts ON ts.workflow_instance_id = wi.id
                WHERE wi.status = 'completed'
                  AND wi.template_id = $1
                  AND wi.completed_at IS NOT NULL
                GROUP BY wi.id, wi.ticket_id, wi.user_id, wi.template_id, wi.status, wi.completed_at, wi.updated_at
                ORDER BY COALESCE(wi.completed_at, wi.updated_at) DESC
                LIMIT $2
                ",
            )
            .bind(template)
            .bind(limit)
            .fetch_all(&self.pool)
            .await?
        } else {
            sqlx::query_as::<_, HistoricalExecutionRow>(
                r"
                SELECT 
                    wi.id as instance_id,
                    wi.ticket_id,
                    wi.user_id,
                    wi.template_id,
                    COALESCE(SUM(ts.total_seconds), 0) as execution_time_seconds,
                    CASE WHEN wi.status = 'completed' THEN true ELSE false END as succeeded,
                    COALESCE(wi.completed_at, wi.updated_at) as completed_at
                FROM workflow_instances wi
                LEFT JOIN time_sessions ts ON ts.workflow_instance_id = wi.id
                WHERE wi.status = 'completed'
                  AND wi.completed_at IS NOT NULL
                GROUP BY wi.id, wi.ticket_id, wi.user_id, wi.template_id, wi.status, wi.completed_at, wi.updated_at
                ORDER BY COALESCE(wi.completed_at, wi.updated_at) DESC
                LIMIT $1
                ",
            )
            .bind(limit)
            .fetch_all(&self.pool)
            .await?
        };

        Ok(rows.into_iter().map(Into::into).collect())
    }
}

/// Historical execution data for baseline calculation.
#[derive(Debug, Clone)]
pub struct WorkflowExecutionData {
    /// Workflow instance ID
    pub instance_id: Uuid,
    /// Ticket ID
    pub ticket_id: String,
    /// User ID
    pub user_id: String,
    /// Template ID
    pub template_id: Uuid,
    /// Execution time in seconds
    pub execution_time_seconds: i32,
    /// Whether the workflow succeeded
    pub succeeded: bool,
    /// When execution completed
    pub completed_at: DateTime<Utc>,
}

// Internal row type for historical execution queries
#[derive(sqlx::FromRow)]
struct HistoricalExecutionRow {
    instance_id: Uuid,
    ticket_id: String,
    user_id: String,
    template_id: Uuid,
    execution_time_seconds: Option<i64>, // SUM returns bigint
    succeeded: bool,
    completed_at: DateTime<Utc>,
}

impl From<HistoricalExecutionRow> for WorkflowExecutionData {
    fn from(row: HistoricalExecutionRow) -> Self {
        Self {
            instance_id: row.instance_id,
            ticket_id: row.ticket_id,
            user_id: row.user_id,
            template_id: row.template_id,
            execution_time_seconds: row.execution_time_seconds.unwrap_or(0) as i32,
            succeeded: row.succeeded,
            completed_at: row.completed_at,
        }
    }
}

impl From<WorkflowExecutionData> for WorkflowExecution {
    fn from(data: WorkflowExecutionData) -> Self {
        Self {
            instance_id: data.instance_id,
            ticket_id: data.ticket_id,
            user_id: data.user_id,
            template_id: data.template_id,
            execution_time_seconds: data.execution_time_seconds,
            succeeded: data.succeeded,
            completed_at: data.completed_at,
        }
    }
}

/// Anomaly count by date for trend analysis.
#[derive(Debug, Clone)]
pub struct AnomalyCountByDate {
    /// Date
    pub date: DateTime<Utc>,
    /// Count of anomalies on this date
    pub count: i64,
}

/// Severity distribution for trend analysis.
#[derive(Debug, Clone)]
pub struct SeverityDistribution {
    /// Severity level
    pub severity: AnomalySeverity,
    /// Count of anomalies with this severity
    pub count: i64,
}

// Internal row type for sqlx
#[derive(sqlx::FromRow)]
struct AnomalyRow {
    id: Uuid,
    anomaly_type: String,
    severity: String,
    description: String,
    metrics: sqlx::types::Json<serde_json::Value>,
    affected_entities: Vec<String>,
    investigation_steps: Vec<String>,
    workflow_instance_id: Option<Uuid>,
    detected_at: DateTime<Utc>,
    created_at: DateTime<Utc>,
}

impl From<AnomalyRow> for Anomaly {
    fn from(row: AnomalyRow) -> Self {
        let metrics: AnomalyMetrics = serde_json::from_value(row.metrics.0.clone())
            .unwrap_or_else(|_| AnomalyMetrics {
                current_value: 0.0,
                baseline_value: 0.0,
                deviation: 0.0,
                z_score: 0.0,
                confidence: 0.0,
            });

        Self {
            id: row.id,
            detected_at: row.detected_at,
            anomaly_type: AnomalyType::from_str(&row.anomaly_type),
            severity: AnomalySeverity::from_str(&row.severity),
            description: row.description,
            metrics,
            affected_entities: row.affected_entities,
            investigation_steps: row.investigation_steps,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_anomaly_row_conversion() {
        let row = AnomalyRow {
            id: Uuid::new_v4(),
            anomaly_type: "performance_degradation".to_string(),
            severity: "warning".to_string(),
            description: "Test anomaly".to_string(),
            metrics: sqlx::types::Json(serde_json::json!({
                "currentValue": 500.0,
                "baselineValue": 100.0,
                "deviation": 400.0,
                "zScore": 2.5,
                "confidence": 0.83
            })),
            affected_entities: vec!["TICKET-1".to_string()],
            investigation_steps: vec!["Step 1".to_string()],
            workflow_instance_id: None,
            detected_at: Utc::now(),
            created_at: Utc::now(),
        };

        let anomaly: Anomaly = row.into();
        assert_eq!(anomaly.anomaly_type, AnomalyType::PerformanceDegradation);
        assert_eq!(anomaly.severity, AnomalySeverity::Warning);
        assert_eq!(anomaly.metrics.current_value, 500.0);
    }

    #[test]
    fn test_workflow_execution_data_conversion() {
        let data = WorkflowExecutionData {
            instance_id: Uuid::new_v4(),
            ticket_id: "TICKET-1".to_string(),
            user_id: "user1".to_string(),
            template_id: Uuid::new_v4(),
            execution_time_seconds: 100,
            succeeded: true,
            completed_at: Utc::now(),
        };

        let exec: WorkflowExecution = data.clone().into();
        assert_eq!(exec.instance_id, data.instance_id);
        assert_eq!(exec.ticket_id, data.ticket_id);
        assert_eq!(exec.execution_time_seconds, data.execution_time_seconds);
        assert_eq!(exec.succeeded, data.succeeded);
    }

    #[test]
    fn test_historical_execution_row_conversion() {
        let row = HistoricalExecutionRow {
            instance_id: Uuid::new_v4(),
            ticket_id: "TICKET-1".to_string(),
            user_id: "user1".to_string(),
            template_id: Uuid::new_v4(),
            execution_time_seconds: Some(150),
            succeeded: true,
            completed_at: Utc::now(),
        };

        let data: WorkflowExecutionData = row.into();
        assert_eq!(data.execution_time_seconds, 150);

        // Test with None execution_time_seconds
        let row_none = HistoricalExecutionRow {
            instance_id: Uuid::new_v4(),
            ticket_id: "TICKET-2".to_string(),
            user_id: "user1".to_string(),
            template_id: Uuid::new_v4(),
            execution_time_seconds: None,
            succeeded: false,
            completed_at: Utc::now(),
        };

        let data_none: WorkflowExecutionData = row_none.into();
        assert_eq!(data_none.execution_time_seconds, 0);
    }
}
