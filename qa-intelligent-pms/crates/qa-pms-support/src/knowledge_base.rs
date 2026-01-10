//! Knowledge base service for troubleshooting suggestions.

use sqlx::PgPool;
use uuid::Uuid;

use crate::error::SupportError;
use crate::repository::SupportRepository;
use crate::types::{ErrorLog, SuggestionSource, TroubleshootingSuggestion};

/// Service for knowledge base and troubleshooting suggestions.
pub struct KnowledgeBaseService {
    repo: SupportRepository,
}

impl KnowledgeBaseService {
    /// Create a new knowledge base service.
    #[must_use]
    pub const fn new(pool: PgPool) -> Self {
        let repo = SupportRepository::new(pool);
        Self { repo }
    }

    /// Get troubleshooting suggestions for an error.
    pub async fn get_suggestions(
        &self,
        error: &ErrorLog,
    ) -> Result<Vec<TroubleshootingSuggestion>, SupportError> {
        let mut suggestions = Vec::new();

        // 1. Find matching knowledge base entries
        let kb_entries = self.repo.find_matching_kb_entries(&error.message).await?;

        for (idx, entry) in kb_entries.iter().enumerate() {
            let relevance = 100 - (idx as i32 * 15); // Decrease relevance by position
            suggestions.push(TroubleshootingSuggestion {
                id: Uuid::new_v4(),
                source: SuggestionSource::KnowledgeBase,
                title: entry.title.clone(),
                description: entry.solution.clone(),
                relevance_score: relevance.max(10),
                kb_entry_id: Some(entry.id),
            });
        }

        // 2. Add diagnostic suggestions based on error source
        let diagnostic_suggestions = self.get_diagnostic_suggestions(error);
        suggestions.extend(diagnostic_suggestions);

        // 3. Sort by relevance
        suggestions.sort_by(|a, b| b.relevance_score.cmp(&a.relevance_score));

        // Limit to top 10 suggestions
        suggestions.truncate(10);

        Ok(suggestions)
    }

    /// Get diagnostic suggestions based on error characteristics.
    fn get_diagnostic_suggestions(&self, error: &ErrorLog) -> Vec<TroubleshootingSuggestion> {
        let mut suggestions = Vec::new();
        let message_lower = error.message.to_lowercase();

        // Connection-related errors
        if message_lower.contains("connection")
            || message_lower.contains("timeout")
            || message_lower.contains("network")
        {
            suggestions.push(TroubleshootingSuggestion {
                id: Uuid::new_v4(),
                source: SuggestionSource::DiagnosticStep,
                title: "Check Network Connectivity".to_string(),
                description: "1. Verify network connection is stable\n2. Check if the target service is accessible\n3. Run integration diagnostics to check all connections".to_string(),
                relevance_score: 85,
                kb_entry_id: None,
            });
        }

        // Authentication errors
        if message_lower.contains("unauthorized")
            || message_lower.contains("401")
            || message_lower.contains("authentication")
            || message_lower.contains("token")
        {
            suggestions.push(TroubleshootingSuggestion {
                id: Uuid::new_v4(),
                source: SuggestionSource::DiagnosticStep,
                title: "Verify Authentication".to_string(),
                description: "1. Check if OAuth tokens are expired\n2. Re-authenticate with the integration\n3. Verify API keys are still valid".to_string(),
                relevance_score: 90,
                kb_entry_id: None,
            });
        }

        // Permission errors
        if message_lower.contains("forbidden")
            || message_lower.contains("403")
            || message_lower.contains("permission")
            || message_lower.contains("access denied")
        {
            suggestions.push(TroubleshootingSuggestion {
                id: Uuid::new_v4(),
                source: SuggestionSource::DiagnosticStep,
                title: "Check Permissions".to_string(),
                description: "1. Verify the user has required permissions\n2. Check integration app scopes\n3. Contact administrator if permissions are correct".to_string(),
                relevance_score: 88,
                kb_entry_id: None,
            });
        }

        // Rate limiting
        if message_lower.contains("rate limit")
            || message_lower.contains("429")
            || message_lower.contains("too many requests")
        {
            suggestions.push(TroubleshootingSuggestion {
                id: Uuid::new_v4(),
                source: SuggestionSource::DiagnosticStep,
                title: "Rate Limit Exceeded".to_string(),
                description: "1. Wait a few minutes before retrying\n2. Reduce frequency of API calls\n3. Consider upgrading API plan if limits are consistently hit".to_string(),
                relevance_score: 95,
                kb_entry_id: None,
            });
        }

        // Database errors
        if message_lower.contains("database")
            || message_lower.contains("sql")
            || message_lower.contains("query")
            || message_lower.contains("postgres")
        {
            suggestions.push(TroubleshootingSuggestion {
                id: Uuid::new_v4(),
                source: SuggestionSource::DiagnosticStep,
                title: "Database Issue".to_string(),
                description: "1. Check database connection status\n2. Verify database is not overloaded\n3. Check for any pending migrations\n4. Review recent database changes".to_string(),
                relevance_score: 85,
                kb_entry_id: None,
            });
        }

        // Validation errors
        if message_lower.contains("validation")
            || message_lower.contains("invalid")
            || message_lower.contains("required")
        {
            suggestions.push(TroubleshootingSuggestion {
                id: Uuid::new_v4(),
                source: SuggestionSource::DiagnosticStep,
                title: "Input Validation Error".to_string(),
                description: "1. Check the input data format\n2. Verify all required fields are provided\n3. Review API documentation for correct format".to_string(),
                relevance_score: 75,
                kb_entry_id: None,
            });
        }

        // Not found errors
        if message_lower.contains("not found")
            || message_lower.contains("404")
            || message_lower.contains("does not exist")
        {
            suggestions.push(TroubleshootingSuggestion {
                id: Uuid::new_v4(),
                source: SuggestionSource::DiagnosticStep,
                title: "Resource Not Found".to_string(),
                description: "1. Verify the resource ID is correct\n2. Check if the resource was deleted\n3. Ensure you have access to the resource".to_string(),
                relevance_score: 70,
                kb_entry_id: None,
            });
        }

        suggestions
    }

    /// Get default knowledge base entries for seeding.
    #[must_use]
    pub fn get_default_entries() -> Vec<crate::types::CreateKbEntryInput> {
        vec![
            crate::types::CreateKbEntryInput {
                title: "Jira OAuth Token Expired".to_string(),
                problem: "Jira API calls fail with 401 Unauthorized after some time".to_string(),
                cause: "OAuth access tokens expire after a period of time (usually 1 hour). The refresh token may also have expired if the app hasn't been used in 90 days.".to_string(),
                solution: "1. Go to Settings > Integrations > Jira\n2. Click 'Reconnect'\n3. Complete the OAuth flow to get new tokens\n4. If refresh fails, you may need to revoke and re-authorize the app in Jira".to_string(),
                related_errors: vec![
                    "401".to_string(),
                    "unauthorized".to_string(),
                    "token expired".to_string(),
                    "invalid_grant".to_string(),
                ],
                tags: vec!["jira".to_string(), "oauth".to_string(), "authentication".to_string()],
            },
            crate::types::CreateKbEntryInput {
                title: "Postman API Key Invalid".to_string(),
                problem: "Postman API calls fail with authentication errors".to_string(),
                cause: "The Postman API key may have been revoked, expired, or was entered incorrectly.".to_string(),
                solution: "1. Log into Postman\n2. Go to Account Settings > API Keys\n3. Generate a new API key\n4. Update the key in Settings > Integrations > Postman".to_string(),
                related_errors: vec![
                    "401".to_string(),
                    "invalid api key".to_string(),
                    "postman".to_string(),
                ],
                tags: vec!["postman".to_string(), "api-key".to_string(), "authentication".to_string()],
            },
            crate::types::CreateKbEntryInput {
                title: "Database Connection Timeout".to_string(),
                problem: "Application fails to connect to database with timeout errors".to_string(),
                cause: "Network issues, database overload, or incorrect connection settings can cause timeouts.".to_string(),
                solution: "1. Check DATABASE_URL environment variable\n2. Verify PostgreSQL server is running\n3. Check network connectivity to database host\n4. Review connection pool settings\n5. Check if database is under heavy load".to_string(),
                related_errors: vec![
                    "connection timeout".to_string(),
                    "database".to_string(),
                    "postgres".to_string(),
                    "pool".to_string(),
                ],
                tags: vec!["database".to_string(), "connection".to_string(), "timeout".to_string()],
            },
            crate::types::CreateKbEntryInput {
                title: "Rate Limit Exceeded".to_string(),
                problem: "API calls fail with 429 Too Many Requests".to_string(),
                cause: "The application is making too many requests to an external API within the rate limit window.".to_string(),
                solution: "1. Wait for the rate limit window to reset (usually 1-15 minutes)\n2. Reduce the frequency of API calls\n3. Implement request batching where possible\n4. Consider upgrading to a higher API tier".to_string(),
                related_errors: vec![
                    "429".to_string(),
                    "rate limit".to_string(),
                    "too many requests".to_string(),
                ],
                tags: vec!["rate-limit".to_string(), "api".to_string()],
            },
            crate::types::CreateKbEntryInput {
                title: "Testmo API Connection Failed".to_string(),
                problem: "Cannot connect to Testmo API or sync test results".to_string(),
                cause: "Incorrect Testmo URL, invalid API key, or network issues.".to_string(),
                solution: "1. Verify Testmo URL is correct (should be https://your-org.testmo.net)\n2. Check API key is valid in Testmo settings\n3. Ensure network allows outbound HTTPS to Testmo\n4. Run integration diagnostics".to_string(),
                related_errors: vec![
                    "testmo".to_string(),
                    "connection failed".to_string(),
                    "api error".to_string(),
                ],
                tags: vec!["testmo".to_string(), "connection".to_string(), "api".to_string()],
            },
        ]
    }
}
