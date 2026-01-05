# Story 4.3: Contextual Search on Ticket Selection

Status: done

## Story

As a QA (Ana),
I want automatic search when I select a ticket,
So that related tests appear without manual searching.

## Acceptance Criteria

1. **Given** user selects a Jira ticket
   **When** the ticket detail view loads
   **Then** the framework automatically extracts keywords from ticket title and description

2. **Given** keywords are extracted
   **When** search is triggered
   **Then** Postman collections are searched for matches

3. **Given** keywords are extracted
   **When** search is triggered
   **Then** Testmo test cases are searched for matches

4. **Given** search completes
   **When** results are available
   **Then** results are displayed in "Related Tests" section

5. **Given** search is initiated
   **When** both systems are queried
   **Then** search runs in parallel across both systems

6. **Given** search is executed
   **When** performance is measured
   **Then** search completes in < 3s (NFR-PERF-03)

7. **Given** search is in progress
   **When** UI is displayed
   **Then** loading state shows "Searching Postman... Searching Testmo..."

## Tasks / Subtasks

- [x] Task 1: Create keyword extraction service (AC: #1)
  - [x] 1.1: Create `KeywordExtractor` in `qa-pms-core`
  - [x] 1.2: Extract keywords from ticket title
  - [x] 1.3: Extract keywords from description
  - [x] 1.4: Filter common stop words
  - [x] 1.5: Deduplicate and rank by frequency

- [x] Task 2: Create unified search service (AC: #2, #3, #5)
  - [x] 2.1: Create `SearchService` in `qa-pms-api`
  - [x] 2.2: Accept keywords and optional ticket context
  - [x] 2.3: Run Postman and Testmo searches in parallel
  - [x] 2.4: Merge and sort results by score

- [x] Task 3: Create search trigger hook (AC: #1)
  - [x] 3.1: Create `useContextualSearch` hook
  - [x] 3.2: Trigger on ticket selection
  - [x] 3.3: Debounce to prevent excessive calls

- [x] Task 4: Create Related Tests section (AC: #4)
  - [x] 4.1: Create `RelatedTests.tsx` component
  - [x] 4.2: Display search results grouped by source
  - [x] 4.3: Show match relevance

- [x] Task 5: Implement loading states (AC: #7)
  - [x] 5.1: Create `SearchProgress.tsx` component
  - [x] 5.2: Show individual service status
  - [x] 5.3: Update as each service completes

- [x] Task 6: Add performance monitoring (AC: #6)
  - [x] 6.1: Add timing metrics to search
  - [x] 6.2: Log slow searches (> 3s)
  - [x] 6.3: Show search duration in dev mode

- [x] Task 7: Integrate into ticket detail page (AC: #4)
  - [x] 7.1: Add RelatedTests to TicketDetailPage
  - [x] 7.2: Pass ticket data for keyword extraction
  - [x] 7.3: Handle error states gracefully

## Dev Notes

### Architecture Alignment

This story implements **Contextual Search** per Epic 4 requirements:

- **Backend**: Unified search service with parallel execution
- **Frontend**: Auto-search on ticket selection
- **Performance**: < 3s total search time

### Technical Implementation Details

#### Keyword Extraction

```rust
// crates/qa-pms-core/src/keywords.rs
use std::collections::HashMap;

const STOP_WORDS: &[&str] = &[
    "a", "an", "the", "is", "are", "was", "were", "be", "been", "being",
    "have", "has", "had", "do", "does", "did", "will", "would", "could",
    "should", "may", "might", "must", "shall", "can", "need", "dare",
    "to", "of", "in", "for", "on", "with", "at", "by", "from", "as",
    "into", "through", "during", "before", "after", "above", "below",
    "up", "down", "out", "off", "over", "under", "again", "further",
    "then", "once", "here", "there", "when", "where", "why", "how",
    "all", "each", "every", "both", "few", "more", "most", "other",
    "some", "such", "no", "not", "only", "own", "same", "so", "than",
    "too", "very", "just", "and", "but", "if", "or", "because", "until",
    "while", "this", "that", "these", "those", "it", "its",
    // QA-specific common words to filter
    "test", "testing", "tests", "qa", "bug", "issue", "ticket",
];

pub struct KeywordExtractor {
    min_length: usize,
    max_keywords: usize,
}

impl Default for KeywordExtractor {
    fn default() -> Self {
        Self {
            min_length: 3,
            max_keywords: 10,
        }
    }
}

impl KeywordExtractor {
    pub fn new(min_length: usize, max_keywords: usize) -> Self {
        Self { min_length, max_keywords }
    }

    /// Extract keywords from text
    pub fn extract(&self, texts: &[&str]) -> Vec<String> {
        let mut word_counts: HashMap<String, usize> = HashMap::new();
        
        for text in texts {
            for word in self.tokenize(text) {
                if self.is_valid_keyword(&word) {
                    *word_counts.entry(word).or_insert(0) += 1;
                }
            }
        }

        // Sort by frequency and take top keywords
        let mut keywords: Vec<_> = word_counts.into_iter().collect();
        keywords.sort_by(|a, b| b.1.cmp(&a.1));
        
        keywords
            .into_iter()
            .take(self.max_keywords)
            .map(|(word, _)| word)
            .collect()
    }

    fn tokenize(&self, text: &str) -> Vec<String> {
        text.to_lowercase()
            .split(|c: char| !c.is_alphanumeric() && c != '-' && c != '_')
            .filter(|s| !s.is_empty())
            .map(String::from)
            .collect()
    }

    fn is_valid_keyword(&self, word: &str) -> bool {
        word.len() >= self.min_length
            && !STOP_WORDS.contains(&word.as_str())
            && !word.chars().all(|c| c.is_numeric())
    }

    /// Extract from ticket
    pub fn extract_from_ticket(&self, title: &str, description: Option<&str>) -> Vec<String> {
        let mut texts = vec![title];
        if let Some(desc) = description {
            texts.push(desc);
        }
        self.extract(&texts)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_keyword_extraction() {
        let extractor = KeywordExtractor::default();
        let keywords = extractor.extract_from_ticket(
            "Login authentication fails with invalid credentials",
            Some("When user enters wrong password, the system shows generic error")
        );
        
        assert!(keywords.contains(&"login".to_string()));
        assert!(keywords.contains(&"authentication".to_string()));
        assert!(keywords.contains(&"password".to_string()));
        assert!(!keywords.contains(&"the".to_string())); // Stop word
    }
}
```

#### Unified Search Service

```rust
// crates/qa-pms-api/src/services/search.rs
use qa_pms_core::keywords::KeywordExtractor;
use qa_pms_postman::{PostmanClient, SearchResult as PostmanSearchResult};
use qa_pms_testmo::{TestmoClient, SearchResult as TestmoSearchResult};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::time::Instant;
use tokio::try_join;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UnifiedSearchResult {
    pub source: String,
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub url: String,
    pub score: f32,
    pub matches: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchResponse {
    pub results: Vec<UnifiedSearchResult>,
    pub postman_count: usize,
    pub testmo_count: usize,
    pub search_time_ms: u64,
    pub keywords_used: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchProgress {
    pub postman: SearchStatus,
    pub testmo: SearchStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SearchStatus {
    Pending,
    Searching,
    Complete,
    Error,
    Skipped,
}

pub struct SearchService {
    postman_client: Option<Arc<PostmanClient>>,
    testmo_client: Option<Arc<TestmoClient>>,
    testmo_project_id: Option<i64>,
    keyword_extractor: KeywordExtractor,
}

impl SearchService {
    pub fn new(
        postman_client: Option<Arc<PostmanClient>>,
        testmo_client: Option<Arc<TestmoClient>>,
        testmo_project_id: Option<i64>,
    ) -> Self {
        Self {
            postman_client,
            testmo_client,
            testmo_project_id,
            keyword_extractor: KeywordExtractor::default(),
        }
    }

    /// Search both Postman and Testmo in parallel
    pub async fn search_all(
        &self,
        title: &str,
        description: Option<&str>,
    ) -> SearchResponse {
        let start = Instant::now();
        
        // Extract keywords
        let keywords = self.keyword_extractor.extract_from_ticket(title, description);
        
        if keywords.is_empty() {
            return SearchResponse {
                results: vec![],
                postman_count: 0,
                testmo_count: 0,
                search_time_ms: start.elapsed().as_millis() as u64,
                keywords_used: keywords,
            };
        }

        // Run searches in parallel
        let postman_future = self.search_postman(&keywords);
        let testmo_future = self.search_testmo(&keywords);
        
        let (postman_results, testmo_results) = tokio::join!(
            postman_future,
            testmo_future
        );

        let mut all_results: Vec<UnifiedSearchResult> = Vec::new();
        
        // Collect Postman results
        let postman_count = if let Ok(results) = postman_results {
            let count = results.len();
            all_results.extend(results.into_iter().map(|r| UnifiedSearchResult {
                source: r.source,
                id: r.id,
                name: r.name,
                description: r.description,
                url: r.url,
                score: r.score,
                matches: r.matches,
            }));
            count
        } else {
            0
        };

        // Collect Testmo results
        let testmo_count = if let Ok(results) = testmo_results {
            let count = results.len();
            all_results.extend(results.into_iter().map(|r| UnifiedSearchResult {
                source: r.source,
                id: r.id,
                name: r.name,
                description: r.description,
                url: r.url,
                score: r.score,
                matches: r.matches,
            }));
            count
        } else {
            0
        };

        // Sort all results by score
        all_results.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap());

        let duration = start.elapsed();
        if duration.as_secs() > 3 {
            tracing::warn!("Slow contextual search: {:?}", duration);
        }

        SearchResponse {
            results: all_results,
            postman_count,
            testmo_count,
            search_time_ms: duration.as_millis() as u64,
            keywords_used: keywords,
        }
    }

    async fn search_postman(&self, keywords: &[String]) -> Result<Vec<PostmanSearchResult>, ()> {
        if let Some(client) = &self.postman_client {
            client
                .search_collections(keywords, None)
                .await
                .map_err(|e| {
                    tracing::error!("Postman search failed: {}", e);
                })
        } else {
            Ok(vec![])
        }
    }

    async fn search_testmo(&self, keywords: &[String]) -> Result<Vec<TestmoSearchResult>, ()> {
        match (&self.testmo_client, self.testmo_project_id) {
            (Some(client), Some(project_id)) => {
                client
                    .search_test_cases(project_id, keywords)
                    .await
                    .map_err(|e| {
                        tracing::error!("Testmo search failed: {}", e);
                    })
            }
            _ => Ok(vec![]),
        }
    }
}
```

#### Frontend Hook

```tsx
// frontend/src/hooks/useContextualSearch.ts
import { useState, useEffect, useCallback } from "react";
import { useQuery } from "@tanstack/react-query";
import { useDebouncedValue } from "./useDebouncedValue";

interface SearchResult {
  source: string;
  id: string;
  name: string;
  description: string | null;
  url: string;
  score: number;
  matches: string[];
}

interface SearchResponse {
  results: SearchResult[];
  postmanCount: number;
  testmoCount: number;
  searchTimeMs: number;
  keywordsUsed: string[];
}

interface SearchProgress {
  postman: "pending" | "searching" | "complete" | "error" | "skipped";
  testmo: "pending" | "searching" | "complete" | "error" | "skipped";
}

interface UseContextualSearchOptions {
  ticketKey: string;
  title: string;
  description?: string | null;
  enabled?: boolean;
}

export function useContextualSearch({
  ticketKey,
  title,
  description,
  enabled = true,
}: UseContextualSearchOptions) {
  const [progress, setProgress] = useState<SearchProgress>({
    postman: "pending",
    testmo: "pending",
  });

  // Debounce to prevent excessive API calls
  const debouncedTitle = useDebouncedValue(title, 300);

  const searchFn = useCallback(async (): Promise<SearchResponse> => {
    setProgress({ postman: "searching", testmo: "searching" });

    const response = await fetch("/api/v1/search/all", {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({
        ticketKey,
        title: debouncedTitle,
        description: description || undefined,
      }),
    });

    if (!response.ok) {
      throw new Error("Search failed");
    }

    const data: SearchResponse = await response.json();

    setProgress({
      postman: data.postmanCount > 0 ? "complete" : "complete",
      testmo: data.testmoCount > 0 ? "complete" : "complete",
    });

    return data;
  }, [ticketKey, debouncedTitle, description]);

  const query = useQuery({
    queryKey: ["contextualSearch", ticketKey, debouncedTitle],
    queryFn: searchFn,
    enabled: enabled && !!debouncedTitle,
    staleTime: 5 * 60 * 1000, // Cache for 5 minutes
  });

  // Reset progress when search starts
  useEffect(() => {
    if (query.isFetching) {
      setProgress({ postman: "searching", testmo: "searching" });
    }
  }, [query.isFetching]);

  return {
    results: query.data?.results || [],
    postmanCount: query.data?.postmanCount || 0,
    testmoCount: query.data?.testmoCount || 0,
    searchTimeMs: query.data?.searchTimeMs,
    keywordsUsed: query.data?.keywordsUsed || [],
    isLoading: query.isLoading,
    isFetching: query.isFetching,
    error: query.error,
    progress,
  };
}
```

#### Related Tests Component

```tsx
// frontend/src/components/RelatedTests.tsx
import { ExternalLinkIcon, MagnifyingGlassIcon } from "@radix-ui/react-icons";
import { useContextualSearch } from "@/hooks/useContextualSearch";

interface RelatedTestsProps {
  ticketKey: string;
  title: string;
  description?: string | null;
}

export function RelatedTests({ ticketKey, title, description }: RelatedTestsProps) {
  const {
    results,
    postmanCount,
    testmoCount,
    keywordsUsed,
    searchTimeMs,
    isLoading,
    progress,
    error,
  } = useContextualSearch({ ticketKey, title, description });

  return (
    <div className="bg-white border border-neutral-200 rounded-lg p-6">
      <div className="flex items-center justify-between mb-4">
        <h2 className="text-lg font-medium text-neutral-900">Related Tests</h2>
        {searchTimeMs && (
          <span className="text-xs text-neutral-400">
            Found in {searchTimeMs}ms
          </span>
        )}
      </div>

      {/* Search Progress */}
      {isLoading && (
        <div className="space-y-2 mb-4">
          <SearchProgressItem
            label="Postman"
            status={progress.postman}
          />
          <SearchProgressItem
            label="Testmo"
            status={progress.testmo}
          />
        </div>
      )}

      {/* Keywords Used */}
      {keywordsUsed.length > 0 && !isLoading && (
        <div className="flex flex-wrap gap-1 mb-4">
          <span className="text-xs text-neutral-500">Keywords:</span>
          {keywordsUsed.map((keyword) => (
            <span
              key={keyword}
              className="text-xs px-2 py-0.5 bg-neutral-100 text-neutral-600 rounded"
            >
              {keyword}
            </span>
          ))}
        </div>
      )}

      {/* Results */}
      {!isLoading && results.length === 0 && !error && (
        <EmptySearchResults />
      )}

      {error && (
        <div className="text-center py-4 text-error-500">
          Failed to search for related tests
        </div>
      )}

      {results.length > 0 && (
        <div className="space-y-3">
          {results.map((result) => (
            <SearchResultCard key={`${result.source}-${result.id}`} result={result} />
          ))}
        </div>
      )}

      {/* Summary */}
      {!isLoading && results.length > 0 && (
        <div className="mt-4 pt-4 border-t border-neutral-100 text-sm text-neutral-500">
          Found {postmanCount} in Postman, {testmoCount} in Testmo
        </div>
      )}
    </div>
  );
}

function SearchProgressItem({ label, status }: { label: string; status: string }) {
  const isSearching = status === "searching";
  const isComplete = status === "complete";

  return (
    <div className="flex items-center gap-2 text-sm">
      {isSearching && (
        <div className="w-4 h-4 border-2 border-primary-200 border-t-primary-500 rounded-full animate-spin" />
      )}
      {isComplete && (
        <div className="w-4 h-4 bg-success-500 rounded-full" />
      )}
      {!isSearching && !isComplete && (
        <div className="w-4 h-4 bg-neutral-200 rounded-full" />
      )}
      <span className={isSearching ? "text-neutral-700" : "text-neutral-500"}>
        {isSearching ? `Searching ${label}...` : label}
      </span>
    </div>
  );
}

function SearchResultCard({ result }: { result: SearchResult }) {
  const sourceColors = {
    postman: "bg-orange-100 text-orange-700",
    testmo: "bg-blue-100 text-blue-700",
  };

  return (
    <a
      href={result.url}
      target="_blank"
      rel="noopener noreferrer"
      className="block p-3 border border-neutral-200 rounded-lg hover:border-primary-300 
                 hover:shadow-sm transition-all"
    >
      <div className="flex items-start justify-between gap-2">
        <div className="flex-1 min-w-0">
          <div className="flex items-center gap-2 mb-1">
            <span className={`text-xs px-2 py-0.5 rounded font-medium ${sourceColors[result.source as keyof typeof sourceColors] || "bg-neutral-100 text-neutral-700"}`}>
              {result.source}
            </span>
            <span className="text-xs text-neutral-400">
              {Math.round(result.score * 10) / 10} match
            </span>
          </div>
          <h4 className="font-medium text-neutral-900 truncate">
            {result.name}
          </h4>
          {result.description && (
            <p className="text-sm text-neutral-500 line-clamp-2 mt-1">
              {result.description}
            </p>
          )}
        </div>
        <ExternalLinkIcon className="w-4 h-4 text-neutral-400 flex-shrink-0" />
      </div>
    </a>
  );
}

function EmptySearchResults() {
  return (
    <div className="text-center py-8">
      <MagnifyingGlassIcon className="w-8 h-8 text-neutral-300 mx-auto mb-2" />
      <p className="text-neutral-500 mb-2">No related tests found</p>
      <p className="text-sm text-neutral-400">
        Try searching manually with different keywords
      </p>
    </div>
  );
}

interface SearchResult {
  source: string;
  id: string;
  name: string;
  description: string | null;
  url: string;
  score: number;
  matches: string[];
}
```

### Project Structure Notes

Files to create:
```
crates/qa-pms-core/src/
└── keywords.rs          # KeywordExtractor

crates/qa-pms-api/src/services/
└── search.rs            # SearchService

frontend/src/
├── hooks/
│   ├── useContextualSearch.ts
│   └── useDebouncedValue.ts
└── components/
    └── RelatedTests.tsx
```

### Testing Notes

- Unit test keyword extraction with various inputs
- Unit test stop word filtering
- Test parallel search execution
- Test search result merging and sorting
- Performance test: Verify < 3s search time

### References

- [Source: _bmad-output/planning-artifacts/epics.md#Story 4.3]
- [Source: _bmad-output/planning-artifacts/prd.md#NFR-PERF-03]

## Dev Agent Record

### Agent Model Used

Claude Opus 4.5

### Debug Log References

N/A

### Completion Notes List

1. Created `KeywordExtractor` in `qa-pms-core/src/keywords.rs`:
   - Stop word filtering (100+ common words including QA-specific terms)
   - Minimum word length 3, maximum 10 keywords
   - Frequency-based ranking
   - Handles hyphenated and underscored words
   - Filters numbers and version patterns

2. Created search API endpoint `POST /api/v1/search/contextual`:
   - Extracts keywords from ticket title/description
   - Runs Postman and Testmo searches in parallel with `tokio::join!`
   - Returns unified results sorted by score
   - Logs slow searches (>3s) per NFR-PERF-03

3. Created frontend hooks:
   - `useDebouncedValue` - Generic debounce hook (300ms)
   - `useContextualSearch` - Auto-search on ticket selection with React Query

4. Created `RelatedTests` component:
   - Progress indicators for each integration
   - Keywords display
   - Search results with source badges and scores
   - Empty and error states
   - Refresh button

5. Integrated into `TicketDetailPage.tsx`

6. Updated `TestmoSettings` to include `project_id` field

### File List

- `crates/qa-pms-core/src/keywords.rs` - KeywordExtractor
- `crates/qa-pms-core/src/lib.rs` - Module export
- `crates/qa-pms-api/src/routes/search.rs` - Search API endpoint
- `crates/qa-pms-api/src/routes/mod.rs` - Route registration
- `crates/qa-pms-api/src/app.rs` - Router merge
- `crates/qa-pms-config/src/settings.rs` - TestmoSettings update
- `frontend/src/hooks/useDebouncedValue.ts` - Debounce hook
- `frontend/src/hooks/useContextualSearch.ts` - Search hook
- `frontend/src/hooks/index.ts` - Hook exports
- `frontend/src/components/search/RelatedTests.tsx` - UI component
- `frontend/src/components/search/index.ts` - Component export
- `frontend/src/pages/Tickets/TicketDetailPage.tsx` - Integration
