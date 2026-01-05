//! Postman API client.
//!
//! HTTP client for interacting with the Postman API.

use crate::error::PostmanError;
use crate::types::{
    Collection, CollectionResponse, CollectionSummary, CollectionsResponse, SearchResult,
    Workspace, WorkspacesResponse,
};
use reqwest::Client;
use std::time::Duration;
use tokio::time::sleep;
use tracing::{debug, warn};

/// Postman API base URL.
const BASE_URL: &str = "https://api.getpostman.com";

/// Default request timeout in seconds.
const DEFAULT_TIMEOUT_SECS: u64 = 10;

/// Maximum retry attempts.
const MAX_RETRIES: u32 = 3;

/// Base delay for exponential backoff (1 second).
const BASE_DELAY_SECS: u64 = 1;

/// Postman API client.
///
/// Provides methods for interacting with the Postman API including
/// listing workspaces, collections, and searching.
#[derive(Clone)]
pub struct PostmanClient {
    http_client: Client,
    api_key: String,
    base_url: String,
}

impl PostmanClient {
    /// Create a new Postman client.
    ///
    /// # Arguments
    /// * `api_key` - Postman API key for authentication
    ///
    /// # Panics
    /// Panics if the HTTP client cannot be created.
    #[must_use]
    pub fn new(api_key: String) -> Self {
        let http_client = Client::builder()
            .timeout(Duration::from_secs(DEFAULT_TIMEOUT_SECS))
            .build()
            .expect("Failed to create HTTP client");

        Self {
            http_client,
            api_key,
            base_url: BASE_URL.to_string(),
        }
    }

    /// Create a new Postman client with custom base URL.
    ///
    /// Useful for testing with mock servers.
    #[cfg(test)]
    pub fn with_base_url(api_key: String, base_url: String) -> Self {
        let http_client = Client::builder()
            .timeout(Duration::from_secs(DEFAULT_TIMEOUT_SECS))
            .build()
            .expect("Failed to create HTTP client");

        Self {
            http_client,
            api_key,
            base_url,
        }
    }

    /// Make an authenticated GET request with retry logic.
    async fn request<T: serde::de::DeserializeOwned>(
        &self,
        endpoint: &str,
    ) -> Result<T, PostmanError> {
        let url = format!("{}{}", self.base_url, endpoint);

        self.with_retry(|| async {
            debug!(endpoint = %endpoint, "Making Postman API request");

            let response = self
                .http_client
                .get(&url)
                .header("X-Api-Key", &self.api_key)
                .send()
                .await?;

            let status = response.status();

            if status.is_success() {
                let body = response.text().await?;
                serde_json::from_str(&body)
                    .map_err(|e| PostmanError::Parse(format!("{}: {}", e, &body[..200.min(body.len())])))
            } else if status == reqwest::StatusCode::UNAUTHORIZED {
                Err(PostmanError::Unauthorized)
            } else if status == reqwest::StatusCode::NOT_FOUND {
                Err(PostmanError::NotFound(endpoint.to_string()))
            } else if status == reqwest::StatusCode::TOO_MANY_REQUESTS {
                Err(PostmanError::RateLimited)
            } else {
                let body = response.text().await.unwrap_or_default();
                Err(PostmanError::ApiError { status, body })
            }
        })
        .await
    }

    /// Execute a function with exponential backoff retry.
    async fn with_retry<T, F, Fut>(&self, f: F) -> Result<T, PostmanError>
    where
        F: Fn() -> Fut,
        Fut: std::future::Future<Output = Result<T, PostmanError>>,
    {
        let mut attempt = 0;

        loop {
            attempt += 1;
            match f().await {
                Ok(result) => return Ok(result),
                Err(e) if e.is_retryable() && attempt < MAX_RETRIES => {
                    let delay = Duration::from_secs(BASE_DELAY_SECS * 2u64.pow(attempt - 1));
                    warn!(
                        attempt = attempt,
                        max_attempts = MAX_RETRIES,
                        delay_ms = delay.as_millis(),
                        error = %e,
                        "Postman API error, retrying"
                    );
                    sleep(delay).await;
                }
                Err(e) => return Err(e),
            }
        }
    }

    // ========================================================================
    // Workspace Operations
    // ========================================================================

    /// List all workspaces.
    ///
    /// Returns all workspaces accessible with the configured API key.
    ///
    /// # Errors
    /// Returns error if the API call fails or response cannot be parsed.
    pub async fn list_workspaces(&self) -> Result<Vec<Workspace>, PostmanError> {
        debug!("Listing Postman workspaces");
        let response: WorkspacesResponse = self.request("/workspaces").await?;
        debug!(count = response.workspaces.len(), "Retrieved workspaces");
        Ok(response.workspaces)
    }

    // ========================================================================
    // Collection Operations
    // ========================================================================

    /// List all collections.
    ///
    /// Optionally filter by workspace ID.
    ///
    /// # Arguments
    /// * `workspace_id` - Optional workspace ID to filter collections
    ///
    /// # Errors
    /// Returns error if the API call fails or response cannot be parsed.
    pub async fn list_collections(
        &self,
        workspace_id: Option<&str>,
    ) -> Result<Vec<CollectionSummary>, PostmanError> {
        let endpoint = match workspace_id {
            Some(id) => format!("/collections?workspace={}", id),
            None => "/collections".to_string(),
        };

        debug!(
            workspace_id = workspace_id,
            "Listing Postman collections"
        );

        let response: CollectionsResponse = self.request(&endpoint).await?;
        debug!(count = response.collections.len(), "Retrieved collections");
        Ok(response.collections)
    }

    /// Get full collection details.
    ///
    /// Returns the complete collection including all requests and folders.
    ///
    /// # Arguments
    /// * `collection_id` - Collection ID or UID
    ///
    /// # Errors
    /// Returns error if the collection is not found or API call fails.
    pub async fn get_collection(&self, collection_id: &str) -> Result<Collection, PostmanError> {
        let endpoint = format!("/collections/{}", collection_id);
        debug!(collection_id = %collection_id, "Getting collection details");
        let response: CollectionResponse = self.request(&endpoint).await?;
        Ok(response.collection)
    }

    // ========================================================================
    // Search Operations
    // ========================================================================

    /// Search collections by keywords.
    ///
    /// Searches collection names and request names within collections.
    /// Returns results ranked by match score.
    ///
    /// # Arguments
    /// * `keywords` - Keywords to search for
    /// * `workspace_id` - Optional workspace ID to limit search scope
    ///
    /// # Errors
    /// Returns error if API calls fail.
    pub async fn search_collections(
        &self,
        keywords: &[String],
        workspace_id: Option<&str>,
    ) -> Result<Vec<SearchResult>, PostmanError> {
        debug!(
            keywords = ?keywords,
            workspace_id = workspace_id,
            "Searching Postman collections"
        );

        // Get all collections in scope
        let collections = self.list_collections(workspace_id).await?;

        let mut results = Vec::new();

        for collection in collections {
            let name_score = calculate_match_score(&collection.name, keywords);

            // Only process collections with name match or fetch all for deeper search
            if name_score > 0.0 || keywords.is_empty() {
                // Get full collection for request details
                match self.get_collection(&collection.id).await {
                    Ok(full_collection) => {
                        // Search within requests
                        let request_matches = search_requests(&full_collection, keywords);
                        let total_score = name_score + (request_matches.len() as f32 * 0.1);

                        if total_score > 0.0 || keywords.is_empty() {
                            results.push(SearchResult {
                                source: "postman".to_string(),
                                id: collection.id.clone(),
                                name: collection.name.clone(),
                                description: full_collection.info.description,
                                url: format!("https://go.postman.co/collection/{}", collection.uid),
                                score: total_score,
                                matches: request_matches,
                            });
                        }
                    }
                    Err(e) => {
                        warn!(
                            collection_id = %collection.id,
                            error = %e,
                            "Failed to fetch collection details for search"
                        );
                    }
                }
            }
        }

        // Sort by score descending
        results.sort_by(|a, b| {
            b.score
                .partial_cmp(&a.score)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        debug!(results_count = results.len(), "Search completed");
        Ok(results)
    }
}

/// Calculate match score for text against keywords.
fn calculate_match_score(text: &str, keywords: &[String]) -> f32 {
    if keywords.is_empty() {
        return 0.0;
    }

    let text_lower = text.to_lowercase();
    let mut score = 0.0;

    for keyword in keywords {
        let keyword_lower = keyword.to_lowercase();
        if text_lower.contains(&keyword_lower) {
            score += 1.0;
            // Bonus for exact word match
            if text_lower
                .split_whitespace()
                .any(|w| w == keyword_lower)
            {
                score += 0.5;
            }
        }
    }

    score
}

/// Search for matching requests within a collection.
fn search_requests(collection: &Collection, keywords: &[String]) -> Vec<String> {
    let mut matches = Vec::new();

    fn search_items(items: &[crate::types::CollectionItem], keywords: &[String], matches: &mut Vec<String>) {
        for item in items {
            if let Some(name) = &item.name {
                if calculate_match_score(name, keywords) > 0.0 {
                    matches.push(name.clone());
                }
            }
            // Recurse into nested folders
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{CollectionInfo, CollectionItem};

    #[test]
    fn test_calculate_match_score_no_match() {
        let score = calculate_match_score("User API", &["payment".to_string()]);
        assert_eq!(score, 0.0);
    }

    #[test]
    fn test_calculate_match_score_partial_match() {
        let score = calculate_match_score("User Authentication API", &["user".to_string()]);
        assert!(score > 0.0);
    }

    #[test]
    fn test_calculate_match_score_exact_word_bonus() {
        let partial = calculate_match_score("UserAuth", &["user".to_string()]);
        let exact = calculate_match_score("User Auth", &["user".to_string()]);
        assert!(exact > partial);
    }

    #[test]
    fn test_calculate_match_score_multiple_keywords() {
        let score = calculate_match_score("User Authentication API", &["user".to_string(), "auth".to_string()]);
        assert!(score >= 2.0);
    }

    #[test]
    fn test_calculate_match_score_empty_keywords() {
        let score = calculate_match_score("Anything", &[]);
        assert_eq!(score, 0.0);
    }

    #[test]
    fn test_search_requests_finds_matches() {
        let collection = Collection {
            info: CollectionInfo {
                postman_id: None,
                name: "Test Collection".to_string(),
                description: None,
                schema: None,
            },
            item: Some(vec![
                CollectionItem {
                    id: Some("1".to_string()),
                    name: Some("Get Users".to_string()),
                    description: None,
                    request: None,
                    item: None,
                },
                CollectionItem {
                    id: Some("2".to_string()),
                    name: Some("Create Payment".to_string()),
                    description: None,
                    request: None,
                    item: None,
                },
            ]),
        };

        let matches = search_requests(&collection, &["user".to_string()]);
        assert_eq!(matches.len(), 1);
        assert_eq!(matches[0], "Get Users");
    }

    #[test]
    fn test_search_requests_nested_folders() {
        let collection = Collection {
            info: CollectionInfo {
                postman_id: None,
                name: "Test Collection".to_string(),
                description: None,
                schema: None,
            },
            item: Some(vec![CollectionItem {
                id: Some("folder".to_string()),
                name: Some("Users Folder".to_string()),
                description: None,
                request: None,
                item: Some(vec![CollectionItem {
                    id: Some("1".to_string()),
                    name: Some("Get User Details".to_string()),
                    description: None,
                    request: None,
                    item: None,
                }]),
            }]),
        };

        let matches = search_requests(&collection, &["user".to_string()]);
        assert_eq!(matches.len(), 2); // Folder name + request name
    }
}
