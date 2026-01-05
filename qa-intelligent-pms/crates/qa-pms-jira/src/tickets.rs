//! Jira ticket listing and management.
//!
//! Provides functionality to:
//! - List tickets with JQL filters
//! - Retrieve ticket details with comments and attachments
//! - Update ticket status
//!
//! Supports both API Token (Basic Auth) and OAuth authentication.

use anyhow::Result;
use base64::{engine::general_purpose::STANDARD as BASE64, Engine};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::time::Duration;
use tracing::{debug, info, instrument, warn};

/// Jira authentication credentials.
#[derive(Clone)]
pub enum JiraAuth {
    /// API Token authentication (Basic Auth with email:token)
    /// This is the recommended method for most use cases.
    ApiToken {
        /// Jira instance URL (e.g., "<https://company.atlassian.net>")
        instance_url: String,
        /// User email address
        email: String,
        /// API token from <https://id.atlassian.com/manage-profile/security/api-tokens>
        api_token: String,
    },
    /// OAuth 2.0 authentication (Bearer token)
    /// Used for advanced apps that went through OAuth flow.
    OAuth {
        /// Jira Cloud ID (obtained from OAuth flow)
        cloud_id: String,
        /// OAuth access token
        access_token: String,
    },
}

/// Jira ticket from search results.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JiraTicket {
    /// Ticket key (e.g., "PROJ-123")
    pub key: String,
    /// Internal ticket ID
    pub id: String,
    /// Ticket fields
    pub fields: TicketFields,
}

/// Ticket fields from Jira API.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TicketFields {
    /// Ticket summary/title
    pub summary: String,
    /// Ticket description (optional)
    pub description: Option<serde_json::Value>,
    /// Current status
    pub status: StatusField,
    /// Priority (optional)
    pub priority: Option<PriorityField>,
    /// Assignee (optional)
    pub assignee: Option<UserField>,
    /// Reporter (optional)
    pub reporter: Option<UserField>,
    /// Creation timestamp
    pub created: String,
    /// Last update timestamp
    pub updated: String,
}

/// Status field from Jira.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StatusField {
    /// Status name (e.g., "In Progress")
    pub name: String,
    /// Status category
    pub status_category: StatusCategory,
}

/// Status category for color coding.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StatusCategory {
    /// Category key: "new", "indeterminate", "done"
    pub key: String,
    /// Color name for UI
    pub color_name: String,
}

/// Priority field from Jira.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PriorityField {
    /// Priority name (e.g., "High", "Medium")
    pub name: String,
    /// Priority ID
    pub id: String,
}

/// User field from Jira.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserField {
    /// Display name
    pub display_name: String,
    /// Email address (optional)
    pub email_address: Option<String>,
    /// Account ID
    pub account_id: Option<String>,
    /// Avatar URLs
    pub avatar_urls: Option<AvatarUrls>,
}

/// Avatar URL sizes.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AvatarUrls {
    /// 24x24 avatar
    #[serde(rename = "24x24")]
    pub small: Option<String>,
    /// 48x48 avatar
    #[serde(rename = "48x48")]
    pub medium: Option<String>,
}

// ============================================================================
// Ticket Detail Types (Story 3.3)
// ============================================================================

/// Full ticket details including comments and attachments.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TicketDetail {
    /// Ticket key (e.g., "PROJ-123")
    pub key: String,
    /// Internal ticket ID
    pub id: String,
    /// Ticket fields
    pub fields: TicketDetailFields,
}

/// Extended ticket fields with comments and attachments.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TicketDetailFields {
    /// Ticket summary/title
    pub summary: String,
    /// Ticket description (Atlassian Document Format)
    pub description: Option<serde_json::Value>,
    /// Current status
    pub status: StatusField,
    /// Priority (optional)
    pub priority: Option<PriorityField>,
    /// Assignee (optional)
    pub assignee: Option<UserField>,
    /// Reporter (optional)
    pub reporter: Option<UserField>,
    /// Creation timestamp
    pub created: String,
    /// Last update timestamp
    pub updated: String,
    /// Comments container
    pub comment: Option<CommentContainer>,
    /// Attachments list
    pub attachment: Option<Vec<Attachment>>,
    /// Labels
    #[serde(default)]
    pub labels: Vec<String>,
}

/// Container for comments from Jira API.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommentContainer {
    /// List of comments
    pub comments: Vec<Comment>,
    /// Total number of comments
    pub total: u32,
}

/// A single comment on a ticket.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Comment {
    /// Comment ID
    pub id: String,
    /// Comment author
    pub author: UserField,
    /// Comment body (Atlassian Document Format)
    pub body: Option<serde_json::Value>,
    /// Creation timestamp
    pub created: String,
    /// Last update timestamp
    pub updated: String,
}

/// An attachment on a ticket.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Attachment {
    /// Attachment ID
    pub id: String,
    /// File name
    pub filename: String,
    /// MIME type
    pub mime_type: String,
    /// File size in bytes
    pub size: u64,
    /// Download URL
    pub content: String,
    /// Creation timestamp
    pub created: String,
}

/// Search response from Jira API.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchResponse {
    /// List of tickets
    pub issues: Vec<JiraTicket>,
    /// Total number of matching tickets
    pub total: u32,
    /// Starting index
    pub start_at: u32,
    /// Maximum results per page
    pub max_results: u32,
}

/// Filters for ticket search.
#[derive(Debug, Clone, Default)]
pub struct TicketFilters {
    /// Filter by statuses
    pub statuses: Vec<String>,
    /// Filter by assignee (email or account ID)
    pub assignee: Option<String>,
    /// Filter by project key
    pub project: Option<String>,
}

// ============================================================================
// Transition Types (Story 3.4)
// ============================================================================

/// Response from Jira transitions endpoint.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransitionsResponse {
    /// List of available transitions
    pub transitions: Vec<Transition>,
}

/// A workflow transition for a ticket.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Transition {
    /// Transition ID
    pub id: String,
    /// Transition name (e.g., "Start Progress", "Close Issue")
    pub name: String,
    /// Target status after transition
    pub to: TransitionTarget,
    /// Whether the transition has a screen (requires additional fields)
    #[serde(default)]
    pub has_screen: bool,
    /// Whether the transition is available
    #[serde(default = "default_true")]
    pub is_available: bool,
}

const fn default_true() -> bool {
    true
}

/// Target status for a transition.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransitionTarget {
    /// Status ID
    pub id: String,
    /// Status name
    pub name: String,
    /// Status category for color coding
    pub status_category: StatusCategory,
}

/// Request body for transitioning a ticket.
#[derive(Debug, Clone, Serialize)]
pub struct TransitionRequest {
    /// The transition to perform
    pub transition: TransitionId,
}

/// Transition ID wrapper for request body.
#[derive(Debug, Clone, Serialize)]
pub struct TransitionId {
    /// The ID of the transition
    pub id: String,
}

/// Jira API client for ticket operations.
pub struct JiraTicketsClient {
    http_client: Client,
    auth: JiraAuth,
}

impl JiraTicketsClient {
    /// Jira API base URL for OAuth (via Atlassian gateway).
    const OAUTH_API_BASE: &'static str = "https://api.atlassian.com/ex/jira";

    /// Fields to fetch in search queries.
    const SEARCH_FIELDS: &'static str =
        "summary,status,priority,assignee,reporter,created,updated,description";

    /// Create a new tickets client with API Token authentication.
    ///
    /// This is the recommended method for most use cases.
    ///
    /// # Arguments
    /// * `instance_url` - Jira Cloud URL (e.g., "<https://company.atlassian.net>")
    /// * `email` - User email address
    /// * `api_token` - API token from Atlassian account settings
    #[must_use]
    pub fn with_api_token(instance_url: String, email: String, api_token: String) -> Self {
        let http_client = Client::builder()
            .timeout(Duration::from_secs(30))
            .build()
            .expect("Failed to build HTTP client");

        Self {
            http_client,
            auth: JiraAuth::ApiToken {
                instance_url: instance_url.trim_end_matches('/').to_string(),
                email,
                api_token,
            },
        }
    }

    /// Create a new tickets client with OAuth authentication.
    ///
    /// # Arguments
    /// * `cloud_id` - Jira Cloud ID for the site (from OAuth flow)
    /// * `access_token` - Valid OAuth access token
    #[must_use]
    pub fn with_oauth(cloud_id: String, access_token: String) -> Self {
        let http_client = Client::builder()
            .timeout(Duration::from_secs(30))
            .build()
            .expect("Failed to build HTTP client");

        Self {
            http_client,
            auth: JiraAuth::OAuth {
                cloud_id,
                access_token,
            },
        }
    }

    /// Legacy constructor for OAuth (kept for compatibility).
    #[must_use]
    #[deprecated(since = "0.2.0", note = "Use with_api_token or with_oauth instead")]
    pub fn new(cloud_id: String, access_token: String) -> Self {
        Self::with_oauth(cloud_id, access_token)
    }

    /// Get the base URL for API requests.
    fn base_url(&self) -> String {
        match &self.auth {
            JiraAuth::ApiToken { instance_url, .. } => instance_url.clone(),
            JiraAuth::OAuth { cloud_id, .. } => {
                format!("{}/{}", Self::OAUTH_API_BASE, cloud_id)
            }
        }
    }

    /// Build the authorization header value.
    fn auth_header(&self) -> String {
        match &self.auth {
            JiraAuth::ApiToken { email, api_token, .. } => {
                let credentials = format!("{email}:{api_token}");
                format!("Basic {}", BASE64.encode(credentials.as_bytes()))
            }
            JiraAuth::OAuth { access_token, .. } => {
                format!("Bearer {access_token}")
            }
        }
    }

    /// Get a display name for logging (hides sensitive data).
    fn display_name(&self) -> String {
        match &self.auth {
            JiraAuth::ApiToken { instance_url, email, .. } => {
                format!("{instance_url} ({email})")
            }
            JiraAuth::OAuth { cloud_id, .. } => {
                format!("cloud:{cloud_id}")
            }
        }
    }

    /// List tickets with filters using JQL.
    ///
    /// # Arguments
    /// * `filters` - Filters to apply
    /// * `start_at` - Starting index for pagination
    /// * `max_results` - Maximum results per page (max 100)
    ///
    /// # Errors
    /// Returns error if API call fails or response cannot be parsed.
    #[instrument(skip(self), fields(jira = %self.display_name()))]
    pub async fn list_tickets(
        &self,
        filters: &TicketFilters,
        start_at: u32,
        max_results: u32,
    ) -> Result<SearchResponse> {
        let jql = Self::build_jql(filters);
        let max_results = max_results.min(100);

        // Note: Atlassian deprecated /search in favor of /search/jql
        // See: https://developer.atlassian.com/changelog/#CHANGE-2046
        let url = format!("{}/rest/api/3/search/jql", self.base_url());

        debug!(jql = %jql, start_at, max_results, "Searching Jira tickets");

        let response = self
            .http_client
            .get(&url)
            .header("Authorization", self.auth_header())
            .query(&[
                ("jql", jql.as_str()),
                ("startAt", &start_at.to_string()),
                ("maxResults", &max_results.to_string()),
                ("fields", Self::SEARCH_FIELDS),
            ])
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            warn!(status = %status, body = %body, "Jira search failed");
            anyhow::bail!("Jira API error: {status} - {body}");
        }

        let search_response: SearchResponse = response.json().await?;

        debug!(
            total = search_response.total,
            returned = search_response.issues.len(),
            "Jira search completed"
        );

        Ok(search_response)
    }

    /// Build JQL query from filters.
    fn build_jql(filters: &TicketFilters) -> String {
        let mut clauses = Vec::new();

        if let Some(project) = &filters.project {
            clauses.push(format!("project = \"{project}\""));
        }

        if !filters.statuses.is_empty() {
            let statuses = filters
                .statuses
                .iter()
                .map(|s| format!("\"{s}\""))
                .collect::<Vec<_>>()
                .join(", ");
            clauses.push(format!("status IN ({statuses})"));
        }

        if let Some(assignee) = &filters.assignee {
            // Support both email and "currentUser()" special value
            if assignee == "currentUser()" {
                clauses.push("assignee = currentUser()".to_string());
            } else {
                clauses.push(format!("assignee = \"{assignee}\""));
            }
        }

        let base = if clauses.is_empty() {
            String::new()
        } else {
            clauses.join(" AND ")
        };

        if base.is_empty() {
            "ORDER BY updated DESC".to_string()
        } else {
            format!("{base} ORDER BY updated DESC")
        }
    }

    /// Update the access token (after refresh).
    /// Update OAuth access token (for token refresh).
    ///
    /// Only works for OAuth-based clients.
    pub fn update_token(&mut self, access_token: String) {
        if let JiraAuth::OAuth { access_token: ref mut token, .. } = self.auth {
            *token = access_token;
        }
    }

    /// Get full ticket details by key.
    ///
    /// # Arguments
    /// * `key` - Jira ticket key (e.g., "PROJ-123")
    ///
    /// # Returns
    /// Full ticket details including comments (latest 10) and attachments.
    ///
    /// # Errors
    /// Returns error if API call fails, ticket not found, or response cannot be parsed.
    #[instrument(skip(self), fields(jira = %self.display_name(), ticket_key = %key))]
    pub async fn get_ticket(&self, key: &str) -> Result<TicketDetail> {
        let url = format!("{}/rest/api/3/issue/{}", self.base_url(), key);

        // Fields to fetch for detail view
        let fields = "summary,description,status,priority,assignee,reporter,created,updated,comment,attachment,labels";

        debug!(key = %key, "Fetching ticket details from Jira");

        let response = self
            .http_client
            .get(&url)
            .header("Authorization", self.auth_header())
            .query(&[("fields", fields)])
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();

            if status.as_u16() == 404 {
                anyhow::bail!("Ticket not found: {key}");
            }

            warn!(status = %status, body = %body, "Jira get ticket failed");
            anyhow::bail!("Jira API error: {status} - {body}");
        }

        let ticket: TicketDetail = response.json().await?;

        info!(
            key = %ticket.key,
            has_comments = ticket.fields.comment.is_some(),
            has_attachments = ticket.fields.attachment.is_some(),
            "Ticket details fetched successfully"
        );

        Ok(ticket)
    }

    /// Get available transitions for a ticket.
    ///
    /// # Arguments
    /// * `key` - Jira ticket key (e.g., "PROJ-123")
    ///
    /// # Returns
    /// List of available transitions the user can perform on the ticket.
    ///
    /// # Errors
    /// Returns error if API call fails or response cannot be parsed.
    #[instrument(skip(self), fields(jira = %self.display_name(), ticket_key = %key))]
    pub async fn get_transitions(&self, key: &str) -> Result<Vec<Transition>> {
        let url = format!("{}/rest/api/3/issue/{}/transitions", self.base_url(), key);

        debug!(key = %key, "Fetching available transitions from Jira");

        let response = self
            .http_client
            .get(&url)
            .header("Authorization", self.auth_header())
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();

            if status.as_u16() == 404 {
                anyhow::bail!("Ticket not found: {key}");
            }

            warn!(status = %status, body = %body, "Jira get transitions failed");
            anyhow::bail!("Jira API error: {status} - {body}");
        }

        let transitions_response: TransitionsResponse = response.json().await?;

        // Filter to only available transitions
        let available: Vec<Transition> = transitions_response
            .transitions
            .into_iter()
            .filter(|t| t.is_available)
            .collect();

        info!(
            key = %key,
            count = available.len(),
            "Transitions fetched successfully"
        );

        Ok(available)
    }

    /// Transition a ticket to a new status.
    ///
    /// Implements retry with exponential backoff per NFR-REL-03:
    /// - Attempt 1: Immediate
    /// - Attempt 2: Wait 1 second
    /// - Attempt 3: Wait 2 seconds
    ///
    /// # Arguments
    /// * `key` - Jira ticket key (e.g., "PROJ-123")
    /// * `transition_id` - ID of the transition to perform
    ///
    /// # Errors
    /// Returns error if transition fails after all retry attempts.
    #[instrument(skip(self), fields(jira = %self.display_name(), ticket_key = %key, transition_id = %transition_id))]
    pub async fn transition_ticket(&self, key: &str, transition_id: &str) -> Result<()> {
        let url = format!("{}/rest/api/3/issue/{}/transitions", self.base_url(), key);

        let body = TransitionRequest {
            transition: TransitionId {
                id: transition_id.to_string(),
            },
        };

        // Retry configuration per NFR-REL-03
        const MAX_ATTEMPTS: u32 = 3;
        let base_delay = Duration::from_secs(1);

        let mut attempt = 0;

        loop {
            attempt += 1;

            debug!(
                key = %key,
                transition_id = %transition_id,
                attempt = attempt,
                "Attempting ticket transition"
            );

            let result = self
                .http_client
                .post(&url)
                .header("Authorization", self.auth_header())
                .json(&body)
                .send()
                .await;

            match result {
                Ok(response) if response.status().is_success() => {
                    info!(
                        key = %key,
                        transition_id = %transition_id,
                        attempt = attempt,
                        "Ticket transition successful"
                    );
                    return Ok(());
                }
                Ok(response) if response.status().is_server_error() && attempt < MAX_ATTEMPTS => {
                    let delay = base_delay * 2u32.pow(attempt - 1);
                    warn!(
                        key = %key,
                        status = %response.status(),
                        attempt = attempt,
                        delay_ms = delay.as_millis(),
                        "Jira transition failed with server error, retrying"
                    );
                    tokio::time::sleep(delay).await;
                }
                Ok(response) => {
                    let status = response.status();
                    let error_text = response.text().await.unwrap_or_default();

                    if status.as_u16() == 400 {
                        anyhow::bail!("Invalid transition: {error_text}");
                    } else if status.as_u16() == 404 {
                        anyhow::bail!("Ticket not found: {key}");
                    }

                    warn!(
                        key = %key,
                        status = %status,
                        error = %error_text,
                        "Transition failed"
                    );
                    anyhow::bail!("Transition failed: {status} - {error_text}");
                }
                Err(e) if attempt < MAX_ATTEMPTS => {
                    let delay = base_delay * 2u32.pow(attempt - 1);
                    warn!(
                        key = %key,
                        error = %e,
                        attempt = attempt,
                        delay_ms = delay.as_millis(),
                        "Network error during transition, retrying"
                    );
                    tokio::time::sleep(delay).await;
                }
                Err(e) => {
                    return Err(e.into());
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_jql_empty_filters() {
        let filters = TicketFilters::default();
        let jql = JiraTicketsClient::build_jql(&filters);
        assert_eq!(jql, "ORDER BY updated DESC");
    }

    #[test]
    fn test_build_jql_with_statuses() {
        let filters = TicketFilters {
            statuses: vec!["In Progress".to_string(), "Ready for QA".to_string()],
            ..Default::default()
        };
        let jql = JiraTicketsClient::build_jql(&filters);
        assert!(jql.contains("status IN"));
        assert!(jql.contains("\"In Progress\""));
        assert!(jql.contains("\"Ready for QA\""));
        assert!(jql.ends_with("ORDER BY updated DESC"));
    }

    #[test]
    fn test_build_jql_with_assignee() {
        let filters = TicketFilters {
            assignee: Some("user@example.com".to_string()),
            ..Default::default()
        };
        let jql = JiraTicketsClient::build_jql(&filters);
        assert!(jql.contains("assignee = \"user@example.com\""));
    }

    #[test]
    fn test_build_jql_with_current_user() {
        let filters = TicketFilters {
            assignee: Some("currentUser()".to_string()),
            ..Default::default()
        };
        let jql = JiraTicketsClient::build_jql(&filters);
        assert!(jql.contains("assignee = currentUser()"));
    }

    #[test]
    fn test_build_jql_with_project() {
        let filters = TicketFilters {
            project: Some("MYPROJ".to_string()),
            ..Default::default()
        };
        let jql = JiraTicketsClient::build_jql(&filters);
        assert!(jql.contains("project = \"MYPROJ\""));
    }

    #[test]
    fn test_build_jql_combined_filters() {
        let filters = TicketFilters {
            statuses: vec!["Open".to_string()],
            assignee: Some("user@example.com".to_string()),
            project: Some("TEST".to_string()),
        };
        let jql = JiraTicketsClient::build_jql(&filters);
        assert!(jql.contains("project = \"TEST\""));
        assert!(jql.contains("status IN (\"Open\")"));
        assert!(jql.contains("assignee = \"user@example.com\""));
        assert!(jql.contains(" AND "));
    }

    #[test]
    fn test_ticket_fields_deserialization() {
        let json = r#"{
            "key": "PROJ-123",
            "id": "10001",
            "fields": {
                "summary": "Test ticket",
                "description": null,
                "status": {
                    "name": "In Progress",
                    "statusCategory": {
                        "key": "indeterminate",
                        "colorName": "yellow"
                    }
                },
                "priority": {
                    "name": "High",
                    "id": "2"
                },
                "assignee": {
                    "displayName": "John Doe",
                    "emailAddress": "john@example.com",
                    "accountId": "abc123",
                    "avatarUrls": {
                        "24x24": "https://example.com/avatar24.png",
                        "48x48": "https://example.com/avatar48.png"
                    }
                },
                "reporter": null,
                "created": "2026-01-01T10:00:00.000Z",
                "updated": "2026-01-04T15:30:00.000Z"
            }
        }"#;

        let ticket: JiraTicket = serde_json::from_str(json).expect("Failed to parse ticket");
        assert_eq!(ticket.key, "PROJ-123");
        assert_eq!(ticket.fields.summary, "Test ticket");
        assert_eq!(ticket.fields.status.name, "In Progress");
        assert_eq!(ticket.fields.status.status_category.key, "indeterminate");
        assert_eq!(ticket.fields.priority.as_ref().unwrap().name, "High");
        assert_eq!(
            ticket.fields.assignee.as_ref().unwrap().display_name,
            "John Doe"
        );
    }

    #[test]
    fn test_search_response_deserialization() {
        let json = r#"{
            "issues": [],
            "total": 0,
            "startAt": 0,
            "maxResults": 20
        }"#;

        let response: SearchResponse =
            serde_json::from_str(json).expect("Failed to parse search response");
        assert_eq!(response.total, 0);
        assert_eq!(response.start_at, 0);
        assert_eq!(response.max_results, 20);
        assert!(response.issues.is_empty());
    }

    #[test]
    fn test_ticket_detail_deserialization() {
        let json = r#"{
            "key": "PROJ-456",
            "id": "10002",
            "fields": {
                "summary": "Detailed ticket",
                "description": {
                    "type": "doc",
                    "version": 1,
                    "content": [
                        {
                            "type": "paragraph",
                            "content": [
                                {"type": "text", "text": "Test description"}
                            ]
                        }
                    ]
                },
                "status": {
                    "name": "Ready for QA",
                    "statusCategory": {
                        "key": "indeterminate",
                        "colorName": "yellow"
                    }
                },
                "priority": {
                    "name": "High",
                    "id": "2"
                },
                "assignee": {
                    "displayName": "Jane Doe",
                    "emailAddress": "jane@example.com",
                    "accountId": "def456"
                },
                "reporter": {
                    "displayName": "John Smith",
                    "emailAddress": "john@example.com",
                    "accountId": "abc123"
                },
                "created": "2026-01-01T10:00:00.000Z",
                "updated": "2026-01-04T15:30:00.000Z",
                "labels": ["qa", "regression"]
            }
        }"#;

        let ticket: TicketDetail = serde_json::from_str(json).expect("Failed to parse ticket detail");
        assert_eq!(ticket.key, "PROJ-456");
        assert_eq!(ticket.fields.summary, "Detailed ticket");
        assert!(ticket.fields.description.is_some());
        assert_eq!(ticket.fields.labels.len(), 2);
        assert!(ticket.fields.labels.contains(&"qa".to_string()));
    }

    #[test]
    fn test_ticket_detail_with_comments() {
        let json = r#"{
            "key": "PROJ-789",
            "id": "10003",
            "fields": {
                "summary": "Ticket with comments",
                "description": null,
                "status": {
                    "name": "In Progress",
                    "statusCategory": {
                        "key": "indeterminate",
                        "colorName": "yellow"
                    }
                },
                "created": "2026-01-01T10:00:00.000Z",
                "updated": "2026-01-04T15:30:00.000Z",
                "comment": {
                    "comments": [
                        {
                            "id": "1001",
                            "author": {
                                "displayName": "Commenter",
                                "emailAddress": "commenter@example.com"
                            },
                            "body": {
                                "type": "doc",
                                "version": 1,
                                "content": [{"type": "paragraph", "content": [{"type": "text", "text": "Comment text"}]}]
                            },
                            "created": "2026-01-02T12:00:00.000Z",
                            "updated": "2026-01-02T12:00:00.000Z"
                        }
                    ],
                    "total": 1
                },
                "labels": []
            }
        }"#;

        let ticket: TicketDetail = serde_json::from_str(json).expect("Failed to parse ticket with comments");
        assert!(ticket.fields.comment.is_some());
        let comments = ticket.fields.comment.unwrap();
        assert_eq!(comments.total, 1);
        assert_eq!(comments.comments.len(), 1);
        assert_eq!(comments.comments[0].author.display_name, "Commenter");
    }

    #[test]
    fn test_ticket_detail_with_attachments() {
        let json = r#"{
            "key": "PROJ-101",
            "id": "10004",
            "fields": {
                "summary": "Ticket with attachments",
                "description": null,
                "status": {
                    "name": "Done",
                    "statusCategory": {
                        "key": "done",
                        "colorName": "green"
                    }
                },
                "created": "2026-01-01T10:00:00.000Z",
                "updated": "2026-01-04T15:30:00.000Z",
                "attachment": [
                    {
                        "id": "att-001",
                        "filename": "screenshot.png",
                        "mimeType": "image/png",
                        "size": 102400,
                        "content": "https://jira.example.com/attachments/screenshot.png",
                        "created": "2026-01-03T09:00:00.000Z"
                    }
                ],
                "labels": []
            }
        }"#;

        let ticket: TicketDetail = serde_json::from_str(json).expect("Failed to parse ticket with attachments");
        assert!(ticket.fields.attachment.is_some());
        let attachments = ticket.fields.attachment.unwrap();
        assert_eq!(attachments.len(), 1);
        assert_eq!(attachments[0].filename, "screenshot.png");
        assert_eq!(attachments[0].size, 102400);
    }

    #[test]
    fn test_transitions_response_deserialization() {
        let json = r#"{
            "transitions": [
                {
                    "id": "11",
                    "name": "Start Progress",
                    "to": {
                        "id": "3",
                        "name": "In Progress",
                        "statusCategory": {
                            "key": "indeterminate",
                            "colorName": "yellow"
                        }
                    },
                    "hasScreen": false,
                    "isAvailable": true
                },
                {
                    "id": "21",
                    "name": "Close Issue",
                    "to": {
                        "id": "5",
                        "name": "Done",
                        "statusCategory": {
                            "key": "done",
                            "colorName": "green"
                        }
                    },
                    "hasScreen": true,
                    "isAvailable": true
                }
            ]
        }"#;

        let response: TransitionsResponse =
            serde_json::from_str(json).expect("Failed to parse transitions response");
        assert_eq!(response.transitions.len(), 2);

        let first = &response.transitions[0];
        assert_eq!(first.id, "11");
        assert_eq!(first.name, "Start Progress");
        assert_eq!(first.to.name, "In Progress");
        assert_eq!(first.to.status_category.color_name, "yellow");
        assert!(!first.has_screen);
        assert!(first.is_available);

        let second = &response.transitions[1];
        assert_eq!(second.id, "21");
        assert_eq!(second.name, "Close Issue");
        assert!(second.has_screen);
    }

    #[test]
    fn test_transition_request_serialization() {
        let request = TransitionRequest {
            transition: TransitionId {
                id: "21".to_string(),
            },
        };

        let json = serde_json::to_string(&request).expect("Failed to serialize transition request");
        assert!(json.contains("\"transition\""));
        assert!(json.contains("\"id\":\"21\""));
    }

    #[test]
    fn test_transitions_with_missing_optional_fields() {
        // Test that missing optional fields use defaults
        let json = r#"{
            "transitions": [
                {
                    "id": "31",
                    "name": "Reopen",
                    "to": {
                        "id": "1",
                        "name": "Open",
                        "statusCategory": {
                            "key": "new",
                            "colorName": "blue"
                        }
                    }
                }
            ]
        }"#;

        let response: TransitionsResponse =
            serde_json::from_str(json).expect("Failed to parse transitions with missing fields");
        assert_eq!(response.transitions.len(), 1);

        let transition = &response.transitions[0];
        assert_eq!(transition.id, "31");
        assert!(!transition.has_screen); // default false
        assert!(transition.is_available); // default true
    }
}
