import { useState, useEffect, useCallback } from "react";
import { useQuery } from "@tanstack/react-query";
import { useDebouncedValue } from "./useDebouncedValue";

/**
 * Search result from any source.
 */
export interface SearchResult {
  source: string;
  id: string;
  name: string;
  description: string | null;
  url: string;
  score: number;
  matches: string[];
}

/**
 * Search response from the API.
 */
export interface SearchResponse {
  results: SearchResult[];
  postmanCount: number;
  testmoCount: number;
  searchTimeMs: number;
  keywordsUsed: string[];
}

/**
 * Search progress for each integration.
 */
export type SearchStatus = "pending" | "searching" | "complete" | "error" | "skipped";

export interface SearchProgress {
  postman: SearchStatus;
  testmo: SearchStatus;
}

/**
 * Options for the contextual search hook.
 */
export interface UseContextualSearchOptions {
  /** Ticket key for caching */
  ticketKey: string;
  /** Ticket title to extract keywords from */
  title: string;
  /** Optional ticket description */
  description?: string | null;
  /** Whether search is enabled */
  enabled?: boolean;
}

/**
 * Hook for contextual search on ticket selection.
 *
 * Automatically extracts keywords from ticket title/description
 * and searches Postman and Testmo in parallel.
 */
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

    const response = await fetch("/api/v1/search/contextual", {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({
        ticketKey,
        title: debouncedTitle,
        description: description || undefined,
      }),
    });

    if (!response.ok) {
      setProgress({ postman: "error", testmo: "error" });
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
    enabled: enabled && !!debouncedTitle && debouncedTitle.length > 0,
    staleTime: 5 * 60 * 1000, // Cache for 5 minutes
    retry: 1,
  });

  // Reset progress when search starts
  useEffect(() => {
    if (query.isFetching) {
      setProgress({ postman: "searching", testmo: "searching" });
    }
  }, [query.isFetching]);

  // Update progress on error
  useEffect(() => {
    if (query.isError) {
      setProgress({ postman: "error", testmo: "error" });
    }
  }, [query.isError]);

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
    refetch: query.refetch,
  };
}
