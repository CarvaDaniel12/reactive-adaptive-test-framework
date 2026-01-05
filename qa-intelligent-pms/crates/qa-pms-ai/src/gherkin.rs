//! Gherkin analysis and test suggestion service.

use tracing::debug;

use crate::error::AIError;
use crate::provider::AIClient;
use crate::types::{
    ChatMessage, GherkinAnalysisResult, GherkinInput, GherkinScenario, MessageRole,
};

/// Service for analyzing Gherkin acceptance criteria.
pub struct GherkinAnalyzer {
    client: AIClient,
}

impl GherkinAnalyzer {
    /// Create a new Gherkin analyzer.
    #[must_use] 
    pub const fn new(client: AIClient) -> Self {
        Self { client }
    }

    /// Analyze Gherkin acceptance criteria and generate test suggestions.
    pub async fn analyze(&self, input: GherkinInput) -> Result<GherkinAnalysisResult, AIError> {
        let prompt = self.build_prompt(&input);

        let messages = vec![
            ChatMessage {
                id: uuid::Uuid::new_v4(),
                role: MessageRole::System,
                content: GHERKIN_SYSTEM_PROMPT.to_string(),
                timestamp: chrono::Utc::now(),
            },
            ChatMessage {
                id: uuid::Uuid::new_v4(),
                role: MessageRole::User,
                content: prompt,
                timestamp: chrono::Utc::now(),
            },
        ];

        debug!("Analyzing Gherkin acceptance criteria");

        let (response, _) = self.client.chat(messages).await?;

        self.parse_response(&response.content)
    }

    /// Build the prompt for Gherkin analysis.
    fn build_prompt(&self, input: &GherkinInput) -> String {
        let mut prompt = format!(
            "Analyze these acceptance criteria:\n\n{}\n",
            input.acceptance_criteria
        );

        if let Some(ticket) = &input.ticket_context {
            prompt.push_str(&format!(
                "\nTicket context:\n- Key: {}\n- Title: {}\n- Type: {}\n",
                ticket.key, ticket.title, ticket.ticket_type
            ));
        }

        prompt.push_str("\nProvide your analysis as JSON.");

        prompt
    }

    /// Parse the AI response into a structured result.
    fn parse_response(&self, content: &str) -> Result<GherkinAnalysisResult, AIError> {
        // Try to extract JSON from the response
        let json_start = content.find('{');
        let json_end = content.rfind('}');

        if let (Some(start), Some(end)) = (json_start, json_end) {
            let json_str = &content[start..=end];
            if let Ok(result) = serde_json::from_str::<GherkinAnalysisResult>(json_str) {
                return Ok(result);
            }
        }

        // Fallback: parse manually
        let scenarios = self.parse_scenarios_from_text(content);
        let edge_cases = self.extract_suggestions(content, "edge");
        let negative_tests = self.extract_suggestions(content, "negative");

        Ok(GherkinAnalysisResult {
            scenarios,
            edge_cases,
            negative_tests,
        })
    }

    /// Parse scenarios from text content.
    fn parse_scenarios_from_text(&self, content: &str) -> Vec<GherkinScenario> {
        let mut scenarios = Vec::new();

        // Look for Given/When/Then patterns
        let lines: Vec<&str> = content.lines().collect();
        let mut current_scenario: Option<GherkinScenario> = None;

        for line in lines {
            let trimmed = line.trim();

            if trimmed.starts_with("Scenario") || trimmed.starts_with("**Scenario") {
                // Save previous scenario
                if let Some(scenario) = current_scenario.take() {
                    scenarios.push(scenario);
                }

                // Start new scenario
                let name = trimmed
                    .trim_start_matches("Scenario")
                    .trim_start_matches("**Scenario")
                    .trim_start_matches(':')
                    .trim_start_matches("**")
                    .trim()
                    .to_string();

                current_scenario = Some(GherkinScenario {
                    name,
                    given: Vec::new(),
                    when: Vec::new(),
                    then: Vec::new(),
                    suggested_test_steps: Vec::new(),
                });
            } else if let Some(ref mut scenario) = current_scenario {
                if trimmed.starts_with("Given") || trimmed.starts_with("- Given") {
                    let step = trimmed
                        .trim_start_matches("- ")
                        .trim_start_matches("Given ")
                        .to_string();
                    scenario.given.push(step);
                } else if trimmed.starts_with("When") || trimmed.starts_with("- When") {
                    let step = trimmed
                        .trim_start_matches("- ")
                        .trim_start_matches("When ")
                        .to_string();
                    scenario.when.push(step);
                } else if trimmed.starts_with("Then") || trimmed.starts_with("- Then") {
                    let step = trimmed
                        .trim_start_matches("- ")
                        .trim_start_matches("Then ")
                        .to_string();
                    scenario.then.push(step);
                } else if trimmed.starts_with("And") || trimmed.starts_with("- And") {
                    // Add to the last category
                    let step = trimmed
                        .trim_start_matches("- ")
                        .trim_start_matches("And ")
                        .to_string();
                    if !scenario.then.is_empty() {
                        scenario.then.push(step);
                    } else if !scenario.when.is_empty() {
                        scenario.when.push(step);
                    } else {
                        scenario.given.push(step);
                    }
                }
            }
        }

        // Don't forget the last scenario
        if let Some(scenario) = current_scenario {
            scenarios.push(scenario);
        }

        // Generate suggested test steps for each scenario
        for scenario in &mut scenarios {
            scenario.suggested_test_steps = Self::generate_test_steps(scenario);
        }

        scenarios
    }

    /// Generate suggested test steps for a scenario.
    fn generate_test_steps(scenario: &GherkinScenario) -> Vec<String> {
        let mut steps = Vec::new();

        // Setup steps from Given
        for given in &scenario.given {
            steps.push(format!("Setup: {given}"));
        }

        // Action steps from When
        for when in &scenario.when {
            steps.push(format!("Action: {when}"));
        }

        // Verification steps from Then
        for then in &scenario.then {
            steps.push(format!("Verify: {then}"));
        }

        steps
    }

    /// Extract suggestions from content.
    fn extract_suggestions(&self, content: &str, keyword: &str) -> Vec<String> {
        let mut suggestions = Vec::new();
        let _content_lower = content.to_lowercase();

        // Find sections containing the keyword
        for line in content.lines() {
            let line_lower = line.to_lowercase();
            if line_lower.contains(keyword) && (line.starts_with('-') || line.starts_with('*')) {
                let cleaned = line
                    .trim_start_matches('-')
                    .trim_start_matches('*')
                    .trim()
                    .to_string();
                if !cleaned.is_empty() && cleaned.len() > 10 {
                    suggestions.push(cleaned);
                }
            }
        }

        suggestions
    }

    /// Perform a fallback analysis (when AI is unavailable).
    #[must_use] 
    pub fn fallback_analysis(input: &GherkinInput) -> GherkinAnalysisResult {
        let mut scenarios = Vec::new();
        let mut edge_cases = Vec::new();
        let mut negative_tests = Vec::new();

        // Parse Gherkin from the acceptance criteria
        let lines: Vec<&str> = input.acceptance_criteria.lines().collect();
        let mut current_scenario: Option<GherkinScenario> = None;

        for line in lines {
            let trimmed = line.trim();

            if trimmed.starts_with("Given") {
                if current_scenario.is_none() {
                    current_scenario = Some(GherkinScenario {
                        name: "Scenario from AC".to_string(),
                        given: Vec::new(),
                        when: Vec::new(),
                        then: Vec::new(),
                        suggested_test_steps: Vec::new(),
                    });
                }
                if let Some(ref mut scenario) = current_scenario {
                    scenario.given.push(trimmed.trim_start_matches("Given ").to_string());
                }
            } else if trimmed.starts_with("When") {
                if let Some(ref mut scenario) = current_scenario {
                    scenario.when.push(trimmed.trim_start_matches("When ").to_string());
                }
            } else if trimmed.starts_with("Then") {
                if let Some(ref mut scenario) = current_scenario {
                    scenario.then.push(trimmed.trim_start_matches("Then ").to_string());
                }
            } else if trimmed.starts_with("And") {
                if let Some(ref mut scenario) = current_scenario {
                    let step = trimmed.trim_start_matches("And ").to_string();
                    if !scenario.then.is_empty() {
                        scenario.then.push(step);
                    } else if !scenario.when.is_empty() {
                        scenario.when.push(step);
                    } else {
                        scenario.given.push(step);
                    }
                }
            }
        }

        if let Some(mut scenario) = current_scenario {
            scenario.suggested_test_steps = Self::generate_test_steps(&scenario);
            scenarios.push(scenario);
        }

        // Add generic edge cases
        edge_cases.push("Test with empty/null inputs".to_string());
        edge_cases.push("Test with maximum length inputs".to_string());
        edge_cases.push("Test with special characters".to_string());

        // Add generic negative tests
        negative_tests.push("Test with invalid credentials".to_string());
        negative_tests.push("Test with missing required fields".to_string());
        negative_tests.push("Test with unauthorized access".to_string());

        GherkinAnalysisResult {
            scenarios,
            edge_cases,
            negative_tests,
        }
    }
}

const GHERKIN_SYSTEM_PROMPT: &str = r#"You are a QA test analyst expert in Gherkin/BDD. Analyze acceptance criteria and generate test suggestions.

Your task:
1. Parse Given/When/Then scenarios from the acceptance criteria
2. Suggest additional edge cases to test
3. Suggest negative test cases

Output ONLY valid JSON in this format:
{
  "scenarios": [
    {
      "name": "Scenario name",
      "given": ["precondition 1", "precondition 2"],
      "when": ["action 1"],
      "then": ["expected result 1", "expected result 2"],
      "suggested_test_steps": ["Step 1: ...", "Step 2: ..."]
    }
  ],
  "edge_cases": ["Edge case 1", "Edge case 2"],
  "negative_tests": ["Negative test 1", "Negative test 2"]
}

Be thorough but concise. Focus on actionable test suggestions."#;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fallback_analysis() {
        let input = GherkinInput {
            acceptance_criteria: r#"
Given I am on the login page
When I enter valid credentials
And I click the login button
Then I should be redirected to the dashboard
And I should see my username
"#
            .to_string(),
            ticket_context: None,
        };

        let result = GherkinAnalyzer::fallback_analysis(&input);

        assert!(!result.scenarios.is_empty());
        let scenario = &result.scenarios[0];
        assert!(!scenario.given.is_empty());
        assert!(!scenario.when.is_empty());
        assert!(!scenario.then.is_empty());
        assert!(!result.edge_cases.is_empty());
        assert!(!result.negative_tests.is_empty());
    }

    #[test]
    fn test_generate_test_steps() {
        let scenario = GherkinScenario {
            name: "Test".to_string(),
            given: vec!["user is logged in".to_string()],
            when: vec!["user clicks button".to_string()],
            then: vec!["action is performed".to_string()],
            suggested_test_steps: Vec::new(),
        };

        let steps = GherkinAnalyzer::generate_test_steps(&scenario);

        assert_eq!(steps.len(), 3);
        assert!(steps[0].starts_with("Setup:"));
        assert!(steps[1].starts_with("Action:"));
        assert!(steps[2].starts_with("Verify:"));
    }
}
