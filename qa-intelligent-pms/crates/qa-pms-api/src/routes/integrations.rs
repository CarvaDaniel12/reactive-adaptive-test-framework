//! Integration health API endpoints.
//!
//! Provides REST API endpoints for integration health monitoring.

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    routing::get,
    Json, Router,
};
use qa_pms_core::error::ApiError;
use qa_pms_dashboard::{default_period, parse_period, Period};
use qa_pms_integration_health::{
    IntegrationEvent, IntegrationHealth, IntegrationHealthRepository, IntegrationHealthService,
    IntegrationId,
};
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

use crate::app::AppState;

type ApiResult<T> = Result<T, ApiError>;

/// Create the integrations router.
pub fn router() -> Router<AppState> {
    Router::new()
        .route("/api/v1/integrations/health", get(get_integration_health).post(store_integration_health))
        .route("/api/v1/integrations/health/:integration_id", get(get_integration_health_by_id))
        .route("/api/v1/integrations/health/:integration_id/events", get(get_integration_events))
}

/// Query parameters for integration health endpoints.
#[derive(Debug, Deserialize, IntoParams)]
#[serde(rename_all = "camelCase")]
pub struct IntegrationHealthQuery {
    /// Period filter: "7d", "30d", "90d", "1y"
    #[param(example = "30d")]
    #[serde(default = "default_period")]
    pub period: String,
}

/// Response for integration health status.
#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct IntegrationHealthResponse {
    pub integrations: Vec<IntegrationHealth>,
}

/// Response for integration events.
#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct IntegrationEventsResponse {
    pub events: Vec<IntegrationEvent>,
}

/// Get health status for all integrations.
#[utoipa::path(
    get,
    path = "/api/v1/integrations/health",
    params(
        ("period" = String, Query, description = "Period filter: 7d, 30d, 90d, 1y")
    ),
    responses(
        (status = 200, description = "Integration health status", body = IntegrationHealthResponse),
        (status = 500, description = "Internal server error"),
    ),
    tag = "integrations"
)]
pub async fn get_integration_health(
    State(state): State<AppState>,
    Query(query): Query<IntegrationHealthQuery>,
) -> ApiResult<Json<IntegrationHealthResponse>> {
    let repository = IntegrationHealthRepository::new(state.db.clone());
    let service = IntegrationHealthService::new(repository);

    // Note: period parameter is accepted for API consistency, but get_health_status
    // currently returns latest health for all integrations (period not yet implemented)
    let integrations = service.get_health_status().await.map_err(ApiError::from)?;

    Ok(Json(IntegrationHealthResponse { integrations }))
}

/// Get health status for specific integration.
#[utoipa::path(
    get,
    path = "/api/v1/integrations/health/{integration_id}",
    params(
        ("integration_id" = String, Path, description = "Integration ID: booking-com, airbnb, vrbo, hmbn"),
        ("period" = String, Query, description = "Period filter: 7d, 30d, 90d, 1y")
    ),
    responses(
        (status = 200, description = "Integration health status", body = IntegrationHealth),
        (status = 404, description = "Integration not found"),
        (status = 500, description = "Internal server error"),
    ),
    tag = "integrations"
)]
pub async fn get_integration_health_by_id(
    State(state): State<AppState>,
    Path(integration_id): Path<String>,
    Query(query): Query<IntegrationHealthQuery>,
) -> ApiResult<Json<IntegrationHealth>> {
    let integration_id = parse_integration_id(&integration_id)?;

    let repository = IntegrationHealthRepository::new(state.db.clone());
    let service = IntegrationHealthService::new(repository);

    let days = parse_period(&query.period);
    let period = days_to_period(days);

    let history = service.get_health_history(integration_id, period).await.map_err(ApiError::from)?;

    // Return latest health status (first item in history)
    history
        .first()
        .map(|h| Ok(Json(h.clone())))
        .unwrap_or_else(|| Err(ApiError::NotFound(format!("Integration not found: {}", integration_id))))
}

/// Store integration health status manually (Phase 1).
#[utoipa::path(
    post,
    path = "/api/v1/integrations/health",
    request_body = IntegrationHealth,
    responses(
        (status = 201, description = "Health status stored"),
        (status = 400, description = "Invalid input"),
        (status = 500, description = "Internal server error"),
    ),
    tag = "integrations"
)]
pub async fn store_integration_health(
    State(state): State<AppState>,
    Json(health): Json<IntegrationHealth>,
) -> ApiResult<StatusCode> {
    let repository = IntegrationHealthRepository::new(state.db.clone());
    let service = IntegrationHealthService::new(repository);

    service.update_health_status(&health).await.map_err(ApiError::from)?;

    Ok(StatusCode::CREATED)
}

/// Get integration events for specific integration.
#[utoipa::path(
    get,
    path = "/api/v1/integrations/health/{integration_id}/events",
    params(
        ("integration_id" = String, Path, description = "Integration ID: booking-com, airbnb, vrbo, hmbn"),
        ("period" = String, Query, description = "Period filter: 7d, 30d, 90d, 1y")
    ),
    responses(
        (status = 200, description = "Integration events", body = IntegrationEventsResponse),
        (status = 404, description = "Integration not found"),
        (status = 500, description = "Internal server error"),
    ),
    tag = "integrations"
)]
pub async fn get_integration_events(
    State(_state): State<AppState>,
    Path(integration_id): Path<String>,
    Query(_query): Query<IntegrationHealthQuery>,
) -> ApiResult<Json<IntegrationEventsResponse>> {
    // Parse integration_id to validate it
    let _integration_id = parse_integration_id(&integration_id)?;

    // Future: Get events from repository (when implemented)
    // For now, return empty events
    Ok(Json(IntegrationEventsResponse { events: vec![] }))
}

/// Parse integration ID string to enum.
fn parse_integration_id(s: &str) -> ApiResult<IntegrationId> {
    match s {
        "booking-com" => Ok(IntegrationId::BookingCom),
        "airbnb" => Ok(IntegrationId::Airbnb),
        "vrbo" => Ok(IntegrationId::Vrbo),
        "hmbn" => Ok(IntegrationId::Hmbn),
        _ => Err(ApiError::Validation(format!(
            "Invalid integration ID: {}. Must be one of: booking-com, airbnb, vrbo, hmbn",
            s
        ))),
    }
}

/// Convert days (i64) to Period enum.
fn days_to_period(days: i64) -> Period {
    match days {
        7 => Period::Week,
        30 => Period::Month,
        90 => Period::Quarter,
        365 => Period::Year,
        _ => Period::Month, // Default to Month
    }
}
