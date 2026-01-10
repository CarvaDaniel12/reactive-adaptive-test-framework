//! Search API endpoints.
//!
//! Provides contextual search across Postman and Testmo.

use axum::{extract::State, response::IntoResponse, routing::post, Json, Router};
use qa_pms_core::KeywordExtractor;
use qa_pms_postman::{PostmanClient, SearchResult as PostmanSearchResult};
use qa_pms_testmo::{SearchResult as TestmoSearchResult, TestmoClient};
use secrecy::ExposeSecret;
use serde::{Deserialize, Serialize};
use std::time::Instant;
use tracing::{debug, info, warn};
use utoipa::ToSchema;

use crate::app::AppState;

/// Create the search router.
pub fn router() -> Router<AppState> {
    Router::new()
        .route("/api/v1/search/contextual", post(contextual_search))
        .route("/api/v1/search/postman", post(search_postman_endpoint))
        .route("/api/v1/search/testmo", post(search_testmo_endpoint))
        .route("/api/v1/search/all", post(search_all))
}

// ============================================================================
// Request/Response Types
// ============================================================================

/// Request body for contextual search.
#[derive(Debug, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ContextualSearchRequest {
    /// Ticket key for caching/logging purposes.
    pub ticket_key: String,
    /// Ticket title to extract keywords from.
    pub title: String,
    /// Optional ticket description for additional context.
    pub description: Option<String>,
}

/// Unified search result from any source.
#[derive(Debug, Clone, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct UnifiedSearchResult {
    /// Source integration (postman, testmo).
    pub source: String,
    /// Item ID.
    pub id: String,
    /// Item name/title.
    pub name: String,
    /// Item description.
    pub description: Option<String>,
    /// URL to view item in source system.
    pub url: String,
    /// Match score (higher is better).
    pub score: f32,
    /// Matching text snippets.
    pub matches: Vec<String>,
}

/// Search response with results and metadata.
#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct SearchResponse {
    /// Search results sorted by score.
    pub results: Vec<UnifiedSearchResult>,
    /// Number of results from Postman.
    pub postman_count: usize,
    /// Number of results from Testmo.
    pub testmo_count: usize,
    /// Total search time in milliseconds.
    pub search_time_ms: u64,
    /// Keywords used for search.
    pub keywords_used: Vec<String>,
}

/// Request body for keyword-based search.
#[derive(Debug, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct KeywordSearchRequest {
    /// Keywords to search for.
    pub keywords: Vec<String>,
    /// Optional ticket ID for logging.
    pub ticket_id: Option<String>,
}

/// Single-source search response.
#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct SingleSourceSearchResponse {
    /// Search results sorted by score.
    pub results: Vec<UnifiedSearchResult>,
    /// Number of results.
    pub count: usize,
    /// Search time in milliseconds.
    pub search_time_ms: u64,
}

// ============================================================================
// Handlers
// ============================================================================

/// Contextual search endpoint.
///
/// Extracts keywords from ticket title/description and searches
/// both Postman and Testmo in parallel.
#[utoipa::path(
    post,
    path = "/api/v1/search/contextual",
    request_body = ContextualSearchRequest,
    responses(
        (status = 200, description = "Search results", body = SearchResponse),
        (status = 400, description = "Invalid request"),
        (status = 500, description = "Search failed")
    ),
    tag = "Search"
)]
pub async fn contextual_search(
    State(state): State<AppState>,
    Json(request): Json<ContextualSearchRequest>,
) -> impl IntoResponse {
    let start = Instant::now();

    info!(
        ticket_key = %request.ticket_key,
        "Starting contextual search"
    );

    // Extract keywords
    let extractor = KeywordExtractor::default();
    let keywords = extractor.extract_from_ticket(&request.title, request.description.as_deref());

    if keywords.is_empty() {
        debug!("No keywords extracted, returning empty results");
        return Json(SearchResponse {
            results: vec![],
            postman_count: 0,
            testmo_count: 0,
            search_time_ms: start.elapsed().as_millis() as u64,
            keywords_used: keywords,
        });
    }

    debug!(keywords = ?keywords, "Extracted keywords");

    // Create clients from settings
    let postman_client = create_postman_client(&state);
    let (testmo_client, testmo_project_id) = create_testmo_client(&state);

    // Run searches in parallel
    let postman_future = search_postman(postman_client, &keywords);
    let testmo_future = search_testmo(testmo_client, testmo_project_id, &keywords);

    let (postman_results, testmo_results) = tokio::join!(postman_future, testmo_future);

    // Collect results
    let mut all_results: Vec<UnifiedSearchResult> = Vec::new();

    let postman_count = match postman_results {
        Ok(results) => {
            let count = results.len();
            debug!(count = count, "Postman search completed");
            all_results.extend(results.into_iter().map(|r| UnifiedSearchResult {
                source: r.source,
                id: r.id,
                name: r.name,
                description: r.description,
                url: r.url,
                score: r.score,
                matches: r.matches,
            }));
            count
        }
        Err(e) => {
            warn!(error = %e, "Postman search failed");
            0
        }
    };

    let testmo_count = match testmo_results {
        Ok(results) => {
            let count = results.len();
            debug!(count = count, "Testmo search completed");
            all_results.extend(results.into_iter().map(|r| UnifiedSearchResult {
                source: r.source,
                id: r.id,
                name: r.name,
                description: r.description,
                url: r.url,
                score: r.score,
                matches: r.matches,
            }));
            count
        }
        Err(e) => {
            warn!(error = %e, "Testmo search failed");
            0
        }
    };

    // Sort all results by score (descending)
    all_results.sort_by(|a, b| {
        b.score
            .partial_cmp(&a.score)
            .unwrap_or(std::cmp::Ordering::Equal)
    });

    let duration = start.elapsed();
    let search_time_ms = duration.as_millis() as u64;

    // Log slow searches (NFR-PERF-03: < 3s)
    if duration.as_secs() > 3 {
        warn!(
            ticket_key = %request.ticket_key,
            duration_ms = search_time_ms,
            "Slow contextual search exceeded 3s threshold"
        );
    }

    info!(
        ticket_key = %request.ticket_key,
        postman_count = postman_count,
        testmo_count = testmo_count,
        total_results = all_results.len(),
        duration_ms = search_time_ms,
        "Contextual search completed"
    );

    Json(SearchResponse {
        results: all_results,
        postman_count,
        testmo_count,
        search_time_ms,
        keywords_used: keywords,
    })
}

/// Search Postman collections only.
#[utoipa::path(
    post,
    path = "/api/v1/search/postman",
    request_body = KeywordSearchRequest,
    responses(
        (status = 200, description = "Postman search results", body = SingleSourceSearchResponse),
        (status = 400, description = "Invalid request"),
        (status = 503, description = "Postman not configured")
    ),
    tag = "Search"
)]
pub async fn search_postman_endpoint(
    State(state): State<AppState>,
    Json(request): Json<KeywordSearchRequest>,
) -> impl IntoResponse {
    let start = Instant::now();

    info!(
        ticket_id = ?request.ticket_id,
        keywords = ?request.keywords,
        "Starting Postman search"
    );

    if request.keywords.is_empty() {
        return Json(SingleSourceSearchResponse {
            results: vec![],
            count: 0,
            search_time_ms: start.elapsed().as_millis() as u64,
        });
    }

    let postman_client = create_postman_client(&state);
    let results = search_postman(postman_client, &request.keywords).await;

    let mapped_results: Vec<UnifiedSearchResult> = match results {
        Ok(r) => r
            .into_iter()
            .map(|r| UnifiedSearchResult {
                source: r.source,
                id: r.id,
                name: r.name,
                description: r.description,
                url: r.url,
                score: r.score,
                matches: r.matches,
            })
            .collect(),
        Err(e) => {
            warn!(error = %e, "Postman search failed");
            vec![]
        }
    };

    let count = mapped_results.len();
    info!(count = count, "Postman search completed");

    Json(SingleSourceSearchResponse {
        results: mapped_results,
        count,
        search_time_ms: start.elapsed().as_millis() as u64,
    })
}

/// Search Testmo test cases only.
#[utoipa::path(
    post,
    path = "/api/v1/search/testmo",
    request_body = KeywordSearchRequest,
    responses(
        (status = 200, description = "Testmo search results", body = SingleSourceSearchResponse),
        (status = 400, description = "Invalid request"),
        (status = 503, description = "Testmo not configured")
    ),
    tag = "Search"
)]
pub async fn search_testmo_endpoint(
    State(state): State<AppState>,
    Json(request): Json<KeywordSearchRequest>,
) -> impl IntoResponse {
    let start = Instant::now();

    info!(
        ticket_id = ?request.ticket_id,
        keywords = ?request.keywords,
        "Starting Testmo search"
    );

    if request.keywords.is_empty() {
        return Json(SingleSourceSearchResponse {
            results: vec![],
            count: 0,
            search_time_ms: start.elapsed().as_millis() as u64,
        });
    }

    let (testmo_client, testmo_project_id) = create_testmo_client(&state);
    let results = search_testmo(testmo_client, testmo_project_id, &request.keywords).await;

    let mapped_results: Vec<UnifiedSearchResult> = match results {
        Ok(r) => r
            .into_iter()
            .map(|r| UnifiedSearchResult {
                source: r.source,
                id: r.id,
                name: r.name,
                description: r.description,
                url: r.url,
                score: r.score,
                matches: r.matches,
            })
            .collect(),
        Err(e) => {
            warn!(error = %e, "Testmo search failed");
            vec![]
        }
    };

    let count = mapped_results.len();
    info!(count = count, "Testmo search completed");

    Json(SingleSourceSearchResponse {
        results: mapped_results,
        count,
        search_time_ms: start.elapsed().as_millis() as u64,
    })
}

/// Search all integrations in parallel.
#[utoipa::path(
    post,
    path = "/api/v1/search/all",
    request_body = KeywordSearchRequest,
    responses(
        (status = 200, description = "Combined search results", body = SearchResponse),
        (status = 400, description = "Invalid request")
    ),
    tag = "Search"
)]
pub async fn search_all(
    State(state): State<AppState>,
    Json(request): Json<KeywordSearchRequest>,
) -> impl IntoResponse {
    let start = Instant::now();

    info!(
        ticket_id = ?request.ticket_id,
        keywords = ?request.keywords,
        "Starting parallel search"
    );

    if request.keywords.is_empty() {
        return Json(SearchResponse {
            results: vec![],
            postman_count: 0,
            testmo_count: 0,
            search_time_ms: start.elapsed().as_millis() as u64,
            keywords_used: vec![],
        });
    }

    let postman_client = create_postman_client(&state);
    let (testmo_client, testmo_project_id) = create_testmo_client(&state);

    // Run searches in parallel
    let postman_future = search_postman(postman_client, &request.keywords);
    let testmo_future = search_testmo(testmo_client, testmo_project_id, &request.keywords);

    let (postman_results, testmo_results) = tokio::join!(postman_future, testmo_future);

    let mut all_results: Vec<UnifiedSearchResult> = Vec::new();

    let postman_count = match postman_results {
        Ok(results) => {
            let count = results.len();
            all_results.extend(results.into_iter().map(|r| UnifiedSearchResult {
                source: r.source,
                id: r.id,
                name: r.name,
                description: r.description,
                url: r.url,
                score: r.score,
                matches: r.matches,
            }));
            count
        }
        Err(e) => {
            warn!(error = %e, "Postman search failed in parallel");
            0
        }
    };

    let testmo_count = match testmo_results {
        Ok(results) => {
            let count = results.len();
            all_results.extend(results.into_iter().map(|r| UnifiedSearchResult {
                source: r.source,
                id: r.id,
                name: r.name,
                description: r.description,
                url: r.url,
                score: r.score,
                matches: r.matches,
            }));
            count
        }
        Err(e) => {
            warn!(error = %e, "Testmo search failed in parallel");
            0
        }
    };

    // Sort by score descending
    all_results.sort_by(|a, b| {
        b.score
            .partial_cmp(&a.score)
            .unwrap_or(std::cmp::Ordering::Equal)
    });

    let duration = start.elapsed();
    if duration.as_secs() > 3 {
        warn!(
            duration_ms = duration.as_millis(),
            "Slow parallel search exceeded 3s"
        );
    }

    info!(
        postman_count = postman_count,
        testmo_count = testmo_count,
        duration_ms = duration.as_millis(),
        "Parallel search completed"
    );

    Json(SearchResponse {
        results: all_results,
        postman_count,
        testmo_count,
        search_time_ms: duration.as_millis() as u64,
        keywords_used: request.keywords,
    })
}

// ============================================================================
// Helper Functions
// ============================================================================

/// Create Postman client from settings.
fn create_postman_client(state: &AppState) -> Option<PostmanClient> {
    let postman_settings = state.settings.postman.as_ref()?;
    let api_key = postman_settings.api_key.expose_secret();
    if api_key.is_empty() {
        return None;
    }
    Some(PostmanClient::new(api_key.clone()))
}

/// Create Testmo client from settings.
fn create_testmo_client(state: &AppState) -> (Option<TestmoClient>, Option<i64>) {
    let Some(testmo_settings) = state.settings.testmo.as_ref() else {
        return (None, None);
    };

    let api_key = testmo_settings.api_key.expose_secret();
    let base_url = &testmo_settings.base_url;

    if api_key.is_empty() || base_url.is_empty() {
        return (None, None);
    }

    let client = TestmoClient::new(base_url.clone(), api_key.clone());
    (Some(client), testmo_settings.project_id)
}

/// Search Postman collections.
async fn search_postman(
    client: Option<PostmanClient>,
    keywords: &[String],
) -> Result<Vec<PostmanSearchResult>, String> {
    let Some(client) = client else {
        debug!("Postman client not configured, skipping search");
        return Ok(vec![]);
    };

    client
        .search_collections(keywords, None)
        .await
        .map_err(|e| e.to_string())
}

/// Search Testmo test cases.
async fn search_testmo(
    client: Option<TestmoClient>,
    project_id: Option<i64>,
    keywords: &[String],
) -> Result<Vec<TestmoSearchResult>, String> {
    let (Some(client), Some(project_id)) = (client, project_id) else {
        debug!("Testmo client not configured or no project ID, skipping search");
        return Ok(vec![]);
    };

    client
        .search_test_cases(project_id, keywords)
        .await
        .map_err(|e| e.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unified_search_result_serialization() {
        let result = UnifiedSearchResult {
            source: "postman".to_string(),
            id: "123".to_string(),
            name: "Login API".to_string(),
            description: Some("Tests for login endpoint".to_string()),
            url: "https://go.postman.co/collection/123".to_string(),
            score: 2.5,
            matches: vec!["login".to_string(), "api".to_string()],
        };

        let json = serde_json::to_string(&result).unwrap();
        assert!(json.contains("\"source\":\"postman\""));
        assert!(json.contains("\"score\":2.5"));
    }

    #[test]
    fn test_search_response_serialization() {
        let response = SearchResponse {
            results: vec![],
            postman_count: 5,
            testmo_count: 3,
            search_time_ms: 150,
            keywords_used: vec!["login".to_string(), "auth".to_string()],
        };

        let json = serde_json::to_string(&response).unwrap();
        assert!(json.contains("\"postmanCount\":5"));
        assert!(json.contains("\"testmoCount\":3"));
        assert!(json.contains("\"searchTimeMs\":150"));
    }
}
