//! Ticket management API endpoints.
//!
//! Provides endpoints for:
//! - Listing tickets with filters
//! - Retrieving ticket details with comments and attachments
//! - Getting available transitions and transitioning tickets

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use qa_pms_core::error::ApiError;
use qa_pms_jira::{JiraTicketsClient, TicketFilters};
use secrecy::ExposeSecret;
use serde::{Deserialize, Serialize};
use std::time::Instant;
use tracing::{info, warn};
use utoipa::{IntoParams, ToSchema};

use crate::app::AppState;

/// Create the tickets router.
pub fn router() -> Router<AppState> {
    Router::new()
        .route("/api/v1/tickets", get(list_tickets))
        .route("/api/v1/tickets/{key}", get(get_ticket))
        .route("/api/v1/tickets/{key}/transitions", get(get_transitions))
        .route("/api/v1/tickets/{key}/transition", post(transition_ticket))
}

/// Query parameters for listing tickets.
#[derive(Debug, Deserialize, IntoParams)]
#[serde(rename_all = "camelCase")]
pub struct ListTicketsQuery {
    /// Comma-separated status filters
    #[param(example = "In Progress,Ready for QA")]
    pub status: Option<String>,
    /// Assignee email or account ID (defaults to current user)
    #[param(example = "user@example.com")]
    pub assignee: Option<String>,
    /// Project key filter
    #[param(example = "MYPROJ")]
    pub project: Option<String>,
    /// Page number (1-indexed, default: 1)
    #[param(example = 1)]
    pub page: Option<u32>,
    /// Items per page (max 100, default: 20)
    #[param(example = 20)]
    pub page_size: Option<u32>,
}

/// Response for ticket list endpoint.
#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct TicketListResponse {
    /// List of tickets
    pub tickets: Vec<TicketSummary>,
    /// Total number of matching tickets
    pub total: u32,
    /// Current page number
    pub page: u32,
    /// Items per page
    pub page_size: u32,
    /// Whether there are more pages
    pub has_more: bool,
    /// Load time in milliseconds (for performance monitoring)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub load_time_ms: Option<u64>,
}

/// Summary of a ticket for list display.
#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct TicketSummary {
    /// Ticket key (e.g., "PROJ-123")
    pub key: String,
    /// Ticket title/summary
    pub title: String,
    /// Current status name
    pub status: String,
    /// Status color category
    pub status_color: String,
    /// Priority name (if set)
    pub priority: Option<String>,
    /// Priority color for UI
    pub priority_color: String,
    /// Assignee display name
    pub assignee_name: Option<String>,
    /// Assignee avatar URL
    pub assignee_avatar: Option<String>,
    /// Last updated timestamp
    pub updated_at: String,
}

// ============================================================================
// Ticket Detail Types (Story 3.3)
// ============================================================================

/// Full ticket detail response.
#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct TicketDetailResponse {
    /// Ticket key (e.g., "PROJ-123")
    pub key: String,
    /// Ticket title/summary
    pub title: String,
    /// Description as HTML (converted from ADF)
    pub description_html: Option<String>,
    /// Description as plain text
    pub description_raw: Option<String>,
    /// Current status name
    pub status: String,
    /// Status color category
    pub status_color: String,
    /// Priority name (if set)
    pub priority: Option<String>,
    /// Priority color for UI
    pub priority_color: String,
    /// Assignee information
    pub assignee: Option<UserInfo>,
    /// Reporter information
    pub reporter: Option<UserInfo>,
    /// Creation timestamp
    pub created_at: String,
    /// Last update timestamp
    pub updated_at: String,
    /// Latest comments (max 10)
    pub comments: Vec<CommentInfo>,
    /// Attachments list
    pub attachments: Vec<AttachmentInfo>,
    /// Labels
    pub labels: Vec<String>,
    /// Whether description contains Gherkin syntax
    pub has_gherkin: bool,
    /// Load time in milliseconds (for performance monitoring)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub load_time_ms: Option<u64>,
}

/// User information for display.
#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct UserInfo {
    /// Display name
    pub name: String,
    /// Email address (optional)
    pub email: Option<String>,
    /// Avatar URL (optional)
    pub avatar_url: Option<String>,
}

/// Comment information for display.
#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CommentInfo {
    /// Comment ID
    pub id: String,
    /// Comment author
    pub author: UserInfo,
    /// Comment body as HTML
    pub body_html: String,
    /// Creation timestamp
    pub created_at: String,
}

/// Attachment information for display.
#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct AttachmentInfo {
    /// Attachment ID
    pub id: String,
    /// File name
    pub filename: String,
    /// MIME type
    pub mime_type: String,
    /// File size in bytes
    pub size: u64,
    /// Human-readable file size
    pub size_human: String,
    /// Download URL
    pub download_url: String,
}

// ============================================================================
// Transition Types (Story 3.4)
// ============================================================================

/// Available transition information for display.
#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct TransitionInfo {
    /// Transition ID
    pub id: String,
    /// Transition name (e.g., "Start Progress")
    pub name: String,
    /// Target status name
    pub to_status: String,
    /// Target status color category
    pub to_status_color: String,
}

/// Request body for transitioning a ticket.
#[derive(Debug, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct TransitionRequest {
    /// ID of the transition to perform
    pub transition_id: String,
}

/// Response after successful transition.
#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct TransitionResponse {
    /// Success message
    pub message: String,
    /// New status name
    pub new_status: String,
}

/// List tickets with optional filters.
///
/// Returns a paginated list of Jira tickets filtered by status, assignee, and project.
#[utoipa::path(
    get,
    path = "/api/v1/tickets",
    params(ListTicketsQuery),
    responses(
        (status = 200, description = "Ticket list", body = TicketListResponse),
        (status = 401, description = "Not authenticated with Jira"),
        (status = 503, description = "Jira service unavailable"),
    ),
    tag = "Tickets"
)]
pub async fn list_tickets(
    State(state): State<AppState>,
    Query(query): Query<ListTicketsQuery>,
) -> Result<Json<TicketListResponse>, ApiError> {
    let start = Instant::now();

    // Get Jira client from setup store
    let jira_client = get_jira_client(&state).await?;

    // Parse pagination
    let page = query.page.unwrap_or(1).max(1);
    let page_size = query.page_size.unwrap_or(20).min(100);
    let start_at = (page - 1) * page_size;

    // Parse status filters
    let statuses = query
        .status
        .map(|s| s.split(',').map(|s| s.trim().to_string()).collect())
        .unwrap_or_default();

    // Build filters
    let filters = TicketFilters {
        statuses,
        assignee: query.assignee,
        project: query.project,
    };

    info!(
        page = page,
        page_size = page_size,
        has_status_filter = !filters.statuses.is_empty(),
        has_assignee = filters.assignee.is_some(),
        "Fetching tickets from Jira"
    );

    // Fetch tickets
    let response = jira_client
        .list_tickets(&filters, start_at, page_size)
        .await
        .map_err(|e| {
            warn!(error = %e, "Failed to fetch tickets from Jira");
            ApiError::ServiceUnavailable(format!("Jira error: {e}"))
        })?;

    // Map to API response
    let tickets: Vec<TicketSummary> = response
        .issues
        .into_iter()
        .map(|t| TicketSummary {
            key: t.key,
            title: t.fields.summary,
            status: t.fields.status.name,
            status_color: t.fields.status.status_category.color_name,
            priority: t.fields.priority.as_ref().map(|p| p.name.clone()),
            priority_color: get_priority_color(t.fields.priority.as_ref().map(|p| p.name.as_str())),
            assignee_name: t.fields.assignee.as_ref().map(|a| a.display_name.clone()),
            assignee_avatar: t
                .fields
                .assignee
                .and_then(|a| a.avatar_urls.and_then(|av| av.small)),
            updated_at: t.fields.updated,
        })
        .collect();

    let duration = start.elapsed();
    let load_time_ms = duration.as_millis() as u64;

    // Log slow requests (> 2s per NFR-PERF-01)
    if duration.as_secs() >= 2 {
        warn!(
            duration_ms = load_time_ms,
            total = response.total,
            "Slow ticket list query (> 2s)"
        );
    }

    info!(
        duration_ms = load_time_ms,
        returned = tickets.len(),
        total = response.total,
        "Tickets fetched successfully"
    );

    Ok(Json(TicketListResponse {
        tickets,
        total: response.total,
        page,
        page_size,
        has_more: start_at + page_size < response.total,
        load_time_ms: Some(load_time_ms),
    }))
}

/// Get ticket details by key.
///
/// Returns full ticket information including description, comments, and attachments.
#[utoipa::path(
    get,
    path = "/api/v1/tickets/{key}",
    params(
        ("key" = String, Path, description = "Jira ticket key (e.g., PROJ-123)")
    ),
    responses(
        (status = 200, description = "Ticket details", body = TicketDetailResponse),
        (status = 401, description = "Not authenticated with Jira"),
        (status = 404, description = "Ticket not found"),
        (status = 503, description = "Jira service unavailable"),
    ),
    tag = "Tickets"
)]
pub async fn get_ticket(
    State(state): State<AppState>,
    Path(key): Path<String>,
) -> Result<Json<TicketDetailResponse>, ApiError> {
    let start = Instant::now();

    // Get Jira client from setup store
    let jira_client = get_jira_client(&state).await?;

    info!(key = %key, "Fetching ticket details from Jira");

    // Fetch ticket details
    let ticket = jira_client.get_ticket(&key).await.map_err(|e| {
        let error_msg = e.to_string();
        if error_msg.contains("not found") {
            warn!(key = %key, "Ticket not found");
            ApiError::NotFound(format!("Ticket not found: {key}"))
        } else {
            warn!(error = %e, key = %key, "Failed to fetch ticket from Jira");
            ApiError::ServiceUnavailable(format!("Jira error: {e}"))
        }
    })?;

    // Convert description from ADF to text/HTML
    let description_raw = adf_to_text(&ticket.fields.description);
    let description_html = adf_to_html(&ticket.fields.description);

    // Detect Gherkin syntax in description
    let has_gherkin = description_raw
        .as_ref()
        .is_some_and(|d| detect_gherkin(d));

    // Convert comments (latest 10)
    let comments: Vec<CommentInfo> = ticket
        .fields
        .comment
        .map(|c| {
            c.comments
                .into_iter()
                .rev() // Latest first
                .take(10)
                .map(|comment| CommentInfo {
                    id: comment.id,
                    author: UserInfo {
                        name: comment.author.display_name,
                        email: comment.author.email_address,
                        avatar_url: comment
                            .author
                            .avatar_urls
                            .and_then(|a| a.medium.or(a.small)),
                    },
                    body_html: adf_to_html(&comment.body).unwrap_or_default(),
                    created_at: comment.created,
                })
                .collect()
        })
        .unwrap_or_default();

    // Convert attachments
    let attachments: Vec<AttachmentInfo> = ticket
        .fields
        .attachment
        .unwrap_or_default()
        .into_iter()
        .map(|a| AttachmentInfo {
            id: a.id,
            filename: a.filename,
            mime_type: a.mime_type,
            size: a.size,
            size_human: humanize_bytes(a.size),
            download_url: a.content,
        })
        .collect();

    let duration = start.elapsed();
    let load_time_ms = duration.as_millis() as u64;

    // Log slow requests (> 2s per NFR-PERF-01)
    if duration.as_secs() >= 2 {
        warn!(
            duration_ms = load_time_ms,
            key = %key,
            "Slow ticket detail fetch (> 2s)"
        );
    }

    info!(
        duration_ms = load_time_ms,
        key = %key,
        comments_count = comments.len(),
        attachments_count = attachments.len(),
        "Ticket details fetched successfully"
    );

    Ok(Json(TicketDetailResponse {
        key: ticket.key,
        title: ticket.fields.summary,
        description_html,
        description_raw,
        status: ticket.fields.status.name,
        status_color: ticket.fields.status.status_category.color_name,
        priority: ticket.fields.priority.as_ref().map(|p| p.name.clone()),
        priority_color: get_priority_color(
            ticket.fields.priority.as_ref().map(|p| p.name.as_str()),
        ),
        assignee: ticket.fields.assignee.map(|a| UserInfo {
            name: a.display_name,
            email: a.email_address,
            avatar_url: a.avatar_urls.and_then(|av| av.medium.or(av.small)),
        }),
        reporter: ticket.fields.reporter.map(|r| UserInfo {
            name: r.display_name,
            email: r.email_address,
            avatar_url: r.avatar_urls.and_then(|av| av.medium.or(av.small)),
        }),
        created_at: ticket.fields.created,
        updated_at: ticket.fields.updated,
        comments,
        attachments,
        labels: ticket.fields.labels,
        has_gherkin,
        load_time_ms: Some(load_time_ms),
    }))
}

/// Get available transitions for a ticket.
///
/// Returns a list of transitions the user can perform on the ticket.
#[utoipa::path(
    get,
    path = "/api/v1/tickets/{key}/transitions",
    params(
        ("key" = String, Path, description = "Jira ticket key (e.g., PROJ-123)")
    ),
    responses(
        (status = 200, description = "Available transitions", body = Vec<TransitionInfo>),
        (status = 401, description = "Not authenticated with Jira"),
        (status = 404, description = "Ticket not found"),
        (status = 503, description = "Jira service unavailable"),
    ),
    tag = "Tickets"
)]
pub async fn get_transitions(
    State(state): State<AppState>,
    Path(key): Path<String>,
) -> Result<Json<Vec<TransitionInfo>>, ApiError> {
    // Get Jira client from setup store
    let jira_client = get_jira_client(&state).await?;

    info!(key = %key, "Fetching available transitions from Jira");

    // Fetch transitions
    let transitions = jira_client.get_transitions(&key).await.map_err(|e| {
        let error_msg = e.to_string();
        if error_msg.contains("not found") {
            warn!(key = %key, "Ticket not found");
            ApiError::NotFound(format!("Ticket not found: {key}"))
        } else {
            warn!(error = %e, key = %key, "Failed to fetch transitions from Jira");
            ApiError::ServiceUnavailable(format!("Jira error: {e}"))
        }
    })?;

    // Map to API response
    let transition_infos: Vec<TransitionInfo> = transitions
        .into_iter()
        .map(|t| TransitionInfo {
            id: t.id,
            name: t.name,
            to_status: t.to.name,
            to_status_color: t.to.status_category.color_name,
        })
        .collect();

    info!(
        key = %key,
        count = transition_infos.len(),
        "Transitions fetched successfully"
    );

    Ok(Json(transition_infos))
}

/// Transition a ticket to a new status.
///
/// Performs the specified transition on the ticket, moving it to a new status.
/// Uses retry with exponential backoff per NFR-REL-03.
#[utoipa::path(
    post,
    path = "/api/v1/tickets/{key}/transition",
    params(
        ("key" = String, Path, description = "Jira ticket key (e.g., PROJ-123)")
    ),
    request_body = TransitionRequest,
    responses(
        (status = 200, description = "Transition successful", body = TransitionResponse),
        (status = 400, description = "Invalid transition"),
        (status = 401, description = "Not authenticated with Jira"),
        (status = 404, description = "Ticket not found"),
        (status = 503, description = "Jira service unavailable"),
    ),
    tag = "Tickets"
)]
pub async fn transition_ticket(
    State(state): State<AppState>,
    Path(key): Path<String>,
    Json(req): Json<TransitionRequest>,
) -> Result<(StatusCode, Json<TransitionResponse>), ApiError> {
    // Get Jira client from setup store
    let jira_client = get_jira_client(&state).await?;

    info!(
        key = %key,
        transition_id = %req.transition_id,
        "Transitioning ticket"
    );

    // First, get the transition details to know the target status
    let transitions = jira_client.get_transitions(&key).await.map_err(|e| {
        let error_msg = e.to_string();
        if error_msg.contains("not found") {
            ApiError::NotFound(format!("Ticket not found: {key}"))
        } else {
            ApiError::ServiceUnavailable(format!("Jira error: {e}"))
        }
    })?;

    let target_transition = transitions
        .iter()
        .find(|t| t.id == req.transition_id)
        .ok_or_else(|| {
            ApiError::Validation(format!(
                "Invalid transition ID: {}. Available transitions: {}",
                req.transition_id,
                transitions
                    .iter()
                    .map(|t| format!("{} ({})", t.name, t.id))
                    .collect::<Vec<_>>()
                    .join(", ")
            ))
        })?;

    let new_status = target_transition.to.name.clone();

    // Perform the transition
    jira_client
        .transition_ticket(&key, &req.transition_id)
        .await
        .map_err(|e| {
            let error_msg = e.to_string();
            if error_msg.contains("Invalid transition") {
                warn!(key = %key, error = %e, "Invalid transition");
                ApiError::Validation(error_msg)
            } else if error_msg.contains("not found") {
                ApiError::NotFound(format!("Ticket not found: {key}"))
            } else {
                warn!(key = %key, error = %e, "Transition failed");
                ApiError::ServiceUnavailable(format!("Jira error: {e}"))
            }
        })?;

    info!(
        key = %key,
        new_status = %new_status,
        "Ticket transitioned successfully"
    );

    Ok((
        StatusCode::OK,
        Json(TransitionResponse {
            message: format!("Ticket {key} transitioned to {new_status}"),
            new_status,
        }),
    ))
}

/// Get priority color based on priority name.
fn get_priority_color(priority: Option<&str>) -> String {
    match priority {
        Some("Highest" | "Blocker") => "error".to_string(),
        Some("High" | "Critical") => "warning".to_string(),
        Some("Medium") => "primary".to_string(),
        Some("Low" | "Lowest" | "Minor" | "Trivial") => "neutral".to_string(),
        _ => "neutral".to_string(),
    }
}

/// Detect Gherkin syntax in text.
fn detect_gherkin(text: &str) -> bool {
    const GHERKIN_KEYWORDS: &[&str] = &[
        "Given ", "When ", "Then ", "And ", "But ", "Scenario:", "Feature:", "Background:",
        "Scenario Outline:", "Examples:",
    ];
    GHERKIN_KEYWORDS.iter().any(|k| text.contains(k))
}

/// Convert Atlassian Document Format (ADF) to plain text.
fn adf_to_text(adf: &Option<serde_json::Value>) -> Option<String> {
    let doc = adf.as_ref()?;
    let content = doc.get("content")?;
    let mut text = String::new();
    extract_text_from_adf(content, &mut text);
    if text.is_empty() {
        None
    } else {
        Some(text.trim().to_string())
    }
}

/// Recursively extract text from ADF nodes.
fn extract_text_from_adf(node: &serde_json::Value, output: &mut String) {
    match node {
        serde_json::Value::Array(arr) => {
            for item in arr {
                extract_text_from_adf(item, output);
            }
        }
        serde_json::Value::Object(obj) => {
            // Handle text nodes
            if let Some(serde_json::Value::String(text)) = obj.get("text") {
                output.push_str(text);
            }

            // Handle different block types
            if let Some(serde_json::Value::String(node_type)) = obj.get("type") {
                // Add newlines for block elements
                if matches!(
                    node_type.as_str(),
                    "paragraph" | "heading" | "bulletList" | "orderedList" | "codeBlock"
                ) && !output.is_empty()
                {
                    output.push('\n');
                }

                // Add list markers
                if node_type == "listItem" {
                    output.push_str("• ");
                }
            }

            // Recurse into content
            if let Some(content) = obj.get("content") {
                extract_text_from_adf(content, output);
            }
        }
        _ => {}
    }
}

/// Convert Atlassian Document Format (ADF) to HTML.
fn adf_to_html(adf: &Option<serde_json::Value>) -> Option<String> {
    let doc = adf.as_ref()?;
    let content = doc.get("content")?;
    let mut html = String::new();
    convert_adf_to_html(content, &mut html);
    if html.is_empty() {
        None
    } else {
        Some(html)
    }
}

/// Recursively convert ADF nodes to HTML.
fn convert_adf_to_html(node: &serde_json::Value, output: &mut String) {
    match node {
        serde_json::Value::Array(arr) => {
            for item in arr {
                convert_adf_to_html(item, output);
            }
        }
        serde_json::Value::Object(obj) => {
            let node_type = obj
                .get("type")
                .and_then(|v| v.as_str())
                .unwrap_or("unknown");

            match node_type {
                "text" => {
                    if let Some(serde_json::Value::String(text)) = obj.get("text") {
                        // Handle marks (bold, italic, code, etc.)
                        let mut styled_text = html_escape(text);
                        if let Some(serde_json::Value::Array(marks)) = obj.get("marks") {
                            for mark in marks {
                                if let Some(mark_type) = mark.get("type").and_then(|v| v.as_str()) {
                                    styled_text = match mark_type {
                                        "strong" => format!("<strong>{styled_text}</strong>"),
                                        "em" => format!("<em>{styled_text}</em>"),
                                        "code" => format!("<code>{styled_text}</code>"),
                                        "underline" => format!("<u>{styled_text}</u>"),
                                        "strike" => format!("<s>{styled_text}</s>"),
                                        "link" => {
                                            if let Some(href) = mark
                                                .get("attrs")
                                                .and_then(|a| a.get("href"))
                                                .and_then(|h| h.as_str())
                                            {
                                                format!(
                                                    "<a href=\"{}\" target=\"_blank\" rel=\"noopener\">{}</a>",
                                                    html_escape(href),
                                                    styled_text
                                                )
                                            } else {
                                                styled_text
                                            }
                                        }
                                        _ => styled_text,
                                    };
                                }
                            }
                        }
                        output.push_str(&styled_text);
                    }
                }
                "paragraph" => {
                    output.push_str("<p>");
                    if let Some(content) = obj.get("content") {
                        convert_adf_to_html(content, output);
                    }
                    output.push_str("</p>");
                }
                "heading" => {
                    let level = obj
                        .get("attrs")
                        .and_then(|a| a.get("level"))
                        .and_then(serde_json::Value::as_u64)
                        .unwrap_or(1)
                        .min(6);
                    output.push_str(&format!("<h{level}>"));
                    if let Some(content) = obj.get("content") {
                        convert_adf_to_html(content, output);
                    }
                    output.push_str(&format!("</h{level}>"));
                }
                "bulletList" => {
                    output.push_str("<ul>");
                    if let Some(content) = obj.get("content") {
                        convert_adf_to_html(content, output);
                    }
                    output.push_str("</ul>");
                }
                "orderedList" => {
                    output.push_str("<ol>");
                    if let Some(content) = obj.get("content") {
                        convert_adf_to_html(content, output);
                    }
                    output.push_str("</ol>");
                }
                "listItem" => {
                    output.push_str("<li>");
                    if let Some(content) = obj.get("content") {
                        convert_adf_to_html(content, output);
                    }
                    output.push_str("</li>");
                }
                "codeBlock" => {
                    let language = obj
                        .get("attrs")
                        .and_then(|a| a.get("language"))
                        .and_then(|l| l.as_str())
                        .unwrap_or("");
                    if language.is_empty() {
                        output.push_str("<pre><code>");
                    } else {
                        output.push_str(&format!("<pre><code class=\"language-{language}\">"));
                    }
                    if let Some(content) = obj.get("content") {
                        convert_adf_to_html(content, output);
                    }
                    output.push_str("</code></pre>");
                }
                "blockquote" => {
                    output.push_str("<blockquote>");
                    if let Some(content) = obj.get("content") {
                        convert_adf_to_html(content, output);
                    }
                    output.push_str("</blockquote>");
                }
                "rule" => {
                    output.push_str("<hr/>");
                }
                "hardBreak" => {
                    output.push_str("<br/>");
                }
                "mention" => {
                    if let Some(attrs) = obj.get("attrs") {
                        let text = attrs
                            .get("text")
                            .and_then(|t| t.as_str())
                            .unwrap_or("@mention");
                        output.push_str(&format!(
                            "<span class=\"mention\">{}</span>",
                            html_escape(text)
                        ));
                    }
                }
                "emoji" => {
                    if let Some(attrs) = obj.get("attrs") {
                        let shortname = attrs
                            .get("shortName")
                            .and_then(|s| s.as_str())
                            .unwrap_or(":emoji:");
                        output.push_str(&html_escape(shortname));
                    }
                }
                "table" => {
                    output.push_str("<table>");
                    if let Some(content) = obj.get("content") {
                        convert_adf_to_html(content, output);
                    }
                    output.push_str("</table>");
                }
                "tableRow" => {
                    output.push_str("<tr>");
                    if let Some(content) = obj.get("content") {
                        convert_adf_to_html(content, output);
                    }
                    output.push_str("</tr>");
                }
                "tableHeader" => {
                    output.push_str("<th>");
                    if let Some(content) = obj.get("content") {
                        convert_adf_to_html(content, output);
                    }
                    output.push_str("</th>");
                }
                "tableCell" => {
                    output.push_str("<td>");
                    if let Some(content) = obj.get("content") {
                        convert_adf_to_html(content, output);
                    }
                    output.push_str("</td>");
                }
                _ => {
                    // Unknown type, try to process content anyway
                    if let Some(content) = obj.get("content") {
                        convert_adf_to_html(content, output);
                    }
                }
            }
        }
        _ => {}
    }
}

/// Escape HTML special characters.
fn html_escape(text: &str) -> String {
    text.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}

/// Convert bytes to human-readable format.
fn humanize_bytes(bytes: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = KB * 1024;
    const GB: u64 = MB * 1024;

    if bytes >= GB {
        format!("{:.1} GB", bytes as f64 / GB as f64)
    } else if bytes >= MB {
        format!("{:.1} MB", bytes as f64 / MB as f64)
    } else if bytes >= KB {
        format!("{:.1} KB", bytes as f64 / KB as f64)
    } else {
        format!("{bytes} B")
    }
}

/// Get or create Jira client from app state.
///
/// For now, this creates a mock client. In production, it will use
/// stored OAuth tokens from the setup wizard.
async fn get_jira_client(state: &AppState) -> Result<JiraTicketsClient, ApiError> {
    // First, check if we have Jira settings from environment (API Token)
    if let Some(jira_settings) = state.settings.jira.as_ref() {
        if let (Some(email), Some(api_token)) = (&jira_settings.email, &jira_settings.api_token) {
            return Ok(JiraTicketsClient::with_api_token(
                jira_settings.instance_url.clone(),
                email.clone(),
                api_token.expose_secret().clone(),
            ));
        }
    }

    // Fallback: Check setup store for credentials from setup wizard
    let setup_state = state.setup_store.lock().await;

    let jira_config = setup_state.jira.as_ref().ok_or_else(|| {
        ApiError::Unauthorized("Jira not configured. Please complete setup wizard or set JIRA_URL, JIRA_EMAIL, JIRA_API_TOKEN environment variables.".to_string())
    })?;

    // Clone all values we need before dropping the lock
    let instance_url = jira_config.instance_url.clone();
    let email = jira_config.email.clone();
    let api_token = jira_config.api_token.clone();
    let cloud_id = jira_config.cloud_id.clone();
    let access_token = jira_config.access_token.clone();

    // Drop the lock before using the values
    drop(setup_state);

    // Prefer API Token if available
    if let (Some(email), Some(api_token)) = (email, api_token) {
        return Ok(JiraTicketsClient::with_api_token(
            instance_url,
            email,
            api_token,
        ));
    }

    // Fallback to OAuth if available
    if let (Some(cloud_id), Some(access_token)) = (cloud_id, access_token) {
        return Ok(JiraTicketsClient::with_oauth(cloud_id, access_token));
    }

    Err(ApiError::Unauthorized(
        "Jira credentials not configured. Please provide API Token (email + api_token) or complete OAuth flow.".to_string(),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_priority_color_highest() {
        assert_eq!(get_priority_color(Some("Highest")), "error");
        assert_eq!(get_priority_color(Some("Blocker")), "error");
    }

    #[test]
    fn test_priority_color_high() {
        assert_eq!(get_priority_color(Some("High")), "warning");
        assert_eq!(get_priority_color(Some("Critical")), "warning");
    }

    #[test]
    fn test_priority_color_medium() {
        assert_eq!(get_priority_color(Some("Medium")), "primary");
    }

    #[test]
    fn test_priority_color_low() {
        assert_eq!(get_priority_color(Some("Low")), "neutral");
        assert_eq!(get_priority_color(Some("Lowest")), "neutral");
        assert_eq!(get_priority_color(Some("Minor")), "neutral");
        assert_eq!(get_priority_color(Some("Trivial")), "neutral");
    }

    #[test]
    fn test_priority_color_unknown() {
        assert_eq!(get_priority_color(None), "neutral");
        assert_eq!(get_priority_color(Some("Unknown")), "neutral");
    }

    #[test]
    fn test_detect_gherkin() {
        assert!(detect_gherkin("Given I am logged in"));
        assert!(detect_gherkin("When I click the button"));
        assert!(detect_gherkin("Then I should see the result"));
        assert!(detect_gherkin("Scenario: User login"));
        assert!(detect_gherkin("Feature: Authentication"));
        assert!(!detect_gherkin("This is a regular description"));
        assert!(!detect_gherkin("given lowercase doesn't match"));
    }

    #[test]
    fn test_humanize_bytes() {
        assert_eq!(humanize_bytes(0), "0 B");
        assert_eq!(humanize_bytes(500), "500 B");
        assert_eq!(humanize_bytes(1024), "1.0 KB");
        assert_eq!(humanize_bytes(1536), "1.5 KB");
        assert_eq!(humanize_bytes(1048576), "1.0 MB");
        assert_eq!(humanize_bytes(1572864), "1.5 MB");
        assert_eq!(humanize_bytes(1073741824), "1.0 GB");
    }

    #[test]
    fn test_html_escape() {
        assert_eq!(html_escape("<script>"), "&lt;script&gt;");
        assert_eq!(html_escape("a & b"), "a &amp; b");
        assert_eq!(html_escape("\"quoted\""), "&quot;quoted&quot;");
        assert_eq!(html_escape("it's"), "it&#39;s");
    }

    #[test]
    fn test_adf_to_text_simple() {
        let adf = serde_json::json!({
            "type": "doc",
            "version": 1,
            "content": [
                {
                    "type": "paragraph",
                    "content": [
                        {"type": "text", "text": "Hello world"}
                    ]
                }
            ]
        });
        let result = adf_to_text(&Some(adf));
        assert_eq!(result, Some("Hello world".to_string()));
    }

    #[test]
    fn test_adf_to_text_none() {
        assert_eq!(adf_to_text(&None), None);
    }

    #[test]
    fn test_adf_to_html_simple() {
        let adf = serde_json::json!({
            "type": "doc",
            "version": 1,
            "content": [
                {
                    "type": "paragraph",
                    "content": [
                        {"type": "text", "text": "Hello world"}
                    ]
                }
            ]
        });
        let result = adf_to_html(&Some(adf));
        assert_eq!(result, Some("<p>Hello world</p>".to_string()));
    }

    #[test]
    fn test_adf_to_html_with_marks() {
        let adf = serde_json::json!({
            "type": "doc",
            "version": 1,
            "content": [
                {
                    "type": "paragraph",
                    "content": [
                        {
                            "type": "text",
                            "text": "bold text",
                            "marks": [{"type": "strong"}]
                        }
                    ]
                }
            ]
        });
        let result = adf_to_html(&Some(adf));
        assert_eq!(result, Some("<p><strong>bold text</strong></p>".to_string()));
    }

    #[test]
    fn test_adf_to_html_code_block() {
        let adf = serde_json::json!({
            "type": "doc",
            "version": 1,
            "content": [
                {
                    "type": "codeBlock",
                    "attrs": {"language": "rust"},
                    "content": [
                        {"type": "text", "text": "fn main() {}"}
                    ]
                }
            ]
        });
        let result = adf_to_html(&Some(adf));
        assert_eq!(
            result,
            Some("<pre><code class=\"language-rust\">fn main() {}</code></pre>".to_string())
        );
    }
}
