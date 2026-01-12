//! In-memory cache layer using Moka.
//!
//! Provides typed caches for tickets, search results, dashboard metrics, and workflow templates.

use moka::future::Cache;
use serde::{Deserialize, Serialize};
use std::time::Duration;
use uuid::Uuid;

/// Application cache wrapper for all in-memory caches.
#[derive(Clone)]
pub struct AppCache {
    /// Cache for Jira tickets.
    pub tickets: Cache<String, CachedTicket>,
    /// Cache for search results.
    pub search: Cache<String, Vec<CachedSearchResult>>,
    /// Cache for dashboard metrics.
    pub metrics: Cache<String, CachedDashboardMetrics>,
    /// Cache for workflow templates.
    pub templates: Cache<Uuid, CachedWorkflowTemplate>,
}

/// Cached Jira ticket data.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CachedTicket {
    /// Ticket key (e.g., "PROJ-123").
    pub key: String,
    /// Ticket title/summary.
    pub title: String,
    /// Current status name.
    pub status: String,
    /// Status color category.
    pub status_color: String,
    /// Priority name (if set).
    pub priority: Option<String>,
    /// Priority color for UI.
    pub priority_color: String,
    /// Assignee display name (if set).
    pub assignee_name: Option<String>,
    /// Last updated timestamp.
    pub updated_at: String,
}

/// Cached search result data.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CachedSearchResult {
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
}

/// Cached dashboard metrics data.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CachedDashboardMetrics {
    /// Number of tickets completed.
    pub tickets_completed: u64,
    /// Average time to completion.
    pub average_time: f64,
    /// Number of patterns detected.
    pub patterns_detected: u64,
    /// When metrics were generated.
    pub generated_at: String,
}

/// Cached workflow template data.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CachedWorkflowTemplate {
    /// Template UUID.
    pub id: Uuid,
    /// Template name.
    pub name: String,
    /// Template steps.
    pub steps: Vec<String>,
    /// Workflow type.
    pub workflow_type: String,
}

impl AppCache {
    /// Create a new AppCache instance with configured caches.
    #[must_use]
    pub fn new() -> Self {
        Self {
            tickets: Cache::builder()
                .max_capacity(1_000)
                .time_to_live(Duration::from_secs(300)) // 5 minutes
                .time_to_idle(Duration::from_secs(60))  // 1 minute idle
                .build(),
            
            search: Cache::builder()
                .max_capacity(500)
                .time_to_live(Duration::from_secs(120)) // 2 minutes
                .build(),
            
            metrics: Cache::builder()
                .max_capacity(50)
                .time_to_live(Duration::from_secs(30)) // 30 seconds
                .build(),
            
            templates: Cache::builder()
                .max_capacity(100)
                .time_to_live(Duration::from_secs(600)) // 10 minutes
                .build(),
        }
    }
    
    /// Invalidate ticket cache for a specific key.
    pub async fn invalidate_ticket(&self, key: &str) {
        self.tickets.invalidate(key).await;
    }
    
    /// Invalidate search results cache.
    pub async fn invalidate_search(&self, key: &str) {
        self.search.invalidate(key).await;
    }
    
    /// Invalidate dashboard metrics cache.
    pub async fn invalidate_metrics(&self, key: &str) {
        self.metrics.invalidate(key).await;
    }
    
    /// Invalidate workflow template cache.
    pub async fn invalidate_template(&self, id: &Uuid) {
        self.templates.invalidate(id).await;
    }
    
    /// Clear all caches (useful for testing or manual invalidation).
    pub fn clear_all(&self) {
        self.tickets.invalidate_all();
        self.search.invalidate_all();
        self.metrics.invalidate_all();
        self.templates.invalidate_all();
    }
    
    /// Get cache statistics for monitoring.
    ///
    /// Note: Hit/miss counts are tracked separately in route handlers via Prometheus metrics.
    #[must_use]
    pub fn get_stats(&self) -> CacheStats {
        CacheStats {
            tickets_size: self.tickets.entry_count(),
            search_size: self.search.entry_count(),
            metrics_size: self.metrics.entry_count(),
            templates_size: self.templates.entry_count(),
            // Hit/miss counts are tracked via Prometheus metrics in route handlers
            tickets_hit_count: 0,
            tickets_miss_count: 0,
            search_hit_count: 0,
            search_miss_count: 0,
            metrics_hit_count: 0,
            metrics_miss_count: 0,
            templates_hit_count: 0,
            templates_miss_count: 0,
        }
    }
}

impl Default for AppCache {
    fn default() -> Self {
        Self::new()
    }
}

/// Cache statistics for metrics.
#[derive(Debug, Clone)]
pub struct CacheStats {
    /// Number of entries in tickets cache.
    pub tickets_size: u64,
    /// Number of entries in search cache.
    pub search_size: u64,
    /// Number of entries in metrics cache.
    pub metrics_size: u64,
    /// Number of entries in templates cache.
    pub templates_size: u64,
    /// Number of hits for tickets cache.
    pub tickets_hit_count: u64,
    /// Number of misses for tickets cache.
    pub tickets_miss_count: u64,
    /// Number of hits for search cache.
    pub search_hit_count: u64,
    /// Number of misses for search cache.
    pub search_miss_count: u64,
    /// Number of hits for metrics cache.
    pub metrics_hit_count: u64,
    /// Number of misses for metrics cache.
    pub metrics_miss_count: u64,
    /// Number of hits for templates cache.
    pub templates_hit_count: u64,
    /// Number of misses for templates cache.
    pub templates_miss_count: u64,
}
