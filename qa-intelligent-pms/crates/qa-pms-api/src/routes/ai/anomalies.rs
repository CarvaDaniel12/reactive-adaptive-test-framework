//! Anomaly detection API endpoints.
//!
//! Story 31.9: Anomaly Detection in Workflows

use axum::{
    extract::{Path, Query, State},
    routing::{get, post},
    Json, Router,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use tracing::info;
use utoipa::{IntoParams, ToSchema};
use uuid::Uuid;

use anyhow::anyhow;
use crate::app::AppState;
use qa_pms_ai::{
    Anomaly, AnomalyDetector, AnomalyRepository, AnomalySeverity, AnomalyType, WorkflowExecution,
};
use qa_pms_core::error::ApiError;
use qa_pms_workflow::repository::get_instance;
use qa_pms_time::repository::get_workflow_sessions;

type ApiResult<T> = Result<T, ApiError>;

/// Create the anomalies router.
pub fn router() -> Router<AppState> {
    Router::new()
        .route("/api/v1/ai/check-anomalies", post(check_anomalies))
        .route("/api/v1/ai/anomalies", get(list_anomalies))
        .route("/api/v1/ai/anomalies/:id", get(get_anomaly))
        .route("/api/v1/ai/anomalies/trends", get(get_anomaly_trends))
}

// ==================== Request/Response Types ====================

/// Request to check anomalies for a workflow execution.
#[derive(Debug, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CheckAnomaliesRequest {
    /// Workflow execution ID
    pub workflow_execution_id: Uuid,
}

/// Anomaly response.
#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct AnomalyResponse {
    pub id: String,
    pub detected_at: String,
    pub anomaly_type: String,
    pub severity: String,
    pub description: String,
    pub metrics: AnomalyMetricsResponse,
    pub affected_entities: Vec<String>,
    pub investigation_steps: Vec<String>,
}

/// Anomaly metrics response.
#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct AnomalyMetricsResponse {
    pub current_value: f64,
    pub baseline_value: f64,
    pub deviation: f64,
    pub z_score: f64,
    pub confidence: f64,
}

/// Anomalies list response.
#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct AnomaliesResponse {
    pub anomalies: Vec<AnomalyResponse>,
    pub total: usize,
}

/// Anomaly query parameters.
#[derive(Debug, Deserialize, IntoParams)]
pub struct AnomalyQueryParams {
    /// Start date (ISO 8601)
    pub start_date: Option<String>,
    /// End date (ISO 8601)
    pub end_date: Option<String>,
    /// Filter by anomaly type
    pub anomaly_type: Option<String>,
    /// Filter by severity
    pub severity: Option<String>,
    /// Limit results
    pub limit: Option<i32>,
}

/// Anomaly trends response.
#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct AnomalyTrendsResponse {
    pub counts_by_date: Vec<AnomalyCountByDateResponse>,
    pub severity_distribution: Vec<SeverityDistributionResponse>,
}

/// Anomaly count by date.
#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct AnomalyCountByDateResponse {
    pub date: String,
    pub count: i64,
}

/// Severity distribution.
#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct SeverityDistributionResponse {
    pub severity: String,
    pub count: i64,
}

// ==================== Endpoints ====================

/// Check anomalies for a workflow execution.
#[utoipa::path(
    post,
    path = "/api/v1/ai/check-anomalies",
    request_body = CheckAnomaliesRequest,
    responses(
        (status = 200, description = "Detected anomalies", body = AnomaliesResponse),
        (status = 404, description = "Workflow not found"),
        (status = 500, description = "Internal server error")
    ),
    tag = "AI"
)]
pub async fn check_anomalies(
    State(state): State<AppState>,
    Json(request): Json<CheckAnomaliesRequest>,
) -> ApiResult<Json<AnomaliesResponse>> {
    // Get workflow instance
    let instance = get_instance(&state.db, request.workflow_execution_id)
        .await
        .map_err(|e| ApiError::Internal(anyhow!("Failed to fetch workflow: {}", e)))?;

    let Some(instance) = instance else {
        return Err(ApiError::NotFound(format!(
            "Workflow {} not found",
            request.workflow_execution_id
        )));
    };

    // Get workflow sessions to calculate execution time
    let sessions = get_workflow_sessions(&state.db, request.workflow_execution_id)
        .await
        .map_err(|e| ApiError::Internal(anyhow!("Failed to fetch sessions: {}", e)))?;

    let total_seconds: i32 = sessions
        .iter()
        .map(|s| s.total_seconds)
        .sum::<i32>()
        .max(0);

    let succeeded = matches!(instance.status.as_str(), "completed");

    // Build execution data
    let execution = WorkflowExecution {
        instance_id: instance.id,
        ticket_id: instance.ticket_id,
        user_id: instance.user_id,
        template_id: instance.template_id,
        execution_time_seconds: total_seconds,
        succeeded,
        completed_at: instance.completed_at.unwrap_or_else(Utc::now),
    };

    // Run anomaly detection
    let mut detector = AnomalyDetector::new();
    let anomalies = detector.check_execution(&execution);

    // Update baseline
    detector.update_baseline(&execution);

    // Store detected anomalies
    let repo = AnomalyRepository::new(state.db.clone());
    let mut stored_anomalies = Vec::new();
    for anomaly in anomalies {
        if let Ok(stored) = repo.create_anomaly(anomaly.clone()).await {
            stored_anomalies.push(stored);
        }
    }

    info!(
        workflow_id = %request.workflow_execution_id,
        anomalies_detected = stored_anomalies.len(),
        "Anomaly detection completed"
    );

    let total = stored_anomalies.len();
    Ok(Json(AnomaliesResponse {
        anomalies: stored_anomalies
            .into_iter()
            .map(|a| a.into())
            .collect(),
        total,
    }))
}

/// List anomalies with filters.
#[utoipa::path(
    get,
    path = "/api/v1/ai/anomalies",
    params(AnomalyQueryParams),
    responses(
        (status = 200, description = "List of anomalies", body = AnomaliesResponse),
        (status = 500, description = "Internal server error")
    ),
    tag = "AI"
)]
pub async fn list_anomalies(
    State(state): State<AppState>,
    Query(params): Query<AnomalyQueryParams>,
) -> ApiResult<Json<AnomaliesResponse>> {
    let repo = AnomalyRepository::new(state.db.clone());

    let anomalies = if let Some(start_date) = params.start_date {
        // Date range filter
        let start = start_date
            .parse::<DateTime<Utc>>()
            .map_err(|_| ApiError::Validation("Invalid start_date format".to_string()))?;
        let end = params
            .end_date
            .and_then(|d| d.parse::<DateTime<Utc>>().ok())
            .unwrap_or_else(Utc::now);

        repo.get_by_date_range(start, end)
            .await
            .map_err(|e| ApiError::Internal(anyhow!("Failed to fetch anomalies: {}", e)))?
    } else if let Some(anomaly_type) = params.anomaly_type {
        // Type filter
        let anomaly_type_enum = AnomalyType::from_str(&anomaly_type);
        let limit = params.limit.unwrap_or(100);
        repo.get_by_type(anomaly_type_enum, limit)
            .await
            .map_err(|e| ApiError::Internal(anyhow!("Failed to fetch anomalies: {}", e)))?
    } else if let Some(severity) = params.severity {
        // Severity filter
        let severity_enum = AnomalySeverity::from_str(&severity);
        let limit = params.limit.unwrap_or(100);
        repo.get_by_severity(severity_enum, limit)
            .await
            .map_err(|e| ApiError::Internal(anyhow!("Failed to fetch anomalies: {}", e)))?
    } else {
        // Default: recent anomalies
        let limit = params.limit.unwrap_or(100);
        repo.get_recent(limit)
            .await
            .map_err(|e| ApiError::Internal(anyhow!("Failed to fetch anomalies: {}", e)))?
    };

    Ok(Json(AnomaliesResponse {
        total: anomalies.len(),
        anomalies: anomalies.into_iter().map(|a| a.into()).collect(),
    }))
}

/// Get anomaly by ID.
#[utoipa::path(
    get,
    path = "/api/v1/ai/anomalies/{id}",
    params(("id" = Uuid, Path, description = "Anomaly ID")),
    responses(
        (status = 200, description = "Anomaly details", body = AnomalyResponse),
        (status = 404, description = "Anomaly not found"),
        (status = 500, description = "Internal server error")
    ),
    tag = "AI"
)]
pub async fn get_anomaly(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<AnomalyResponse>> {
    let repo = AnomalyRepository::new(state.db.clone());

    let anomaly = repo
        .get_anomaly(id)
        .await
        .map_err(|e| ApiError::Internal(anyhow!("Failed to fetch anomaly: {}", e)))?
        .ok_or_else(|| ApiError::NotFound(format!("Anomaly {} not found", id)))?;

    Ok(Json(anomaly.into()))
}

/// Get anomaly trends.
#[utoipa::path(
    get,
    path = "/api/v1/ai/anomalies/trends",
    params(("start_date" = Option<String>, Query), ("end_date" = Option<String>, Query)),
    responses(
        (status = 200, description = "Anomaly trends", body = AnomalyTrendsResponse),
        (status = 500, description = "Internal server error")
    ),
    tag = "AI"
)]
pub async fn get_anomaly_trends(
    State(state): State<AppState>,
    Query(params): Query<AnomalyQueryParams>,
) -> ApiResult<Json<AnomalyTrendsResponse>> {
    let repo = AnomalyRepository::new(state.db.clone());

    let start = params
        .start_date
        .and_then(|d| d.parse::<DateTime<Utc>>().ok())
        .unwrap_or_else(|| Utc::now() - chrono::Duration::days(30));
    let end = params
        .end_date
        .and_then(|d| d.parse::<DateTime<Utc>>().ok())
        .unwrap_or_else(Utc::now);

    let counts_by_date = repo
        .get_count_by_date(start, end)
        .await
        .map_err(|e| ApiError::Internal(anyhow!("Failed to fetch trends: {}", e)))?;

    let severity_distribution = repo
        .get_severity_distribution(start, end)
        .await
        .map_err(|e| ApiError::Internal(anyhow!("Failed to fetch distribution: {}", e)))?;

    Ok(Json(AnomalyTrendsResponse {
        counts_by_date: counts_by_date
            .into_iter()
            .map(|c| AnomalyCountByDateResponse {
                date: c.date.to_rfc3339(),
                count: c.count,
            })
            .collect(),
        severity_distribution: severity_distribution
            .into_iter()
            .map(|s| SeverityDistributionResponse {
                severity: s.severity.as_str().to_string(),
                count: s.count,
            })
            .collect(),
    }))
}

// ==================== Conversions ====================

impl From<Anomaly> for AnomalyResponse {
    fn from(anomaly: Anomaly) -> Self {
        Self {
            id: anomaly.id.to_string(),
            detected_at: anomaly.detected_at.to_rfc3339(),
            anomaly_type: anomaly.anomaly_type.as_str().to_string(),
            severity: anomaly.severity.as_str().to_string(),
            description: anomaly.description,
            metrics: AnomalyMetricsResponse {
                current_value: anomaly.metrics.current_value,
                baseline_value: anomaly.metrics.baseline_value,
                deviation: anomaly.metrics.deviation,
                z_score: anomaly.metrics.z_score,
                confidence: anomaly.metrics.confidence,
            },
            affected_entities: anomaly.affected_entities,
            investigation_steps: anomaly.investigation_steps,
        }
    }
}
