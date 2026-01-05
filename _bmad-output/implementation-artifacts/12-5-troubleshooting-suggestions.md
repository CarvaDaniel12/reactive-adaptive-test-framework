# Story 12.5: Troubleshooting Suggestions

Status: ready-for-dev

## Story

As a system,
I want to suggest solutions based on error type,
So that issues are resolved faster.

## Acceptance Criteria

1. **Given** an error is captured
   **When** support views the error
   **Then** system suggests matching knowledge base entries

2. **Given** error is viewed
   **When** suggestions shown
   **Then** similar past issues and resolutions are displayed

3. **Given** error is viewed
   **When** suggestions shown
   **Then** recommended diagnostic steps are listed

4. **Given** suggestions are displayed
   **When** ranked
   **Then** suggestions are ranked by relevance

5. **Given** suggestions are shown
   **When** feedback given
   **Then** can mark suggestions as helpful/not helpful

6. **Given** feedback is collected
   **When** used
   **Then** suggestion accuracy improves over time (feedback loop)

## Tasks

- [ ] Task 1: Create suggestion matching algorithm
- [ ] Task 2: Create SuggestionsPanel component
- [ ] Task 3: Implement relevance ranking
- [ ] Task 4: Create feedback collection system
- [ ] Task 5: Implement learning from feedback
- [ ] Task 6: Add diagnostic step suggestions

## Dev Notes

### Suggestion Service

```rust
// crates/qa-pms-api/src/services/suggestions.rs
pub struct SuggestionService {
    pool: PgPool,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SuggestionResult {
    pub knowledge_base_matches: Vec<KBMatch>,
    pub similar_errors: Vec<SimilarError>,
    pub diagnostic_steps: Vec<DiagnosticStep>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct KBMatch {
    pub article: KnowledgeBaseArticle,
    pub relevance_score: f64,
    pub match_reason: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SimilarError {
    pub error_id: Uuid,
    pub error_type: String,
    pub message: String,
    pub status: String,
    pub resolution_notes: Option<String>,
    pub similarity_score: f64,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DiagnosticStep {
    pub order: i32,
    pub title: String,
    pub description: String,
    pub action_type: String, // "check", "run_command", "navigate"
    pub action_data: Option<serde_json::Value>,
}

impl SuggestionService {
    pub async fn get_suggestions(&self, error: &ErrorLog) -> Result<SuggestionResult> {
        // Get KB matches
        let kb_matches = self.find_kb_matches(error).await?;
        
        // Find similar resolved errors
        let similar = self.find_similar_errors(error).await?;
        
        // Generate diagnostic steps
        let diagnostic_steps = self.generate_diagnostic_steps(error);

        Ok(SuggestionResult {
            knowledge_base_matches: kb_matches,
            similar_errors: similar,
            diagnostic_steps,
        })
    }

    async fn find_kb_matches(&self, error: &ErrorLog) -> Result<Vec<KBMatch>> {
        // First, try exact error message matches
        let exact_matches = sqlx::query_as::<_, KnowledgeBaseArticle>(
            r#"
            SELECT * FROM knowledge_base_articles
            WHERE EXISTS (
                SELECT 1 FROM unnest(related_errors) AS err
                WHERE $1 ILIKE '%' || err || '%'
            )
            ORDER BY view_count DESC
            LIMIT 3
            "#,
        )
        .bind(&error.message)
        .fetch_all(&self.pool)
        .await?;

        let mut matches: Vec<KBMatch> = exact_matches.into_iter()
            .map(|article| KBMatch {
                article,
                relevance_score: 0.9,
                match_reason: "Error message matches known pattern".into(),
            })
            .collect();

        // If no exact matches, try full-text search
        if matches.is_empty() {
            let search_text = format!("{} {}", error.error_type, error.message);
            let text_matches = sqlx::query_as::<_, (KnowledgeBaseArticle, f32)>(
                r#"
                SELECT kb.*, 
                       ts_rank(to_tsvector('english', title || ' ' || problem || ' ' || solution), 
                               plainto_tsquery('english', $1)) as rank
                FROM knowledge_base_articles kb
                WHERE to_tsvector('english', title || ' ' || problem || ' ' || solution) 
                      @@ plainto_tsquery('english', $1)
                ORDER BY rank DESC
                LIMIT 3
                "#,
            )
            .bind(&search_text)
            .fetch_all(&self.pool)
            .await?;

            matches = text_matches.into_iter()
                .map(|(article, rank)| KBMatch {
                    article,
                    relevance_score: rank as f64,
                    match_reason: "Content similarity".into(),
                })
                .collect();
        }

        Ok(matches)
    }

    async fn find_similar_errors(&self, error: &ErrorLog) -> Result<Vec<SimilarError>> {
        sqlx::query_as(
            r#"
            SELECT 
                id as error_id,
                error_type,
                message,
                status::TEXT,
                resolution_notes,
                1.0 - (
                    levenshtein(LEFT($1, 100), LEFT(message, 100))::FLOAT / 
                    GREATEST(LENGTH(LEFT($1, 100)), LENGTH(LEFT(message, 100)))
                ) as similarity_score
            FROM error_logs
            WHERE id != $2
              AND error_type = $3
              AND status = 'resolved'
              AND resolution_notes IS NOT NULL
            ORDER BY similarity_score DESC
            LIMIT 5
            "#,
        )
        .bind(&error.message)
        .bind(error.id)
        .bind(&error.error_type)
        .fetch_all(&self.pool)
        .await
        .map_err(Into::into)
    }

    fn generate_diagnostic_steps(&self, error: &ErrorLog) -> Vec<DiagnosticStep> {
        let mut steps = Vec::new();
        let error_lower = error.message.to_lowercase();

        // Always start with these
        steps.push(DiagnosticStep {
            order: 1,
            title: "Check Error Context".into(),
            description: "Review the page URL and user action that caused this error".into(),
            action_type: "check".into(),
            action_data: None,
        });

        // Context-specific steps
        if error_lower.contains("network") || error_lower.contains("fetch") {
            steps.push(DiagnosticStep {
                order: 2,
                title: "Run Integration Diagnostics".into(),
                description: "Check if external services are responding".into(),
                action_type: "run_diagnostics".into(),
                action_data: Some(serde_json::json!({ "type": "integrations" })),
            });
        }

        if error_lower.contains("auth") || error_lower.contains("unauthorized") {
            steps.push(DiagnosticStep {
                order: 2,
                title: "Verify User Credentials".into(),
                description: "Check if user's API tokens are still valid".into(),
                action_type: "check_credentials".into(),
                action_data: Some(serde_json::json!({ "user_id": error.user_id })),
            });
        }

        if error_lower.contains("timeout") {
            steps.push(DiagnosticStep {
                order: 2,
                title: "Check Service Latency".into(),
                description: "Review response times for affected services".into(),
                action_type: "check_latency".into(),
                action_data: None,
            });
        }

        steps.push(DiagnosticStep {
            order: steps.len() as i32 + 1,
            title: "Contact User".into(),
            description: "If issue persists, reach out to the user for more context".into(),
            action_type: "contact".into(),
            action_data: Some(serde_json::json!({ "user_id": error.user_id })),
        });

        steps
    }
}
```

### Feedback System

```sql
-- Feedback tracking table
CREATE TABLE suggestion_feedback (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    error_id UUID REFERENCES error_logs(id),
    suggestion_type VARCHAR(50) NOT NULL, -- "kb_article", "similar_error", "diagnostic"
    suggestion_id VARCHAR(255) NOT NULL,
    helpful BOOLEAN NOT NULL,
    user_id VARCHAR(255),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_suggestion_feedback_type ON suggestion_feedback(suggestion_type, suggestion_id);
```

### Suggestions Panel Component

```tsx
// frontend/src/components/support/SuggestionsPanel.tsx
interface SuggestionsPanelProps {
  errorId: string;
}

export function SuggestionsPanel({ errorId }: SuggestionsPanelProps) {
  const { data: suggestions, isLoading } = useSuggestions(errorId);
  const { mutate: submitFeedback } = useSuggestionFeedback();

  if (isLoading) {
    return <SuggestionsSkeleton />;
  }

  return (
    <div className="space-y-6">
      {/* Knowledge Base Matches */}
      {suggestions?.knowledgeBaseMatches.length > 0 && (
        <section>
          <h4 className="font-medium text-neutral-700 mb-3 flex items-center gap-2">
            <BookOpenIcon className="w-4 h-4" />
            Related Knowledge Base Articles
          </h4>
          <div className="space-y-2">
            {suggestions.knowledgeBaseMatches.map((match) => (
              <SuggestionCard
                key={match.article.id}
                type="kb_article"
                id={match.article.id}
                title={match.article.title}
                description={match.article.problem}
                relevance={match.relevanceScore}
                reason={match.matchReason}
                onFeedback={(helpful) => 
                  submitFeedback({ errorId, suggestionType: "kb_article", suggestionId: match.article.id, helpful })
                }
                onSelect={() => window.open(`/admin/knowledge-base/${match.article.id}`, "_blank")}
              />
            ))}
          </div>
        </section>
      )}

      {/* Similar Resolved Errors */}
      {suggestions?.similarErrors.length > 0 && (
        <section>
          <h4 className="font-medium text-neutral-700 mb-3 flex items-center gap-2">
            <ArchiveIcon className="w-4 h-4" />
            Similar Resolved Issues
          </h4>
          <div className="space-y-2">
            {suggestions.similarErrors.map((similar) => (
              <SuggestionCard
                key={similar.errorId}
                type="similar_error"
                id={similar.errorId}
                title={similar.errorType}
                description={similar.resolutionNotes || similar.message}
                relevance={similar.similarityScore}
                onFeedback={(helpful) => 
                  submitFeedback({ errorId, suggestionType: "similar_error", suggestionId: similar.errorId, helpful })
                }
                onSelect={() => navigateToError(similar.errorId)}
              />
            ))}
          </div>
        </section>
      )}

      {/* Diagnostic Steps */}
      <section>
        <h4 className="font-medium text-neutral-700 mb-3 flex items-center gap-2">
          <MixerHorizontalIcon className="w-4 h-4" />
          Recommended Diagnostic Steps
        </h4>
        <div className="space-y-2">
          {suggestions?.diagnosticSteps.map((step) => (
            <DiagnosticStepCard key={step.order} step={step} />
          ))}
        </div>
      </section>
    </div>
  );
}

function SuggestionCard({
  type, id, title, description, relevance, reason, onFeedback, onSelect
}: SuggestionCardProps) {
  const [feedbackGiven, setFeedbackGiven] = useState<boolean | null>(null);

  const handleFeedback = (helpful: boolean) => {
    setFeedbackGiven(helpful);
    onFeedback(helpful);
  };

  return (
    <div className="p-4 bg-white border border-neutral-200 rounded-lg">
      <div className="flex items-start justify-between">
        <button onClick={onSelect} className="text-left flex-1">
          <p className="font-medium text-neutral-900 hover:text-primary-600">{title}</p>
          <p className="text-sm text-neutral-600 mt-1 line-clamp-2">{description}</p>
          {reason && (
            <p className="text-xs text-neutral-400 mt-2">{reason}</p>
          )}
        </button>
        
        {relevance && (
          <span className="px-2 py-1 bg-primary-50 text-primary-700 text-xs rounded-full">
            {(relevance * 100).toFixed(0)}% match
          </span>
        )}
      </div>

      {/* Feedback */}
      <div className="mt-3 pt-3 border-t border-neutral-100 flex items-center gap-2">
        <span className="text-xs text-neutral-500">Was this helpful?</span>
        <button
          onClick={() => handleFeedback(true)}
          disabled={feedbackGiven !== null}
          className={cn(
            "p-1 rounded",
            feedbackGiven === true ? "text-success-500" : "text-neutral-400 hover:text-success-500"
          )}
        >
          <ThumbsUpIcon className="w-4 h-4" />
        </button>
        <button
          onClick={() => handleFeedback(false)}
          disabled={feedbackGiven !== null}
          className={cn(
            "p-1 rounded",
            feedbackGiven === false ? "text-error-500" : "text-neutral-400 hover:text-error-500"
          )}
        >
          <ThumbsDownIcon className="w-4 h-4" />
        </button>
      </div>
    </div>
  );
}
```

### References

- [Source: epics.md#Story 12.5]
