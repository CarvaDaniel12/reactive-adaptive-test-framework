//! Time tracking API endpoints.
//!
//! Refactored to use unified `ApiError` for cleaner error handling.
//! Story 6.7: Added historical time data endpoints.

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use tracing::info;
use utoipa::ToSchema;
use uuid::Uuid;

use qa_pms_time::{
    dismiss_alert as dismiss_gap_alert,
    end_session,
    get_active_session,
    // Story 6.7: Historical aggregates
    get_historical_summary,
    get_trend_data,
    get_undismissed_alerts,
    get_user_averages,
    get_workflow_sessions,
    pause_session,
    resume_session,
    start_session,
    HistoricalSummary,
    TimeGapAlert,
    TimeSession,
    TrendPoint,
    UserAverage,
};

use crate::app::AppState;
use qa_pms_core::error::ApiError;
use qa_pms_dashboard::SqlxResultExt;

/// Result type alias for API handlers.
type ApiResult<T> = Result<T, ApiError>;

/// Create the time tracking router.
pub fn router() -> Router<AppState> {
    Router::new()
        .route(
            "/api/v1/time/sessions/:workflow_id/start/:step_index",
            post(start_time_session),
        )
        .route(
            "/api/v1/time/sessions/:session_id/end",
            post(end_time_session),
        )
        .route(
            "/api/v1/time/sessions/:session_id/pause",
            post(pause_time_session),
        )
        .route(
            "/api/v1/time/sessions/:session_id/resume",
            post(resume_time_session),
        )
        .route(
            "/api/v1/time/sessions/:workflow_id/active",
            get(get_active_time_session),
        )
        .route(
            "/api/v1/time/sessions/:workflow_id",
            get(get_all_time_sessions),
        )
        // Story 6.7: Historical time data endpoints
        .route("/api/v1/time/history/:user_id", get(get_historical_stats))
        .route("/api/v1/time/history/:user_id/trend", get(get_time_trend))
        .route("/api/v1/time/history/:user_id/averages", get(get_averages))
        .route("/api/v1/time/history/:user_id/alerts", get(get_gap_alerts))
        .route("/api/v1/time/alerts/:alert_id/dismiss", post(dismiss_alert))
}

// ============================================================================
// Response Types
// ============================================================================

/// Time session response.
#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct TimeSessionResponse {
    pub id: Uuid,
    pub workflow_instance_id: Uuid,
    pub step_index: i32,
    pub started_at: String,
    pub paused_at: Option<String>,
    pub ended_at: Option<String>,
    pub total_seconds: i32,
    pub is_active: bool,
}

impl From<TimeSession> for TimeSessionResponse {
    fn from(s: TimeSession) -> Self {
        Self {
            id: s.id,
            workflow_instance_id: s.workflow_instance_id,
            step_index: s.step_index,
            started_at: s.started_at.to_rfc3339(),
            paused_at: s.paused_at.map(|t| t.to_rfc3339()),
            ended_at: s.ended_at.map(|t| t.to_rfc3339()),
            total_seconds: s.total_seconds,
            is_active: s.is_active,
        }
    }
}

/// List of time sessions.
#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct TimeSessionsResponse {
    pub sessions: Vec<TimeSessionResponse>,
    pub total_seconds: i32,
}

// ============================================================================
// Handlers
// ============================================================================

/// Start a time session for a workflow step.
#[utoipa::path(
    post,
    path = "/api/v1/time/sessions/{workflow_id}/start/{step_index}",
    params(
        ("workflow_id" = Uuid, Path, description = "Workflow instance ID"),
        ("step_index" = i32, Path, description = "Step index")
    ),
    responses(
        (status = 201, description = "Time session started", body = TimeSessionResponse),
        (status = 500, description = "Internal server error")
    ),
    tag = "Time Tracking"
)]
pub async fn start_time_session(
    State(state): State<AppState>,
    Path((workflow_id, step_index)): Path<(Uuid, i32)>,
) -> ApiResult<impl IntoResponse> {
    let session = start_session(&state.db, workflow_id, step_index)
        .await
        .map_db_err()?;

    info!(workflow_id = %workflow_id, step_index, "Started time session");

    Ok((
        StatusCode::CREATED,
        Json(TimeSessionResponse::from(session)),
    ))
}

/// End a time session.
#[utoipa::path(
    post,
    path = "/api/v1/time/sessions/{session_id}/end",
    params(
        ("session_id" = Uuid, Path, description = "Session ID")
    ),
    responses(
        (status = 200, description = "Time session ended", body = TimeSessionResponse),
        (status = 500, description = "Internal server error")
    ),
    tag = "Time Tracking"
)]
pub async fn end_time_session(
    State(state): State<AppState>,
    Path(session_id): Path<Uuid>,
) -> ApiResult<Json<TimeSessionResponse>> {
    let session = end_session(&state.db, session_id).await.map_db_err()?;

    info!(session_id = %session_id, total_seconds = session.total_seconds, "Ended time session");

    Ok(Json(TimeSessionResponse::from(session)))
}

/// Pause a time session.
#[utoipa::path(
    post,
    path = "/api/v1/time/sessions/{session_id}/pause",
    params(
        ("session_id" = Uuid, Path, description = "Session ID")
    ),
    responses(
        (status = 200, description = "Time session paused"),
        (status = 500, description = "Internal server error")
    ),
    tag = "Time Tracking"
)]
pub async fn pause_time_session(
    State(state): State<AppState>,
    Path(session_id): Path<Uuid>,
) -> ApiResult<Json<serde_json::Value>> {
    pause_session(&state.db, session_id).await.map_db_err()?;

    info!(session_id = %session_id, "Paused time session");

    Ok(Json(serde_json::json!({ "status": "paused" })))
}

/// Resume a paused time session.
#[utoipa::path(
    post,
    path = "/api/v1/time/sessions/{session_id}/resume",
    params(
        ("session_id" = Uuid, Path, description = "Session ID")
    ),
    responses(
        (status = 200, description = "Time session resumed"),
        (status = 500, description = "Internal server error")
    ),
    tag = "Time Tracking"
)]
pub async fn resume_time_session(
    State(state): State<AppState>,
    Path(session_id): Path<Uuid>,
) -> ApiResult<Json<serde_json::Value>> {
    resume_session(&state.db, session_id).await.map_db_err()?;

    info!(session_id = %session_id, "Resumed time session");

    Ok(Json(serde_json::json!({ "status": "resumed" })))
}

/// Get active time session for a workflow.
#[utoipa::path(
    get,
    path = "/api/v1/time/sessions/{workflow_id}/active",
    params(
        ("workflow_id" = Uuid, Path, description = "Workflow instance ID")
    ),
    responses(
        (status = 200, description = "Active time session", body = Option<TimeSessionResponse>),
        (status = 500, description = "Internal server error")
    ),
    tag = "Time Tracking"
)]
pub async fn get_active_time_session(
    State(state): State<AppState>,
    Path(workflow_id): Path<Uuid>,
) -> ApiResult<Json<Option<TimeSessionResponse>>> {
    let session = get_active_session(&state.db, workflow_id)
        .await
        .map_db_err()?;

    Ok(Json(session.map(TimeSessionResponse::from)))
}

/// Get all time sessions for a workflow.
#[utoipa::path(
    get,
    path = "/api/v1/time/sessions/{workflow_id}",
    params(
        ("workflow_id" = Uuid, Path, description = "Workflow instance ID")
    ),
    responses(
        (status = 200, description = "All time sessions", body = TimeSessionsResponse),
        (status = 500, description = "Internal server error")
    ),
    tag = "Time Tracking"
)]
pub async fn get_all_time_sessions(
    State(state): State<AppState>,
    Path(workflow_id): Path<Uuid>,
) -> ApiResult<Json<TimeSessionsResponse>> {
    let sessions = get_workflow_sessions(&state.db, workflow_id)
        .await
        .map_db_err()?;

    let total_seconds: i32 = sessions.iter().map(|s| s.total_seconds).sum();
    let responses: Vec<TimeSessionResponse> = sessions
        .into_iter()
        .map(TimeSessionResponse::from)
        .collect();

    Ok(Json(TimeSessionsResponse {
        sessions: responses,
        total_seconds,
    }))
}

// ============================================================================
// Story 6.7: Historical Time Data Endpoints
// ============================================================================

/// Query parameters for historical data.
#[derive(Debug, Deserialize)]
pub struct HistoryQuery {
    /// Period in days (default: 30)
    #[serde(default = "default_days")]
    pub days: i32,
}

const fn default_days() -> i32 {
    30
}

/// Historical summary response.
#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct HistoricalStatsResponse {
    pub user_id: Uuid,
    pub period_days: i32,
    pub total_tickets: i64,
    pub total_time_seconds: i64,
    pub total_hours: f64,
    pub avg_time_per_ticket_seconds: f64,
    pub avg_time_per_ticket_minutes: f64,
    pub efficiency_ratio: f64,
    pub by_ticket_type: Vec<TicketTypeStats>,
}

/// Stats by ticket type.
#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct TicketTypeStats {
    pub ticket_type: String,
    pub count: i64,
    pub total_seconds: i64,
    pub total_hours: f64,
    pub avg_seconds: f64,
    pub avg_minutes: f64,
}

impl From<HistoricalSummary> for HistoricalStatsResponse {
    fn from(s: HistoricalSummary) -> Self {
        Self {
            user_id: s.user_id,
            period_days: s.period_days,
            total_tickets: s.total_tickets,
            total_time_seconds: s.total_time_seconds,
            total_hours: s.total_time_seconds as f64 / 3600.0,
            avg_time_per_ticket_seconds: s.avg_time_per_ticket_seconds,
            avg_time_per_ticket_minutes: s.avg_time_per_ticket_seconds / 60.0,
            efficiency_ratio: s.efficiency_ratio,
            by_ticket_type: s
                .by_ticket_type
                .into_iter()
                .map(|t| TicketTypeStats {
                    ticket_type: t.ticket_type,
                    count: t.count,
                    total_seconds: t.total_seconds,
                    total_hours: t.total_seconds as f64 / 3600.0,
                    avg_seconds: t.avg_seconds,
                    avg_minutes: t.avg_seconds / 60.0,
                })
                .collect(),
        }
    }
}

/// Trend data response.
#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct TrendResponse {
    pub data: Vec<TrendDataResponse>,
}

/// Single trend data point.
#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct TrendDataResponse {
    pub date: String,
    pub tickets: i32,
    pub hours: f64,
    pub efficiency: f64,
}

impl From<TrendPoint> for TrendDataResponse {
    fn from(p: TrendPoint) -> Self {
        Self {
            date: p.date.format("%Y-%m-%d").to_string(),
            tickets: p.tickets,
            hours: p.hours,
            efficiency: p.efficiency,
        }
    }
}

/// User averages response.
#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct UserAveragesResponse {
    pub averages: Vec<UserAverageResponse>,
}

/// Single user average.
#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct UserAverageResponse {
    pub ticket_type: String,
    pub sample_count: i32,
    pub avg_seconds: i32,
    pub avg_minutes: f64,
    pub min_seconds: i32,
    pub max_seconds: i32,
    pub rolling_avg_seconds: i32,
    pub rolling_avg_minutes: f64,
}

impl From<UserAverage> for UserAverageResponse {
    fn from(a: UserAverage) -> Self {
        Self {
            ticket_type: a.ticket_type,
            sample_count: a.sample_count,
            avg_seconds: a.avg_seconds,
            avg_minutes: f64::from(a.avg_seconds) / 60.0,
            min_seconds: a.min_seconds,
            max_seconds: a.max_seconds,
            rolling_avg_seconds: a.rolling_avg_seconds,
            rolling_avg_minutes: f64::from(a.rolling_avg_seconds) / 60.0,
        }
    }
}

/// Gap alerts response.
#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct GapAlertsResponse {
    pub alerts: Vec<GapAlertResponse>,
}

/// Single gap alert.
#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct GapAlertResponse {
    pub id: Uuid,
    pub workflow_instance_id: Uuid,
    pub step_index: Option<i32>,
    pub actual_seconds: i32,
    pub estimated_seconds: i32,
    pub gap_percentage: f64,
    pub alert_type: String,
    pub created_at: String,
}

impl From<TimeGapAlert> for GapAlertResponse {
    fn from(a: TimeGapAlert) -> Self {
        use rust_decimal::prelude::ToPrimitive;
        Self {
            id: a.id,
            workflow_instance_id: a.workflow_instance_id,
            step_index: a.step_index,
            actual_seconds: a.actual_seconds,
            estimated_seconds: a.estimated_seconds,
            // Use ToPrimitive trait for proper Decimal to f64 conversion
            gap_percentage: a.gap_percentage.to_f64().unwrap_or(0.0),
            alert_type: a.alert_type,
            created_at: a.created_at.to_rfc3339(),
        }
    }
}

/// Get historical time stats for a user.
#[utoipa::path(
    get,
    path = "/api/v1/time/history/{user_id}",
    params(
        ("user_id" = Uuid, Path, description = "User ID"),
        ("days" = i32, Query, description = "Period in days (default: 30)")
    ),
    responses(
        (status = 200, description = "Historical stats", body = HistoricalStatsResponse),
        (status = 500, description = "Internal server error")
    ),
    tag = "Time Tracking"
)]
pub async fn get_historical_stats(
    State(state): State<AppState>,
    Path(user_id): Path<Uuid>,
    Query(query): Query<HistoryQuery>,
) -> ApiResult<Json<HistoricalStatsResponse>> {
    let summary = get_historical_summary(&state.db, user_id, query.days)
        .await
        .map_db_err()?;

    info!(user_id = %user_id, days = query.days, "Retrieved historical stats");

    Ok(Json(HistoricalStatsResponse::from(summary)))
}

/// Get time trend data for charts.
#[utoipa::path(
    get,
    path = "/api/v1/time/history/{user_id}/trend",
    params(
        ("user_id" = Uuid, Path, description = "User ID"),
        ("days" = i32, Query, description = "Period in days (default: 30)")
    ),
    responses(
        (status = 200, description = "Trend data", body = TrendResponse),
        (status = 500, description = "Internal server error")
    ),
    tag = "Time Tracking"
)]
pub async fn get_time_trend(
    State(state): State<AppState>,
    Path(user_id): Path<Uuid>,
    Query(query): Query<HistoryQuery>,
) -> ApiResult<Json<TrendResponse>> {
    let trend = get_trend_data(&state.db, user_id, query.days)
        .await
        .map_db_err()?;

    Ok(Json(TrendResponse {
        data: trend.into_iter().map(TrendDataResponse::from).collect(),
    }))
}

/// Get user averages by ticket type.
#[utoipa::path(
    get,
    path = "/api/v1/time/history/{user_id}/averages",
    params(
        ("user_id" = Uuid, Path, description = "User ID")
    ),
    responses(
        (status = 200, description = "User averages", body = UserAveragesResponse),
        (status = 500, description = "Internal server error")
    ),
    tag = "Time Tracking"
)]
pub async fn get_averages(
    State(state): State<AppState>,
    Path(user_id): Path<Uuid>,
) -> ApiResult<Json<UserAveragesResponse>> {
    let averages = get_user_averages(&state.db, user_id).await.map_db_err()?;

    Ok(Json(UserAveragesResponse {
        averages: averages
            .into_iter()
            .map(UserAverageResponse::from)
            .collect(),
    }))
}

/// Get undismissed gap alerts for a user.
#[utoipa::path(
    get,
    path = "/api/v1/time/history/{user_id}/alerts",
    params(
        ("user_id" = Uuid, Path, description = "User ID")
    ),
    responses(
        (status = 200, description = "Gap alerts", body = GapAlertsResponse),
        (status = 500, description = "Internal server error")
    ),
    tag = "Time Tracking"
)]
pub async fn get_gap_alerts(
    State(state): State<AppState>,
    Path(user_id): Path<Uuid>,
) -> ApiResult<Json<GapAlertsResponse>> {
    let alerts = get_undismissed_alerts(&state.db, user_id, 50)
        .await
        .map_db_err()?;

    Ok(Json(GapAlertsResponse {
        alerts: alerts.into_iter().map(GapAlertResponse::from).collect(),
    }))
}

/// Dismiss a gap alert.
#[utoipa::path(
    post,
    path = "/api/v1/time/alerts/{alert_id}/dismiss",
    params(
        ("alert_id" = Uuid, Path, description = "Alert ID")
    ),
    responses(
        (status = 200, description = "Alert dismissed"),
        (status = 500, description = "Internal server error")
    ),
    tag = "Time Tracking"
)]
pub async fn dismiss_alert(
    State(state): State<AppState>,
    Path(alert_id): Path<Uuid>,
) -> ApiResult<Json<serde_json::Value>> {
    dismiss_gap_alert(&state.db, alert_id).await.map_db_err()?;

    info!(alert_id = %alert_id, "Dismissed gap alert");

    Ok(Json(serde_json::json!({ "status": "dismissed" })))
}
