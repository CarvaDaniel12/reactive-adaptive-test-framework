# Story 4.6: Search API Endpoints

Status: done

## Story

As a developer,
I want search endpoints in the API,
So that the frontend can trigger and display searches.

## Acceptance Criteria

1. **Given** `qa-pms-api` crate
   **When** search endpoints are implemented
   **Then** `POST /api/v1/search/postman` endpoint exists

2. **Given** `qa-pms-api` crate
   **When** search endpoints are implemented
   **Then** `POST /api/v1/search/testmo` endpoint exists

3. **Given** `qa-pms-api` crate
   **When** search endpoints are implemented
   **Then** `POST /api/v1/search/all` endpoint exists (parallel search)

4. **Given** `qa-pms-api` crate
   **When** search endpoints are implemented
   **Then** `POST /api/v1/testmo/runs` endpoint exists (create test run)

5. **Given** search endpoint
   **When** request is made
   **Then** endpoint accepts `{ keywords: string[], ticketId?: string }`

6. **Given** search endpoint
   **When** response is returned
   **Then** results include source, name, description, url

7. **Given** all endpoints
   **When** API documentation is generated
   **Then** endpoints are documented in OpenAPI spec

## Tasks / Subtasks

- [ ] Task 1: Create search request/response types (AC: #5, #6)
  - [ ] 1.1: Define `SearchRequest` struct
  - [ ] 1.2: Define `SearchResponse` struct
  - [ ] 1.3: Define `SearchResult` struct
  - [ ] 1.4: Add serde serialization

- [ ] Task 2: Implement Postman search endpoint (AC: #1)
  - [ ] 2.1: Create `POST /api/v1/search/postman` route
  - [ ] 2.2: Accept keywords in request body
  - [ ] 2.3: Call Postman client search
  - [ ] 2.4: Map results to response type

- [ ] Task 3: Implement Testmo search endpoint (AC: #2)
  - [ ] 3.1: Create `POST /api/v1/search/testmo` route
  - [ ] 3.2: Accept keywords in request body
  - [ ] 3.3: Call Testmo client search
  - [ ] 3.4: Map results to response type

- [ ] Task 4: Implement unified search endpoint (AC: #3)
  - [ ] 4.1: Create `POST /api/v1/search/all` route
  - [ ] 4.2: Run both searches in parallel
  - [ ] 4.3: Merge and sort results
  - [ ] 4.4: Return combined response

- [ ] Task 5: Register routes in router (AC: #1, #2, #3, #4)
  - [ ] 5.1: Create search routes module
  - [ ] 5.2: Register all search endpoints
  - [ ] 5.3: Add middleware (logging, etc.)

- [ ] Task 6: Add OpenAPI documentation (AC: #7)
  - [ ] 6.1: Add utoipa annotations to endpoints
  - [ ] 6.2: Document request/response schemas
  - [ ] 6.3: Add examples to docs

- [ ] Task 7: Add error handling (AC: #1, #2, #3)
  - [ ] 7.1: Handle client not configured
  - [ ] 7.2: Handle search failures gracefully
  - [ ] 7.3: Return appropriate HTTP status codes

## Dev Notes

### Architecture Alignment

This story implements **Search API Endpoints** per Epic 4 requirements:

- **Location**: `crates/qa-pms-api/src/routes/search.rs`
- **Documentation**: OpenAPI via `utoipa`
- **Integration**: Uses clients from Stories 4.1 and 4.2

### Technical Implementation Details

#### Request/Response Types

```rust
// crates/qa-pms-api/src/routes/search.rs
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// Search request payload
#[derive(Debug, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct SearchRequest {
    /// Keywords to search for
    #[schema(example = json!(["login", "authentication"]))]
    pub keywords: Vec<String>,
    
    /// Optional ticket context for better results
    #[schema(example = "PROJ-123")]
    pub ticket_id: Option<String>,
    
    /// Optional ticket title for keyword extraction
    pub title: Option<String>,
    
    /// Optional ticket description for keyword extraction
    pub description: Option<String>,
}

/// Individual search result
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct SearchResult {
    /// Source system (postman, testmo)
    #[schema(example = "postman")]
    pub source: String,
    
    /// Unique identifier in source system
    #[schema(example = "12345")]
    pub id: String,
    
    /// Name/title of the test
    #[schema(example = "Login API Tests")]
    pub name: String,
    
    /// Brief description
    pub description: Option<String>,
    
    /// Direct URL to open in source system
    #[schema(example = "https://go.postman.co/collection/12345")]
    pub url: String,
    
    /// Relevance score (higher = better match)
    #[schema(example = 2.5)]
    pub score: f32,
    
    /// Matched items (e.g., request names)
    #[schema(example = json!(["Login Request", "Logout Request"]))]
    pub matches: Vec<String>,
}

/// Search response payload
#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct SearchResponse {
    /// Search results sorted by relevance
    pub results: Vec<SearchResult>,
    
    /// Total results from Postman
    pub postman_count: usize,
    
    /// Total results from Testmo
    pub testmo_count: usize,
    
    /// Search execution time in milliseconds
    #[schema(example = 450)]
    pub search_time_ms: u64,
    
    /// Keywords that were actually used
    #[schema(example = json!(["login", "authentication"]))]
    pub keywords_used: Vec<String>,
}

/// Postman-only search response
#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct PostmanSearchResponse {
    pub results: Vec<SearchResult>,
    pub count: usize,
    pub search_time_ms: u64,
}

/// Testmo-only search response  
#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct TestmoSearchResponse {
    pub results: Vec<SearchResult>,
    pub count: usize,
    pub search_time_ms: u64,
}
```

#### Search Endpoints

```rust
// crates/qa-pms-api/src/routes/search.rs (continued)
use axum::{extract::State, http::StatusCode, Json};
use qa_pms_core::keywords::KeywordExtractor;
use std::sync::Arc;
use std::time::Instant;

use crate::error::ApiError;
use crate::state::AppState;

/// Search Postman collections
#[utoipa::path(
    post,
    path = "/api/v1/search/postman",
    request_body = SearchRequest,
    responses(
        (status = 200, description = "Search results", body = PostmanSearchResponse),
        (status = 400, description = "Invalid request"),
        (status = 503, description = "Postman not configured")
    ),
    tag = "search"
)]
pub async fn search_postman(
    State(state): State<Arc<AppState>>,
    Json(request): Json<SearchRequest>,
) -> Result<Json<PostmanSearchResponse>, ApiError> {
    let start = Instant::now();
    
    let postman_client = state.postman_client.as_ref()
        .ok_or_else(|| ApiError::ServiceUnavailable("Postman integration not configured".into()))?;

    // Get keywords (from request or extract from title/description)
    let keywords = get_keywords(&request);
    
    if keywords.is_empty() {
        return Ok(Json(PostmanSearchResponse {
            results: vec![],
            count: 0,
            search_time_ms: start.elapsed().as_millis() as u64,
        }));
    }

    let results = postman_client
        .search_collections(&keywords, None)
        .await
        .map_err(|e| {
            tracing::error!("Postman search failed: {}", e);
            ApiError::Internal(format!("Search failed: {}", e))
        })?;

    let mapped_results: Vec<SearchResult> = results
        .into_iter()
        .map(|r| SearchResult {
            source: r.source,
            id: r.id,
            name: r.name,
            description: r.description,
            url: r.url,
            score: r.score,
            matches: r.matches,
        })
        .collect();

    let count = mapped_results.len();
    
    Ok(Json(PostmanSearchResponse {
        results: mapped_results,
        count,
        search_time_ms: start.elapsed().as_millis() as u64,
    }))
}

/// Search Testmo test cases
#[utoipa::path(
    post,
    path = "/api/v1/search/testmo",
    request_body = SearchRequest,
    responses(
        (status = 200, description = "Search results", body = TestmoSearchResponse),
        (status = 400, description = "Invalid request"),
        (status = 503, description = "Testmo not configured")
    ),
    tag = "search"
)]
pub async fn search_testmo(
    State(state): State<Arc<AppState>>,
    Json(request): Json<SearchRequest>,
) -> Result<Json<TestmoSearchResponse>, ApiError> {
    let start = Instant::now();
    
    let testmo_client = state.testmo_client.as_ref()
        .ok_or_else(|| ApiError::ServiceUnavailable("Testmo integration not configured".into()))?;
    
    let project_id = state.testmo_project_id
        .ok_or_else(|| ApiError::ServiceUnavailable("Testmo project not configured".into()))?;

    // Get keywords
    let keywords = get_keywords(&request);
    
    if keywords.is_empty() {
        return Ok(Json(TestmoSearchResponse {
            results: vec![],
            count: 0,
            search_time_ms: start.elapsed().as_millis() as u64,
        }));
    }

    let results = testmo_client
        .search_test_cases(project_id, &keywords)
        .await
        .map_err(|e| {
            tracing::error!("Testmo search failed: {}", e);
            ApiError::Internal(format!("Search failed: {}", e))
        })?;

    let mapped_results: Vec<SearchResult> = results
        .into_iter()
        .map(|r| SearchResult {
            source: r.source,
            id: r.id,
            name: r.name,
            description: r.description,
            url: r.url,
            score: r.score,
            matches: r.matches,
        })
        .collect();

    let count = mapped_results.len();
    
    Ok(Json(TestmoSearchResponse {
        results: mapped_results,
        count,
        search_time_ms: start.elapsed().as_millis() as u64,
    }))
}

/// Search all configured integrations in parallel
#[utoipa::path(
    post,
    path = "/api/v1/search/all",
    request_body = SearchRequest,
    responses(
        (status = 200, description = "Combined search results", body = SearchResponse),
        (status = 400, description = "Invalid request")
    ),
    tag = "search"
)]
pub async fn search_all(
    State(state): State<Arc<AppState>>,
    Json(request): Json<SearchRequest>,
) -> Result<Json<SearchResponse>, ApiError> {
    let start = Instant::now();
    
    // Get keywords
    let keywords = get_keywords(&request);
    
    if keywords.is_empty() {
        return Ok(Json(SearchResponse {
            results: vec![],
            postman_count: 0,
            testmo_count: 0,
            search_time_ms: start.elapsed().as_millis() as u64,
            keywords_used: vec![],
        }));
    }

    // Run searches in parallel
    let postman_future = search_postman_internal(&state, &keywords);
    let testmo_future = search_testmo_internal(&state, &keywords);

    let (postman_results, testmo_results) = tokio::join!(postman_future, testmo_future);

    // Collect results
    let mut all_results: Vec<SearchResult> = Vec::new();
    
    let postman_count = if let Ok(results) = postman_results {
        let count = results.len();
        all_results.extend(results);
        count
    } else {
        0
    };

    let testmo_count = if let Ok(results) = testmo_results {
        let count = results.len();
        all_results.extend(results);
        count
    } else {
        0
    };

    // Sort by score descending
    all_results.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap_or(std::cmp::Ordering::Equal));

    let duration = start.elapsed();
    if duration.as_secs() > 3 {
        tracing::warn!(
            duration_ms = duration.as_millis(),
            "Slow search detected (> 3s)"
        );
    }

    Ok(Json(SearchResponse {
        results: all_results,
        postman_count,
        testmo_count,
        search_time_ms: duration.as_millis() as u64,
        keywords_used: keywords,
    }))
}

// Internal helper functions

fn get_keywords(request: &SearchRequest) -> Vec<String> {
    // If keywords provided, use them
    if !request.keywords.is_empty() {
        return request.keywords.clone();
    }

    // Otherwise extract from title/description
    let extractor = KeywordExtractor::default();
    let mut texts = Vec::new();
    
    if let Some(ref title) = request.title {
        texts.push(title.as_str());
    }
    if let Some(ref desc) = request.description {
        texts.push(desc.as_str());
    }

    if texts.is_empty() {
        vec![]
    } else {
        extractor.extract(&texts)
    }
}

async fn search_postman_internal(
    state: &AppState,
    keywords: &[String],
) -> Result<Vec<SearchResult>, ()> {
    if let Some(client) = &state.postman_client {
        client
            .search_collections(keywords, None)
            .await
            .map(|results| {
                results
                    .into_iter()
                    .map(|r| SearchResult {
                        source: r.source,
                        id: r.id,
                        name: r.name,
                        description: r.description,
                        url: r.url,
                        score: r.score,
                        matches: r.matches,
                    })
                    .collect()
            })
            .map_err(|e| {
                tracing::error!("Postman search failed: {}", e);
            })
    } else {
        Ok(vec![])
    }
}

async fn search_testmo_internal(
    state: &AppState,
    keywords: &[String],
) -> Result<Vec<SearchResult>, ()> {
    match (&state.testmo_client, state.testmo_project_id) {
        (Some(client), Some(project_id)) => {
            client
                .search_test_cases(project_id, keywords)
                .await
                .map(|results| {
                    results
                        .into_iter()
                        .map(|r| SearchResult {
                            source: r.source,
                            id: r.id,
                            name: r.name,
                            description: r.description,
                            url: r.url,
                            score: r.score,
                            matches: r.matches,
                        })
                        .collect()
                })
                .map_err(|e| {
                    tracing::error!("Testmo search failed: {}", e);
                })
        }
        _ => Ok(vec![]),
    }
}
```

#### Route Registration

```rust
// crates/qa-pms-api/src/routes/mod.rs
use axum::{routing::post, Router};
use std::sync::Arc;

use crate::state::AppState;

mod search;
mod testmo;

pub use search::*;
pub use testmo::*;

pub fn search_routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/search/postman", post(search::search_postman))
        .route("/search/testmo", post(search::search_testmo))
        .route("/search/all", post(search::search_all))
}

pub fn testmo_routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/testmo/runs", post(testmo::create_test_run))
}

pub fn api_routes() -> Router<Arc<AppState>> {
    Router::new()
        .merge(search_routes())
        .merge(testmo_routes())
}
```

#### OpenAPI Schema Registration

```rust
// crates/qa-pms-api/src/openapi.rs (additions)
use crate::routes::search::*;
use crate::routes::testmo::*;
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(
        search_postman,
        search_testmo,
        search_all,
        create_test_run,
    ),
    components(
        schemas(
            SearchRequest,
            SearchResult,
            SearchResponse,
            PostmanSearchResponse,
            TestmoSearchResponse,
            CreateTestRunRequest,
            CreateTestRunResponse,
        )
    ),
    tags(
        (name = "search", description = "Search endpoints for Postman and Testmo"),
        (name = "testmo", description = "Testmo-specific operations"),
    )
)]
pub struct ApiDoc;
```

### Project Structure Notes

Files to create/modify:
```
crates/qa-pms-api/src/
├── routes/
│   ├── mod.rs        # Route registration
│   ├── search.rs     # Search endpoints (NEW)
│   └── testmo.rs     # Testmo endpoints (from Story 4.5)
└── openapi.rs        # OpenAPI documentation
```

### API Endpoints Summary

| Endpoint | Method | Description | Request | Response |
|----------|--------|-------------|---------|----------|
| `/api/v1/search/postman` | POST | Search Postman collections | SearchRequest | PostmanSearchResponse |
| `/api/v1/search/testmo` | POST | Search Testmo test cases | SearchRequest | TestmoSearchResponse |
| `/api/v1/search/all` | POST | Search both in parallel | SearchRequest | SearchResponse |
| `/api/v1/testmo/runs` | POST | Create test run | CreateTestRunRequest | CreateTestRunResponse |

### Testing Notes

- Test each endpoint independently
- Test parallel search timing
- Test keyword extraction fallback
- Test empty keywords handling
- Test service unavailable responses
- Load test parallel search endpoint
- Verify OpenAPI spec is valid

### References

- [Source: _bmad-output/planning-artifacts/epics.md#Story 4.6]
- [Source: _bmad-output/planning-artifacts/architecture-decision.md#API Design]

## Dev Agent Record

### Agent Model Used

Claude Opus 4.5

### Debug Log References

N/A

### Completion Notes List

- POST /api/v1/search/postman - search Postman collections only
- POST /api/v1/search/testmo - search Testmo test cases only
- POST /api/v1/search/all - parallel search both integrations
- POST /api/v1/testmo/runs - create test run (from Story 4.5)
- All endpoints accept KeywordSearchRequest with keywords[] and optional ticketId
- All endpoints documented in OpenAPI specification
- Context7 used for Axum documentation

### File List

- crates/qa-pms-api/src/routes/search.rs (updated - added 3 new endpoints)
- crates/qa-pms-api/src/routes/mod.rs (updated - OpenAPI schemas)
