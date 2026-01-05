/**
 * Trend chart component using Recharts.
 * Displays tickets completed or hours over time as an area chart.
 * Supports toggle between metrics (Story 8.4).
 */
import { useState } from "react";
import {
  AreaChart,
  Area,
  XAxis,
  YAxis,
  CartesianGrid,
  Tooltip,
  ResponsiveContainer,
} from "recharts";
import type { Period } from "./PeriodSelector";

export interface TrendDataPoint {
  date: string;
  tickets: number;
  hours: number;
}

type ChartMetric = "tickets" | "hours";

interface TrendChartProps {
  data?: TrendDataPoint[];
  period: Period;
  isLoading: boolean;
}

export function TrendChart({ data, period, isLoading }: TrendChartProps) {
  const [metric, setMetric] = useState<ChartMetric>("tickets");

  if (isLoading) {
    return (
      <div className="bg-white rounded-xl border border-neutral-200 p-5">
        <div className="flex items-center justify-between mb-4">
          <div className="w-32 h-6 bg-neutral-100 rounded animate-pulse" />
          <div className="w-20 h-5 bg-neutral-100 rounded animate-pulse" />
        </div>
        <div className="h-64 bg-neutral-50 rounded-lg animate-pulse" />
      </div>
    );
  }

  const chartData = data ?? [];
  const periodLabel = getPeriodLabel(period);

  const metricConfig = {
    tickets: {
      dataKey: "tickets",
      name: "Tickets",
      color: "#6366f1", // indigo-500
      gradientId: "colorTickets",
      yAxisLabel: "Tickets",
    },
    hours: {
      dataKey: "hours",
      name: "Hours",
      color: "#10b981", // emerald-500
      gradientId: "colorHours",
      yAxisLabel: "Hours",
    },
  };

  const config = metricConfig[metric];

  return (
    <div className="bg-white rounded-xl border border-neutral-200 p-5">
      <div className="flex items-center justify-between mb-4">
        <h3 className="font-semibold text-neutral-900">Activity Trend</h3>
        <div className="flex items-center gap-3">
          {/* Metric Toggle - Story 8.4 */}
          <div className="flex items-center bg-neutral-100 rounded-lg p-0.5">
            <button
              type="button"
              onClick={() => setMetric("tickets")}
              className={`
                px-2.5 py-1 text-xs font-medium rounded-md transition-all
                ${metric === "tickets"
                  ? "bg-white text-neutral-900 shadow-sm"
                  : "text-neutral-500 hover:text-neutral-700"}
              `}
              aria-label="Show tickets trend"
            >
              Tickets
            </button>
            <button
              type="button"
              onClick={() => setMetric("hours")}
              className={`
                px-2.5 py-1 text-xs font-medium rounded-md transition-all
                ${metric === "hours"
                  ? "bg-white text-neutral-900 shadow-sm"
                  : "text-neutral-500 hover:text-neutral-700"}
              `}
              aria-label="Show hours trend"
            >
              Hours
            </button>
          </div>
          <span className="text-sm text-neutral-500">{periodLabel}</span>
        </div>
      </div>

      {chartData.length === 0 ? (
        <div className="h-64 flex items-center justify-center text-neutral-400">
          No data available for this period
        </div>
      ) : (
        <div className="h-64" role="img" aria-label={`${config.name} trend chart for ${periodLabel}`}>
          <ResponsiveContainer width="100%" height="100%">
            <AreaChart
              data={chartData}
              margin={{ top: 10, right: 10, left: 0, bottom: 0 }}
            >
              <defs>
                <linearGradient id="colorTickets" x1="0" y1="0" x2="0" y2="1">
                  <stop offset="5%" stopColor="#6366f1" stopOpacity={0.3} />
                  <stop offset="95%" stopColor="#6366f1" stopOpacity={0} />
                </linearGradient>
                <linearGradient id="colorHours" x1="0" y1="0" x2="0" y2="1">
                  <stop offset="5%" stopColor="#10b981" stopOpacity={0.3} />
                  <stop offset="95%" stopColor="#10b981" stopOpacity={0} />
                </linearGradient>
              </defs>
              <CartesianGrid strokeDasharray="3 3" stroke="#e5e7eb" />
              <XAxis
                dataKey="date"
                tick={{ fontSize: 12, fill: "#6b7280" }}
                tickLine={false}
                axisLine={{ stroke: "#e5e7eb" }}
              />
              <YAxis
                tick={{ fontSize: 12, fill: "#6b7280" }}
                tickLine={false}
                axisLine={false}
                allowDecimals={metric === "hours"}
                label={{
                  value: config.yAxisLabel,
                  angle: -90,
                  position: "insideLeft",
                  style: { fontSize: 11, fill: "#9ca3af" },
                }}
              />
              <Tooltip
                contentStyle={{
                  backgroundColor: "#fff",
                  border: "1px solid #e5e7eb",
                  borderRadius: "8px",
                  boxShadow: "0 4px 6px -1px rgb(0 0 0 / 0.1)",
                }}
                labelStyle={{ fontWeight: 600, marginBottom: 4 }}
                formatter={(value) => {
                  const numValue = typeof value === "number" ? value : 0;
                  return [
                    metric === "hours" ? `${numValue.toFixed(1)}h` : numValue,
                    config.name,
                  ];
                }}
              />
              <Area
                type="monotone"
                dataKey={config.dataKey}
                name={config.name}
                stroke={config.color}
                strokeWidth={2}
                fillOpacity={1}
                fill={`url(#${config.gradientId})`}
              />
            </AreaChart>
          </ResponsiveContainer>
        </div>
      )}
    </div>
  );
}

function getPeriodLabel(period: Period): string {
  switch (period) {
    case "7d":
      return "Last 7 days";
    case "30d":
      return "Last 30 days";
    case "90d":
      return "Last 90 days";
    case "1y":
      return "This year";
  }
}
