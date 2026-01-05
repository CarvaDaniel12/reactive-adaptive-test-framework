/**
 * Status indicator component for integration health.
 *
 * Displays a color-coded dot representing the health status:
 * - Online: Green (success)
 * - Degraded: Yellow/Amber (warning)
 * - Offline: Red (error)
 */

export type HealthStatus = "online" | "degraded" | "offline";

interface StatusIndicatorProps {
  status: HealthStatus;
  size?: "sm" | "md" | "lg";
  showPulse?: boolean;
}

const STATUS_COLORS: Record<HealthStatus, string> = {
  online: "bg-emerald-500",
  degraded: "bg-amber-500",
  offline: "bg-red-500",
};

const STATUS_LABELS: Record<HealthStatus, string> = {
  online: "Online",
  degraded: "Degraded",
  offline: "Offline",
};

const SIZE_CLASSES: Record<"sm" | "md" | "lg", string> = {
  sm: "w-2 h-2",
  md: "w-3 h-3",
  lg: "w-4 h-4",
};

export function StatusIndicator({
  status,
  size = "md",
  showPulse = true,
}: StatusIndicatorProps) {
  const shouldPulse = showPulse && status !== "online";

  return (
    <span
      className={`
        inline-block rounded-full
        ${STATUS_COLORS[status]}
        ${SIZE_CLASSES[size]}
        ${shouldPulse ? "animate-pulse" : ""}
      `}
      role="status"
      aria-label={`Integration is ${STATUS_LABELS[status]}`}
    />
  );
}
