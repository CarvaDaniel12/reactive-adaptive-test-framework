/**
 * KPI Card component for displaying individual metrics.
 * Shows value, trend indicator, and percentage change.
 * 
 * Story 8.2: Supports breakdown by ticket type (tooltip) and click-through navigation.
 */
import type { ReactNode } from "react";
import * as Tooltip from "@radix-ui/react-tooltip";
import type { Trend, TicketBreakdown } from "./types";

export type { Trend, TicketBreakdown };

interface KPICardProps {
  title: string;
  value: string | number;
  change: number;
  trend: Trend;
  icon: ReactNode;
  description?: string;
  invertTrend?: boolean; // When true, "down" is good (e.g., avg time)
  valueColor?: string;   // Optional color class for value (Story 8.3)
  breakdown?: TicketBreakdown[]; // Story 8.2 AC #4: Breakdown by ticket type
  onClick?: () => void; // Story 8.2 AC #6: Click-through navigation
}

export function KPICard({
  title,
  value,
  change,
  trend,
  icon,
  description,
  invertTrend,
  valueColor,
  breakdown,
  onClick,
}: KPICardProps) {
  const trendColor = invertTrend
    ? trend === "up"
      ? "text-red-500"
      : trend === "down"
        ? "text-emerald-500"
        : "text-neutral-500"
    : trend === "up"
      ? "text-emerald-500"
      : trend === "down"
        ? "text-red-500"
        : "text-neutral-500";

  const cardContent = (
    <div
      className={`
        bg-white rounded-xl border border-neutral-200 p-5 transition-shadow
        ${onClick ? "cursor-pointer hover:shadow-md hover:border-indigo-300" : "hover:shadow-sm"}
      `}
      onClick={onClick}
      role={onClick ? "button" : undefined}
      tabIndex={onClick ? 0 : undefined}
      onKeyDown={onClick ? (e) => {
        if (e.key === "Enter" || e.key === " ") {
          e.preventDefault();
          onClick();
        }
      } : undefined}
    >
      <div className="flex items-start justify-between mb-4">
        <div className="p-2 bg-indigo-50 rounded-lg text-indigo-600">
          {icon}
        </div>
        <div className={`flex items-center gap-1 text-sm ${trendColor}`}>
          <TrendIcon trend={trend} />
          <span>{change > 0 ? "+" : ""}{change}%</span>
        </div>
      </div>

      <div className="space-y-1">
        <p className={`text-2xl font-bold ${valueColor ?? "text-neutral-900"}`}>
          {value}
        </p>
        <p className="text-sm text-neutral-500">{title}</p>
        {description && (
          <p className="text-xs text-neutral-400">{description}</p>
        )}
      </div>
    </div>
  );

  // Story 8.2 AC #4: Show breakdown tooltip on hover if available
  if (breakdown && breakdown.length > 0) {
    return (
      <Tooltip.Provider delayDuration={300}>
        <Tooltip.Root>
          <Tooltip.Trigger asChild>
            {cardContent}
          </Tooltip.Trigger>
          <Tooltip.Portal>
            <Tooltip.Content
              side="bottom"
              className="bg-white rounded-lg shadow-xl border border-neutral-200 p-4 z-50 max-w-xs"
              sideOffset={5}
            >
              <p className="font-medium text-sm mb-2 text-neutral-900">Breakdown by Type</p>
              <div className="space-y-2">
                {breakdown.map((item) => (
                  <div
                    key={item.ticket_type}
                    className="flex items-center justify-between gap-4"
                  >
                    <span className="text-sm text-neutral-600 capitalize">
                      {item.ticket_type}
                    </span>
                    <div className="flex items-center gap-2">
                      <span className="text-sm font-medium text-neutral-900">
                        {item.count}
                      </span>
                      <span className="text-xs text-neutral-400">
                        ({item.percentage.toFixed(0)}%)
                      </span>
                    </div>
                  </div>
                ))}
              </div>
              <Tooltip.Arrow className="fill-white" />
            </Tooltip.Content>
          </Tooltip.Portal>
        </Tooltip.Root>
      </Tooltip.Provider>
    );
  }

  return cardContent;
}

function TrendIcon({ trend }: { trend: Trend }) {
  if (trend === "up") {
    return (
      <svg className="w-4 h-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth={2}>
        <path strokeLinecap="round" strokeLinejoin="round" d="M4.5 19.5l15-15m0 0H8.25m11.25 0v11.25" />
      </svg>
    );
  }
  if (trend === "down") {
    return (
      <svg className="w-4 h-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth={2}>
        <path strokeLinecap="round" strokeLinejoin="round" d="M4.5 4.5l15 15m0 0V8.25m0 11.25H8.25" />
      </svg>
    );
  }
  return (
    <svg className="w-4 h-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth={2}>
      <path strokeLinecap="round" strokeLinejoin="round" d="M17.25 12H6.75" />
    </svg>
  );
}

// Skeleton for loading state
export function KPICardSkeleton() {
  return (
    <div className="bg-white rounded-xl border border-neutral-200 p-5">
      <div className="flex items-start justify-between mb-4">
        <div className="w-10 h-10 bg-neutral-100 rounded-lg animate-pulse" />
        <div className="w-12 h-5 bg-neutral-100 rounded animate-pulse" />
      </div>
      <div className="space-y-2">
        <div className="w-20 h-8 bg-neutral-100 rounded animate-pulse" />
        <div className="w-28 h-4 bg-neutral-100 rounded animate-pulse" />
      </div>
    </div>
  );
}
