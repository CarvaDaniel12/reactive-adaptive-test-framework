//! Support portal API endpoints.
//!
//! Epic 12: Support Portal & Troubleshooting

use axum::{
    extract::{Path, Query, State},
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};
use uuid::Uuid;

use qa_pms_core::ApiError;
use qa_pms_support::{
    CreateErrorLogInput, CreateKbEntryInput, DiagnosticsService, ErrorLog, ErrorLogFilter,
    ErrorLogSort, ErrorStatus, KnowledgeBaseEntry, KnowledgeBaseService, Pagination, SupportDashboardSummary, SupportRepository, TroubleshootingSuggestion,
    UpdateErrorStatusInput, UpdateKbEntryInput, DiagnosticsReport,
};

use crate::app::AppState;

type ApiResult<T> = Result<T, ApiError>;

/// Create the support router.
pub fn router() -> Router<AppState> {
    Router::new()
        // Error logs
        .route("/errors", get(list_error_logs).post(create_error_log))
        .route("/errors/:id", get(get_error_log).put(update_error_status))
        .route("/errors/:id/suggestions", get(get_suggestions))
        // Dashboard
        .route("/dashboard", get(get_dashboard_summary))
        // Diagnostics
        .route("/diagnostics", get(run_all_diagnostics))
        .route("/diagnostics/:integration", get(run_diagnostic))
        // Knowledge base
        .route("/kb", get(list_kb_entries).post(create_kb_entry))
        .route("/kb/:id", get(get_kb_entry).put(update_kb_entry).delete(delete_kb_entry))
        .route("/kb/:id/rate", post(rate_kb_entry))
}

// ==================== Request/Response Types ====================

/// Query parameters for listing error logs.
#[derive(Debug, Deserialize, IntoParams)]
#[serde(rename_all = "camelCase")]
pub struct ErrorLogQuery {
    /// Filter by status
    pub status: Option<String>,
    /// Filter by severity
    pub severity: Option<String>,
    /// Filter by source
    pub source: Option<String>,
    /// Filter by user ID
    pub user_id: Option<Uuid>,
    /// Search in error message
    pub search: Option<String>,
    /// Sort order
    #[serde(default)]
    pub sort: Option<String>,
    /// Page number (1-indexed)
    #[serde(default = "default_page")]
    pub page: i32,
    /// Items per page
    #[serde(default = "default_per_page")]
    pub per_page: i32,
}

const fn default_page() -> i32 { 1 }
const fn default_per_page() -> i32 { 20 }

/// Response for error log list.
#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ErrorLogsResponse {
    /// Error logs
    pub items: Vec<ErrorLog>,
    /// Total count
    pub total: i64,
    /// Current page
    pub page: i32,
    /// Items per page
    pub per_page: i32,
    /// Total pages
    pub total_pages: i32,
}

/// Request to create an error log.
#[derive(Debug, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CreateErrorRequest {
    /// Error message
    pub message: String,
    /// Stack trace
    pub stack_trace: Option<String>,
    /// Severity (low, medium, high, critical)
    pub severity: Option<String>,
    /// Source (frontend, backend, integration, database)
    pub source: Option<String>,
    /// User ID
    pub user_id: Option<Uuid>,
    /// Session ID
    pub session_id: Option<String>,
    /// Page URL
    pub page_url: Option<String>,
    /// Action being performed
    pub action: Option<String>,
    /// Browser info
    pub browser_info: Option<String>,
    /// Device info
    pub device_info: Option<String>,
    /// Additional context
    #[serde(default)]
    pub context: serde_json::Value,
}

/// Request to update error status.
#[derive(Debug, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct UpdateStatusRequest {
    /// New status (new, investigating, resolved, dismissed)
    pub status: String,
    /// Resolution notes
    pub resolution_notes: Option<String>,
    /// Link to KB entry
    pub kb_entry_id: Option<Uuid>,
}

/// Response for suggestions.
#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct SuggestionsResponse {
    /// List of suggestions
    pub suggestions: Vec<TroubleshootingSuggestion>,
}

/// Response for dashboard summary.
#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct DashboardSummaryResponse {
    /// Summary data
    #[serde(flatten)]
    pub summary: SupportDashboardSummary,
}

/// Response for diagnostics.
#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct DiagnosticsResponse {
    /// Diagnostics report
    #[serde(flatten)]
    pub report: DiagnosticsReport,
}

/// Query parameters for KB list.
#[derive(Debug, Deserialize, IntoParams)]
#[serde(rename_all = "camelCase")]
pub struct KbQuery {
    /// Search term
    pub search: Option<String>,
    /// Page number
    #[serde(default = "default_page")]
    pub page: i32,
    /// Items per page
    #[serde(default = "default_per_page")]
    pub per_page: i32,
}

/// Response for KB list.
#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct KbEntriesResponse {
    /// KB entries
    pub items: Vec<KnowledgeBaseEntry>,
    /// Total count
    pub total: i64,
    /// Current page
    pub page: i32,
    /// Items per page
    pub per_page: i32,
    /// Total pages
    pub total_pages: i32,
}

/// Request to create KB entry.
#[derive(Debug, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CreateKbRequest {
    /// Title
    pub title: String,
    /// Problem description
    pub problem: String,
    /// Cause
    pub cause: String,
    /// Solution
    pub solution: String,
    /// Related error messages
    #[serde(default)]
    pub related_errors: Vec<String>,
    /// Tags
    #[serde(default)]
    pub tags: Vec<String>,
}

/// Request to update KB entry.
#[derive(Debug, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct UpdateKbRequest {
    /// Title
    pub title: Option<String>,
    /// Problem description
    pub problem: Option<String>,
    /// Cause
    pub cause: Option<String>,
    /// Solution
    pub solution: Option<String>,
    /// Related error messages
    pub related_errors: Option<Vec<String>>,
    /// Tags
    pub tags: Option<Vec<String>>,
}

/// Request to rate KB entry.
#[derive(Debug, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct RateKbRequest {
    /// Whether the entry was helpful
    pub helpful: bool,
}

/// Simple success response.
#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct SuccessResponse {
    /// Success message
    pub message: String,
}

// ==================== Handlers ====================

/// List error logs with filtering and pagination.
#[utoipa::path(
    get,
    path = "/api/v1/support/errors",
    params(ErrorLogQuery),
    responses(
        (status = 200, description = "Error logs retrieved", body = ErrorLogsResponse)
    ),
    tag = "Support"
)]
pub async fn list_error_logs(
    State(state): State<AppState>,
    Query(query): Query<ErrorLogQuery>,
) -> ApiResult<Json<ErrorLogsResponse>> {
    let repo = SupportRepository::new(state.db.clone());

    let filter = ErrorLogFilter {
        status: query.status.and_then(|s| parse_status(&s)),
        severity: query.severity.and_then(|s| parse_severity(&s)),
        source: query.source.and_then(|s| parse_source(&s)),
        user_id: query.user_id,
        search: query.search,
        from_date: None,
        to_date: None,
    };

    let sort = query.sort.map(|s| parse_sort(&s)).unwrap_or_default();
    let pagination = Pagination {
        page: query.page,
        per_page: query.per_page,
    };

    let result = repo.list_error_logs(filter, sort, pagination).await
        .map_err(|e| ApiError::Internal(e.into()))?;

    Ok(Json(ErrorLogsResponse {
        items: result.items,
        total: result.total,
        page: result.page,
        per_page: result.per_page,
        total_pages: result.total_pages,
    }))
}

/// Create or increment an error log.
#[utoipa::path(
    post,
    path = "/api/v1/support/errors",
    request_body = CreateErrorRequest,
    responses(
        (status = 201, description = "Error log created", body = ErrorLog)
    ),
    tag = "Support"
)]
pub async fn create_error_log(
    State(state): State<AppState>,
    Json(req): Json<CreateErrorRequest>,
) -> ApiResult<Json<ErrorLog>> {
    let repo = SupportRepository::new(state.db.clone());

    let input = CreateErrorLogInput {
        message: req.message,
        stack_trace: req.stack_trace,
        severity: req.severity.and_then(|s| parse_severity(&s)).unwrap_or_default(),
        source: req.source.and_then(|s| parse_source(&s)).unwrap_or_default(),
        user_id: req.user_id,
        session_id: req.session_id,
        page_url: req.page_url,
        action: req.action,
        browser_info: req.browser_info,
        device_info: req.device_info,
        context: req.context,
    };

    let error = repo.create_or_increment_error(input).await
        .map_err(|e| ApiError::Internal(e.into()))?;

    Ok(Json(error))
}

/// Get a specific error log.
#[utoipa::path(
    get,
    path = "/api/v1/support/errors/{id}",
    params(("id" = Uuid, Path, description = "Error log ID")),
    responses(
        (status = 200, description = "Error log retrieved", body = ErrorLog),
        (status = 404, description = "Error log not found")
    ),
    tag = "Support"
)]
pub async fn get_error_log(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<ErrorLog>> {
    let repo = SupportRepository::new(state.db.clone());

    let error = repo.get_error_log(id).await
        .map_err(|e| match e {
            qa_pms_support::SupportError::ErrorLogNotFound(_) => ApiError::NotFound("Error log not found".into()),
            _ => ApiError::Internal(e.into()),
        })?;

    Ok(Json(error))
}

/// Update error log status.
#[utoipa::path(
    put,
    path = "/api/v1/support/errors/{id}",
    params(("id" = Uuid, Path, description = "Error log ID")),
    request_body = UpdateStatusRequest,
    responses(
        (status = 200, description = "Error log updated", body = ErrorLog),
        (status = 404, description = "Error log not found")
    ),
    tag = "Support"
)]
pub async fn update_error_status(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(req): Json<UpdateStatusRequest>,
) -> ApiResult<Json<ErrorLog>> {
    let repo = SupportRepository::new(state.db.clone());

    let status = parse_status(&req.status)
        .ok_or_else(|| ApiError::Validation(format!("Invalid status: {}", req.status)))?;

    let input = UpdateErrorStatusInput {
        status,
        resolution_notes: req.resolution_notes,
        kb_entry_id: req.kb_entry_id,
    };

    let error = repo.update_error_status(id, input).await
        .map_err(|e| match e {
            qa_pms_support::SupportError::ErrorLogNotFound(_) => ApiError::NotFound("Error log not found".into()),
            _ => ApiError::Internal(e.into()),
        })?;

    Ok(Json(error))
}

/// Get troubleshooting suggestions for an error.
#[utoipa::path(
    get,
    path = "/api/v1/support/errors/{id}/suggestions",
    params(("id" = Uuid, Path, description = "Error log ID")),
    responses(
        (status = 200, description = "Suggestions retrieved", body = SuggestionsResponse),
        (status = 404, description = "Error log not found")
    ),
    tag = "Support"
)]
pub async fn get_suggestions(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<SuggestionsResponse>> {
    let repo = SupportRepository::new(state.db.clone());
    let kb_service = KnowledgeBaseService::new(state.db.clone());

    let error = repo.get_error_log(id).await
        .map_err(|e| match e {
            qa_pms_support::SupportError::ErrorLogNotFound(_) => ApiError::NotFound("Error log not found".into()),
            _ => ApiError::Internal(e.into()),
        })?;

    let suggestions = kb_service.get_suggestions(&error).await
        .map_err(|e| ApiError::Internal(e.into()))?;

    Ok(Json(SuggestionsResponse { suggestions }))
}

/// Get support dashboard summary.
#[utoipa::path(
    get,
    path = "/api/v1/support/dashboard",
    responses(
        (status = 200, description = "Dashboard summary", body = DashboardSummaryResponse)
    ),
    tag = "Support"
)]
pub async fn get_dashboard_summary(
    State(state): State<AppState>,
) -> ApiResult<Json<DashboardSummaryResponse>> {
    let repo = SupportRepository::new(state.db.clone());

    let summary = repo.get_dashboard_summary().await
        .map_err(|e| ApiError::Internal(e.into()))?;

    Ok(Json(DashboardSummaryResponse { summary }))
}

/// Run diagnostics on all integrations.
#[utoipa::path(
    get,
    path = "/api/v1/support/diagnostics",
    responses(
        (status = 200, description = "Diagnostics report", body = DiagnosticsResponse)
    ),
    tag = "Support"
)]
pub async fn run_all_diagnostics(
    State(state): State<AppState>,
) -> ApiResult<Json<DiagnosticsResponse>> {
    let service = DiagnosticsService::new(state.db.clone());

    let report = service.run_all_diagnostics().await
        .map_err(|e| ApiError::Internal(e.into()))?;

    Ok(Json(DiagnosticsResponse { report }))
}

/// Run diagnostic for a specific integration.
#[utoipa::path(
    get,
    path = "/api/v1/support/diagnostics/{integration}",
    params(("integration" = String, Path, description = "Integration name")),
    responses(
        (status = 200, description = "Diagnostic result", body = qa_pms_support::DiagnosticResult)
    ),
    tag = "Support"
)]
pub async fn run_diagnostic(
    State(state): State<AppState>,
    Path(integration): Path<String>,
) -> ApiResult<Json<qa_pms_support::DiagnosticResult>> {
    let service = DiagnosticsService::new(state.db.clone());

    let result = service.run_diagnostic(&integration).await
        .map_err(|e| match e {
            qa_pms_support::SupportError::InvalidInput(msg) => ApiError::Validation(msg),
            _ => ApiError::Internal(e.into()),
        })?;

    Ok(Json(result))
}

/// List knowledge base entries.
#[utoipa::path(
    get,
    path = "/api/v1/support/kb",
    params(KbQuery),
    responses(
        (status = 200, description = "KB entries retrieved", body = KbEntriesResponse)
    ),
    tag = "Support"
)]
pub async fn list_kb_entries(
    State(state): State<AppState>,
    Query(query): Query<KbQuery>,
) -> ApiResult<Json<KbEntriesResponse>> {
    let repo = SupportRepository::new(state.db.clone());

    let pagination = Pagination {
        page: query.page,
        per_page: query.per_page,
    };

    let result = repo.list_kb_entries(query.search.as_deref(), pagination).await
        .map_err(|e| ApiError::Internal(e.into()))?;

    Ok(Json(KbEntriesResponse {
        items: result.items,
        total: result.total,
        page: result.page,
        per_page: result.per_page,
        total_pages: result.total_pages,
    }))
}

/// Create a knowledge base entry.
#[utoipa::path(
    post,
    path = "/api/v1/support/kb",
    request_body = CreateKbRequest,
    responses(
        (status = 201, description = "KB entry created", body = KnowledgeBaseEntry)
    ),
    tag = "Support"
)]
pub async fn create_kb_entry(
    State(state): State<AppState>,
    Json(req): Json<CreateKbRequest>,
) -> ApiResult<Json<KnowledgeBaseEntry>> {
    let repo = SupportRepository::new(state.db.clone());

    let input = CreateKbEntryInput {
        title: req.title,
        problem: req.problem,
        cause: req.cause,
        solution: req.solution,
        related_errors: req.related_errors,
        tags: req.tags,
    };

    let entry = repo.create_kb_entry(input).await
        .map_err(|e| ApiError::Internal(e.into()))?;

    Ok(Json(entry))
}

/// Get a knowledge base entry.
#[utoipa::path(
    get,
    path = "/api/v1/support/kb/{id}",
    params(("id" = Uuid, Path, description = "KB entry ID")),
    responses(
        (status = 200, description = "KB entry retrieved", body = KnowledgeBaseEntry),
        (status = 404, description = "KB entry not found")
    ),
    tag = "Support"
)]
pub async fn get_kb_entry(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<KnowledgeBaseEntry>> {
    let repo = SupportRepository::new(state.db.clone());

    // Increment view count
    let _ = repo.increment_kb_view(id).await;

    let entry = repo.get_kb_entry(id).await
        .map_err(|e| match e {
            qa_pms_support::SupportError::KbEntryNotFound(_) => ApiError::NotFound("KB entry not found".into()),
            _ => ApiError::Internal(e.into()),
        })?;

    Ok(Json(entry))
}

/// Update a knowledge base entry.
#[utoipa::path(
    put,
    path = "/api/v1/support/kb/{id}",
    params(("id" = Uuid, Path, description = "KB entry ID")),
    request_body = UpdateKbRequest,
    responses(
        (status = 200, description = "KB entry updated", body = KnowledgeBaseEntry),
        (status = 404, description = "KB entry not found")
    ),
    tag = "Support"
)]
pub async fn update_kb_entry(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(req): Json<UpdateKbRequest>,
) -> ApiResult<Json<KnowledgeBaseEntry>> {
    let repo = SupportRepository::new(state.db.clone());

    let input = UpdateKbEntryInput {
        title: req.title,
        problem: req.problem,
        cause: req.cause,
        solution: req.solution,
        related_errors: req.related_errors,
        tags: req.tags,
    };

    let entry = repo.update_kb_entry(id, input).await
        .map_err(|e| match e {
            qa_pms_support::SupportError::KbEntryNotFound(_) => ApiError::NotFound("KB entry not found".into()),
            _ => ApiError::Internal(e.into()),
        })?;

    Ok(Json(entry))
}

/// Delete a knowledge base entry.
#[utoipa::path(
    delete,
    path = "/api/v1/support/kb/{id}",
    params(("id" = Uuid, Path, description = "KB entry ID")),
    responses(
        (status = 200, description = "KB entry deleted", body = SuccessResponse),
        (status = 404, description = "KB entry not found")
    ),
    tag = "Support"
)]
pub async fn delete_kb_entry(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<SuccessResponse>> {
    let repo = SupportRepository::new(state.db.clone());

    repo.delete_kb_entry(id).await
        .map_err(|e| match e {
            qa_pms_support::SupportError::KbEntryNotFound(_) => ApiError::NotFound("KB entry not found".into()),
            _ => ApiError::Internal(e.into()),
        })?;

    Ok(Json(SuccessResponse {
        message: "KB entry deleted successfully".into(),
    }))
}

/// Rate a knowledge base entry as helpful or not.
#[utoipa::path(
    post,
    path = "/api/v1/support/kb/{id}/rate",
    params(("id" = Uuid, Path, description = "KB entry ID")),
    request_body = RateKbRequest,
    responses(
        (status = 200, description = "Rating recorded", body = SuccessResponse)
    ),
    tag = "Support"
)]
pub async fn rate_kb_entry(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(req): Json<RateKbRequest>,
) -> ApiResult<Json<SuccessResponse>> {
    let repo = SupportRepository::new(state.db.clone());

    repo.rate_kb_entry(id, req.helpful).await
        .map_err(|e| ApiError::Internal(e.into()))?;

    Ok(Json(SuccessResponse {
        message: "Rating recorded".into(),
    }))
}

// ==================== Helper Functions ====================

fn parse_status(s: &str) -> Option<ErrorStatus> {
    match s.to_lowercase().as_str() {
        "new" => Some(ErrorStatus::New),
        "investigating" => Some(ErrorStatus::Investigating),
        "resolved" => Some(ErrorStatus::Resolved),
        "dismissed" => Some(ErrorStatus::Dismissed),
        _ => None,
    }
}

fn parse_severity(s: &str) -> Option<qa_pms_support::ErrorSeverity> {
    match s.to_lowercase().as_str() {
        "low" => Some(qa_pms_support::ErrorSeverity::Low),
        "medium" => Some(qa_pms_support::ErrorSeverity::Medium),
        "high" => Some(qa_pms_support::ErrorSeverity::High),
        "critical" => Some(qa_pms_support::ErrorSeverity::Critical),
        _ => None,
    }
}

fn parse_source(s: &str) -> Option<qa_pms_support::ErrorSource> {
    match s.to_lowercase().as_str() {
        "frontend" => Some(qa_pms_support::ErrorSource::Frontend),
        "backend" => Some(qa_pms_support::ErrorSource::Backend),
        "integration" => Some(qa_pms_support::ErrorSource::Integration),
        "database" => Some(qa_pms_support::ErrorSource::Database),
        "unknown" => Some(qa_pms_support::ErrorSource::Unknown),
        _ => None,
    }
}

fn parse_sort(s: &str) -> ErrorLogSort {
    match s.to_lowercase().as_str() {
        "last_seen_asc" => ErrorLogSort::LastSeenAsc,
        "severity_desc" | "severity" => ErrorLogSort::SeverityDesc,
        "occurrence_desc" | "occurrence" => ErrorLogSort::OccurrenceDesc,
        _ => ErrorLogSort::LastSeenDesc,
    }
}
