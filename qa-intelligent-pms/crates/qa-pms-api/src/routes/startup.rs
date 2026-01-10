//! Startup validation endpoint.
//!
//! Provides `/api/v1/startup/validate` for frontend startup checks.

use axum::{extract::State, routing::get, Json, Router};

use crate::app::AppState;
use crate::startup::StartupValidationReport;
use crate::user_config_health::build_startup_validator_from_user_config;

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
    // Build validator from the per-user config file on each request so changes
    // from the setup wizard are immediately reflected without server restart.
    let validator = build_startup_validator_from_user_config(&state.settings);
    let report = validator.validate().await;
    Json(report)
}
