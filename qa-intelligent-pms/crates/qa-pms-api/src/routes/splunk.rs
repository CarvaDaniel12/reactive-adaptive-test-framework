//! Splunk API endpoints.
//!
//! Epic 11: Provides endpoints for:
//! - Query template CRUD operations
//! - Query preparation and execution simulation
//! - Query history tracking

use axum::{
    extract::{Path, Query, State},
    routing::{delete, get, post, put},
    Json, Router,
};
use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use utoipa::ToSchema;
use uuid::Uuid;

use crate::app::AppState;
use qa_pms_core::error::ApiError;
use qa_pms_splunk::{
    CreateTemplateInput, LogEntry, PreparedQuery, QueryTemplate, QueryTemplateService,
    TemplateCategory, UpdateTemplateInput,
};

type ApiResult<T> = Result<T, ApiError>;

/// Create the Splunk router.
pub fn router() -> Router<AppState> {
    Router::new()
        // Templates
        .route("/api/v1/splunk/templates", get(list_templates))
        .route("/api/v1/splunk/templates", post(create_template))
        .route("/api/v1/splunk/templates/:id", get(get_template))
        .route("/api/v1/splunk/templates/:id", put(update_template))
        .route("/api/v1/splunk/templates/:id", delete(delete_template))
        // Query operations
        .route("/api/v1/splunk/query/prepare", post(prepare_query))
        .route("/api/v1/splunk/query/execute", post(execute_query))
        .route("/api/v1/splunk/query/history", get(get_query_history))
        // Placeholders info
        .route("/api/v1/splunk/placeholders", get(get_placeholders))
}

// ============================================================================
// Request/Response Types
// ============================================================================

/// Query parameters for listing templates.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListTemplatesQuery {
    /// Filter by category.
    pub category: Option<TemplateCategory>,
}

/// Request to create a new template.
#[derive(Debug, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CreateTemplateRequest {
    /// Template name.
    pub name: String,
    /// Template description.
    pub description: Option<String>,
    /// SPL query with placeholders.
    pub query: String,
    /// Category for grouping.
    pub category: TemplateCategory,
}

/// Request to update a template.
#[derive(Debug, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct UpdateTemplateRequest {
    /// Updated name.
    pub name: Option<String>,
    /// Updated description.
    pub description: Option<String>,
    /// Updated query.
    pub query: Option<String>,
    /// Updated category.
    pub category: Option<TemplateCategory>,
}

/// Response containing a single template.
#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct TemplateResponse {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub query: String,
    pub category: TemplateCategory,
    pub is_system: bool,
    pub placeholders: Vec<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<QueryTemplate> for TemplateResponse {
    fn from(t: QueryTemplate) -> Self {
        let placeholders = QueryTemplateService::extract_placeholders(&t.query);
        Self {
            id: t.id,
            name: t.name,
            description: t.description,
            query: t.query,
            category: t.category,
            is_system: t.is_system,
            placeholders,
            created_at: t.created_at,
            updated_at: t.updated_at,
        }
    }
}

/// Response containing a list of templates.
#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct TemplatesListResponse {
    pub templates: Vec<TemplateResponse>,
    pub total: usize,
}

/// Request to prepare a query.
#[derive(Debug, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct PrepareQueryRequest {
    /// Template ID to use.
    pub template_id: Option<Uuid>,
    /// Raw query (if not using template).
    pub raw_query: Option<String>,
    /// Placeholder values.
    #[serde(default)]
    pub placeholders: HashMap<String, String>,
    /// Time range start (default: -24h).
    pub time_start: Option<DateTime<Utc>>,
    /// Time range end (default: now).
    pub time_end: Option<DateTime<Utc>>,
    /// Index to search.
    pub index: Option<String>,
}

/// Response with prepared query.
#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct PrepareQueryResponse {
    pub query: String,
    pub time_start: DateTime<Utc>,
    pub time_end: DateTime<Utc>,
    pub index: Option<String>,
    pub splunk_url: Option<String>,
}

/// Request to execute a query (simulation).
#[derive(Debug, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ExecuteQueryRequest {
    /// The query to execute.
    pub query: String,
    /// Time range start.
    pub time_start: DateTime<Utc>,
    /// Time range end.
    pub time_end: DateTime<Utc>,
    /// Index to search.
    pub index: Option<String>,
    /// Maximum results to return.
    #[serde(default = "default_limit")]
    pub limit: i32,
}

const fn default_limit() -> i32 {
    100
}

/// Response with query results.
#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ExecuteQueryResponse {
    pub query: String,
    pub entries: Vec<LogEntryResponse>,
    pub total_count: i64,
    pub truncated: bool,
    pub execution_time_ms: i64,
    pub message: String,
}

/// Log entry response.
#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct LogEntryResponse {
    pub timestamp: DateTime<Utc>,
    pub level: String,
    pub message: String,
    pub source: Option<String>,
    pub host: Option<String>,
    pub fields: serde_json::Value,
}

/// Query history entry.
#[derive(Debug, Serialize, ToSchema, sqlx::FromRow)]
#[serde(rename_all = "camelCase")]
pub struct QueryHistoryEntry {
    pub id: Uuid,
    pub query: String,
    pub template_name: Option<String>,
    pub time_start: DateTime<Utc>,
    pub time_end: DateTime<Utc>,
    pub execution_time_ms: Option<i32>,
    pub result_count: Option<i32>,
    pub created_at: DateTime<Utc>,
}

/// Query history response.
#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct QueryHistoryResponse {
    pub entries: Vec<QueryHistoryEntry>,
    pub total: i64,
}

/// Placeholder information.
#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct PlaceholderInfo {
    pub key: String,
    pub label: String,
    pub description: String,
    pub example: String,
}

/// Placeholders response.
#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct PlaceholdersResponse {
    pub placeholders: Vec<PlaceholderInfo>,
}

// ============================================================================
// Handlers
// ============================================================================

/// List all query templates.
#[utoipa::path(
    get,
    path = "/api/v1/splunk/templates",
    params(
        ("category" = Option<String>, Query, description = "Filter by category")
    ),
    responses(
        (status = 200, description = "List of templates", body = TemplatesListResponse),
        (status = 500, description = "Internal server error")
    ),
    tag = "Splunk"
)]
pub async fn list_templates(
    State(state): State<AppState>,
    Query(query): Query<ListTemplatesQuery>,
) -> ApiResult<Json<TemplatesListResponse>> {
    let service = QueryTemplateService::new(state.db.clone());

    // TODO: Get user_id from auth context
    let user_id: Option<Uuid> = None;

    let templates = service
        .list_templates(query.category, user_id)
        .await
        .map_err(|e| ApiError::Internal(anyhow::anyhow!("Failed to list templates: {e}")))?;

    let responses: Vec<TemplateResponse> = templates.into_iter().map(Into::into).collect();
    let total = responses.len();

    Ok(Json(TemplatesListResponse {
        templates: responses,
        total,
    }))
}

/// Get a template by ID.
#[utoipa::path(
    get,
    path = "/api/v1/splunk/templates/{id}",
    params(
        ("id" = Uuid, Path, description = "Template ID")
    ),
    responses(
        (status = 200, description = "Template details", body = TemplateResponse),
        (status = 404, description = "Template not found"),
        (status = 500, description = "Internal server error")
    ),
    tag = "Splunk"
)]
pub async fn get_template(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<TemplateResponse>> {
    let service = QueryTemplateService::new(state.db.clone());

    let template = service.get_template(id).await.map_err(|e| match e {
        qa_pms_splunk::SplunkError::TemplateNotFound(_) => {
            ApiError::NotFound(format!("Template {id} not found"))
        }
        _ => ApiError::Internal(anyhow::anyhow!("Failed to get template: {e}")),
    })?;

    Ok(Json(template.into()))
}

/// Create a new template.
#[utoipa::path(
    post,
    path = "/api/v1/splunk/templates",
    request_body = CreateTemplateRequest,
    responses(
        (status = 201, description = "Template created", body = TemplateResponse),
        (status = 400, description = "Invalid request"),
        (status = 500, description = "Internal server error")
    ),
    tag = "Splunk"
)]
pub async fn create_template(
    State(state): State<AppState>,
    Json(req): Json<CreateTemplateRequest>,
) -> ApiResult<Json<TemplateResponse>> {
    let service = QueryTemplateService::new(state.db.clone());

    // TODO: Get user_id from auth context
    let user_id = Uuid::new_v4(); // Placeholder

    let input = CreateTemplateInput {
        name: req.name,
        description: req.description,
        query: req.query,
        category: req.category,
    };

    let template = service
        .create_template(input, user_id)
        .await
        .map_err(|e| match e {
            qa_pms_splunk::SplunkError::InvalidTemplate(msg) => ApiError::Validation(msg),
            _ => ApiError::Internal(anyhow::anyhow!("Failed to create template: {e}")),
        })?;

    Ok(Json(template.into()))
}

/// Update a template.
#[utoipa::path(
    put,
    path = "/api/v1/splunk/templates/{id}",
    params(
        ("id" = Uuid, Path, description = "Template ID")
    ),
    request_body = UpdateTemplateRequest,
    responses(
        (status = 200, description = "Template updated", body = TemplateResponse),
        (status = 400, description = "Invalid request"),
        (status = 404, description = "Template not found"),
        (status = 500, description = "Internal server error")
    ),
    tag = "Splunk"
)]
pub async fn update_template(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(req): Json<UpdateTemplateRequest>,
) -> ApiResult<Json<TemplateResponse>> {
    let service = QueryTemplateService::new(state.db.clone());

    // TODO: Get user_id from auth context
    let user_id = Uuid::new_v4(); // Placeholder

    let input = UpdateTemplateInput {
        name: req.name,
        description: req.description,
        query: req.query,
        category: req.category,
    };

    let template = service
        .update_template(id, input, user_id)
        .await
        .map_err(|e| match e {
            qa_pms_splunk::SplunkError::TemplateNotFound(_) => {
                ApiError::NotFound(format!("Template {id} not found"))
            }
            qa_pms_splunk::SplunkError::InvalidTemplate(msg) => ApiError::Validation(msg),
            _ => ApiError::Internal(anyhow::anyhow!("Failed to update template: {e}")),
        })?;

    Ok(Json(template.into()))
}

/// Delete a template.
#[utoipa::path(
    delete,
    path = "/api/v1/splunk/templates/{id}",
    params(
        ("id" = Uuid, Path, description = "Template ID")
    ),
    responses(
        (status = 204, description = "Template deleted"),
        (status = 400, description = "Cannot delete system template"),
        (status = 404, description = "Template not found"),
        (status = 500, description = "Internal server error")
    ),
    tag = "Splunk"
)]
pub async fn delete_template(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> ApiResult<axum::http::StatusCode> {
    let service = QueryTemplateService::new(state.db.clone());

    // TODO: Get user_id from auth context
    let user_id = Uuid::new_v4(); // Placeholder

    service
        .delete_template(id, user_id)
        .await
        .map_err(|e| match e {
            qa_pms_splunk::SplunkError::TemplateNotFound(_) => {
                ApiError::NotFound(format!("Template {id} not found"))
            }
            qa_pms_splunk::SplunkError::InvalidTemplate(msg) => ApiError::Validation(msg),
            _ => ApiError::Internal(anyhow::anyhow!("Failed to delete template: {e}")),
        })?;

    Ok(axum::http::StatusCode::NO_CONTENT)
}

/// Prepare a query by filling placeholders.
#[utoipa::path(
    post,
    path = "/api/v1/splunk/query/prepare",
    request_body = PrepareQueryRequest,
    responses(
        (status = 200, description = "Prepared query", body = PrepareQueryResponse),
        (status = 400, description = "Missing placeholder or invalid request"),
        (status = 404, description = "Template not found"),
        (status = 500, description = "Internal server error")
    ),
    tag = "Splunk"
)]
pub async fn prepare_query(
    State(state): State<AppState>,
    Json(req): Json<PrepareQueryRequest>,
) -> ApiResult<Json<PrepareQueryResponse>> {
    let service = QueryTemplateService::new(state.db.clone());

    let now = Utc::now();
    let time_start = req.time_start.unwrap_or_else(|| now - Duration::hours(24));
    let time_end = req.time_end.unwrap_or(now);

    let prepared = if let Some(template_id) = req.template_id {
        let template = service
            .get_template(template_id)
            .await
            .map_err(|e| match e {
                qa_pms_splunk::SplunkError::TemplateNotFound(_) => {
                    ApiError::NotFound(format!("Template {template_id} not found"))
                }
                _ => ApiError::Internal(anyhow::anyhow!("Failed to get template: {e}")),
            })?;

        service
            .prepare_query(
                &template,
                &req.placeholders,
                time_start,
                time_end,
                req.index.clone(),
            )
            .map_err(|e| match e {
                qa_pms_splunk::SplunkError::MissingPlaceholder(p) => {
                    ApiError::Validation(format!("Missing placeholder value: {p}"))
                }
                _ => ApiError::Internal(anyhow::anyhow!("Failed to prepare query: {e}")),
            })?
    } else if let Some(raw_query) = req.raw_query {
        PreparedQuery {
            template_id: None,
            query: raw_query,
            time_start,
            time_end,
            index: req.index.clone(),
        }
    } else {
        return Err(ApiError::Validation(
            "Either template_id or raw_query must be provided".to_string(),
        ));
    };

    // Build Splunk URL if base URL is configured
    // TODO: Get Splunk base URL from config
    let splunk_url = None;

    Ok(Json(PrepareQueryResponse {
        query: prepared.query,
        time_start: prepared.time_start,
        time_end: prepared.time_end,
        index: prepared.index,
        splunk_url,
    }))
}

/// Execute a query (simulation - returns mock data).
///
/// Note: This is a simulation endpoint. Real Splunk Cloud integration
/// requires direct access to Splunk which is not available via API.
#[utoipa::path(
    post,
    path = "/api/v1/splunk/query/execute",
    request_body = ExecuteQueryRequest,
    responses(
        (status = 200, description = "Query results (simulated)", body = ExecuteQueryResponse),
        (status = 400, description = "Invalid request"),
        (status = 500, description = "Internal server error")
    ),
    tag = "Splunk"
)]
pub async fn execute_query(
    State(state): State<AppState>,
    Json(req): Json<ExecuteQueryRequest>,
) -> ApiResult<Json<ExecuteQueryResponse>> {
    let start_time = std::time::Instant::now();

    // TODO: Get user_id from auth context
    let user_id = Uuid::new_v4();

    // Generate mock log entries for demonstration
    let mock_entries = generate_mock_logs(&req.query, req.limit as usize);
    let total_count = mock_entries.len() as i64;

    let execution_time_ms = start_time.elapsed().as_millis() as i64;

    // Save to query history
    let _ = sqlx::query(
        r"
        INSERT INTO splunk_query_history (id, user_id, query, time_start, time_end, index_name, execution_time_ms, result_count)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
        ",
    )
    .bind(Uuid::new_v4())
    .bind(user_id)
    .bind(&req.query)
    .bind(req.time_start)
    .bind(req.time_end)
    .bind(&req.index)
    .bind(execution_time_ms as i32)
    .bind(total_count as i32)
    .execute(&state.db)
    .await;

    let entries: Vec<LogEntryResponse> = mock_entries
        .into_iter()
        .map(|e| LogEntryResponse {
            timestamp: e.timestamp,
            level: e.level,
            message: e.message,
            source: e.source,
            host: e.host,
            fields: e.fields,
        })
        .collect();

    Ok(Json(ExecuteQueryResponse {
        query: req.query,
        entries,
        total_count,
        truncated: false,
        execution_time_ms,
        message: "This is simulated data. For real Splunk queries, use the Splunk web interface with the prepared query.".to_string(),
    }))
}

/// Get query history for the current user.
#[utoipa::path(
    get,
    path = "/api/v1/splunk/query/history",
    responses(
        (status = 200, description = "Query history", body = QueryHistoryResponse),
        (status = 500, description = "Internal server error")
    ),
    tag = "Splunk"
)]
pub async fn get_query_history(
    State(state): State<AppState>,
) -> ApiResult<Json<QueryHistoryResponse>> {
    // TODO: Get user_id from auth context
    let user_id = Uuid::new_v4();

    let entries: Vec<QueryHistoryEntry> = sqlx::query_as(
        r"
        SELECT 
            h.id,
            h.query,
            t.name as template_name,
            h.time_start,
            h.time_end,
            h.execution_time_ms,
            h.result_count,
            h.created_at
        FROM splunk_query_history h
        LEFT JOIN splunk_query_templates t ON h.template_id = t.id
        WHERE h.user_id = $1
        ORDER BY h.created_at DESC
        LIMIT 50
        ",
    )
    .bind(user_id)
    .fetch_all(&state.db)
    .await
    .map_err(|e| ApiError::Internal(anyhow::anyhow!("Failed to fetch query history: {e}")))?;

    let total = entries.len() as i64;

    Ok(Json(QueryHistoryResponse { entries, total }))
}

/// Get common placeholder information.
#[utoipa::path(
    get,
    path = "/api/v1/splunk/placeholders",
    responses(
        (status = 200, description = "Placeholder information", body = PlaceholdersResponse)
    ),
    tag = "Splunk"
)]
pub async fn get_placeholders() -> Json<PlaceholdersResponse> {
    let placeholders = vec![
        PlaceholderInfo {
            key: "TICKET_KEY".to_string(),
            label: "Ticket Key".to_string(),
            description: "The Jira ticket key (e.g., PROJ-123)".to_string(),
            example: "PROJ-123".to_string(),
        },
        PlaceholderInfo {
            key: "USER_ID".to_string(),
            label: "User ID".to_string(),
            description: "The user identifier to search for".to_string(),
            example: "user@example.com".to_string(),
        },
        PlaceholderInfo {
            key: "ERROR_TYPE".to_string(),
            label: "Error Type".to_string(),
            description: "Type of error to filter (e.g., NullPointerException)".to_string(),
            example: "NullPointerException".to_string(),
        },
        PlaceholderInfo {
            key: "ENDPOINT".to_string(),
            label: "API Endpoint".to_string(),
            description: "API endpoint path to search for".to_string(),
            example: "/api/v1/users".to_string(),
        },
    ];

    Json(PlaceholdersResponse { placeholders })
}

// ============================================================================
// Helper Functions
// ============================================================================

/// Generate mock log entries for demonstration purposes.
fn generate_mock_logs(query: &str, limit: usize) -> Vec<LogEntry> {
    let now = Utc::now();
    let levels = ["INFO", "WARN", "ERROR", "DEBUG"];
    let hosts = ["app-server-01", "app-server-02", "api-gateway", "worker-01"];
    let sources = [
        "/var/log/app.log",
        "/var/log/api.log",
        "/var/log/worker.log",
    ];

    let messages = if query.to_lowercase().contains("error") {
        vec![
            "Connection timeout to database",
            "Failed to process request: invalid input",
            "NullPointerException in UserService",
            "HTTP 500: Internal Server Error",
            "Memory allocation failed",
        ]
    } else if query.to_lowercase().contains("performance") {
        vec![
            "Request completed in 1250ms",
            "Slow query detected: 3.5s",
            "Cache miss for key: user_session",
            "High CPU usage: 85%",
            "Memory usage at 72%",
        ]
    } else {
        vec![
            "User login successful",
            "API request processed",
            "Background job completed",
            "Cache refreshed",
            "Health check passed",
        ]
    };

    (0..limit.min(20))
        .map(|i| {
            let level_idx = if query.to_lowercase().contains("error") {
                2 // ERROR
            } else {
                i % levels.len()
            };

            LogEntry {
                timestamp: now - Duration::minutes(i as i64 * 5),
                level: levels[level_idx].to_string(),
                message: messages[i % messages.len()].to_string(),
                source: Some(sources[i % sources.len()].to_string()),
                host: Some(hosts[i % hosts.len()].to_string()),
                fields: serde_json::json!({
                    "request_id": format!("req-{}", uuid::Uuid::new_v4()),
                    "trace_id": format!("trace-{}", i),
                }),
            }
        })
        .collect()
}
