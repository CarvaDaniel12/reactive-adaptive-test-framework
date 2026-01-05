# Story 4.1: Postman API Client Implementation

Status: done

## Story

As a developer,
I want a Postman API client in the backend,
So that I can search and retrieve test collections.

## Acceptance Criteria

1. **Given** `qa-pms-postman` crate
   **When** Postman client is implemented
   **Then** the client supports authentication with API key

2. **Given** authenticated client
   **When** workspace query is made
   **Then** list workspaces operation is available

3. **Given** authenticated client
   **When** collection query is made
   **Then** list collections in workspace operation is available

4. **Given** authenticated client
   **When** search is performed
   **Then** search collections by name/keyword is available

5. **Given** authenticated client
   **When** collection details are requested
   **Then** get collection details with requests is available

6. **Given** any API call
   **When** network issues occur
   **Then** all API calls use retry with exponential backoff

7. **Given** API responses
   **When** deserialization occurs
   **Then** responses are typed with serde structs

8. **Given** API errors
   **When** error handling occurs
   **Then** errors are mapped to domain error types

## Tasks / Subtasks

- [x] Task 1: Create Postman client struct (AC: #1)
  - [x] 1.1: Create `PostmanClient` struct with API key
  - [x] 1.2: Configure reqwest client with default headers
  - [x] 1.3: Set base URL to `https://api.getpostman.com`
  - [x] 1.4: Add timeout configuration (10s)

- [x] Task 2: Create API response types (AC: #7)
  - [x] 2.1: Create `Workspace` struct
  - [x] 2.2: Create `Collection` struct
  - [x] 2.3: Create `CollectionInfo` struct with requests
  - [x] 2.4: Create `Request` struct for collection items

- [x] Task 3: Implement list workspaces (AC: #2)
  - [x] 3.1: Add `list_workspaces()` method
  - [x] 3.2: Parse JSON response to typed struct
  - [x] 3.3: Handle pagination if needed

- [x] Task 4: Implement list collections (AC: #3)
  - [x] 4.1: Add `list_collections(workspace_id)` method
  - [x] 4.2: Filter by workspace
  - [x] 4.3: Return collection summaries

- [x] Task 5: Implement search collections (AC: #4)
  - [x] 5.1: Add `search_collections(keywords)` method
  - [x] 5.2: Search by collection name
  - [x] 5.3: Search by request names within collections
  - [x] 5.4: Return ranked results

- [x] Task 6: Implement get collection details (AC: #5)
  - [x] 6.1: Add `get_collection(collection_id)` method
  - [x] 6.2: Include all requests in response
  - [x] 6.3: Parse nested folder structure

- [x] Task 7: Implement retry logic (AC: #6)
  - [x] 7.1: Create retry wrapper with exponential backoff
  - [x] 7.2: Configure max 3 retries
  - [x] 7.3: Retry on 5xx and network errors

- [x] Task 8: Create error types (AC: #8)
  - [x] 8.1: Create `PostmanError` enum with thiserror
  - [x] 8.2: Map HTTP errors to domain errors
  - [x] 8.3: Include context in error messages

## Dev Notes

### Architecture Alignment

This story implements **Postman API Client** per Epic 4 requirements:

- **Location**: `crates/qa-pms-postman/src/`
- **API**: Postman API v10 (https://api.getpostman.com)
- **Auth**: API Key via `X-Api-Key` header

### Technical Implementation Details

#### Postman Client

```rust
// crates/qa-pms-postman/src/client.rs
use crate::error::PostmanError;
use crate::types::*;
use reqwest::Client;
use std::time::Duration;
use tokio::time::sleep;

pub struct PostmanClient {
    http_client: Client,
    api_key: String,
    base_url: String,
}

impl PostmanClient {
    const BASE_URL: &'static str = "https://api.getpostman.com";

    pub fn new(api_key: String) -> Self {
        let http_client = Client::builder()
            .timeout(Duration::from_secs(10))
            .build()
            .expect("Failed to create HTTP client");

        Self {
            http_client,
            api_key,
            base_url: Self::BASE_URL.to_string(),
        }
    }

    /// Make authenticated request with retry
    async fn request<T: serde::de::DeserializeOwned>(
        &self,
        endpoint: &str,
    ) -> Result<T, PostmanError> {
        let url = format!("{}{}", self.base_url, endpoint);
        
        self.with_retry(|| async {
            let response = self
                .http_client
                .get(&url)
                .header("X-Api-Key", &self.api_key)
                .send()
                .await?;

            if response.status().is_success() {
                Ok(response.json::<T>().await?)
            } else if response.status() == 401 {
                Err(PostmanError::Unauthorized)
            } else if response.status() == 404 {
                Err(PostmanError::NotFound(endpoint.to_string()))
            } else if response.status() == 429 {
                Err(PostmanError::RateLimited)
            } else {
                let status = response.status();
                let body = response.text().await.unwrap_or_default();
                Err(PostmanError::ApiError { status, body })
            }
        })
        .await
    }

    /// Retry with exponential backoff
    async fn with_retry<T, F, Fut>(&self, f: F) -> Result<T, PostmanError>
    where
        F: Fn() -> Fut,
        Fut: std::future::Future<Output = Result<T, PostmanError>>,
    {
        let mut attempt = 0;
        let max_attempts = 3;
        let base_delay = Duration::from_secs(1);

        loop {
            attempt += 1;
            match f().await {
                Ok(result) => return Ok(result),
                Err(e) if e.is_retryable() && attempt < max_attempts => {
                    let delay = base_delay * 2u32.pow(attempt - 1);
                    tracing::warn!(
                        "Postman API error (attempt {}): {}, retrying in {:?}",
                        attempt, e, delay
                    );
                    sleep(delay).await;
                }
                Err(e) => return Err(e),
            }
        }
    }

    /// List all workspaces
    pub async fn list_workspaces(&self) -> Result<Vec<Workspace>, PostmanError> {
        let response: WorkspacesResponse = self.request("/workspaces").await?;
        Ok(response.workspaces)
    }

    /// List collections in a workspace
    pub async fn list_collections(
        &self,
        workspace_id: Option<&str>,
    ) -> Result<Vec<CollectionSummary>, PostmanError> {
        let endpoint = match workspace_id {
            Some(id) => format!("/collections?workspace={}", id),
            None => "/collections".to_string(),
        };
        let response: CollectionsResponse = self.request(&endpoint).await?;
        Ok(response.collections)
    }

    /// Get full collection with requests
    pub async fn get_collection(&self, collection_id: &str) -> Result<Collection, PostmanError> {
        let endpoint = format!("/collections/{}", collection_id);
        let response: CollectionResponse = self.request(&endpoint).await?;
        Ok(response.collection)
    }

    /// Search collections by keywords
    pub async fn search_collections(
        &self,
        keywords: &[String],
        workspace_id: Option<&str>,
    ) -> Result<Vec<SearchResult>, PostmanError> {
        // Get all collections
        let collections = self.list_collections(workspace_id).await?;
        
        let mut results = Vec::new();
        
        for collection in collections {
            let score = calculate_match_score(&collection.name, keywords);
            
            if score > 0.0 {
                // Get full collection for request details
                let full_collection = self.get_collection(&collection.id).await?;
                
                // Search within requests
                let request_matches = search_requests(&full_collection, keywords);
                
                results.push(SearchResult {
                    source: "postman".to_string(),
                    id: collection.id,
                    name: collection.name,
                    description: full_collection.info.description,
                    url: format!("https://go.postman.co/collection/{}", collection.uid),
                    score: score + (request_matches.len() as f32 * 0.1),
                    matches: request_matches,
                });
            }
        }

        // Sort by score descending
        results.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap());
        
        Ok(results)
    }
}

fn calculate_match_score(text: &str, keywords: &[String]) -> f32 {
    let text_lower = text.to_lowercase();
    let mut score = 0.0;
    
    for keyword in keywords {
        let keyword_lower = keyword.to_lowercase();
        if text_lower.contains(&keyword_lower) {
            score += 1.0;
            // Bonus for exact word match
            if text_lower.split_whitespace().any(|w| w == keyword_lower) {
                score += 0.5;
            }
        }
    }
    
    score
}

fn search_requests(collection: &Collection, keywords: &[String]) -> Vec<String> {
    let mut matches = Vec::new();
    
    fn search_items(items: &[CollectionItem], keywords: &[String], matches: &mut Vec<String>) {
        for item in items {
            if let Some(name) = &item.name {
                if calculate_match_score(name, keywords) > 0.0 {
                    matches.push(name.clone());
                }
            }
            if let Some(children) = &item.item {
                search_items(children, keywords, matches);
            }
        }
    }
    
    if let Some(items) = &collection.item {
        search_items(items, keywords, &mut matches);
    }
    
    matches
}
```

#### API Types

```rust
// crates/qa-pms-postman/src/types.rs
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkspacesResponse {
    pub workspaces: Vec<Workspace>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Workspace {
    pub id: String,
    pub name: String,
    #[serde(rename = "type")]
    pub workspace_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CollectionsResponse {
    pub collections: Vec<CollectionSummary>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollectionSummary {
    pub id: String,
    pub uid: String,
    pub name: String,
    pub owner: Option<String>,
    #[serde(rename = "createdAt")]
    pub created_at: Option<String>,
    #[serde(rename = "updatedAt")]
    pub updated_at: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CollectionResponse {
    pub collection: Collection,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Collection {
    pub info: CollectionInfo,
    pub item: Option<Vec<CollectionItem>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollectionInfo {
    #[serde(rename = "_postman_id")]
    pub postman_id: Option<String>,
    pub name: String,
    pub description: Option<String>,
    pub schema: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollectionItem {
    pub id: Option<String>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub request: Option<RequestInfo>,
    pub item: Option<Vec<CollectionItem>>, // Nested folders
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequestInfo {
    pub method: Option<String>,
    pub url: Option<RequestUrl>,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RequestUrl {
    Simple(String),
    Complex {
        raw: Option<String>,
        host: Option<Vec<String>>,
        path: Option<Vec<String>>,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchResult {
    pub source: String,
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub url: String,
    pub score: f32,
    pub matches: Vec<String>,
}
```

#### Error Types

```rust
// crates/qa-pms-postman/src/error.rs
use reqwest::StatusCode;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum PostmanError {
    #[error("Unauthorized - invalid API key")]
    Unauthorized,

    #[error("Resource not found: {0}")]
    NotFound(String),

    #[error("Rate limited - too many requests")]
    RateLimited,

    #[error("API error (HTTP {status}): {body}")]
    ApiError { status: StatusCode, body: String },

    #[error("Network error: {0}")]
    Network(#[from] reqwest::Error),

    #[error("Failed to parse response: {0}")]
    Parse(String),
}

impl PostmanError {
    pub fn is_retryable(&self) -> bool {
        matches!(
            self,
            PostmanError::RateLimited
                | PostmanError::Network(_)
                | PostmanError::ApiError { status, .. } if status.is_server_error()
        )
    }
}
```

#### Module Exports

```rust
// crates/qa-pms-postman/src/lib.rs
mod client;
mod error;
mod types;
pub mod health;

pub use client::PostmanClient;
pub use error::PostmanError;
pub use types::*;
```

### Project Structure Notes

Files to create:
```
crates/qa-pms-postman/src/
├── lib.rs          # Module exports
├── client.rs       # PostmanClient implementation
├── types.rs        # API response types
├── error.rs        # Error types
└── health.rs       # Health check (from Epic 3)
```

### Postman API Reference

| Endpoint | Method | Description |
|----------|--------|-------------|
| `/workspaces` | GET | List all workspaces |
| `/collections` | GET | List all collections |
| `/collections?workspace={id}` | GET | List collections in workspace |
| `/collections/{id}` | GET | Get collection details |

### Testing Notes

- Unit test retry logic with mocked failures
- Unit test search scoring algorithm
- Integration test: List workspaces with real API (optional)
- Test error mapping for all HTTP status codes
- Test nested folder parsing in collections

### References

- [Source: _bmad-output/planning-artifacts/epics.md#Story 4.1]
- [Source: Postman API Documentation](https://learning.postman.com/docs/developer/postman-api/intro-api/)

## Dev Agent Record

### Agent Model Used

Claude Opus 4.5

### Debug Log References

N/A

### Completion Notes List

1. Created `PostmanClient` in `client.rs` with:
   - API key authentication via `X-Api-Key` header
   - 10s request timeout
   - `list_workspaces()`, `list_collections()`, `get_collection()`, `search_collections()`
   - Retry with exponential backoff (max 3 retries, 1s base delay)

2. Created `PostmanError` in `error.rs` with:
   - `Unauthorized`, `NotFound`, `RateLimited`, `ApiError`, `Network`, `Parse` variants
   - `is_retryable()` method for retry logic

3. Created API types in `types.rs`:
   - `Workspace`, `WorkspacesResponse`
   - `Collection`, `CollectionSummary`, `CollectionsResponse`, `CollectionResponse`
   - `CollectionInfo`, `CollectionItem`, `RequestInfo`, `RequestUrl`
   - `SearchResult` with keyword matching and score ranking

4. Updated `lib.rs` to export all public types

### File List

- `crates/qa-pms-postman/src/client.rs` - PostmanClient implementation
- `crates/qa-pms-postman/src/error.rs` - PostmanError enum
- `crates/qa-pms-postman/src/types.rs` - API response types
- `crates/qa-pms-postman/src/lib.rs` - Module exports (updated)
