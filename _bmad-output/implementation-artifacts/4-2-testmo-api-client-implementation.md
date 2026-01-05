# Story 4.2: Testmo API Client Implementation

Status: done

## Story

As a developer,
I want a Testmo API client in the backend,
So that I can search and retrieve test cases.

## Acceptance Criteria

1. **Given** `qa-pms-testmo` crate
   **When** Testmo client is implemented
   **Then** the client supports authentication with API key

2. **Given** authenticated client
   **When** project query is made
   **Then** list projects operation is available

3. **Given** authenticated client
   **When** test suite query is made
   **Then** list test suites operation is available

4. **Given** authenticated client
   **When** search is performed
   **Then** search test cases by keyword is available

5. **Given** authenticated client
   **When** test case details are requested
   **Then** get test case details operation is available

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

- [x] Task 1: Create Testmo client struct (AC: #1)
  - [x] 1.1: Create `TestmoClient` struct with base URL and API key
  - [x] 1.2: Configure reqwest client with Bearer auth
  - [x] 1.3: Set configurable base URL (per-instance)
  - [x] 1.4: Add timeout configuration (10s)

- [x] Task 2: Create API response types (AC: #7)
  - [x] 2.1: Create `Project` struct
  - [x] 2.2: Create `TestSuite` struct
  - [x] 2.3: Create `TestCase` struct
  - [x] 2.4: Create `TestRun` struct

- [x] Task 3: Implement list projects (AC: #2)
  - [x] 3.1: Add `list_projects()` method
  - [x] 3.2: Handle pagination
  - [x] 3.3: Parse response to typed structs

- [x] Task 4: Implement list test suites (AC: #3)
  - [x] 4.1: Add `list_test_suites(project_id)` method
  - [x] 4.2: Filter by project
  - [x] 4.3: Return suite summaries

- [x] Task 5: Implement search test cases (AC: #4)
  - [x] 5.1: Add `search_test_cases(keywords)` method
  - [x] 5.2: Search by title
  - [x] 5.3: Search by description/preconditions
  - [x] 5.4: Return ranked results

- [x] Task 6: Implement get test case details (AC: #5)
  - [x] 6.1: Add `get_test_case(case_id)` method
  - [x] 6.2: Include steps and expected results
  - [x] 6.3: Include attachments info

- [x] Task 7: Implement retry logic (AC: #6)
  - [x] 7.1: Create retry wrapper with exponential backoff
  - [x] 7.2: Configure max 3 retries
  - [x] 7.3: Retry on 5xx and network errors

- [x] Task 8: Create error types (AC: #8)
  - [x] 8.1: Create `TestmoError` enum with thiserror
  - [x] 8.2: Map HTTP errors to domain errors
  - [x] 8.3: Include context in error messages

## Dev Notes

### Architecture Alignment

This story implements **Testmo API Client** per Epic 4 requirements:

- **Location**: `crates/qa-pms-testmo/src/`
- **API**: Testmo REST API v1
- **Auth**: Bearer token via `Authorization` header

### Technical Implementation Details

#### Testmo Client

```rust
// crates/qa-pms-testmo/src/client.rs
use crate::error::TestmoError;
use crate::types::*;
use reqwest::Client;
use std::time::Duration;
use tokio::time::sleep;

pub struct TestmoClient {
    http_client: Client,
    api_key: String,
    base_url: String,
}

impl TestmoClient {
    pub fn new(base_url: String, api_key: String) -> Self {
        let http_client = Client::builder()
            .timeout(Duration::from_secs(10))
            .build()
            .expect("Failed to create HTTP client");

        // Ensure base_url doesn't have trailing slash
        let base_url = base_url.trim_end_matches('/').to_string();

        Self {
            http_client,
            api_key,
            base_url,
        }
    }

    /// Make authenticated request with retry
    async fn request<T: serde::de::DeserializeOwned>(
        &self,
        endpoint: &str,
    ) -> Result<T, TestmoError> {
        let url = format!("{}/api/v1{}", self.base_url, endpoint);
        
        self.with_retry(|| async {
            let response = self
                .http_client
                .get(&url)
                .bearer_auth(&self.api_key)
                .send()
                .await?;

            if response.status().is_success() {
                Ok(response.json::<T>().await?)
            } else if response.status() == 401 {
                Err(TestmoError::Unauthorized)
            } else if response.status() == 404 {
                Err(TestmoError::NotFound(endpoint.to_string()))
            } else if response.status() == 429 {
                Err(TestmoError::RateLimited)
            } else {
                let status = response.status();
                let body = response.text().await.unwrap_or_default();
                Err(TestmoError::ApiError { status, body })
            }
        })
        .await
    }

    /// Make POST request with body
    async fn post<T, B>(&self, endpoint: &str, body: &B) -> Result<T, TestmoError>
    where
        T: serde::de::DeserializeOwned,
        B: serde::Serialize,
    {
        let url = format!("{}/api/v1{}", self.base_url, endpoint);
        
        self.with_retry(|| async {
            let response = self
                .http_client
                .post(&url)
                .bearer_auth(&self.api_key)
                .json(body)
                .send()
                .await?;

            if response.status().is_success() {
                Ok(response.json::<T>().await?)
            } else if response.status() == 401 {
                Err(TestmoError::Unauthorized)
            } else {
                let status = response.status();
                let body = response.text().await.unwrap_or_default();
                Err(TestmoError::ApiError { status, body })
            }
        })
        .await
    }

    /// Retry with exponential backoff
    async fn with_retry<T, F, Fut>(&self, f: F) -> Result<T, TestmoError>
    where
        F: Fn() -> Fut,
        Fut: std::future::Future<Output = Result<T, TestmoError>>,
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
                        "Testmo API error (attempt {}): {}, retrying in {:?}",
                        attempt, e, delay
                    );
                    sleep(delay).await;
                }
                Err(e) => return Err(e),
            }
        }
    }

    /// List all projects
    pub async fn list_projects(&self) -> Result<Vec<Project>, TestmoError> {
        let response: ProjectsResponse = self.request("/projects").await?;
        Ok(response.data)
    }

    /// List test suites in a project
    pub async fn list_test_suites(&self, project_id: i64) -> Result<Vec<TestSuite>, TestmoError> {
        let endpoint = format!("/projects/{}/suites", project_id);
        let response: TestSuitesResponse = self.request(&endpoint).await?;
        Ok(response.data)
    }

    /// List test cases in a suite
    pub async fn list_test_cases(
        &self,
        project_id: i64,
        suite_id: Option<i64>,
    ) -> Result<Vec<TestCase>, TestmoError> {
        let endpoint = match suite_id {
            Some(id) => format!("/projects/{}/cases?suite_id={}", project_id, id),
            None => format!("/projects/{}/cases", project_id),
        };
        let response: TestCasesResponse = self.request(&endpoint).await?;
        Ok(response.data)
    }

    /// Get test case details
    pub async fn get_test_case(
        &self,
        project_id: i64,
        case_id: i64,
    ) -> Result<TestCase, TestmoError> {
        let endpoint = format!("/projects/{}/cases/{}", project_id, case_id);
        let response: TestCaseResponse = self.request(&endpoint).await?;
        Ok(response.data)
    }

    /// Search test cases by keywords
    pub async fn search_test_cases(
        &self,
        project_id: i64,
        keywords: &[String],
    ) -> Result<Vec<SearchResult>, TestmoError> {
        // Get all test cases in the project
        let test_cases = self.list_test_cases(project_id, None).await?;
        
        let mut results = Vec::new();
        
        for case in test_cases {
            let mut score = 0.0;
            
            // Score by title
            score += calculate_match_score(&case.title, keywords) * 2.0;
            
            // Score by preconditions
            if let Some(ref preconditions) = case.preconditions {
                score += calculate_match_score(preconditions, keywords);
            }
            
            // Score by steps
            if let Some(ref steps) = case.steps {
                for step in steps {
                    score += calculate_match_score(&step.content, keywords) * 0.5;
                }
            }
            
            if score > 0.0 {
                results.push(SearchResult {
                    source: "testmo".to_string(),
                    id: case.id.to_string(),
                    name: case.title.clone(),
                    description: case.preconditions.clone(),
                    url: format!(
                        "{}/projects/{}/cases/{}",
                        self.base_url, project_id, case.id
                    ),
                    score,
                    matches: vec![case.title],
                });
            }
        }

        // Sort by score descending
        results.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap());
        
        Ok(results)
    }

    /// Create a test run
    pub async fn create_test_run(
        &self,
        project_id: i64,
        name: &str,
        case_ids: &[i64],
    ) -> Result<TestRun, TestmoError> {
        let endpoint = format!("/projects/{}/runs", project_id);
        
        let body = CreateTestRunRequest {
            name: name.to_string(),
            case_ids: case_ids.to_vec(),
        };
        
        let response: TestRunResponse = self.post(&endpoint, &body).await?;
        Ok(response.data)
    }
}

fn calculate_match_score(text: &str, keywords: &[String]) -> f32 {
    let text_lower = text.to_lowercase();
    let mut score = 0.0;
    
    for keyword in keywords {
        let keyword_lower = keyword.to_lowercase();
        if text_lower.contains(&keyword_lower) {
            score += 1.0;
            if text_lower.split_whitespace().any(|w| w == keyword_lower) {
                score += 0.5;
            }
        }
    }
    
    score
}
```

#### API Types

```rust
// crates/qa-pms-testmo/src/types.rs
use serde::{Deserialize, Serialize};

// Response wrappers
#[derive(Debug, Deserialize)]
pub struct ProjectsResponse {
    pub data: Vec<Project>,
}

#[derive(Debug, Deserialize)]
pub struct TestSuitesResponse {
    pub data: Vec<TestSuite>,
}

#[derive(Debug, Deserialize)]
pub struct TestCasesResponse {
    pub data: Vec<TestCase>,
}

#[derive(Debug, Deserialize)]
pub struct TestCaseResponse {
    pub data: TestCase,
}

#[derive(Debug, Deserialize)]
pub struct TestRunResponse {
    pub data: TestRun,
}

// Core types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    pub id: i64,
    pub name: String,
    pub description: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestSuite {
    pub id: i64,
    pub project_id: i64,
    pub name: String,
    pub description: Option<String>,
    pub parent_id: Option<i64>,
    pub depth: i32,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestCase {
    pub id: i64,
    pub project_id: i64,
    pub suite_id: Option<i64>,
    pub title: String,
    pub preconditions: Option<String>,
    pub priority_id: Option<i32>,
    pub type_id: Option<i32>,
    pub template_id: Option<i32>,
    pub steps: Option<Vec<TestStep>>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestStep {
    pub content: String,
    pub expected: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestRun {
    pub id: i64,
    pub project_id: i64,
    pub name: String,
    pub description: Option<String>,
    pub status_id: i32,
    pub created_at: String,
    pub updated_at: String,
}

// Request types
#[derive(Debug, Serialize)]
pub struct CreateTestRunRequest {
    pub name: String,
    pub case_ids: Vec<i64>,
}

// Search result (shared with Postman)
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
// crates/qa-pms-testmo/src/error.rs
use reqwest::StatusCode;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum TestmoError {
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

impl TestmoError {
    pub fn is_retryable(&self) -> bool {
        matches!(
            self,
            TestmoError::RateLimited
                | TestmoError::Network(_)
                | TestmoError::ApiError { status, .. } if status.is_server_error()
        )
    }
}
```

#### Module Exports

```rust
// crates/qa-pms-testmo/src/lib.rs
mod client;
mod error;
mod types;
pub mod health;

pub use client::TestmoClient;
pub use error::TestmoError;
pub use types::*;
```

### Project Structure Notes

Files to create:
```
crates/qa-pms-testmo/src/
├── lib.rs          # Module exports
├── client.rs       # TestmoClient implementation
├── types.rs        # API response types
├── error.rs        # Error types
└── health.rs       # Health check (from Epic 3)
```

### Testmo API Reference

| Endpoint | Method | Description |
|----------|--------|-------------|
| `/api/v1/projects` | GET | List all projects |
| `/api/v1/projects/{id}/suites` | GET | List test suites |
| `/api/v1/projects/{id}/cases` | GET | List test cases |
| `/api/v1/projects/{id}/cases/{id}` | GET | Get test case details |
| `/api/v1/projects/{id}/runs` | POST | Create test run |

### Testing Notes

- Unit test retry logic with mocked failures
- Unit test search scoring algorithm
- Integration test: List projects with real API (optional)
- Test error mapping for all HTTP status codes
- Test test run creation

### References

- [Source: _bmad-output/planning-artifacts/epics.md#Story 4.2]
- [Source: Testmo API Documentation](https://docs.testmo.com/api/)

## Dev Agent Record

### Agent Model Used

Claude Opus 4.5

### Debug Log References

N/A

### Completion Notes List

1. Created `TestmoClient` in `client.rs` with:
   - Configurable base URL and API key authentication via Bearer token
   - 10s request timeout
   - `list_projects()`, `list_test_suites()`, `list_test_cases()`, `get_test_case()`
   - `search_test_cases()` with weighted scoring (title 2x, preconditions 1x, steps 0.5x)
   - `create_test_run()` for creating test runs with case IDs
   - Retry with exponential backoff (max 3 retries, 1s base delay)

2. Created `TestmoError` in `error.rs` with:
   - `Unauthorized`, `NotFound`, `RateLimited`, `ApiError`, `Network`, `Parse` variants
   - `is_retryable()` method for retry logic

3. Created API types in `types.rs`:
   - `Project`, `ProjectsResponse`
   - `TestSuite`, `TestSuitesResponse`
   - `TestCase`, `TestCaseResponse`, `TestCasesResponse`, `TestStep`
   - `TestRun`, `TestRunResponse`, `CreateTestRunRequest`
   - `SearchResult` with keyword matching and score ranking

4. Updated `lib.rs` to export all public types

### File List

- `crates/qa-pms-testmo/src/client.rs` - TestmoClient implementation
- `crates/qa-pms-testmo/src/error.rs` - TestmoError enum
- `crates/qa-pms-testmo/src/types.rs` - API response types
- `crates/qa-pms-testmo/src/lib.rs` - Module exports (updated)
