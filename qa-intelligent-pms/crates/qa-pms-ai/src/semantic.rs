//! Semantic search enhancement service.

use tracing::debug;

use crate::error::AIError;
use crate::provider::AIClient;
use crate::types::{ChatMessage, MessageRole, SemanticSearchInput, SemanticSearchResult};

/// Service for AI-enhanced semantic search.
pub struct SemanticSearchService {
    client: AIClient,
}

impl SemanticSearchService {
    /// Create a new semantic search service.
    #[must_use] 
    pub const fn new(client: AIClient) -> Self {
        Self { client }
    }

    /// Analyze a ticket and generate semantic search queries.
    pub async fn analyze(&self, input: SemanticSearchInput) -> Result<SemanticSearchResult, AIError> {
        let prompt = self.build_prompt(&input);

        let messages = vec![
            ChatMessage {
                id: uuid::Uuid::new_v4(),
                role: MessageRole::System,
                content: SEMANTIC_SYSTEM_PROMPT.to_string(),
                timestamp: chrono::Utc::now(),
            },
            ChatMessage {
                id: uuid::Uuid::new_v4(),
                role: MessageRole::User,
                content: prompt,
                timestamp: chrono::Utc::now(),
            },
        ];

        debug!("Analyzing ticket for semantic search");

        let (response, _) = self.client.chat(messages).await?;

        self.parse_response(&response.content)
    }

    /// Build the prompt for semantic analysis.
    fn build_prompt(&self, input: &SemanticSearchInput) -> String {
        let mut prompt = format!("Analyze this ticket for test search:\n\nTitle: {}\n", input.title);

        if let Some(desc) = &input.description {
            prompt.push_str(&format!("\nDescription:\n{desc}\n"));
        }

        if let Some(ac) = &input.acceptance_criteria {
            prompt.push_str(&format!("\nAcceptance Criteria:\n{ac}\n"));
        }

        prompt.push_str("\nProvide your analysis in the following JSON format:\n");
        prompt.push_str(r#"{"queries": ["query1", "query2"], "key_concepts": ["concept1"], "test_areas": ["area1"]}"#);

        prompt
    }

    /// Parse the AI response into a structured result.
    fn parse_response(&self, content: &str) -> Result<SemanticSearchResult, AIError> {
        // Try to extract JSON from the response
        let json_start = content.find('{');
        let json_end = content.rfind('}');

        if let (Some(start), Some(end)) = (json_start, json_end) {
            let json_str = &content[start..=end];
            if let Ok(result) = serde_json::from_str::<SemanticSearchResult>(json_str) {
                return Ok(result);
            }
        }

        // Fallback: extract information manually
        let queries = self.extract_list(content, "queries");
        let key_concepts = self.extract_list(content, "key_concepts");
        let test_areas = self.extract_list(content, "test_areas");

        Ok(SemanticSearchResult {
            queries: if queries.is_empty() {
                vec!["API test".to_string(), "integration test".to_string()]
            } else {
                queries
            },
            key_concepts: if key_concepts.is_empty() {
                vec!["functionality".to_string()]
            } else {
                key_concepts
            },
            test_areas: if test_areas.is_empty() {
                vec!["functional testing".to_string()]
            } else {
                test_areas
            },
        })
    }

    /// Extract a list from the response content.
    fn extract_list(&self, content: &str, key: &str) -> Vec<String> {
        let mut results = Vec::new();

        // Look for the key and extract items
        if let Some(start) = content.find(key) {
            let rest = &content[start..];
            if let Some(bracket_start) = rest.find('[') {
                if let Some(bracket_end) = rest[bracket_start..].find(']') {
                    let items_str = &rest[bracket_start + 1..bracket_start + bracket_end];
                    for item in items_str.split(',') {
                        let cleaned = item
                            .trim()
                            .trim_matches('"')
                            .trim_matches('\'')
                            .to_string();
                        if !cleaned.is_empty() {
                            results.push(cleaned);
                        }
                    }
                }
            }
        }

        results
    }

    /// Perform a fallback keyword-based search (when AI is unavailable).
    #[must_use] 
    pub fn fallback_search(input: &SemanticSearchInput) -> SemanticSearchResult {
        let mut queries = Vec::new();
        let mut key_concepts = Vec::new();

        // Extract words from title
        let title_words: Vec<&str> = input
            .title
            .split_whitespace()
            .filter(|w| w.len() > 3)
            .collect();

        for word in title_words.iter().take(5) {
            queries.push(word.to_lowercase());
            key_concepts.push(word.to_lowercase());
        }

        // Add common test areas based on keywords
        let mut test_areas = Vec::new();
        let combined = format!(
            "{} {} {}",
            input.title,
            input.description.as_deref().unwrap_or(""),
            input.acceptance_criteria.as_deref().unwrap_or("")
        )
        .to_lowercase();

        if combined.contains("api") || combined.contains("endpoint") {
            test_areas.push("API testing".to_string());
        }
        if combined.contains("login") || combined.contains("auth") {
            test_areas.push("Authentication testing".to_string());
        }
        if combined.contains("form") || combined.contains("input") {
            test_areas.push("Form validation".to_string());
        }
        if combined.contains("performance") || combined.contains("load") {
            test_areas.push("Performance testing".to_string());
        }
        if combined.contains("security") || combined.contains("permission") {
            test_areas.push("Security testing".to_string());
        }

        if test_areas.is_empty() {
            test_areas.push("Functional testing".to_string());
        }

        SemanticSearchResult {
            queries,
            key_concepts,
            test_areas,
        }
    }
}

const SEMANTIC_SYSTEM_PROMPT: &str = r#"You are a QA test search assistant. Analyze tickets to generate effective search queries for finding related tests.

Your task:
1. Extract key concepts from the ticket
2. Generate search queries that would find related API tests, UI tests, and integration tests
3. Identify test areas that should be covered

Output ONLY valid JSON in this format:
{
  "queries": ["search query 1", "search query 2", "search query 3"],
  "key_concepts": ["concept1", "concept2"],
  "test_areas": ["area1", "area2"]
}

Keep queries concise (2-4 words each). Focus on technical terms and functionality names."#;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fallback_search() {
        let input = SemanticSearchInput {
            title: "Fix login API endpoint".to_string(),
            description: Some("The login endpoint returns 500 error".to_string()),
            acceptance_criteria: None,
        };

        let result = SemanticSearchService::fallback_search(&input);

        assert!(!result.queries.is_empty());
        assert!(result.test_areas.iter().any(|a| a.contains("API") || a.contains("Authentication")));
    }

    #[test]
    fn test_fallback_search_performance() {
        let input = SemanticSearchInput {
            title: "Improve performance of dashboard".to_string(),
            description: Some("Dashboard loads slowly under load".to_string()),
            acceptance_criteria: None,
        };

        let result = SemanticSearchService::fallback_search(&input);

        assert!(result.test_areas.iter().any(|a| a.contains("Performance")));
    }
}
