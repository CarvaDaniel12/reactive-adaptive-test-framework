# Story 13.2: Semantic Search Enhancement

Status: ready-for-dev

## Story

As a QA (Ana),
I want AI-powered semantic search,
So that I find related tests even with different wording.

## Acceptance Criteria

1. **Given** user has AI configured
   **When** contextual search runs (ticket selection)
   **Then** AI analyzes ticket title, description, acceptance criteria

2. **Given** AI analysis runs
   **When** query generated
   **Then** generates semantic query for search

3. **Given** semantic query exists
   **When** search runs
   **Then** searches Postman/Testmo with meaning-based matching

4. **Given** search results returned
   **When** displayed
   **Then** results are ranked by semantic relevance

5. **Given** results are shown
   **When** relevance displayed
   **Then** results show relevance score (e.g., 85% match)

6. **Given** AI fails or is unavailable
   **When** search runs
   **Then** falls back to keyword search (FR-AI-07)

7. **Given** performance requirements
   **When** search executes
   **Then** search still completes in < 3s

## Tasks

- [ ] Task 1: Create SemanticSearchService
- [ ] Task 2: Implement ticket content extraction
- [ ] Task 3: Create AI query generation
- [ ] Task 4: Implement embedding-based matching
- [ ] Task 5: Create relevance scoring
- [ ] Task 6: Implement fallback to keyword search
- [ ] Task 7: Add timeout handling

## Dev Notes

### Semantic Search Service

```rust
// crates/qa-pms-ai/src/semantic_search.rs
use tokio::time::{timeout, Duration};

pub struct SemanticSearchService {
    ai_client: Option<Box<dyn AIClient>>,
    postman_client: PostmanClient,
    testmo_client: TestmoClient,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SemanticSearchResult {
    pub postman_results: Vec<PostmanMatch>,
    pub testmo_results: Vec<TestmoMatch>,
    pub search_type: String, // "semantic" or "keyword"
    pub query_used: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PostmanMatch {
    pub collection: PostmanCollection,
    pub relevance_score: f64,
    pub matched_keywords: Vec<String>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TestmoMatch {
    pub test_case: TestmoCase,
    pub relevance_score: f64,
    pub matched_keywords: Vec<String>,
}

impl SemanticSearchService {
    pub async fn search(&self, ticket: &JiraTicket) -> Result<SemanticSearchResult> {
        // Try semantic search first, with timeout
        let semantic_result = timeout(
            Duration::from_secs(2),
            self.semantic_search(ticket)
        ).await;

        match semantic_result {
            Ok(Ok(result)) => Ok(result),
            Ok(Err(e)) => {
                tracing::warn!("Semantic search failed, falling back to keyword: {}", e);
                self.keyword_search(ticket).await
            }
            Err(_) => {
                tracing::warn!("Semantic search timed out, falling back to keyword");
                self.keyword_search(ticket).await
            }
        }
    }

    async fn semantic_search(&self, ticket: &JiraTicket) -> Result<SemanticSearchResult> {
        let Some(ai_client) = &self.ai_client else {
            return Err(anyhow!("AI not configured"));
        };

        // Extract searchable content
        let content = self.extract_ticket_content(ticket);
        
        // Generate semantic query using AI
        let semantic_query = self.generate_semantic_query(ai_client.as_ref(), &content).await?;

        // Search with semantic understanding
        let (postman, testmo) = tokio::join!(
            self.search_postman_semantic(&semantic_query),
            self.search_testmo_semantic(&semantic_query)
        );

        Ok(SemanticSearchResult {
            postman_results: postman?,
            testmo_results: testmo?,
            search_type: "semantic".into(),
            query_used: semantic_query.keywords.join(", "),
        })
    }

    fn extract_ticket_content(&self, ticket: &JiraTicket) -> TicketContent {
        TicketContent {
            title: ticket.fields.summary.clone(),
            description: ticket.fields.description.clone().unwrap_or_default(),
            acceptance_criteria: self.extract_acceptance_criteria(&ticket.fields.description),
            labels: ticket.fields.labels.clone(),
            components: ticket.fields.components.iter()
                .map(|c| c.name.clone())
                .collect(),
        }
    }

    async fn generate_semantic_query(
        &self,
        ai_client: &dyn AIClient,
        content: &TicketContent,
    ) -> Result<SemanticQuery> {
        let prompt = format!(
            r#"Analyze this Jira ticket and extract search keywords for finding related API tests.

Title: {}
Description: {}
Acceptance Criteria: {}
Labels: {}

Return a JSON object with:
- keywords: array of relevant search terms (5-10 terms)
- entities: specific names/IDs mentioned (API endpoints, user types, etc.)
- test_types: suggested test types (e.g., "authentication", "validation", "error handling")

Be concise. Focus on testable aspects."#,
            content.title,
            content.description,
            content.acceptance_criteria.join("\n"),
            content.labels.join(", ")
        );

        let response = ai_client.complete(&prompt).await?;
        let query: SemanticQuery = serde_json::from_str(&response)?;
        
        Ok(query)
    }

    async fn search_postman_semantic(&self, query: &SemanticQuery) -> Result<Vec<PostmanMatch>> {
        // Combine keywords for search
        let search_terms: Vec<&str> = query.keywords.iter()
            .chain(query.entities.iter())
            .map(|s| s.as_str())
            .collect();

        let collections = self.postman_client
            .search_collections(&search_terms.join(" "))
            .await?;

        // Score relevance
        let mut matches: Vec<PostmanMatch> = collections.into_iter()
            .map(|col| {
                let score = self.calculate_relevance(&col.name, &query.keywords);
                let matched = self.find_matched_keywords(&col.name, &query.keywords);
                PostmanMatch {
                    collection: col,
                    relevance_score: score,
                    matched_keywords: matched,
                }
            })
            .collect();

        // Sort by relevance
        matches.sort_by(|a, b| b.relevance_score.partial_cmp(&a.relevance_score).unwrap());
        matches.truncate(10);

        Ok(matches)
    }

    fn calculate_relevance(&self, text: &str, keywords: &[String]) -> f64 {
        let text_lower = text.to_lowercase();
        let matched = keywords.iter()
            .filter(|kw| text_lower.contains(&kw.to_lowercase()))
            .count();
        
        if keywords.is_empty() {
            0.0
        } else {
            (matched as f64 / keywords.len() as f64) * 100.0
        }
    }

    async fn keyword_search(&self, ticket: &JiraTicket) -> Result<SemanticSearchResult> {
        // Simple keyword extraction (fallback)
        let keywords: Vec<String> = ticket.fields.summary
            .split_whitespace()
            .filter(|w| w.len() > 3)
            .map(|w| w.to_lowercase())
            .collect();

        let query = keywords.join(" ");

        let (postman, testmo) = tokio::join!(
            self.postman_client.search_collections(&query),
            self.testmo_client.search_cases(&query)
        );

        Ok(SemanticSearchResult {
            postman_results: postman?.into_iter()
                .map(|c| PostmanMatch {
                    collection: c,
                    relevance_score: 50.0, // Basic score for keyword
                    matched_keywords: keywords.clone(),
                })
                .collect(),
            testmo_results: testmo?.into_iter()
                .map(|c| TestmoMatch {
                    test_case: c,
                    relevance_score: 50.0,
                    matched_keywords: keywords.clone(),
                })
                .collect(),
            search_type: "keyword".into(),
            query_used: query,
        })
    }
}
```

### Frontend Integration

```tsx
// frontend/src/hooks/useContextualSearch.ts
export function useContextualSearch(ticketKey: string) {
  const { data: aiConfig } = useAIConfig();

  return useQuery({
    queryKey: ["contextual-search", ticketKey],
    queryFn: async () => {
      const response = await fetch(`/api/v1/search/contextual?ticketKey=${ticketKey}`);
      if (!response.ok) throw new Error("Search failed");
      return response.json() as Promise<SemanticSearchResult>;
    },
    staleTime: 5 * 60 * 1000, // 5 minutes
  });
}

// In search results display
export function SearchResultCard({ result }: { result: PostmanMatch | TestmoMatch }) {
  return (
    <div className="p-4 bg-white rounded-lg border border-neutral-200">
      <div className="flex items-center justify-between mb-2">
        <h4 className="font-medium">{result.collection?.name || result.testCase?.title}</h4>
        <span className={cn(
          "px-2 py-1 text-xs font-medium rounded-full",
          result.relevanceScore >= 80 ? "bg-success-100 text-success-700" :
          result.relevanceScore >= 50 ? "bg-warning-100 text-warning-700" :
          "bg-neutral-100 text-neutral-600"
        )}>
          {result.relevanceScore.toFixed(0)}% match
        </span>
      </div>
      
      {result.matchedKeywords.length > 0 && (
        <div className="flex flex-wrap gap-1">
          {result.matchedKeywords.map((kw) => (
            <span key={kw} className="px-2 py-0.5 text-xs bg-primary-50 text-primary-700 rounded">
              {kw}
            </span>
          ))}
        </div>
      )}
    </div>
  );
}
```

### References

- [Source: epics.md#Story 13.2]
- [FR: FR-AI-07 - Graceful fallback]
