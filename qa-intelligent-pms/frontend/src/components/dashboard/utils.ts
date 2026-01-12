/**
 * Dashboard utility functions.
 * Business logic for dashboard metrics calculations.
 */

/**
 * Get color class based on efficiency ratio.
 * Story 8.3: Color coding - 🟢 ≥1.0, 🟡 0.8-1.0, 🔴 <0.8
 * Note: Higher efficiency % is better (100% = on target)
 */
export function getEfficiencyColor(ratio: number): string {
  if (ratio >= 1.0) return "text-emerald-600"; // 🟢 On or ahead of schedule
  if (ratio >= 0.8) return "text-amber-600";   // 🟡 Slightly behind
  return "text-red-600";                        // 🔴 Significantly behind
}

/**
 * Get human-readable description for efficiency ratio.
 */
export function getEfficiencyDescription(ratio: number): string {
  if (ratio >= 1.0) return "On or ahead of schedule";
  if (ratio >= 0.8) return "Slightly behind estimate";
  return "Behind schedule";
}
