//! Report generation API endpoints.
//!
//! Refactored to use unified `ApiError` for cleaner error handling.

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use tracing::info;
use utoipa::ToSchema;
use uuid::Uuid;

use qa_pms_workflow::{get_instance, get_step_results, get_template};

use crate::app::AppState;
use qa_pms_core::error::ApiError;

/// Result type alias for API handlers.
type ApiResult<T> = Result<T, ApiError>;

/// Helper trait to convert sqlx errors to ApiError.
trait SqlxResultExt<T> {
    fn map_db_err(self) -> Result<T, ApiError>;
}

impl<T> SqlxResultExt<T> for Result<T, sqlx::Error> {
    fn map_db_err(self) -> Result<T, ApiError> {
        self.map_err(|e| ApiError::Internal(e.into()))
    }
}

/// Create the reports router.
pub fn router() -> Router<AppState> {
    Router::new()
        .route("/api/v1/reports", post(generate_report))
        .route("/api/v1/reports/:id", get(get_report))
        .route("/api/v1/reports/workflow/:workflow_id", get(get_report_by_workflow))
}

// ============================================================================
// Types
// ============================================================================

/// Request to generate a report.
#[derive(Debug, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct GenerateReportRequest {
    pub workflow_instance_id: Uuid,
    pub ticket_title: Option<String>,
}

/// Report content structure.
#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ReportContent {
    pub steps: Vec<ReportStep>,
    pub notes: Vec<String>,
    pub tests_covered: Vec<String>,
    pub strategies: Vec<String>,
}

impl Default for ReportContent {
    fn default() -> Self {
        Self {
            steps: vec![],
            notes: vec![],
            tests_covered: vec![],
            strategies: vec![],
        }
    }
}

/// Step in report.
#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ReportStep {
    pub index: usize,
    pub name: String,
    pub status: String,
    pub notes: Option<String>,
    pub time_seconds: i32,
}

/// Report response.
#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ReportResponse {
    pub id: Uuid,
    pub workflow_instance_id: Uuid,
    pub ticket_id: String,
    pub ticket_title: Option<String>,
    pub template_name: String,
    pub content: ReportContent,
    pub total_time_seconds: i32,
    pub generated_at: String,
}

/// Database row for report queries.
#[derive(sqlx::FromRow)]
struct ReportRow {
    id: Uuid,
    workflow_instance_id: Uuid,
    ticket_id: String,
    ticket_title: Option<String>,
    template_name: String,
    content: serde_json::Value,
    total_time_seconds: i32,
    generated_at: chrono::DateTime<chrono::Utc>,
}

impl From<ReportRow> for ReportResponse {
    fn from(row: ReportRow) -> Self {
        let content: ReportContent = serde_json::from_value(row.content)
            .unwrap_or_default();

        Self {
            id: row.id,
            workflow_instance_id: row.workflow_instance_id,
            ticket_id: row.ticket_id,
            ticket_title: row.ticket_title,
            template_name: row.template_name,
            content,
            total_time_seconds: row.total_time_seconds,
            generated_at: row.generated_at.to_rfc3339(),
        }
    }
}

// ============================================================================
// Helper Functions
// ============================================================================

/// Fetch a report by a SQL query condition.
async fn fetch_report(
    db: &sqlx::PgPool,
    query: &str,
    bind_value: Uuid,
) -> ApiResult<ReportResponse> {
    sqlx::query_as::<_, ReportRow>(query)
        .bind(bind_value)
        .fetch_optional(db)
        .await
        .map_db_err()?
        .map(ReportResponse::from)
        .ok_or_else(|| ApiError::NotFound("Report not found".into()))
}

// ============================================================================
// Handlers
// ============================================================================

/// Generate a report for a completed workflow.
#[utoipa::path(
    post,
    path = "/api/v1/reports",
    request_body = GenerateReportRequest,
    responses(
        (status = 201, description = "Report generated", body = ReportResponse),
        (status = 404, description = "Workflow not found"),
        (status = 500, description = "Internal server error")
    ),
    tag = "Reports"
)]
pub async fn generate_report(
    State(state): State<AppState>,
    Json(request): Json<GenerateReportRequest>,
) -> ApiResult<impl IntoResponse> {
    // Get workflow instance
    let instance = get_instance(&state.db, request.workflow_instance_id)
        .await
        .map_db_err()?
        .ok_or_else(|| ApiError::NotFound("Workflow not found".into()))?;

    // Get template
    let template = get_template(&state.db, instance.template_id)
        .await
        .map_db_err()?
        .ok_or_else(|| ApiError::NotFound("Template not found".into()))?;

    // Get step results
    let step_results = get_step_results(&state.db, request.workflow_instance_id)
        .await
        .unwrap_or_default();

    // Build report content
    let steps: Vec<ReportStep> = template
        .steps()
        .iter()
        .enumerate()
        .map(|(i, step)| {
            let result = step_results.iter().find(|r| r.step_index == i as i32);
            ReportStep {
                index: i,
                name: step.name.clone(),
                status: result.map_or("pending".to_string(), |r| r.status.clone()),
                notes: result.and_then(|r| r.notes.clone()),
                time_seconds: 0, // TODO: Get from time sessions
            }
        })
        .collect();

    let notes: Vec<String> = steps.iter().filter_map(|s| s.notes.clone()).collect();

    let content = ReportContent {
        steps,
        notes,
        tests_covered: vec![],
        strategies: vec![],
    };

    let content_json = serde_json::to_value(&content).unwrap_or_default();

    // Save report to database
    let report_id = Uuid::new_v4();
    sqlx::query(
        r#"
        INSERT INTO workflow_reports (id, workflow_instance_id, ticket_id, ticket_title, template_name, content, total_time_seconds)
        VALUES ($1, $2, $3, $4, $5, $6, $7)
        "#,
    )
    .bind(report_id)
    .bind(request.workflow_instance_id)
    .bind(&instance.ticket_id)
    .bind(&request.ticket_title)
    .bind(&template.name)
    .bind(&content_json)
    .bind(0i32)
    .execute(&state.db)
    .await
    .map_db_err()?;

    info!(report_id = %report_id, workflow_id = %request.workflow_instance_id, "Generated report");

    Ok((
        StatusCode::CREATED,
        Json(ReportResponse {
            id: report_id,
            workflow_instance_id: request.workflow_instance_id,
            ticket_id: instance.ticket_id,
            ticket_title: request.ticket_title,
            template_name: template.name,
            content,
            total_time_seconds: 0,
            generated_at: chrono::Utc::now().to_rfc3339(),
        }),
    ))
}

/// Get a report by ID.
#[utoipa::path(
    get,
    path = "/api/v1/reports/{id}",
    params(
        ("id" = Uuid, Path, description = "Report ID")
    ),
    responses(
        (status = 200, description = "Report details", body = ReportResponse),
        (status = 404, description = "Report not found"),
        (status = 500, description = "Internal server error")
    ),
    tag = "Reports"
)]
pub async fn get_report(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<ReportResponse>> {
    fetch_report(
        &state.db,
        r#"
        SELECT id, workflow_instance_id, ticket_id, ticket_title, template_name, content, total_time_seconds, generated_at
        FROM workflow_reports WHERE id = $1
        "#,
        id,
    )
    .await
    .map(Json)
}

/// Get report by workflow ID.
#[utoipa::path(
    get,
    path = "/api/v1/reports/workflow/{workflow_id}",
    params(
        ("workflow_id" = Uuid, Path, description = "Workflow instance ID")
    ),
    responses(
        (status = 200, description = "Report details", body = ReportResponse),
        (status = 404, description = "Report not found"),
        (status = 500, description = "Internal server error")
    ),
    tag = "Reports"
)]
pub async fn get_report_by_workflow(
    State(state): State<AppState>,
    Path(workflow_id): Path<Uuid>,
) -> ApiResult<Json<ReportResponse>> {
    fetch_report(
        &state.db,
        r#"
        SELECT id, workflow_instance_id, ticket_id, ticket_title, template_name, content, total_time_seconds, generated_at
        FROM workflow_reports WHERE workflow_instance_id = $1
        ORDER BY generated_at DESC
        LIMIT 1
        "#,
        workflow_id,
    )
    .await
    .map(Json)
}
