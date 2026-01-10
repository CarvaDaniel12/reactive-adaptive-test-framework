//! Testmo API routes.
//!
//! Endpoints for interacting with Testmo test management.

use axum::{extract::State, http::StatusCode, routing::post, Json, Router};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::app::AppState;

/// Create test run request.
#[derive(Debug, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CreateTestRunRequest {
    /// Ticket key for naming convention (e.g., "PROJ-123")
    pub ticket_key: String,
    /// Test case IDs to include in the run
    pub case_ids: Vec<i64>,
    /// Optional custom name (overrides generated name)
    pub custom_name: Option<String>,
}

/// Create test run response.
#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CreateTestRunResponse {
    /// Created run ID
    pub run_id: i64,
    /// Run name
    pub name: String,
    /// URL to the run in Testmo
    pub url: String,
    /// Number of test cases included
    pub case_count: usize,
}

/// Error response.
#[derive(Debug, Serialize, ToSchema)]
pub struct ErrorResponse {
    pub message: String,
}

/// Create Testmo routes.
pub fn router() -> Router<AppState> {
    Router::new().route("/runs", post(create_test_run))
}

/// Create a test run in Testmo.
///
/// Creates a new test run with the specified test cases.
/// The run name follows the pattern "QA-{ticket-key}-{date}" unless a custom name is provided.
#[utoipa::path(
    post,
    path = "/api/v1/testmo/runs",
    request_body = CreateTestRunRequest,
    responses(
        (status = 201, description = "Test run created successfully", body = CreateTestRunResponse),
        (status = 400, description = "Invalid request", body = ErrorResponse),
        (status = 503, description = "Testmo not configured", body = ErrorResponse)
    ),
    tag = "testmo"
)]
async fn create_test_run(
    State(state): State<AppState>,
    Json(request): Json<CreateTestRunRequest>,
) -> Result<(StatusCode, Json<CreateTestRunResponse>), (StatusCode, Json<ErrorResponse>)> {
    // Check if Testmo is configured
    let testmo_client = state.testmo_client.as_ref().ok_or_else(|| {
        (
            StatusCode::SERVICE_UNAVAILABLE,
            Json(ErrorResponse {
                message: "Testmo integration not configured".to_string(),
            }),
        )
    })?;

    let project_id = state.testmo_project_id.ok_or_else(|| {
        (
            StatusCode::SERVICE_UNAVAILABLE,
            Json(ErrorResponse {
                message: "Testmo project ID not configured".to_string(),
            }),
        )
    })?;

    // Validate request
    if request.case_ids.is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                message: "At least one test case is required".to_string(),
            }),
        ));
    }

    // Generate run name
    let run_name = match &request.custom_name {
        Some(name) if !name.is_empty() => name.clone(),
        _ => generate_run_name(&request.ticket_key),
    };

    // Create test run
    let test_run = testmo_client
        .create_test_run(project_id, &run_name, &request.case_ids)
        .await
        .map_err(|e| {
            tracing::error!(error = %e, "Failed to create Testmo test run");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    message: format!("Failed to create test run: {e}"),
                }),
            )
        })?;

    // Generate URL to the run
    let url = format!(
        "{}/projects/{}/runs/{}",
        testmo_client.base_url(),
        project_id,
        test_run.id
    );

    tracing::info!(
        run_id = test_run.id,
        ticket = %request.ticket_key,
        cases = request.case_ids.len(),
        "Created Testmo test run"
    );

    Ok((
        StatusCode::CREATED,
        Json(CreateTestRunResponse {
            run_id: test_run.id,
            name: run_name,
            url,
            case_count: request.case_ids.len(),
        }),
    ))
}

/// Generate test run name from ticket key.
///
/// Format: QA-{ticket-key}-{YYYY-MM-DD}
fn generate_run_name(ticket_key: &str) -> String {
    let date = Utc::now().format("%Y-%m-%d");
    format!("QA-{ticket_key}-{date}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_run_name() {
        let name = generate_run_name("PROJ-123");
        assert!(name.starts_with("QA-PROJ-123-"));
        // Check date is included
        assert!(name.contains(&Utc::now().format("%Y-%m-%d").to_string()));
    }

    #[test]
    fn test_generate_run_name_format() {
        let name = generate_run_name("TEST-456");
        // Should start with QA-TEST-456-
        assert!(name.starts_with("QA-TEST-456-"));
        // Should have date suffix in format YYYY-MM-DD
        let date_part = &name["QA-TEST-456-".len()..];
        assert_eq!(date_part.len(), 10); // YYYY-MM-DD
        assert!(date_part.chars().nth(4) == Some('-'));
        assert!(date_part.chars().nth(7) == Some('-'));
    }
}
