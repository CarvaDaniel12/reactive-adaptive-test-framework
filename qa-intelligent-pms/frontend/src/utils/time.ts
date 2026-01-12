/**
 * Time formatting utilities.
 * Centralized to avoid duplication across components.
 */

/**
 * Format seconds to human-readable duration string.
 * Examples: "30s", "45m", "2h 30m", "3h"
 */
export function formatDuration(seconds: number): string {
  if (seconds < 60) return `${seconds}s`;
  if (seconds < 3600) return `${Math.round(seconds / 60)}m`;
  const hours = Math.floor(seconds / 3600);
  const mins = Math.round((seconds % 3600) / 60);
  return mins > 0 ? `${hours}h ${mins}m` : `${hours}h`;
}
