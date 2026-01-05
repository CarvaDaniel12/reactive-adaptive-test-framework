# Story 2.8: Setup Wizard Backend API Endpoints

Status: ready-for-dev

## Story

As a developer,
I want API endpoints for the setup wizard,
So that the frontend can save and validate configuration.

## Acceptance Criteria

1. **Given** `qa-pms-api` crate
   **When** setup wizard endpoints are implemented
   **Then** `POST /api/v1/setup/profile` endpoint exists to save user profile

2. **Given** setup wizard endpoints
   **When** Jira test endpoint is called
   **Then** `POST /api/v1/setup/integrations/jira/test` validates Jira connection

3. **Given** setup wizard endpoints
   **When** Postman test endpoint is called
   **Then** `POST /api/v1/setup/integrations/postman/test` validates Postman connection

4. **Given** setup wizard endpoints
   **When** Testmo test endpoint is called
   **Then** `POST /api/v1/setup/integrations/testmo/test` validates Testmo connection

5. **Given** setup wizard endpoints
   **When** complete endpoint is called
   **Then** `POST /api/v1/setup/complete` finalizes and validates full config

6. **Given** setup wizard endpoints
   **When** status endpoint is called
   **Then** `GET /api/v1/setup/status` returns whether setup is complete

7. **Given** all endpoints
   **When** errors occur
   **Then** standardized error responses are returned

8. **Given** all endpoints
   **When** OpenAPI spec is generated
   **Then** all endpoints are documented in OpenAPI spec

## Tasks / Subtasks

- [ ] Task 1: Create setup routes module (AC: #1-6)
  - [ ] 1.1: Create `routes/setup.rs` in qa-pms-api
  - [ ] 1.2: Define route structure with Axum Router
  - [ ] 1.3: Register routes in main app router

- [ ] Task 2: Implement profile endpoint (AC: #1)
  - [ ] 2.1: Create `POST /api/v1/setup/profile` handler
  - [ ] 2.2: Validate request body with serde
  - [ ] 2.3: Store profile temporarily in state
  - [ ] 2.4: Return success response

- [ ] Task 3: Implement Jira test endpoint (AC: #2)
  - [ ] 3.1: Create `POST /api/v1/setup/integrations/jira/test` handler
  - [ ] 3.2: Validate Jira URL format
  - [ ] 3.3: Attempt OAuth token exchange or API ping
  - [ ] 3.4: Return connection result with workspace info

- [ ] Task 4: Implement Postman test endpoint (AC: #3)
  - [ ] 4.1: Create `POST /api/v1/setup/integrations/postman/test` handler
  - [ ] 4.2: Validate API key against Postman API
  - [ ] 4.3: List accessible workspaces
  - [ ] 4.4: Return connection result with workspace count

- [ ] Task 5: Implement Testmo test endpoint (AC: #4)
  - [ ] 5.1: Create `POST /api/v1/setup/integrations/testmo/test` handler
  - [ ] 5.2: Validate credentials against Testmo API
  - [ ] 5.3: List accessible projects
  - [ ] 5.4: Return connection result with project count

- [ ] Task 6: Implement complete endpoint (AC: #5)
  - [ ] 6.1: Create `POST /api/v1/setup/complete` handler
  - [ ] 6.2: Run full validation (from Story 2.7)
  - [ ] 6.3: Generate and save YAML config
  - [ ] 6.4: Return validation results

- [ ] Task 7: Implement status endpoint (AC: #6)
  - [ ] 7.1: Create `GET /api/v1/setup/status` handler
  - [ ] 7.2: Check if config file exists and is valid
  - [ ] 7.3: Return setup completion status

- [ ] Task 8: Implement error handling (AC: #7)
  - [ ] 8.1: Create SetupError enum with thiserror
  - [ ] 8.2: Implement IntoResponse for SetupError
  - [ ] 8.3: Return consistent error structure

- [ ] Task 9: Add OpenAPI documentation (AC: #8)
  - [ ] 9.1: Add utoipa annotations to all handlers
  - [ ] 9.2: Define request/response schemas
  - [ ] 9.3: Verify endpoints appear in Swagger UI

## Dev Notes

### Architecture Alignment

This story implements the **Setup Wizard Backend API Endpoints** per Epic 2 requirements:

- **Location**: `crates/qa-pms-api/src/routes/setup.rs`
- **Error Handling**: `thiserror` for API boundaries
- **Documentation**: `utoipa` for OpenAPI spec

### Technical Implementation Details

#### Setup Routes Module

```rust
// crates/qa-pms-api/src/routes/setup.rs
use axum::{
    extract::{Json, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::{error::ApiError, state::AppState};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/profile", post(save_profile))
        .route("/integrations/jira/test", post(test_jira))
        .route("/integrations/postman/test", post(test_postman))
        .route("/integrations/testmo/test", post(test_testmo))
        .route("/complete", post(complete_setup))
        .route("/status", get(get_status))
}

// Request/Response Types
#[derive(Debug, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ProfileRequest {
    pub display_name: String,
    pub jira_email: String,
    pub ticket_states: Vec<String>,
}

#[derive(Debug, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct JiraTestRequest {
    pub instance_url: String,
    pub client_id: String,
    pub client_secret: String,
}

#[derive(Debug, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct PostmanTestRequest {
    pub api_key: String,
    pub workspace_id: Option<String>,
}

#[derive(Debug, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct TestmoTestRequest {
    pub instance_url: String,
    pub api_key: String,
}

#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ConnectionTestResponse {
    pub success: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace_count: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project_count: Option<u32>,
}

#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct SetupStatusResponse {
    pub complete: bool,
    pub configured_integrations: Vec<String>,
}
```

#### Handler Implementations

```rust
// Profile Handler
#[utoipa::path(
    post,
    path = "/api/v1/setup/profile",
    request_body = ProfileRequest,
    responses(
        (status = 200, description = "Profile saved successfully"),
        (status = 400, description = "Invalid request body")
    ),
    tag = "Setup"
)]
async fn save_profile(
    State(state): State<AppState>,
    Json(req): Json<ProfileRequest>,
) -> Result<impl IntoResponse, ApiError> {
    // Validate required fields
    if req.display_name.is_empty() {
        return Err(ApiError::BadRequest("Display name is required".into()));
    }
    if req.jira_email.is_empty() {
        return Err(ApiError::BadRequest("Jira email is required".into()));
    }

    // Store in state (temporary until complete)
    state.setup_store.save_profile(req).await?;

    Ok(StatusCode::OK)
}

// Jira Test Handler
#[utoipa::path(
    post,
    path = "/api/v1/setup/integrations/jira/test",
    request_body = JiraTestRequest,
    responses(
        (status = 200, description = "Connection test result", body = ConnectionTestResponse),
        (status = 400, description = "Invalid credentials")
    ),
    tag = "Setup"
)]
async fn test_jira(
    State(state): State<AppState>,
    Json(req): Json<JiraTestRequest>,
) -> Result<Json<ConnectionTestResponse>, ApiError> {
    // Validate URL format
    if !req.instance_url.starts_with("https://") {
        return Ok(Json(ConnectionTestResponse {
            success: false,
            message: Some("Jira URL must use HTTPS".into()),
            workspace_count: None,
            project_count: None,
        }));
    }

    // Test connection using qa-pms-jira crate
    match state.jira_client.test_connection(&req).await {
        Ok(info) => Ok(Json(ConnectionTestResponse {
            success: true,
            message: Some(format!("Connected to {}", info.site_name)),
            workspace_count: None,
            project_count: Some(info.project_count),
        })),
        Err(e) => Ok(Json(ConnectionTestResponse {
            success: false,
            message: Some(e.to_string()),
            workspace_count: None,
            project_count: None,
        })),
    }
}

// Postman Test Handler
#[utoipa::path(
    post,
    path = "/api/v1/setup/integrations/postman/test",
    request_body = PostmanTestRequest,
    responses(
        (status = 200, description = "Connection test result", body = ConnectionTestResponse)
    ),
    tag = "Setup"
)]
async fn test_postman(
    State(state): State<AppState>,
    Json(req): Json<PostmanTestRequest>,
) -> Result<Json<ConnectionTestResponse>, ApiError> {
    match state.postman_client.test_connection(&req).await {
        Ok(workspaces) => Ok(Json(ConnectionTestResponse {
            success: true,
            message: None,
            workspace_count: Some(workspaces.len() as u32),
            project_count: None,
        })),
        Err(e) => Ok(Json(ConnectionTestResponse {
            success: false,
            message: Some(e.to_string()),
            workspace_count: None,
            project_count: None,
        })),
    }
}

// Testmo Test Handler
#[utoipa::path(
    post,
    path = "/api/v1/setup/integrations/testmo/test",
    request_body = TestmoTestRequest,
    responses(
        (status = 200, description = "Connection test result", body = ConnectionTestResponse)
    ),
    tag = "Setup"
)]
async fn test_testmo(
    State(state): State<AppState>,
    Json(req): Json<TestmoTestRequest>,
) -> Result<Json<ConnectionTestResponse>, ApiError> {
    match state.testmo_client.test_connection(&req).await {
        Ok(projects) => Ok(Json(ConnectionTestResponse {
            success: true,
            message: None,
            workspace_count: None,
            project_count: Some(projects.len() as u32),
        })),
        Err(e) => Ok(Json(ConnectionTestResponse {
            success: false,
            message: Some(e.to_string()),
            workspace_count: None,
            project_count: None,
        })),
    }
}

// Setup Status Handler
#[utoipa::path(
    get,
    path = "/api/v1/setup/status",
    responses(
        (status = 200, description = "Setup status", body = SetupStatusResponse)
    ),
    tag = "Setup"
)]
async fn get_status(
    State(state): State<AppState>,
) -> Json<SetupStatusResponse> {
    let config = state.config_service.load_config().await;
    
    Json(SetupStatusResponse {
        complete: config.is_ok(),
        configured_integrations: config
            .map(|c| c.get_configured_integrations())
            .unwrap_or_default(),
    })
}
```

#### Error Types

```rust
// crates/qa-pms-api/src/error.rs
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ApiError {
    #[error("Bad request: {0}")]
    BadRequest(String),
    
    #[error("Unauthorized: {0}")]
    Unauthorized(String),
    
    #[error("Not found: {0}")]
    NotFound(String),
    
    #[error("Internal error: {0}")]
    Internal(#[from] anyhow::Error),
}

#[derive(Serialize)]
struct ErrorResponse {
    success: bool,
    message: String,
    code: String,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, code, message) = match &self {
            ApiError::BadRequest(msg) => (StatusCode::BAD_REQUEST, "BAD_REQUEST", msg.clone()),
            ApiError::Unauthorized(msg) => (StatusCode::UNAUTHORIZED, "UNAUTHORIZED", msg.clone()),
            ApiError::NotFound(msg) => (StatusCode::NOT_FOUND, "NOT_FOUND", msg.clone()),
            ApiError::Internal(e) => (StatusCode::INTERNAL_SERVER_ERROR, "INTERNAL", e.to_string()),
        };

        let body = Json(ErrorResponse {
            success: false,
            message,
            code: code.to_string(),
        });

        (status, body).into_response()
    }
}
```

### API Endpoints Summary

| Endpoint | Method | Description |
|----------|--------|-------------|
| `/api/v1/setup/profile` | POST | Save user profile data |
| `/api/v1/setup/integrations/jira/test` | POST | Test Jira connection |
| `/api/v1/setup/integrations/postman/test` | POST | Test Postman connection |
| `/api/v1/setup/integrations/testmo/test` | POST | Test Testmo connection |
| `/api/v1/setup/complete` | POST | Finalize setup and validate config |
| `/api/v1/setup/status` | GET | Check if setup is complete |

### Project Structure Notes

Files to create:
```
crates/qa-pms-api/src/
├── routes/
│   ├── mod.rs           # Add setup module
│   ├── setup.rs         # Setup wizard routes (create)
│   └── health.rs        # Existing health routes
└── error.rs             # API error types (create/modify)
```

### Testing Notes

- Unit test each handler with mock state
- Test error responses for invalid input
- Integration test: Full setup flow with mocked external APIs
- Verify OpenAPI spec includes all endpoints

### Dependencies

Add to `qa-pms-api/Cargo.toml` if not present:
```toml
[dependencies]
thiserror = "1.0"
utoipa = { version = "5.0", features = ["axum_extras"] }
```

### References

- [Source: _bmad-output/planning-artifacts/epics.md#Story 2.8]
- [Source: _bmad-output/planning-artifacts/architecture.md#API Design]

## Dev Agent Record

### Agent Model Used

{{agent_model_name_version}}

### Debug Log References

### Completion Notes List

### File List
