//! Integration tests for request ID middleware.

use axum::{
    body::Body,
    extract::Request,
    http::{header::HeaderValue, Request as HttpRequest, StatusCode},
    routing::get,
    Router,
};
use qa_pms_api::middleware::request_id_middleware;
use tower::ServiceExt;
use uuid::Uuid;

const REQUEST_ID_HEADER: &str = "x-request-id";

async fn handler() -> &'static str {
    "ok"
}

fn create_app() -> Router {
    Router::new()
        .route("/", get(handler))
        .route("/health", get(handler))
        .layer(axum::middleware::from_fn(request_id_middleware))
}

#[tokio::test]
async fn test_middleware_applied_to_all_routes() {
    let app = create_app();

    let routes = vec!["/", "/health"];

    for route in routes {
        let request = HttpRequest::builder()
            .uri(route)
            .body(Body::empty())
            .unwrap();

        let response = app.clone().oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        assert!(
            response.headers().get(REQUEST_ID_HEADER).is_some(),
            "Response should include x-request-id header for route {}",
            route
        );
    }
}

#[tokio::test]
async fn test_request_id_in_response_headers() {
    let app = create_app();

    let request = HttpRequest::builder()
        .uri("/")
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let request_id = response
        .headers()
        .get(REQUEST_ID_HEADER)
        .unwrap()
        .to_str()
        .unwrap();

    // Verify it's a valid UUID
    assert!(
        Uuid::parse_str(request_id).is_ok(),
        "Request ID should be a valid UUID"
    );
}

#[tokio::test]
async fn test_preserves_upstream_request_id() {
    let app = create_app();

    let existing_id = "550e8400-e29b-41d4-a716-446655440000";
    let request = HttpRequest::builder()
        .uri("/")
        .header(REQUEST_ID_HEADER, existing_id)
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let response_id = response
        .headers()
        .get(REQUEST_ID_HEADER)
        .unwrap()
        .to_str()
        .unwrap();

    assert_eq!(response_id, existing_id);
}

#[tokio::test]
async fn test_middleware_does_not_interfere_with_other_middleware() {
    use tower_http::trace::TraceLayer;
    use tracing::info_span;

    let app = Router::new()
        .route("/", get(handler))
        .layer(axum::middleware::from_fn(request_id_middleware))
        .layer(
            TraceLayer::new_for_http().make_span_with(|request: &Request| {
                info_span!(
                    "http_request",
                    method = ?request.method(),
                    uri = %request.uri(),
                )
            }),
        );

    let request = HttpRequest::builder()
        .uri("/")
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    assert!(response.headers().get(REQUEST_ID_HEADER).is_some());
}

#[tokio::test]
async fn test_concurrent_requests_get_unique_ids() {
    let app = create_app();

    let mut handles = Vec::new();

    for _ in 0..20 {
        let app_clone = app.clone();
        let handle = tokio::spawn(async move {
            let request = HttpRequest::builder()
                .uri("/")
                .body(Body::empty())
                .unwrap();

            let response = app_clone.oneshot(request).await.unwrap();

            response
                .headers()
                .get(REQUEST_ID_HEADER)
                .unwrap()
                .to_str()
                .unwrap()
                .to_string()
        });

        handles.push(handle);
    }

    let ids: Vec<String> = futures::future::join_all(handles)
        .await
        .into_iter()
        .map(|r| r.unwrap())
        .collect();

    // All IDs should be unique
    let unique_ids: std::collections::HashSet<&String> = ids.iter().collect();
    assert_eq!(
        unique_ids.len(),
        ids.len(),
        "All request IDs should be unique in concurrent requests"
    );
}

#[tokio::test]
async fn test_invalid_header_handled_gracefully() {
    let app = create_app();

    let mut request = HttpRequest::builder()
        .uri("/")
        .body(Body::empty())
        .unwrap();

    // Insert invalid header directly
    request.headers_mut().insert(
        REQUEST_ID_HEADER,
        HeaderValue::from_bytes(&[0xFF, 0xFE]).unwrap(),
    );

    let response = app.oneshot(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    assert!(response.headers().get(REQUEST_ID_HEADER).is_some());

    let request_id = response
        .headers()
        .get(REQUEST_ID_HEADER)
        .unwrap()
        .to_str()
        .unwrap();

    // Should generate new UUID when header is invalid
    assert!(
        Uuid::parse_str(request_id).is_ok(),
        "Invalid header should result in new valid UUID"
    );
}
