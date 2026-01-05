# Story 9.5: Pattern History Tracking

Status: done

## Story

As a PM (Carlos),
I want to see history of detected patterns,
So that I can identify recurring issues.

## Acceptance Criteria

1. **Given** patterns have been detected over time
   **When** user views pattern history
   **Then** list of all patterns detected is shown

2. **Given** pattern list is displayed
   **When** filters available
   **Then** date range filter works

3. **Given** pattern list is displayed
   **When** filters available
   **Then** pattern type filter works

4. **Given** pattern is displayed
   **When** status shown
   **Then** resolution status is visible (addressed / ignored / recurring)

5. **Given** pattern exists
   **When** clicked
   **Then** clicking pattern shows full details

6. **Given** pattern detail is shown
   **When** user resolves
   **Then** can mark patterns as "resolved" with notes

7. **Given** patterns exist
   **When** recurring detected
   **Then** recurring patterns are highlighted

## Tasks

- [ ] Task 1: Create PatternHistoryPage component
- [ ] Task 2: Create PatternFilters component
- [ ] Task 3: Create PatternList component
- [ ] Task 4: Create PatternDetailDialog
- [ ] Task 5: Implement resolve pattern functionality
- [ ] Task 6: Detect and highlight recurring patterns
- [ ] Task 7: Create pattern analytics

## Dev Notes

### Pattern History API

```rust
// GET /api/v1/patterns
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PatternHistoryQuery {
    pub start_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
    pub pattern_type: Option<String>,
    pub resolved: Option<bool>,
    pub page: Option<i32>,
    pub per_page: Option<i32>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PatternHistoryResponse {
    pub patterns: Vec<PatternWithRecurrence>,
    pub total: i64,
    pub page: i32,
    pub total_pages: i32,
    pub summary: PatternSummary,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PatternWithRecurrence {
    #[serde(flatten)]
    pub pattern: DetectedPattern,
    pub is_recurring: bool,
    pub recurrence_count: i32,
    pub last_occurred: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PatternSummary {
    pub total_detected: i64,
    pub resolved: i64,
    pub unresolved: i64,
    pub recurring: i64,
    pub by_type: Vec<TypeCount>,
    pub by_severity: Vec<SeverityCount>,
}

// PUT /api/v1/patterns/:id/resolve
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResolvePatternRequest {
    pub resolution_notes: String,
}
```

### Pattern History Service

```rust
impl PatternService {
    pub async fn get_pattern_history(
        &self,
        query: &PatternHistoryQuery,
    ) -> Result<PatternHistoryResponse> {
        let page = query.page.unwrap_or(1).max(1);
        let per_page = query.per_page.unwrap_or(20).clamp(1, 100);
        let offset = (page - 1) * per_page;

        // Build query with filters
        let patterns = sqlx::query_as::<_, DetectedPattern>(
            r#"
            SELECT * FROM detected_patterns
            WHERE ($1::DATE IS NULL OR DATE(created_at) >= $1)
              AND ($2::DATE IS NULL OR DATE(created_at) <= $2)
              AND ($3::TEXT IS NULL OR pattern_type::TEXT = $3)
              AND ($4::BOOLEAN IS NULL OR resolved = $4)
            ORDER BY created_at DESC
            LIMIT $5 OFFSET $6
            "#,
        )
        .bind(query.start_date)
        .bind(query.end_date)
        .bind(&query.pattern_type)
        .bind(query.resolved)
        .bind(per_page)
        .bind(offset)
        .fetch_all(&self.pool)
        .await?;

        // Check for recurring patterns
        let patterns_with_recurrence = self.check_recurrence(&patterns).await?;

        // Get totals
        let total: i64 = sqlx::query_scalar(/* count query */).await?;
        
        // Get summary
        let summary = self.get_pattern_summary(query).await?;

        Ok(PatternHistoryResponse {
            patterns: patterns_with_recurrence,
            total,
            page,
            total_pages: ((total as f64) / (per_page as f64)).ceil() as i32,
            summary,
        })
    }

    async fn check_recurrence(
        &self,
        patterns: &[DetectedPattern],
    ) -> Result<Vec<PatternWithRecurrence>> {
        let mut result = Vec::new();

        for pattern in patterns {
            // Check if similar pattern occurred before
            let (count, last): (i64, Option<DateTime<Utc>>) = sqlx::query_as(
                r#"
                SELECT COUNT(*), MAX(created_at)
                FROM detected_patterns
                WHERE pattern_type = $1
                  AND id != $2
                  AND (
                    -- Same tickets affected
                    affected_ticket_ids && $3
                    OR
                    -- Same keywords in description
                    details->>'description' ILIKE '%' || $4 || '%'
                  )
                  AND created_at < $5
                "#,
            )
            .bind(&pattern.pattern_type)
            .bind(pattern.id)
            .bind(&pattern.affected_ticket_ids)
            .bind(self.extract_key_phrase(&pattern.details.description))
            .bind(pattern.created_at)
            .fetch_one(&self.pool)
            .await?;

            result.push(PatternWithRecurrence {
                pattern: pattern.clone(),
                is_recurring: count > 0,
                recurrence_count: count as i32,
                last_occurred: last,
            });
        }

        Ok(result)
    }

    pub async fn resolve_pattern(
        &self,
        pattern_id: Uuid,
        notes: &str,
    ) -> Result<DetectedPattern> {
        sqlx::query_as::<_, DetectedPattern>(
            r#"
            UPDATE detected_patterns
            SET resolved = true,
                resolved_at = NOW(),
                resolution_notes = $2
            WHERE id = $1
            RETURNING *
            "#,
        )
        .bind(pattern_id)
        .bind(notes)
        .fetch_one(&self.pool)
        .await
        .map_err(Into::into)
    }
}
```

### Pattern History Page

```tsx
// frontend/src/pages/PatternHistory.tsx
import { useState } from "react";
import { usePatternHistory } from "@/hooks/usePatterns";
import { PatternFilters } from "@/components/patterns/PatternFilters";
import { PatternList } from "@/components/patterns/PatternList";
import { PatternSummaryCards } from "@/components/patterns/PatternSummaryCards";

export function PatternHistoryPage() {
  const [filters, setFilters] = useState<PatternFilters>({});
  const [page, setPage] = useState(1);
  const { data, isLoading } = usePatternHistory({ ...filters, page });

  return (
    <div className="p-6 space-y-6">
      <div className="flex items-center justify-between">
        <div>
          <h1 className="text-2xl font-bold">Pattern History</h1>
          <p className="text-sm text-neutral-500">
            Track and analyze detected patterns over time
          </p>
        </div>
      </div>

      {/* Summary Cards */}
      <PatternSummaryCards summary={data?.summary} isLoading={isLoading} />

      {/* Filters */}
      <PatternFilters value={filters} onChange={setFilters} />

      {/* Pattern List */}
      <PatternList
        patterns={data?.patterns}
        isLoading={isLoading}
        onPatternClick={(id) => setSelectedPattern(id)}
      />

      {/* Pagination */}
      {data && data.totalPages > 1 && (
        <Pagination
          currentPage={page}
          totalPages={data.totalPages}
          onPageChange={setPage}
        />
      )}
    </div>
  );
}
```

### Pattern List Component

```tsx
// frontend/src/components/patterns/PatternList.tsx
interface PatternListProps {
  patterns?: PatternWithRecurrence[];
  isLoading: boolean;
  onPatternClick: (id: string) => void;
}

export function PatternList({ patterns, isLoading, onPatternClick }: PatternListProps) {
  if (isLoading) {
    return <PatternListSkeleton />;
  }

  return (
    <div className="bg-white rounded-xl border border-neutral-200 overflow-hidden">
      <div className="divide-y divide-neutral-100">
        {patterns?.map((item) => (
          <button
            key={item.pattern.id}
            onClick={() => onPatternClick(item.pattern.id)}
            className="w-full text-left p-4 hover:bg-neutral-50 transition-colors"
          >
            <div className="flex items-start justify-between">
              <div className="flex items-start gap-3">
                <PatternTypeIcon type={item.pattern.patternType} />
                <div>
                  <div className="flex items-center gap-2">
                    <h4 className="font-medium">{item.pattern.details.description}</h4>
                    {item.isRecurring && (
                      <span className="px-2 py-0.5 text-xs bg-warning-100 text-warning-700 rounded-full">
                        Recurring ({item.recurrenceCount}x)
                      </span>
                    )}
                  </div>
                  <div className="flex items-center gap-2 mt-1 text-sm text-neutral-500">
                    <SeverityBadge severity={item.pattern.severity} />
                    <span>•</span>
                    <span>{item.pattern.affectedTicketIds.length} tickets</span>
                    <span>•</span>
                    <span>
                      {formatDistanceToNow(new Date(item.pattern.createdAt), { addSuffix: true })}
                    </span>
                  </div>
                </div>
              </div>

              <div className="flex items-center gap-2">
                {item.pattern.resolved ? (
                  <span className="px-2 py-1 text-xs bg-success-100 text-success-700 rounded">
                    Resolved
                  </span>
                ) : (
                  <span className="px-2 py-1 text-xs bg-neutral-100 text-neutral-600 rounded">
                    Unresolved
                  </span>
                )}
                <ChevronRightIcon className="w-5 h-5 text-neutral-400" />
              </div>
            </div>
          </button>
        ))}
      </div>
    </div>
  );
}
```

### Resolve Pattern Dialog

```tsx
// frontend/src/components/patterns/ResolvePatternDialog.tsx
interface ResolvePatternDialogProps {
  pattern: DetectedPattern;
  open: boolean;
  onOpenChange: (open: boolean) => void;
}

export function ResolvePatternDialog({
  pattern,
  open,
  onOpenChange,
}: ResolvePatternDialogProps) {
  const [notes, setNotes] = useState("");
  const { mutate: resolve, isPending } = useResolvePattern();

  const handleResolve = () => {
    resolve(
      { patternId: pattern.id, notes },
      {
        onSuccess: () => {
          onOpenChange(false);
          toast.success("Pattern marked as resolved");
        },
      }
    );
  };

  return (
    <Dialog.Root open={open} onOpenChange={onOpenChange}>
      <Dialog.Portal>
        <Dialog.Overlay className="fixed inset-0 bg-black/50 z-50" />
        <Dialog.Content className="fixed top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2 bg-white rounded-xl p-6 w-full max-w-md z-50">
          <Dialog.Title className="text-xl font-semibold mb-4">
            Resolve Pattern
          </Dialog.Title>

          <p className="text-sm text-neutral-600 mb-4">
            {pattern.details.description}
          </p>

          <div className="mb-4">
            <label className="block text-sm font-medium text-neutral-700 mb-2">
              Resolution Notes
            </label>
            <textarea
              value={notes}
              onChange={(e) => setNotes(e.target.value)}
              placeholder="What was done to address this pattern?"
              rows={4}
              className="w-full px-3 py-2 border border-neutral-300 rounded-lg"
            />
          </div>

          <div className="flex justify-end gap-2">
            <Dialog.Close asChild>
              <button className="px-4 py-2 text-neutral-600">Cancel</button>
            </Dialog.Close>
            <button
              onClick={handleResolve}
              disabled={isPending || !notes.trim()}
              className="px-4 py-2 bg-success-500 text-white rounded-lg disabled:opacity-50"
            >
              {isPending ? "Resolving..." : "Mark Resolved"}
            </button>
          </div>
        </Dialog.Content>
      </Dialog.Portal>
    </Dialog.Root>
  );
}
```

### References

- [Source: epics.md#Story 9.5]
