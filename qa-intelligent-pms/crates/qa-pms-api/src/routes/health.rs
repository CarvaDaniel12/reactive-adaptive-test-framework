//! Health check endpoints.
//!
//! Provides:
//! - `/api/v1/health` - Overall application health
//! - `/api/v1/health/integrations` - Integration-specific health status
//! - `/api/v1/health/integrations/refresh` - Trigger manual health check

use axum::{
    extract::State,
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use chrono::{DateTime, Utc};
use qa_pms_core::IntegrationHealth;
use serde::Serialize;
use sqlx::PgPool;
use tracing::info;
use utoipa::ToSchema;

use crate::app::AppState;

/// Health check router.
pub fn router() -> Router<AppState> {
    Router::new()
        .route("/api/v1/health", get(health_check))
        .route("/api/v1/health/integrations", get(get_integration_health))
        .route(
            "/api/v1/health/integrations/refresh",
            post(trigger_health_check),
        )
}

/// Health check response.
#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct HealthResponse {
    /// Overall health status
    pub status: String,
    /// API version
    pub version: String,
    /// Current server timestamp (ISO 8601)
    pub timestamp: DateTime<Utc>,
    /// Database connection status
    pub database: DatabaseStatus,
}

/// Database connection status.
#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct DatabaseStatus {
    /// Whether the database is connected
    pub connected: bool,
    /// Response time in milliseconds (if connected)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_time_ms: Option<u64>,
    /// Error message (if not connected)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

/// Health check endpoint.
///
/// Returns the current health status of the API and its dependencies.
#[utoipa::path(
    get,
    path = "/api/v1/health",
    tag = "health",
    responses(
        (status = 200, description = "Health check successful", body = HealthResponse),
    )
)]
pub async fn health_check(State(state): State<AppState>) -> Json<HealthResponse> {
    let database = check_database(&state.db).await;

    let status = if database.connected {
        "healthy"
    } else {
        "unhealthy"
    };

    Json(HealthResponse {
        status: status.to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        timestamp: Utc::now(),
        database,
    })
}

/// Check database connectivity.
async fn check_database(pool: &PgPool) -> DatabaseStatus {
    let start = std::time::Instant::now();

    match sqlx::query("SELECT 1").execute(pool).await {
        Ok(_) => DatabaseStatus {
            connected: true,
            #[allow(clippy::cast_possible_truncation)]
            response_time_ms: Some(start.elapsed().as_millis() as u64),
            error: None,
        },
        Err(e) => DatabaseStatus {
            connected: false,
            response_time_ms: None,
            error: Some(e.to_string()),
        },
    }
}

/// Integration health response item.
#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct IntegrationHealthResponse {
    /// Integration name
    pub integration: String,
    /// Current status: "online", "degraded", or "offline"
    pub status: String,
    /// Last successful check timestamp
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_successful_check: Option<DateTime<Utc>>,
    /// Last check timestamp
    pub last_check: DateTime<Utc>,
    /// Response time in milliseconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_time_ms: Option<u64>,
    /// Error message if offline
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    /// Number of consecutive failures
    pub consecutive_failures: u32,
    /// When downtime started (if offline)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub downtime_start: Option<DateTime<Utc>>,
}

impl From<IntegrationHealth> for IntegrationHealthResponse {
    fn from(h: IntegrationHealth) -> Self {
        Self {
            integration: h.integration,
            status: format!("{:?}", h.status).to_lowercase(),
            last_successful_check: h.last_successful_check,
            last_check: h.last_check,
            response_time_ms: h.response_time_ms,
            error_message: h.error_message,
            consecutive_failures: h.consecutive_failures,
            downtime_start: h.downtime_start,
        }
    }
}

/// Get integration health status.
///
/// Returns the health status of all configured integrations.
#[utoipa::path(
    get,
    path = "/api/v1/health/integrations",
    tag = "health",
    responses(
        (status = 200, description = "Integration health status", body = Vec<IntegrationHealthResponse>),
    )
)]
pub async fn get_integration_health(
    State(state): State<AppState>,
) -> Json<Vec<IntegrationHealthResponse>> {
    let health = state.health_store.get_all().await;
    let response: Vec<IntegrationHealthResponse> = health.into_iter().map(Into::into).collect();
    Json(response)
}

/// Trigger manual health check refresh.
///
/// Triggers an immediate health check for all integrations.
/// Note: This is a placeholder - actual health check execution requires
/// the scheduler to be accessible. In production, this would signal
/// the scheduler to run checks immediately.
#[utoipa::path(
    post,
    path = "/api/v1/health/integrations/refresh",
    tag = "health",
    responses(
        (status = 200, description = "Health check triggered"),
    )
)]
pub async fn trigger_health_check(State(_state): State<AppState>) -> StatusCode {
    info!("Manual health check refresh requested");
    // Note: In a full implementation, this would trigger the health scheduler
    // to run checks immediately. For now, we just acknowledge the request.
    // The next scheduled check will run within 60 seconds.
    StatusCode::OK
}
