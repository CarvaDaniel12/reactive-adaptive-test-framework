import { useState, useCallback } from "react";
import * as Checkbox from "@radix-ui/react-checkbox";
import { SearchResultCard } from "./SearchResultCard";
import { SourceBadge } from "./SourceBadge";
import { CreateTestRunDialog } from "./CreateTestRunDialog";
import type { SearchResult } from "@/hooks/useContextualSearch";

// SVG Icons
function CheckIcon({ className }: { className?: string }) {
  return (
    <svg className={className} viewBox="0 0 15 15" fill="none" xmlns="http://www.w3.org/2000/svg">
      <path
        d="M11.4669 3.72684C11.7558 3.91574 11.8369 4.30308 11.648 4.59198L7.39799 11.092C7.29783 11.2452 7.13556 11.3467 6.95402 11.3699C6.77247 11.3931 6.58989 11.3355 6.45446 11.2124L3.70446 8.71241C3.44905 8.48022 3.43023 8.08494 3.66242 7.82953C3.89461 7.57412 4.28989 7.5553 4.5453 7.78749L6.75292 9.79441L10.6018 3.90792C10.7907 3.61902 11.178 3.53795 11.4669 3.72684Z"
        fill="currentColor"
        fillRule="evenodd"
        clipRule="evenodd"
      />
    </svg>
  );
}

function RocketIcon({ className }: { className?: string }) {
  return (
    <svg className={className} viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2">
      <path strokeLinecap="round" strokeLinejoin="round" d="M15.59 14.37a6 6 0 01-5.84 7.38v-4.8m5.84-2.58a14.98 14.98 0 006.16-12.12A14.98 14.98 0 009.631 8.41m5.96 5.96a14.926 14.926 0 01-5.841 2.58m-.119-8.54a6 6 0 00-7.381 5.84h4.8m2.581-5.84a14.927 14.927 0 00-2.58 5.84m2.699 2.7c-.103.021-.207.041-.311.06a15.09 15.09 0 01-2.448-2.448 14.9 14.9 0 01.06-.312m-2.24 2.39a4.493 4.493 0 00-1.757 4.306 4.493 4.493 0 004.306-1.758M16.5 9a1.5 1.5 0 11-3 0 1.5 1.5 0 013 0z" />
    </svg>
  );
}

interface SelectableSearchResultsProps {
  results: SearchResult[];
  ticketKey: string;
}

/**
 * Component that displays search results with selectable Testmo test cases.
 *
 * Allows selecting Testmo results to create a test run.
 * Postman results are shown but not selectable.
 */
export function SelectableSearchResults({
  results,
  ticketKey,
}: SelectableSearchResultsProps) {
  const [selectedIds, setSelectedIds] = useState<Set<string>>(new Set());
  const [dialogOpen, setDialogOpen] = useState(false);

  // Separate Testmo (selectable) from other results
  const testmoResults = results.filter((r) => r.source === "testmo");
  const otherResults = results.filter((r) => r.source !== "testmo");

  const toggleSelection = useCallback((id: string) => {
    setSelectedIds((prev) => {
      const next = new Set(prev);
      if (next.has(id)) {
        next.delete(id);
      } else {
        next.add(id);
      }
      return next;
    });
  }, []);

  const selectAll = useCallback(() => {
    setSelectedIds(new Set(testmoResults.map((r) => r.id)));
  }, [testmoResults]);

  const clearSelection = useCallback(() => {
    setSelectedIds(new Set());
  }, []);

  const selectedCases = testmoResults
    .filter((r) => selectedIds.has(r.id))
    .map((r) => ({ id: parseInt(r.id, 10), name: r.name }));

  return (
    <div className="space-y-6">
      {/* Testmo Results with Selection */}
      {testmoResults.length > 0 && (
        <div>
          <div className="flex items-center justify-between mb-3">
            <div className="flex items-center gap-2">
              <SourceBadge source="testmo" />
              <span className="text-sm text-neutral-500">
                {testmoResults.length} result{testmoResults.length !== 1 ? "s" : ""}
              </span>
            </div>

            <div className="flex items-center gap-2">
              {selectedIds.size > 0 && (
                <>
                  <span className="text-sm text-blue-600 font-medium">
                    {selectedIds.size} selected
                  </span>
                  <button
                    onClick={clearSelection}
                    className="text-sm text-neutral-500 hover:text-neutral-700"
                  >
                    Clear
                  </button>
                </>
              )}
              {selectedIds.size < testmoResults.length && (
                <button
                  onClick={selectAll}
                  className="text-sm text-blue-600 hover:text-blue-700"
                >
                  Select all
                </button>
              )}
            </div>
          </div>

          <div className="space-y-2">
            {testmoResults.map((result) => (
              <SelectableResultCard
                key={result.id}
                result={result}
                selected={selectedIds.has(result.id)}
                onToggle={() => toggleSelection(result.id)}
              />
            ))}
          </div>

          {/* Create Run Button */}
          <button
            onClick={() => setDialogOpen(true)}
            disabled={selectedIds.size === 0}
            className="mt-4 w-full flex items-center justify-center gap-2 px-4 py-2.5
                       bg-blue-500 text-white font-medium rounded-lg
                       hover:bg-blue-600 transition-colors
                       disabled:opacity-50 disabled:cursor-not-allowed"
          >
            <RocketIcon className="w-4 h-4" />
            Create Test Run ({selectedIds.size} case{selectedIds.size !== 1 ? "s" : ""})
          </button>
        </div>
      )}

      {/* Other Results (not selectable) */}
      {otherResults.length > 0 && (
        <div>
          <div className="flex items-center gap-2 mb-3">
            <SourceBadge source="postman" />
            <span className="text-sm text-neutral-500">
              {otherResults.length} result{otherResults.length !== 1 ? "s" : ""}
            </span>
          </div>
          <div className="space-y-3">
            {otherResults.map((result) => (
              <SearchResultCard key={`${result.source}-${result.id}`} result={result} />
            ))}
          </div>
        </div>
      )}

      {/* Create Test Run Dialog */}
      <CreateTestRunDialog
        open={dialogOpen}
        onOpenChange={setDialogOpen}
        ticketKey={ticketKey}
        selectedCases={selectedCases}
        onSuccess={() => setSelectedIds(new Set())}
      />
    </div>
  );
}

interface SelectableResultCardProps {
  result: SearchResult;
  selected: boolean;
  onToggle: () => void;
}

function SelectableResultCard({ result, selected, onToggle }: SelectableResultCardProps) {
  return (
    <div
      className={`flex items-start gap-3 p-3 border rounded-lg transition-colors cursor-pointer
        ${selected
          ? "border-blue-300 bg-blue-50"
          : "border-neutral-200 hover:border-blue-200"
        }`}
      onClick={onToggle}
      role="button"
      tabIndex={0}
      onKeyDown={(e) => {
        if (e.key === "Enter" || e.key === " ") {
          e.preventDefault();
          onToggle();
        }
      }}
    >
      <Checkbox.Root
        checked={selected}
        onCheckedChange={() => onToggle()}
        className="w-5 h-5 rounded border border-neutral-300 bg-white
                   flex items-center justify-center flex-shrink-0
                   data-[state=checked]:bg-blue-500 data-[state=checked]:border-blue-500"
        onClick={(e) => e.stopPropagation()}
      >
        <Checkbox.Indicator>
          <CheckIcon className="w-4 h-4 text-white" />
        </Checkbox.Indicator>
      </Checkbox.Root>

      <div className="flex-1 min-w-0">
        <h4 className="font-medium text-neutral-900 truncate">{result.name}</h4>
        {result.description && (
          <p className="text-sm text-neutral-500 line-clamp-1">{result.description}</p>
        )}
      </div>

      <a
        href={result.url}
        target="_blank"
        rel="noopener noreferrer"
        onClick={(e) => e.stopPropagation()}
        className="text-xs text-blue-600 hover:underline flex-shrink-0"
      >
        View
      </a>
    </div>
  );
}
