# Story 13.3: Gherkin-Based Test Suggestions

Status: ready-for-dev

## Story

As a QA (Ana),
I want AI to suggest tests from Gherkin acceptance criteria,
So that I can quickly create test cases.

## Acceptance Criteria

1. **Given** ticket has Gherkin-style acceptance criteria
   **When** AI analyzes the ticket
   **Then** Given/When/Then scenarios are parsed

2. **Given** scenarios are parsed
   **When** suggestions generated
   **Then** suggested test steps for each scenario are shown

3. **Given** scenarios are parsed
   **When** suggestions generated
   **Then** edge cases to consider are listed

4. **Given** scenarios are parsed
   **When** suggestions generated
   **Then** potential negative test cases are suggested

5. **Given** suggestions exist
   **When** displayed
   **Then** suggestions are shown in collapsible section

6. **Given** suggestion is shown
   **When** user interacts
   **Then** user can accept, edit, or reject each suggestion

7. **Given** suggestion is accepted
   **When** saved
   **Then** accepted suggestions can be saved to notes

## Tasks

- [ ] Task 1: Create Gherkin parser
- [ ] Task 2: Implement AI test suggestion generation
- [ ] Task 3: Create TestSuggestionsPanel component
- [ ] Task 4: Create SuggestionCard with actions
- [ ] Task 5: Implement accept/edit/reject flow
- [ ] Task 6: Create save to notes functionality

## Dev Notes

### Gherkin Parser & AI Service

```rust
// crates/qa-pms-ai/src/gherkin.rs
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GherkinScenario {
    pub name: Option<String>,
    pub given: Vec<String>,
    pub when: Vec<String>,
    pub then: Vec<String>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TestSuggestion {
    pub id: String,
    pub scenario_name: String,
    pub test_steps: Vec<TestStep>,
    pub edge_cases: Vec<String>,
    pub negative_cases: Vec<NegativeCase>,
    pub estimated_time_minutes: i32,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TestStep {
    pub order: i32,
    pub action: String,
    pub expected_result: String,
    pub data_requirements: Option<String>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NegativeCase {
    pub description: String,
    pub test_approach: String,
    pub expected_behavior: String,
}

pub struct GherkinTestSuggestionService {
    ai_client: Box<dyn AIClient>,
}

impl GherkinTestSuggestionService {
    pub async fn generate_suggestions(&self, ticket: &JiraTicket) -> Result<Vec<TestSuggestion>> {
        // First, try to parse Gherkin from description
        let scenarios = self.parse_gherkin(&ticket.fields.description)?;

        if scenarios.is_empty() {
            // No Gherkin found, try to infer from description
            return self.generate_suggestions_from_description(ticket).await;
        }

        let mut suggestions = Vec::new();

        for scenario in scenarios {
            let suggestion = self.generate_suggestion_for_scenario(&scenario).await?;
            suggestions.push(suggestion);
        }

        Ok(suggestions)
    }

    fn parse_gherkin(&self, description: &Option<String>) -> Result<Vec<GherkinScenario>> {
        let Some(desc) = description else {
            return Ok(Vec::new());
        };

        let mut scenarios = Vec::new();
        let mut current_scenario: Option<GherkinScenario> = None;

        for line in desc.lines() {
            let trimmed = line.trim();
            
            if trimmed.starts_with("Scenario:") || trimmed.starts_with("Feature:") {
                if let Some(scenario) = current_scenario.take() {
                    scenarios.push(scenario);
                }
                current_scenario = Some(GherkinScenario {
                    name: Some(trimmed.split(':').nth(1).unwrap_or("").trim().to_string()),
                    given: Vec::new(),
                    when: Vec::new(),
                    then: Vec::new(),
                });
            } else if let Some(ref mut scenario) = current_scenario {
                if trimmed.starts_with("Given") {
                    scenario.given.push(trimmed.trim_start_matches("Given").trim().to_string());
                } else if trimmed.starts_with("And") && !scenario.given.is_empty() && scenario.when.is_empty() {
                    scenario.given.push(trimmed.trim_start_matches("And").trim().to_string());
                } else if trimmed.starts_with("When") {
                    scenario.when.push(trimmed.trim_start_matches("When").trim().to_string());
                } else if trimmed.starts_with("And") && !scenario.when.is_empty() && scenario.then.is_empty() {
                    scenario.when.push(trimmed.trim_start_matches("And").trim().to_string());
                } else if trimmed.starts_with("Then") {
                    scenario.then.push(trimmed.trim_start_matches("Then").trim().to_string());
                } else if trimmed.starts_with("And") && !scenario.then.is_empty() {
                    scenario.then.push(trimmed.trim_start_matches("And").trim().to_string());
                }
            }
        }

        if let Some(scenario) = current_scenario {
            scenarios.push(scenario);
        }

        Ok(scenarios)
    }

    async fn generate_suggestion_for_scenario(&self, scenario: &GherkinScenario) -> Result<TestSuggestion> {
        let prompt = format!(
            r#"Generate test suggestions for this Gherkin scenario:

{}

Given: {}
When: {}
Then: {}

Provide a JSON response with:
1. test_steps: Array of {{ order, action, expected_result, data_requirements }}
2. edge_cases: Array of edge case strings to consider
3. negative_cases: Array of {{ description, test_approach, expected_behavior }}
4. estimated_time_minutes: Integer estimate

Be specific and practical. Focus on testable actions."#,
            scenario.name.as_deref().unwrap_or("Unnamed"),
            scenario.given.join(", "),
            scenario.when.join(", "),
            scenario.then.join(", ")
        );

        let response = self.ai_client.complete(&prompt).await?;
        let mut suggestion: TestSuggestion = serde_json::from_str(&response)?;
        suggestion.id = uuid::Uuid::new_v4().to_string();
        suggestion.scenario_name = scenario.name.clone().unwrap_or_else(|| "Test Scenario".into());

        Ok(suggestion)
    }
}
```

### Test Suggestions Panel

```tsx
// frontend/src/components/ai/TestSuggestionsPanel.tsx
interface TestSuggestionsPanelProps {
  ticketKey: string;
  onAddToNotes: (text: string) => void;
}

export function TestSuggestionsPanel({ ticketKey, onAddToNotes }: TestSuggestionsPanelProps) {
  const { data: suggestions, isLoading, error } = useTestSuggestions(ticketKey);
  const { data: aiConfig } = useAIConfig();

  if (!aiConfig?.isEnabled) {
    return (
      <div className="p-4 bg-neutral-50 rounded-lg text-center">
        <SparklesIcon className="w-8 h-8 mx-auto text-neutral-400 mb-2" />
        <p className="text-neutral-600">AI features not enabled</p>
        <Link to="/settings/ai" className="text-primary-600 hover:underline text-sm">
          Configure AI →
        </Link>
      </div>
    );
  }

  if (isLoading) {
    return <SuggestionsSkeleton />;
  }

  if (error) {
    return (
      <div className="p-4 bg-error-50 rounded-lg text-error-700">
        Failed to generate suggestions. Using basic mode.
      </div>
    );
  }

  return (
    <div className="space-y-4">
      <div className="flex items-center gap-2">
        <SparklesIcon className="w-5 h-5 text-primary-500" />
        <h3 className="font-semibold">AI Test Suggestions</h3>
        <span className="text-xs text-neutral-500">
          {suggestions?.length || 0} scenarios
        </span>
      </div>

      {suggestions?.map((suggestion) => (
        <SuggestionCard
          key={suggestion.id}
          suggestion={suggestion}
          onAddToNotes={onAddToNotes}
        />
      ))}
    </div>
  );
}

interface SuggestionCardProps {
  suggestion: TestSuggestion;
  onAddToNotes: (text: string) => void;
}

function SuggestionCard({ suggestion, onAddToNotes }: SuggestionCardProps) {
  const [isExpanded, setIsExpanded] = useState(false);
  const [editedSteps, setEditedSteps] = useState(suggestion.testSteps);
  const [acceptedItems, setAcceptedItems] = useState<Set<string>>(new Set());

  const handleAccept = (itemId: string) => {
    setAcceptedItems(prev => new Set([...prev, itemId]));
  };

  const handleReject = (itemId: string) => {
    setAcceptedItems(prev => {
      const next = new Set(prev);
      next.delete(itemId);
      return next;
    });
  };

  const handleAddAllToNotes = () => {
    const text = formatSuggestionAsMarkdown(suggestion, editedSteps);
    onAddToNotes(text);
    toast.success("Added to notes");
  };

  return (
    <Collapsible.Root open={isExpanded} onOpenChange={setIsExpanded}>
      <div className="border border-neutral-200 rounded-lg overflow-hidden">
        <Collapsible.Trigger asChild>
          <button className="w-full p-4 flex items-center justify-between hover:bg-neutral-50">
            <div className="text-left">
              <p className="font-medium">{suggestion.scenarioName}</p>
              <p className="text-sm text-neutral-500">
                {suggestion.testSteps.length} steps • {suggestion.edgeCases.length} edge cases
              </p>
            </div>
            <div className="flex items-center gap-2">
              <span className="text-xs text-neutral-400">
                ~{suggestion.estimatedTimeMinutes} min
              </span>
              <ChevronDownIcon className={cn("w-5 h-5", isExpanded && "rotate-180")} />
            </div>
          </button>
        </Collapsible.Trigger>

        <Collapsible.Content>
          <div className="p-4 pt-0 space-y-4 border-t border-neutral-100">
            {/* Test Steps */}
            <section>
              <h5 className="text-sm font-medium text-neutral-700 mb-2">Test Steps</h5>
              <div className="space-y-2">
                {editedSteps.map((step, i) => (
                  <div key={i} className="flex items-start gap-2 p-2 bg-neutral-50 rounded">
                    <span className="w-6 h-6 rounded-full bg-primary-100 text-primary-700 text-xs flex items-center justify-center flex-shrink-0">
                      {step.order}
                    </span>
                    <div className="flex-1">
                      <p className="text-sm">{step.action}</p>
                      <p className="text-xs text-neutral-500">Expected: {step.expectedResult}</p>
                    </div>
                  </div>
                ))}
              </div>
            </section>

            {/* Edge Cases */}
            {suggestion.edgeCases.length > 0 && (
              <section>
                <h5 className="text-sm font-medium text-neutral-700 mb-2">Edge Cases to Consider</h5>
                <ul className="text-sm text-neutral-600 space-y-1">
                  {suggestion.edgeCases.map((ec, i) => (
                    <li key={i} className="flex items-start gap-2">
                      <span className="text-warning-500">•</span>
                      {ec}
                    </li>
                  ))}
                </ul>
              </section>
            )}

            {/* Negative Cases */}
            {suggestion.negativeCases.length > 0 && (
              <section>
                <h5 className="text-sm font-medium text-neutral-700 mb-2">Negative Test Cases</h5>
                <div className="space-y-2">
                  {suggestion.negativeCases.map((nc, i) => (
                    <div key={i} className="p-2 bg-error-50 rounded text-sm">
                      <p className="font-medium text-error-700">{nc.description}</p>
                      <p className="text-error-600 text-xs mt-1">
                        Approach: {nc.testApproach}
                      </p>
                    </div>
                  ))}
                </div>
              </section>
            )}

            {/* Actions */}
            <div className="flex items-center gap-2 pt-2 border-t">
              <button
                onClick={handleAddAllToNotes}
                className="px-3 py-1.5 bg-primary-500 text-white text-sm rounded hover:bg-primary-600"
              >
                Add to Notes
              </button>
              <button
                onClick={() => {/* Copy to clipboard */}}
                className="px-3 py-1.5 text-sm text-neutral-600 hover:text-neutral-800"
              >
                Copy as Markdown
              </button>
            </div>
          </div>
        </Collapsible.Content>
      </div>
    </Collapsible.Root>
  );
}
```

### References

- [Source: epics.md#Story 13.3]
