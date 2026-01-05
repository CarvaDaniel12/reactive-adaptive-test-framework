/**
 * Period selector for dashboard time range filtering.
 * Provides buttons for 7d, 30d, 90d, and 1y periods.
 */

export type Period = "7d" | "30d" | "90d" | "1y";

interface PeriodSelectorProps {
  value: Period;
  onChange: (period: Period) => void;
}

const PERIODS: { value: Period; label: string }[] = [
  { value: "7d", label: "7 days" },
  { value: "30d", label: "30 days" },
  { value: "90d", label: "90 days" },
  { value: "1y", label: "Year" },
];

export function PeriodSelector({ value, onChange }: PeriodSelectorProps) {
  return (
    <div className="flex items-center bg-neutral-100 rounded-lg p-1">
      {PERIODS.map((period) => (
        <button
          key={period.value}
          type="button"
          onClick={() => onChange(period.value)}
          className={`
            px-3 py-1.5 text-sm font-medium rounded-md transition-all
            ${value === period.value
              ? "bg-white text-neutral-900 shadow-sm"
              : "text-neutral-500 hover:text-neutral-700"}
          `}
        >
          {period.label}
        </button>
      ))}
    </div>
  );
}
