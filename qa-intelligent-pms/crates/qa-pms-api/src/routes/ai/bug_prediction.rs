//! Bug prediction API endpoints.
//!
//! Story 31.2: Bug Prediction ML Model

use axum::{
    extract::State,
    routing::post,
    Json, Router,
};
use serde::{Deserialize, Serialize};
use tracing::{debug, info, warn};
use utoipa::ToSchema;

use crate::app::AppState;
use crate::user_config_health::jira_tickets_client_from_user_config;
use qa_pms_ai::{BugPredictor, BugRiskScore, TicketDetails};
use qa_pms_core::error::ApiError;

type ApiResult<T> = Result<T, ApiError>;

/// Create the bug prediction router.
pub fn router() -> Router<AppState> {
    Router::new()
        .route("/api/v1/ai/predict-bug-risk", post(predict_bug_risk))
        .route("/api/v1/ai/predict-release-risk", post(predict_release_risk))
        // TODO: Add train endpoint (Task 5 - ML training)
        // .route("/api/v1/ai/train-bug-predictor", post(train_bug_predictor))
}

// ==================== Request/Response Types ====================

/// Request to predict bug risk for a ticket.
#[derive(Debug, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct PredictBugRiskRequest {
    /// Jira ticket key (e.g., "PROJ-123")
    pub ticket_key: String,
}

/// Request to predict bug risk for a release.
#[derive(Debug, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct PredictReleaseRiskRequest {
    /// Release version
    pub release_version: String,
    /// Ticket keys to analyze
    pub ticket_keys: Vec<String>,
}

/// Response for release risk prediction.
#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ReleaseRiskResponse {
    /// Release version
    pub release_version: String,
    /// Risk scores for each ticket
    pub ticket_risks: Vec<TicketRiskEntry>,
    /// Summary statistics
    pub summary: ReleaseRiskSummary,
}

/// Risk entry for a single ticket in release.
#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct TicketRiskEntry {
    /// Ticket key
    pub ticket_key: String,
    /// Risk score (0.0-1.0)
    pub risk_score: f32,
    /// Risk level
    pub risk_level: String,
    /// Risk factors count
    pub risk_factors_count: usize,
    /// Predicted bug types count
    pub predicted_bugs_count: usize,
}

/// Summary statistics for release risk.
#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ReleaseRiskSummary {
    /// Total tickets analyzed
    pub total_tickets: usize,
    /// Average risk score
    pub average_risk_score: f32,
    /// Tickets by risk level
    pub by_risk_level: RiskLevelCounts,
    /// Most common risk factors
    pub common_risk_factors: Vec<String>,
    /// Most common predicted bug types
    pub common_bug_types: Vec<String>,
}

/// Counts by risk level.
#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct RiskLevelCounts {
    /// Low risk count
    pub low: usize,
    /// Medium risk count
    pub medium: usize,
    /// High risk count
    pub high: usize,
    /// Critical risk count
    pub critical: usize,
}

// ==================== Handlers ====================

/// Predict bug risk for a single ticket.
#[utoipa::path(
    post,
    path = "/api/v1/ai/predict-bug-risk",
    request_body = PredictBugRiskRequest,
    responses(
        (status = 200, description = "Bug risk predicted successfully", body = BugRiskScore),
        (status = 404, description = "Ticket not found"),
        (status = 503, description = "Jira not configured")
    ),
    tag = "AI"
)]
pub async fn predict_bug_risk(
    State(state): State<AppState>,
    Json(req): Json<PredictBugRiskRequest>,
) -> ApiResult<Json<BugRiskScore>> {
    info!("Predicting bug risk for ticket: {}", req.ticket_key);

    // Fetch Jira ticket (Task 7.3)
    let jira_client = jira_tickets_client_from_user_config(&state.settings)
        .map_err(|e| ApiError::Internal(anyhow::anyhow!("Failed to create Jira client: {e}")))?
        .ok_or_else(|| ApiError::ServiceUnavailable("Jira not configured".into()))?;

    let ticket_detail = jira_client
        .get_ticket(&req.ticket_key)
        .await
        .map_err(|e| {
            warn!("Failed to fetch ticket {}: {}", req.ticket_key, e);
            ApiError::NotFound(format!("Ticket {} not found: {}", req.ticket_key, e))
        })?;

    // Convert Jira ticket to TicketDetails
    let ticket_details = TicketDetails {
        key: ticket_detail.key.clone(),
        title: ticket_detail.fields.summary.clone(),
        ticket_type: "Task".to_string(), // TODO: Extract from ticket.fields if available
        description: extract_description(&ticket_detail.fields.description),
        acceptance_criteria: extract_acceptance_criteria(&ticket_detail.fields.description),
    };

    // Predict bug risk using BugPredictor (Task 7.4)
    let predictor = BugPredictor::new();
    let risk_score = predictor
        .predict(&ticket_details)
        .await
        .map_err(|e| {
            warn!("Bug prediction failed: {}", e);
            ApiError::Internal(anyhow::anyhow!("Bug prediction failed: {e}"))
        })?;

    info!(
        ticket = %req.ticket_key,
        risk_score = %risk_score.risk_score,
        risk_level = %risk_score.risk_level,
        "Bug risk predicted successfully"
    );

    Ok(Json(risk_score))
}

/// Predict bug risk for an entire release.
#[utoipa::path(
    post,
    path = "/api/v1/ai/predict-release-risk",
    request_body = PredictReleaseRiskRequest,
    responses(
        (status = 200, description = "Release risk predicted successfully", body = ReleaseRiskResponse),
        (status = 503, description = "Jira not configured")
    ),
    tag = "AI"
)]
pub async fn predict_release_risk(
    State(state): State<AppState>,
    Json(req): Json<PredictReleaseRiskRequest>,
) -> ApiResult<Json<ReleaseRiskResponse>> {
    info!("Predicting release risk for version: {} ({} tickets)", req.release_version, req.ticket_keys.len());

    // Fetch Jira tickets (Task 7.6)
    let jira_client = jira_tickets_client_from_user_config(&state.settings)
        .map_err(|e| ApiError::Internal(anyhow::anyhow!("Failed to create Jira client: {e}")))?
        .ok_or_else(|| ApiError::ServiceUnavailable("Jira not configured".into()))?;

    let predictor = BugPredictor::new();
    let mut ticket_risks = Vec::new();
    let mut risk_scores_sum = 0.0f32;
    let mut risk_level_counts = RiskLevelCounts {
        low: 0,
        medium: 0,
        high: 0,
        critical: 0,
    };
    let mut risk_factor_counts: std::collections::HashMap<String, usize> = std::collections::HashMap::new();
    let mut bug_type_counts: std::collections::HashMap<String, usize> = std::collections::HashMap::new();

    // Predict risk for each ticket (Task 7.7)
    for ticket_key in &req.ticket_keys {
        match jira_client.get_ticket(ticket_key).await {
            Ok(ticket_detail) => {
                let ticket_details = TicketDetails {
                    key: ticket_detail.key.clone(),
                    title: ticket_detail.fields.summary.clone(),
                    ticket_type: "Task".to_string(),
                    description: extract_description(&ticket_detail.fields.description),
                    acceptance_criteria: extract_acceptance_criteria(&ticket_detail.fields.description),
                };

                match predictor.predict(&ticket_details).await {
                    Ok(risk_score) => {
                        risk_scores_sum += risk_score.risk_score;

                        // Count risk levels
                        match risk_score.risk_level.as_str() {
                            "low" => risk_level_counts.low += 1,
                            "medium" => risk_level_counts.medium += 1,
                            "high" => risk_level_counts.high += 1,
                            "critical" => risk_level_counts.critical += 1,
                            _ => {}
                        }

                        // Count risk factors
                        for factor in &risk_score.risk_factors {
                            *risk_factor_counts.entry(factor.factor.clone()).or_insert(0) += 1;
                        }

                        // Count bug types
                        for bug_type in &risk_score.predicted_bugs {
                            *bug_type_counts.entry(bug_type.clone()).or_insert(0) += 1;
                        }

                        ticket_risks.push(TicketRiskEntry {
                            ticket_key: ticket_key.clone(),
                            risk_score: risk_score.risk_score,
                            risk_level: risk_score.risk_level.as_str().to_string(),
                            risk_factors_count: risk_score.risk_factors.len(),
                            predicted_bugs_count: risk_score.predicted_bugs.len(),
                        });
                    }
                    Err(e) => {
                        warn!("Failed to predict risk for ticket {}: {}", ticket_key, e);
                        // Continue with other tickets
                    }
                }
            }
            Err(e) => {
                warn!("Failed to fetch ticket {}: {}", ticket_key, e);
                // Continue with other tickets
            }
        }
    }

    // Calculate average risk score
    let average_risk_score = if ticket_risks.is_empty() {
        0.0
    } else {
        risk_scores_sum / ticket_risks.len() as f32
    };

    // Get most common risk factors (top 5)
    let mut common_risk_factors: Vec<(String, usize)> = risk_factor_counts.into_iter().collect();
    common_risk_factors.sort_by(|a, b| b.1.cmp(&a.1));
    let common_risk_factors: Vec<String> = common_risk_factors
        .into_iter()
        .take(5)
        .map(|(factor, _)| factor)
        .collect();

    // Get most common bug types (top 5)
    let mut common_bug_types: Vec<(String, usize)> = bug_type_counts.into_iter().collect();
    common_bug_types.sort_by(|a, b| b.1.cmp(&a.1));
    let common_bug_types: Vec<String> = common_bug_types
        .into_iter()
        .take(5)
        .map(|(bug_type, _)| bug_type)
        .collect();

    let summary = ReleaseRiskSummary {
        total_tickets: ticket_risks.len(),
        average_risk_score,
        by_risk_level: risk_level_counts,
        common_risk_factors,
        common_bug_types,
    };

    Ok(Json(ReleaseRiskResponse {
        release_version: req.release_version,
        ticket_risks,
        summary,
    }))
}

// ==================== Helper Functions ====================

/// Extract description from Jira ticket description (ADF or text).
fn extract_description(description: &Option<serde_json::Value>) -> String {
    if let Some(desc) = description {
        // Try to extract text from ADF format
        if let Some(content) = desc.get("content") {
            let mut text = String::new();
            extract_text_from_adf(content, &mut text);
            if !text.trim().is_empty() {
                return text.trim().to_string();
            }
        }
        // Fallback: try as plain string
        if let Some(text) = desc.as_str() {
            return text.to_string();
        }
    }
    "No description provided".to_string()
}

/// Recursively extract text from ADF nodes (simplified version).
fn extract_text_from_adf(node: &serde_json::Value, output: &mut String) {
    match node {
        serde_json::Value::Array(arr) => {
            for item in arr {
                extract_text_from_adf(item, output);
            }
        }
        serde_json::Value::Object(obj) => {
            if let Some(serde_json::Value::String(text)) = obj.get("text") {
                output.push_str(text);
            }
            if let Some(content) = obj.get("content") {
                extract_text_from_adf(content, output);
            }
        }
        _ => {}
    }
}

/// Extract acceptance criteria from ticket description.
fn extract_acceptance_criteria(_description: &Option<serde_json::Value>) -> Option<String> {
    // TODO: Implement proper ADF parsing to extract ACs
    // For now, return None
    None
}
