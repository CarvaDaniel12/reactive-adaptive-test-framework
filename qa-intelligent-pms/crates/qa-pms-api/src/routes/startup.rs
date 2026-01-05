//! Startup validation endpoint.
//!
//! Provides `/api/v1/startup/validate` for frontend startup checks.

use axum::{extract::State, routing::get, Json, Router};

use crate::app::AppState;
use crate::startup::StartupValidationReport;

/// Startup routes.
pub fn router() -> Router<AppState> {
    Router::new().route("/api/v1/startup/validate", get(validate_startup))
}

/// Validate all configured integrations.
///
/// Returns a report indicating which integrations are working
/// and whether the application can start.
#[utoipa::path(
    get,
    path = "/api/v1/startup/validate",
    tag = "Startup",
    responses(
        (status = 200, description = "Validation complete", body = StartupValidationReport),
    )
)]
pub async fn validate_startup(State(state): State<AppState>) -> Json<StartupValidationReport> {
    let report = state.startup_validator.validate().await;
    Json(report)
}
