# Story 10.1: PM Dashboard Layout

Status: ready-for-dev

## Story

As a PM (Carlos),
I want a dashboard focused on product quality,
So that I can make data-driven decisions.

## Acceptance Criteria

1. **Given** user has PM role or access
   **When** user navigates to PM Dashboard
   **Then** large KPI cards are displayed (bugs discovered, bugs prevented, economy)

2. **Given** PM dashboard layout
   **When** rendered
   **Then** component health section is visible

3. **Given** PM dashboard layout
   **When** rendered
   **Then** problematic endpoints section is visible

4. **Given** PM dashboard layout
   **When** rendered
   **Then** export button is prominently placed

5. **Given** dashboard styling
   **When** rendered
   **Then** dashboard uses same design system as QA dashboard

6. **Given** navigation
   **When** accessing PM dashboard
   **Then** navigation via sidebar "PM Dashboard" item

## Tasks

- [ ] Task 1: Create PMDashboardPage component
- [ ] Task 2: Create large KPI card variants
- [ ] Task 3: Create ComponentHealthSection
- [ ] Task 4: Create ProblematicEndpointsSection
- [ ] Task 5: Add export button with menu
- [ ] Task 6: Add PM Dashboard to sidebar
- [ ] Task 7: Implement role-based access

## Dev Notes

### PM Dashboard Page

```tsx
// frontend/src/pages/PMDashboard.tsx
import { useState } from "react";
import { PMKPICards } from "@/components/pm-dashboard/PMKPICards";
import { ComponentHealthSection } from "@/components/pm-dashboard/ComponentHealthSection";
import { ProblematicEndpointsSection } from "@/components/pm-dashboard/ProblematicEndpointsSection";
import { PeriodSelector } from "@/components/dashboard/PeriodSelector";
import { ExportMenu } from "@/components/pm-dashboard/ExportMenu";
import { usePMDashboardData } from "@/hooks/usePMDashboardData";

export function PMDashboardPage() {
  const [period, setPeriod] = useState<Period>("30d");
  const { data, isLoading } = usePMDashboardData(period);

  return (
    <div className="p-6 space-y-6">
      {/* Header */}
      <div className="flex items-center justify-between">
        <div>
          <h1 className="text-2xl font-bold text-neutral-900">PM Dashboard</h1>
          <p className="text-sm text-neutral-500">Product quality and QA value metrics</p>
        </div>
        
        <div className="flex items-center gap-4">
          <PeriodSelector value={period} onChange={setPeriod} />
          <ExportMenu data={data} period={period} />
        </div>
      </div>

      {/* Large KPI Cards */}
      <PMKPICards data={data?.kpis} isLoading={isLoading} />

      {/* Two Column Layout */}
      <div className="grid grid-cols-1 lg:grid-cols-2 gap-6">
        {/* Component Health */}
        <ComponentHealthSection
          data={data?.componentHealth}
          isLoading={isLoading}
        />

        {/* Problematic Endpoints */}
        <ProblematicEndpointsSection
          data={data?.problematicEndpoints}
          isLoading={isLoading}
        />
      </div>
    </div>
  );
}
```

### PM KPI Cards

```tsx
// frontend/src/components/pm-dashboard/PMKPICards.tsx
interface PMKPICardsProps {
  data?: PMKPIs;
  isLoading: boolean;
}

interface PMKPIs {
  bugsDiscovered: { value: number; change: number };
  bugsPrevented: { value: number; change: number };
  preventionRate: { value: number; change: number };
  economyEstimate: { value: number; change: number };
}

export function PMKPICards({ data, isLoading }: PMKPICardsProps) {
  if (isLoading) {
    return (
      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4">
        {[1, 2, 3, 4].map((i) => (
          <div key={i} className="h-40 bg-neutral-100 rounded-xl animate-pulse" />
        ))}
      </div>
    );
  }

  return (
    <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4">
      <LargeKPICard
        title="Bugs Discovered"
        value={data?.bugsDiscovered.value || 0}
        change={data?.bugsDiscovered.change || 0}
        icon={<BugIcon className="w-6 h-6" />}
        color="error"
        description="Found during testing"
      />
      <LargeKPICard
        title="Bugs Prevented"
        value={data?.bugsPrevented.value || 0}
        change={data?.bugsPrevented.change || 0}
        icon={<ShieldIcon className="w-6 h-6" />}
        color="success"
        description="Caught before production"
      />
      <LargeKPICard
        title="Prevention Rate"
        value={`${((data?.preventionRate.value || 0) * 100).toFixed(0)}%`}
        change={data?.preventionRate.change || 0}
        icon={<TrendingUpIcon className="w-6 h-6" />}
        color="primary"
        description="Prevented / Total"
      />
      <LargeKPICard
        title="Economy Estimate"
        value={formatCurrency(data?.economyEstimate.value || 0)}
        change={data?.economyEstimate.change || 0}
        icon={<DollarIcon className="w-6 h-6" />}
        color="success"
        description="Estimated savings"
      />
    </div>
  );
}

interface LargeKPICardProps {
  title: string;
  value: string | number;
  change: number;
  icon: React.ReactNode;
  color: "error" | "success" | "primary" | "warning";
  description: string;
}

function LargeKPICard({
  title,
  value,
  change,
  icon,
  color,
  description,
}: LargeKPICardProps) {
  const colorClasses = {
    error: "bg-error-50 text-error-600 border-error-200",
    success: "bg-success-50 text-success-600 border-success-200",
    primary: "bg-primary-50 text-primary-600 border-primary-200",
    warning: "bg-warning-50 text-warning-600 border-warning-200",
  };

  const iconClasses = {
    error: "bg-error-100 text-error-600",
    success: "bg-success-100 text-success-600",
    primary: "bg-primary-100 text-primary-600",
    warning: "bg-warning-100 text-warning-600",
  };

  return (
    <div className={cn(
      "rounded-xl border-2 p-6 transition-all hover:shadow-md",
      colorClasses[color]
    )}>
      <div className="flex items-start justify-between mb-4">
        <div className={cn("p-3 rounded-lg", iconClasses[color])}>
          {icon}
        </div>
        <TrendIndicator value={change} />
      </div>

      <div className="space-y-1">
        <p className="text-3xl font-bold text-neutral-900">{value}</p>
        <p className="font-medium text-neutral-700">{title}</p>
        <p className="text-sm text-neutral-500">{description}</p>
      </div>
    </div>
  );
}
```

### Export Menu

```tsx
// frontend/src/components/pm-dashboard/ExportMenu.tsx
import * as DropdownMenu from "@radix-ui/react-dropdown-menu";
import { DownloadIcon, FileTextIcon, TableIcon, FileIcon } from "@radix-ui/react-icons";

interface ExportMenuProps {
  data: PMDashboardData | undefined;
  period: Period;
}

export function ExportMenu({ data, period }: ExportMenuProps) {
  const handleExport = async (format: "pdf" | "html" | "csv") => {
    const response = await fetch(
      `/api/v1/pm-dashboard/export?format=${format}&period=${period}`
    );
    const blob = await response.blob();
    const url = URL.createObjectURL(blob);
    const a = document.createElement("a");
    a.href = url;
    a.download = `QA-Metrics-${period}-${format(new Date(), "yyyy-MM-dd")}.${format}`;
    a.click();
    URL.revokeObjectURL(url);
  };

  return (
    <DropdownMenu.Root>
      <DropdownMenu.Trigger asChild>
        <button className="flex items-center gap-2 px-4 py-2 bg-primary-500 text-white font-medium rounded-lg hover:bg-primary-600">
          <DownloadIcon className="w-4 h-4" />
          Export
        </button>
      </DropdownMenu.Trigger>

      <DropdownMenu.Portal>
        <DropdownMenu.Content
          className="bg-white rounded-lg shadow-xl border border-neutral-200 py-1 min-w-48"
          sideOffset={8}
        >
          <DropdownMenu.Item
            onClick={() => handleExport("pdf")}
            className="flex items-center gap-2 px-4 py-2 text-sm cursor-pointer hover:bg-neutral-50"
          >
            <FileIcon className="w-4 h-4 text-error-500" />
            Export as PDF
          </DropdownMenu.Item>
          <DropdownMenu.Item
            onClick={() => handleExport("html")}
            className="flex items-center gap-2 px-4 py-2 text-sm cursor-pointer hover:bg-neutral-50"
          >
            <FileTextIcon className="w-4 h-4 text-primary-500" />
            Export as HTML
          </DropdownMenu.Item>
          <DropdownMenu.Item
            onClick={() => handleExport("csv")}
            className="flex items-center gap-2 px-4 py-2 text-sm cursor-pointer hover:bg-neutral-50"
          >
            <TableIcon className="w-4 h-4 text-success-500" />
            Export as CSV
          </DropdownMenu.Item>
        </DropdownMenu.Content>
      </DropdownMenu.Portal>
    </DropdownMenu.Root>
  );
}
```

### Sidebar Navigation Update

```tsx
// Add to sidebar for PM role
{
  label: "PM Dashboard",
  icon: <BarChartIcon />,
  path: "/pm-dashboard",
  roles: ["pm", "admin"],
}
```

### References

- [Source: epics.md#Story 10.1]
