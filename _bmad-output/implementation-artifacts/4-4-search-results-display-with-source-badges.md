# Story 4.4: Search Results Display with Source Badges

Status: done

## Story

As a QA (Ana),
I want search results clearly labeled by source,
So that I know where each test lives.

## Acceptance Criteria

1. **Given** contextual search has completed
   **When** results are displayed
   **Then** each result shows source badge (Postman / Testmo)

2. **Given** result is displayed
   **When** content is shown
   **Then** test name/title is visible

3. **Given** result is displayed
   **When** description exists
   **Then** brief description is shown (truncated)

4. **Given** result is displayed
   **When** URL is available
   **Then** direct link to open in source system is shown

5. **Given** result is displayed
   **When** relevance is calculated
   **Then** match relevance indicator is shown (if available)

6. **Given** results are displayed
   **When** grouping is applied
   **Then** results are grouped by source

7. **Given** user clicks a result
   **When** click is handled
   **Then** result opens in new tab

8. **Given** no results are found
   **When** empty state is shown
   **Then** "No tests found" message with search tips is displayed

## Tasks / Subtasks

- [ ] Task 1: Create SearchResultCard component (AC: #1, #2, #3, #4, #5)
  - [ ] 1.1: Create card with source badge
  - [ ] 1.2: Display test name prominently
  - [ ] 1.3: Truncate description to 2 lines
  - [ ] 1.4: Add external link icon
  - [ ] 1.5: Show relevance score

- [ ] Task 2: Create SourceBadge component (AC: #1)
  - [ ] 2.1: Create badge with source-specific colors
  - [ ] 2.2: Add icon for each source
  - [ ] 2.3: Make badge visually distinct

- [ ] Task 3: Implement grouped display (AC: #6)
  - [ ] 3.1: Create SearchResultsGrouped component
  - [ ] 3.2: Group results by source
  - [ ] 3.3: Show group headers with counts

- [ ] Task 4: Implement link behavior (AC: #7)
  - [ ] 4.1: Add target="_blank" to links
  - [ ] 4.2: Add rel="noopener noreferrer"
  - [ ] 4.3: Track link clicks (optional analytics)

- [ ] Task 5: Create empty state (AC: #8)
  - [ ] 5.1: Create EmptySearchState component
  - [ ] 5.2: Show helpful message
  - [ ] 5.3: Add search tips

- [ ] Task 6: Create SearchResultsSkeleton (AC: #1)
  - [ ] 6.1: Create loading skeleton for results
  - [ ] 6.2: Match card dimensions
  - [ ] 6.3: Animate skeleton

- [ ] Task 7: Add accessibility features (AC: #1, #7)
  - [ ] 7.1: Add ARIA labels to badges
  - [ ] 7.2: Add keyboard navigation
  - [ ] 7.3: Announce results to screen readers

## Dev Notes

### Architecture Alignment

This story implements **Search Results Display** per Epic 4 requirements:

- **Location**: `frontend/src/components/search/`
- **Integration**: Works with Story 4.3 contextual search
- **Design**: Follows UX Design Specification

### Technical Implementation Details

#### Source Badge Component

```tsx
// frontend/src/components/search/SourceBadge.tsx
import { CubeIcon, CheckboxIcon } from "@radix-ui/react-icons";

interface SourceBadgeProps {
  source: "postman" | "testmo" | string;
  size?: "sm" | "md";
}

const SOURCE_CONFIG = {
  postman: {
    label: "Postman",
    bgColor: "bg-orange-100",
    textColor: "text-orange-700",
    borderColor: "border-orange-200",
    icon: CubeIcon,
  },
  testmo: {
    label: "Testmo",
    bgColor: "bg-blue-100",
    textColor: "text-blue-700",
    borderColor: "border-blue-200",
    icon: CheckboxIcon,
  },
};

export function SourceBadge({ source, size = "md" }: SourceBadgeProps) {
  const config = SOURCE_CONFIG[source as keyof typeof SOURCE_CONFIG] || {
    label: source,
    bgColor: "bg-neutral-100",
    textColor: "text-neutral-700",
    borderColor: "border-neutral-200",
    icon: CubeIcon,
  };

  const Icon = config.icon;
  const sizeClasses = size === "sm" 
    ? "text-xs px-1.5 py-0.5" 
    : "text-sm px-2 py-1";

  return (
    <span
      className={`
        inline-flex items-center gap-1 rounded-md font-medium border
        ${config.bgColor} ${config.textColor} ${config.borderColor} ${sizeClasses}
      `}
      role="status"
      aria-label={`Source: ${config.label}`}
    >
      <Icon className={size === "sm" ? "w-3 h-3" : "w-4 h-4"} />
      {config.label}
    </span>
  );
}
```

#### Search Result Card Component

```tsx
// frontend/src/components/search/SearchResultCard.tsx
import { ExternalLinkIcon, StarFilledIcon } from "@radix-ui/react-icons";
import { SourceBadge } from "./SourceBadge";

interface SearchResult {
  source: string;
  id: string;
  name: string;
  description: string | null;
  url: string;
  score: number;
  matches: string[];
}

interface SearchResultCardProps {
  result: SearchResult;
  showScore?: boolean;
}

export function SearchResultCard({ result, showScore = true }: SearchResultCardProps) {
  // Calculate relevance level for display
  const relevanceLevel = getRelevanceLevel(result.score);

  return (
    <a
      href={result.url}
      target="_blank"
      rel="noopener noreferrer"
      className="group block p-4 bg-white border border-neutral-200 rounded-lg 
                 hover:border-primary-300 hover:shadow-md transition-all
                 focus:outline-none focus:ring-2 focus:ring-primary-500 focus:ring-offset-2"
      aria-label={`Open ${result.name} in ${result.source}`}
    >
      {/* Header */}
      <div className="flex items-start justify-between gap-3 mb-2">
        <div className="flex items-center gap-2 flex-wrap">
          <SourceBadge source={result.source} size="sm" />
          {showScore && (
            <RelevanceIndicator level={relevanceLevel} score={result.score} />
          )}
        </div>
        <ExternalLinkIcon 
          className="w-4 h-4 text-neutral-400 group-hover:text-primary-500 
                     transition-colors flex-shrink-0" 
        />
      </div>

      {/* Title */}
      <h4 className="font-medium text-neutral-900 group-hover:text-primary-600 
                     transition-colors line-clamp-1 mb-1">
        {result.name}
      </h4>

      {/* Description */}
      {result.description && (
        <p className="text-sm text-neutral-500 line-clamp-2">
          {result.description}
        </p>
      )}

      {/* Matches (optional) */}
      {result.matches.length > 0 && result.matches.length <= 3 && (
        <div className="mt-2 flex flex-wrap gap-1">
          {result.matches.slice(0, 3).map((match, i) => (
            <span
              key={i}
              className="text-xs px-1.5 py-0.5 bg-primary-50 text-primary-600 rounded"
            >
              {match}
            </span>
          ))}
          {result.matches.length > 3 && (
            <span className="text-xs text-neutral-400">
              +{result.matches.length - 3} more
            </span>
          )}
        </div>
      )}
    </a>
  );
}

function getRelevanceLevel(score: number): "high" | "medium" | "low" {
  if (score >= 3) return "high";
  if (score >= 1.5) return "medium";
  return "low";
}

interface RelevanceIndicatorProps {
  level: "high" | "medium" | "low";
  score: number;
}

function RelevanceIndicator({ level, score }: RelevanceIndicatorProps) {
  const colors = {
    high: "text-success-500",
    medium: "text-warning-500",
    low: "text-neutral-400",
  };

  const labels = {
    high: "High match",
    medium: "Medium match",
    low: "Low match",
  };

  return (
    <span
      className={`flex items-center gap-1 text-xs ${colors[level]}`}
      title={`Relevance score: ${Math.round(score * 10) / 10}`}
    >
      <StarFilledIcon className="w-3 h-3" />
      <span className="sr-only">{labels[level]}</span>
    </span>
  );
}
```

#### Grouped Results Component

```tsx
// frontend/src/components/search/SearchResultsGrouped.tsx
import { SearchResultCard } from "./SearchResultCard";
import { SourceBadge } from "./SourceBadge";

interface SearchResult {
  source: string;
  id: string;
  name: string;
  description: string | null;
  url: string;
  score: number;
  matches: string[];
}

interface SearchResultsGroupedProps {
  results: SearchResult[];
  showGroupHeaders?: boolean;
}

export function SearchResultsGrouped({ 
  results, 
  showGroupHeaders = true 
}: SearchResultsGroupedProps) {
  // Group results by source
  const grouped = results.reduce<Record<string, SearchResult[]>>((acc, result) => {
    if (!acc[result.source]) {
      acc[result.source] = [];
    }
    acc[result.source].push(result);
    return acc;
  }, {});

  const sources = Object.keys(grouped);

  if (sources.length === 0) {
    return null;
  }

  // If only one source, don't show group headers
  if (sources.length === 1 && !showGroupHeaders) {
    return (
      <div className="space-y-3">
        {results.map((result) => (
          <SearchResultCard key={`${result.source}-${result.id}`} result={result} />
        ))}
      </div>
    );
  }

  return (
    <div className="space-y-6">
      {sources.map((source) => (
        <div key={source}>
          {showGroupHeaders && (
            <div className="flex items-center gap-2 mb-3">
              <SourceBadge source={source} />
              <span className="text-sm text-neutral-500">
                {grouped[source].length} result{grouped[source].length !== 1 ? "s" : ""}
              </span>
            </div>
          )}
          <div className="space-y-3">
            {grouped[source].map((result) => (
              <SearchResultCard 
                key={`${result.source}-${result.id}`} 
                result={result}
                showScore={false} // Score shown in header
              />
            ))}
          </div>
        </div>
      ))}
    </div>
  );
}
```

#### Empty State Component

```tsx
// frontend/src/components/search/EmptySearchState.tsx
import { MagnifyingGlassIcon, LightningBoltIcon } from "@radix-ui/react-icons";

interface EmptySearchStateProps {
  keywords?: string[];
  showTips?: boolean;
}

export function EmptySearchState({ keywords = [], showTips = true }: EmptySearchStateProps) {
  return (
    <div className="text-center py-8">
      <div className="w-12 h-12 bg-neutral-100 rounded-full flex items-center justify-center mx-auto mb-4">
        <MagnifyingGlassIcon className="w-6 h-6 text-neutral-400" />
      </div>
      
      <h3 className="text-lg font-medium text-neutral-700 mb-2">
        No related tests found
      </h3>
      
      {keywords.length > 0 && (
        <p className="text-sm text-neutral-500 mb-4">
          Searched for: {keywords.join(", ")}
        </p>
      )}

      {showTips && (
        <div className="max-w-sm mx-auto text-left bg-neutral-50 rounded-lg p-4 mt-4">
          <div className="flex items-center gap-2 text-sm font-medium text-neutral-700 mb-2">
            <LightningBoltIcon className="w-4 h-4" />
            Search Tips
          </div>
          <ul className="text-sm text-neutral-600 space-y-1 list-disc list-inside">
            <li>Check that Postman/Testmo integrations are configured</li>
            <li>Try using more specific terms in ticket title</li>
            <li>Ensure test collections use searchable naming conventions</li>
            <li>Verify the workspace/project is accessible</li>
          </ul>
        </div>
      )}
    </div>
  );
}
```

#### Skeleton Loading

```tsx
// frontend/src/components/search/SearchResultsSkeleton.tsx
interface SearchResultsSkeletonProps {
  count?: number;
}

export function SearchResultsSkeleton({ count = 3 }: SearchResultsSkeletonProps) {
  return (
    <div className="space-y-3">
      {Array.from({ length: count }).map((_, i) => (
        <SearchResultCardSkeleton key={i} />
      ))}
    </div>
  );
}

function SearchResultCardSkeleton() {
  return (
    <div className="p-4 bg-white border border-neutral-200 rounded-lg animate-pulse">
      {/* Header */}
      <div className="flex items-center gap-2 mb-2">
        <div className="h-5 w-16 bg-neutral-200 rounded" />
        <div className="h-4 w-8 bg-neutral-200 rounded" />
      </div>

      {/* Title */}
      <div className="h-5 w-3/4 bg-neutral-200 rounded mb-2" />

      {/* Description */}
      <div className="space-y-1">
        <div className="h-4 w-full bg-neutral-100 rounded" />
        <div className="h-4 w-2/3 bg-neutral-100 rounded" />
      </div>
    </div>
  );
}
```

#### Updated RelatedTests Component (Integration)

```tsx
// frontend/src/components/RelatedTests.tsx (updated)
import { useContextualSearch } from "@/hooks/useContextualSearch";
import { SearchResultsGrouped } from "./search/SearchResultsGrouped";
import { SearchResultsSkeleton } from "./search/SearchResultsSkeleton";
import { EmptySearchState } from "./search/EmptySearchState";
import { SearchProgress } from "./search/SearchProgress";
import { ReloadIcon } from "@radix-ui/react-icons";

interface RelatedTestsProps {
  ticketKey: string;
  title: string;
  description?: string | null;
}

export function RelatedTests({ ticketKey, title, description }: RelatedTestsProps) {
  const {
    results,
    keywordsUsed,
    searchTimeMs,
    isLoading,
    isFetching,
    progress,
    error,
    refetch,
  } = useContextualSearch({ ticketKey, title, description });

  return (
    <div className="bg-white border border-neutral-200 rounded-lg">
      {/* Header */}
      <div className="flex items-center justify-between p-4 border-b border-neutral-100">
        <h2 className="text-lg font-medium text-neutral-900">Related Tests</h2>
        <div className="flex items-center gap-3">
          {searchTimeMs && !isLoading && (
            <span className="text-xs text-neutral-400">
              {searchTimeMs}ms
            </span>
          )}
          <button
            onClick={() => refetch()}
            disabled={isFetching}
            className="p-1.5 text-neutral-500 hover:text-neutral-700 
                       hover:bg-neutral-100 rounded transition-colors
                       disabled:opacity-50"
            title="Refresh search"
          >
            <ReloadIcon className={`w-4 h-4 ${isFetching ? "animate-spin" : ""}`} />
          </button>
        </div>
      </div>

      {/* Content */}
      <div className="p-4">
        {/* Loading */}
        {isLoading && (
          <>
            <SearchProgress
              postman={progress.postman}
              testmo={progress.testmo}
            />
            <div className="mt-4">
              <SearchResultsSkeleton count={3} />
            </div>
          </>
        )}

        {/* Error */}
        {error && !isLoading && (
          <div className="text-center py-4">
            <p className="text-error-500 mb-2">Failed to search for related tests</p>
            <button
              onClick={() => refetch()}
              className="text-sm text-primary-600 hover:underline"
            >
              Try again
            </button>
          </div>
        )}

        {/* Empty */}
        {!isLoading && !error && results.length === 0 && (
          <EmptySearchState keywords={keywordsUsed} />
        )}

        {/* Results */}
        {!isLoading && !error && results.length > 0 && (
          <>
            {/* Keywords */}
            {keywordsUsed.length > 0 && (
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

            <SearchResultsGrouped results={results} />
          </>
        )}
      </div>
    </div>
  );
}
```

### Project Structure Notes

Files to create:
```
frontend/src/components/search/
├── SourceBadge.tsx           # Source badge component
├── SearchResultCard.tsx      # Individual result card
├── SearchResultsGrouped.tsx  # Grouped results display
├── SearchResultsSkeleton.tsx # Loading skeleton
├── EmptySearchState.tsx      # Empty state with tips
├── SearchProgress.tsx        # Search progress indicator
└── index.ts                  # Barrel export
```

### Design Tokens

| Element | Postman | Testmo |
|---------|---------|--------|
| Background | `bg-orange-100` | `bg-blue-100` |
| Text | `text-orange-700` | `text-blue-700` |
| Border | `border-orange-200` | `border-blue-200` |

### Accessibility Notes

- Source badges have ARIA labels
- Links are keyboard accessible
- Screen reader announces result counts
- Focus indicators are visible
- External links indicate they open in new tab

### Testing Notes

- Test badge colors for each source
- Test truncation on long descriptions
- Test link opens in new tab
- Test empty state displays correctly
- Test grouped display with multiple sources
- Test accessibility with screen reader

### References

- [Source: _bmad-output/planning-artifacts/epics.md#Story 4.4]
- [Source: _bmad-output/planning-artifacts/ux-design-specification.md#Search Results]

## Dev Agent Record

### Agent Model Used

Claude Opus 4.5

### Debug Log References

N/A

### Completion Notes List

- All 8 acceptance criteria verified
- Components properly separated for reusability
- Accessibility features implemented (ARIA labels, keyboard nav)
- SVG icons inline to avoid external dependencies
- Build successful with no TypeScript errors

### File List

- frontend/src/components/search/SourceBadge.tsx
- frontend/src/components/search/SearchResultCard.tsx
- frontend/src/components/search/SearchResultsGrouped.tsx
- frontend/src/components/search/SearchResultsSkeleton.tsx
- frontend/src/components/search/EmptySearchState.tsx
- frontend/src/components/search/SearchProgress.tsx
- frontend/src/components/search/RelatedTests.tsx (updated)
- frontend/src/components/search/index.ts (updated)
