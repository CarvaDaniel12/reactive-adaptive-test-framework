//! Alert API endpoints.
//!
//! Provides endpoints for managing alerts from pattern detection.

use axum::{
    extract::{Path, State},
    routing::{get, post},
    Json, Router,
};
use serde::Serialize;
use utoipa::ToSchema;
use uuid::Uuid;

use crate::app::AppState;
use qa_pms_core::error::ApiError;

type ApiResult<T> = Result<T, ApiError>;

/// Create the alerts router.
pub fn router() -> Router<AppState> {
    Router::new()
        .route("/api/v1/alerts", get(get_alerts))
        .route("/api/v1/alerts/count", get(get_unread_count))
        .route("/api/v1/alerts/:id/read", post(mark_read))
        .route("/api/v1/alerts/:id/dismiss", post(dismiss_alert))
        .route("/api/v1/patterns", get(get_patterns))
        .route("/api/v1/patterns/:id", get(get_pattern))
}

/// Alert response.
#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct AlertResponse {
    pub id: String,
    pub pattern_id: Option<String>,
    pub alert_type: String,
    pub severity: String,
    pub title: String,
    pub message: Option<String>,
    pub affected_tickets: Vec<String>,
    pub suggested_actions: Vec<String>,
    pub is_read: bool,
    pub is_dismissed: bool,
    pub created_at: String,
}

/// Alerts list response.
#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct AlertsResponse {
    pub alerts: Vec<AlertResponse>,
    pub total: i64,
}

/// Unread count response.
#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct UnreadCountResponse {
    pub count: i64,
}

/// Pattern response.
#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct PatternResponse {
    pub id: String,
    pub pattern_type: String,
    pub severity: String,
    pub title: String,
    pub description: Option<String>,
    pub affected_tickets: Vec<String>,
    pub common_factor: Option<String>,
    pub average_excess_percent: Option<f64>,
    pub confidence_score: f64,
    pub suggested_actions: Vec<String>,
    pub detected_at: String,
}

/// Patterns list response.
#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct PatternsResponse {
    pub patterns: Vec<PatternResponse>,
}

/// Get all unread alerts.
#[utoipa::path(
    get,
    path = "/api/v1/alerts",
    responses(
        (status = 200, description = "List of alerts", body = AlertsResponse),
    ),
    tag = "Alerts"
)]
pub async fn get_alerts(State(state): State<AppState>) -> ApiResult<Json<AlertsResponse>> {
    // Query alerts from database
    let rows: Vec<AlertRow> = sqlx::query_as(
        r"
        SELECT 
            id, pattern_id, alert_type, severity, title, message,
            affected_tickets, suggested_actions, is_read, is_dismissed, created_at
        FROM alerts
        WHERE NOT is_dismissed
        ORDER BY 
            CASE severity 
                WHEN 'critical' THEN 1 
                WHEN 'warning' THEN 2 
                ELSE 3 
            END,
            created_at DESC
        LIMIT 50
        ",
    )
    .fetch_all(&state.db)
    .await
    .map_err(|e| ApiError::Internal(anyhow::anyhow!("Failed to fetch alerts: {e}")))?;

    let total = rows.len() as i64;
    let alerts = rows.into_iter().map(Into::into).collect();

    Ok(Json(AlertsResponse { alerts, total }))
}

/// Get unread alert count.
#[utoipa::path(
    get,
    path = "/api/v1/alerts/count",
    responses(
        (status = 200, description = "Unread count", body = UnreadCountResponse),
    ),
    tag = "Alerts"
)]
pub async fn get_unread_count(
    State(state): State<AppState>,
) -> ApiResult<Json<UnreadCountResponse>> {
    let (count,): (i64,) =
        sqlx::query_as("SELECT COUNT(*) FROM alerts WHERE NOT is_read AND NOT is_dismissed")
            .fetch_one(&state.db)
            .await
            .map_err(|e| ApiError::Internal(anyhow::anyhow!("Failed to count alerts: {e}")))?;

    Ok(Json(UnreadCountResponse { count }))
}

/// Mark alert as read.
#[utoipa::path(
    post,
    path = "/api/v1/alerts/{id}/read",
    params(
        ("id" = String, Path, description = "Alert ID")
    ),
    responses(
        (status = 200, description = "Alert marked as read"),
        (status = 404, description = "Alert not found"),
    ),
    tag = "Alerts"
)]
pub async fn mark_read(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<serde_json::Value>> {
    let result = sqlx::query("UPDATE alerts SET is_read = TRUE WHERE id = $1")
        .bind(id)
        .execute(&state.db)
        .await
        .map_err(|e| ApiError::Internal(anyhow::anyhow!("Failed to mark alert read: {e}")))?;

    if result.rows_affected() == 0 {
        return Err(ApiError::NotFound(format!("Alert {id} not found")));
    }

    Ok(Json(serde_json::json!({ "success": true })))
}

/// Dismiss an alert.
#[utoipa::path(
    post,
    path = "/api/v1/alerts/{id}/dismiss",
    params(
        ("id" = String, Path, description = "Alert ID")
    ),
    responses(
        (status = 200, description = "Alert dismissed"),
        (status = 404, description = "Alert not found"),
    ),
    tag = "Alerts"
)]
pub async fn dismiss_alert(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<serde_json::Value>> {
    let result =
        sqlx::query("UPDATE alerts SET is_dismissed = TRUE, dismissed_at = NOW() WHERE id = $1")
            .bind(id)
            .execute(&state.db)
            .await
            .map_err(|e| ApiError::Internal(anyhow::anyhow!("Failed to dismiss alert: {e}")))?;

    if result.rows_affected() == 0 {
        return Err(ApiError::NotFound(format!("Alert {id} not found")));
    }

    Ok(Json(serde_json::json!({ "success": true })))
}

/// Get recent patterns.
#[utoipa::path(
    get,
    path = "/api/v1/patterns",
    responses(
        (status = 200, description = "List of patterns", body = PatternsResponse),
    ),
    tag = "Alerts"
)]
pub async fn get_patterns(State(state): State<AppState>) -> ApiResult<Json<PatternsResponse>> {
    let rows: Vec<PatternRow> = sqlx::query_as(
        r"
        SELECT 
            id, pattern_type, severity, title, description,
            affected_tickets, common_factor, average_excess_percent,
            confidence_score, suggested_actions, detected_at
        FROM detected_patterns
        ORDER BY detected_at DESC
        LIMIT 50
        ",
    )
    .fetch_all(&state.db)
    .await
    .map_err(|e| ApiError::Internal(anyhow::anyhow!("Failed to fetch patterns: {e}")))?;

    let patterns = rows.into_iter().map(Into::into).collect();
    Ok(Json(PatternsResponse { patterns }))
}

/// Get pattern by ID.
#[utoipa::path(
    get,
    path = "/api/v1/patterns/{id}",
    params(
        ("id" = String, Path, description = "Pattern ID")
    ),
    responses(
        (status = 200, description = "Pattern details", body = PatternResponse),
        (status = 404, description = "Pattern not found"),
    ),
    tag = "Alerts"
)]
pub async fn get_pattern(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<PatternResponse>> {
    let row: Option<PatternRow> = sqlx::query_as(
        r"
        SELECT 
            id, pattern_type, severity, title, description,
            affected_tickets, common_factor, average_excess_percent,
            confidence_score, suggested_actions, detected_at
        FROM detected_patterns
        WHERE id = $1
        ",
    )
    .bind(id)
    .fetch_optional(&state.db)
    .await
    .map_err(|e| ApiError::Internal(anyhow::anyhow!("Failed to fetch pattern: {e}")))?;

    match row {
        Some(r) => Ok(Json(r.into())),
        None => Err(ApiError::NotFound(format!("Pattern {id} not found"))),
    }
}

// Internal row types
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
    created_at: chrono::DateTime<chrono::Utc>,
}

impl From<AlertRow> for AlertResponse {
    fn from(row: AlertRow) -> Self {
        Self {
            id: row.id.to_string(),
            pattern_id: row.pattern_id.map(|id| id.to_string()),
            alert_type: row.alert_type,
            severity: row.severity,
            title: row.title,
            message: row.message,
            affected_tickets: row.affected_tickets,
            suggested_actions: row.suggested_actions,
            is_read: row.is_read,
            is_dismissed: row.is_dismissed,
            created_at: row.created_at.to_rfc3339(),
        }
    }
}

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
    detected_at: chrono::DateTime<chrono::Utc>,
}

impl From<PatternRow> for PatternResponse {
    fn from(row: PatternRow) -> Self {
        Self {
            id: row.id.to_string(),
            pattern_type: row.pattern_type,
            severity: row.severity,
            title: row.title,
            description: row.description,
            affected_tickets: row.affected_tickets,
            common_factor: row.common_factor,
            average_excess_percent: row.average_excess_percent,
            confidence_score: row.confidence_score,
            suggested_actions: row.suggested_actions,
            detected_at: row.detected_at.to_rfc3339(),
        }
    }
}
