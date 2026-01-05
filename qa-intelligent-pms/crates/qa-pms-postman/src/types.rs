//! Postman API response types.
//!
//! Typed structs for Postman API responses.

use serde::{Deserialize, Serialize};

// ============================================================================
// Workspace Types
// ============================================================================

/// Response wrapper for workspaces list.
#[derive(Debug, Deserialize)]
pub struct WorkspacesResponse {
    /// List of workspaces.
    pub workspaces: Vec<Workspace>,
}

/// Postman workspace.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Workspace {
    /// Workspace unique ID.
    pub id: String,
    /// Workspace name.
    pub name: String,
    /// Workspace type (personal, team, public, private).
    #[serde(rename = "type")]
    pub workspace_type: String,
}

// ============================================================================
// Collection Types
// ============================================================================

/// Response wrapper for collections list.
#[derive(Debug, Deserialize)]
pub struct CollectionsResponse {
    /// List of collections.
    pub collections: Vec<CollectionSummary>,
}

/// Collection summary (from list endpoint).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollectionSummary {
    /// Collection unique ID.
    pub id: String,
    /// Collection UID (owner-id format).
    pub uid: String,
    /// Collection name.
    pub name: String,
    /// Owner username.
    pub owner: Option<String>,
    /// Creation timestamp.
    #[serde(rename = "createdAt")]
    pub created_at: Option<String>,
    /// Last update timestamp.
    #[serde(rename = "updatedAt")]
    pub updated_at: Option<String>,
}

/// Response wrapper for single collection.
#[derive(Debug, Deserialize)]
pub struct CollectionResponse {
    /// Full collection data.
    pub collection: Collection,
}

/// Full collection with items.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Collection {
    /// Collection metadata.
    pub info: CollectionInfo,
    /// Collection items (requests and folders).
    pub item: Option<Vec<CollectionItem>>,
}

/// Collection metadata.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollectionInfo {
    /// Postman internal ID.
    #[serde(rename = "_postman_id")]
    pub postman_id: Option<String>,
    /// Collection name.
    pub name: String,
    /// Collection description.
    pub description: Option<String>,
    /// Collection schema URL.
    pub schema: Option<String>,
}

/// Collection item (request or folder).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollectionItem {
    /// Item ID.
    pub id: Option<String>,
    /// Item name.
    pub name: Option<String>,
    /// Item description.
    pub description: Option<String>,
    /// Request details (if this is a request item).
    pub request: Option<RequestInfo>,
    /// Nested items (if this is a folder).
    pub item: Option<Vec<Self>>,
}

/// Request information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequestInfo {
    /// HTTP method (GET, POST, etc.).
    pub method: Option<String>,
    /// Request URL.
    pub url: Option<RequestUrl>,
    /// Request description.
    pub description: Option<String>,
}

/// Request URL (can be simple string or complex object).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RequestUrl {
    /// Simple URL string.
    Simple(String),
    /// Complex URL with parts.
    Complex {
        /// Raw URL string.
        raw: Option<String>,
        /// Host parts.
        host: Option<Vec<String>>,
        /// Path parts.
        path: Option<Vec<String>>,
    },
}

impl RequestUrl {
    /// Get the URL as a string.
    #[must_use]
    pub fn as_string(&self) -> String {
        match self {
            Self::Simple(s) => s.clone(),
            Self::Complex { raw, host, path } => {
                if let Some(raw) = raw {
                    raw.clone()
                } else {
                    let host_str = host
                        .as_ref()
                        .map(|h| h.join("."))
                        .unwrap_or_default();
                    let path_str = path
                        .as_ref()
                        .map(|p| p.join("/"))
                        .unwrap_or_default();
                    format!("{host_str}/{path_str}")
                }
            }
        }
    }
}

// ============================================================================
// Search Types
// ============================================================================

/// Search result for collections.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchResult {
    /// Source integration (always "postman").
    pub source: String,
    /// Collection ID.
    pub id: String,
    /// Collection name.
    pub name: String,
    /// Collection description.
    pub description: Option<String>,
    /// URL to view collection in Postman.
    pub url: String,
    /// Match score (higher is better).
    pub score: f32,
    /// Matching request names.
    pub matches: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_request_url_simple() {
        let url = RequestUrl::Simple("https://api.example.com/users".to_string());
        assert_eq!(url.as_string(), "https://api.example.com/users");
    }

    #[test]
    fn test_request_url_complex_with_raw() {
        let url = RequestUrl::Complex {
            raw: Some("https://api.example.com/users".to_string()),
            host: None,
            path: None,
        };
        assert_eq!(url.as_string(), "https://api.example.com/users");
    }

    #[test]
    fn test_request_url_complex_without_raw() {
        let url = RequestUrl::Complex {
            raw: None,
            host: Some(vec!["api".to_string(), "example".to_string(), "com".to_string()]),
            path: Some(vec!["users".to_string(), "123".to_string()]),
        };
        assert_eq!(url.as_string(), "api.example.com/users/123");
    }

    #[test]
    fn test_deserialize_workspace() {
        let json = r#"{"id": "ws-123", "name": "My Workspace", "type": "personal"}"#;
        let workspace: Workspace = serde_json::from_str(json).unwrap();
        assert_eq!(workspace.id, "ws-123");
        assert_eq!(workspace.name, "My Workspace");
        assert_eq!(workspace.workspace_type, "personal");
    }

    #[test]
    fn test_deserialize_collection_summary() {
        let json = r#"{
            "id": "col-123",
            "uid": "owner-col-123",
            "name": "Test Collection",
            "owner": "testuser",
            "createdAt": "2024-01-01T00:00:00.000Z",
            "updatedAt": "2024-01-02T00:00:00.000Z"
        }"#;
        let collection: CollectionSummary = serde_json::from_str(json).unwrap();
        assert_eq!(collection.id, "col-123");
        assert_eq!(collection.name, "Test Collection");
    }
}
