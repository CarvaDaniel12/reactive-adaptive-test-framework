/**
 * ReportListItem component with hover preview.
 * Story 7.5 Task 4: Create ReportListItem with hover preview
 */
import { Link } from 'react-router-dom';
import { formatDistanceToNow, format } from 'date-fns';
import * as HoverCard from '@radix-ui/react-hover-card';
import type { ReportSummary } from '@/types';
import { useReport } from '@/hooks/useReports';

interface ReportListItemProps {
  report: ReportSummary;
}

/**
 * Formats seconds to a human-readable duration string.
 */
function formatDurationSeconds(seconds: number): string {
  if (seconds < 60) return `${seconds}s`;
  if (seconds < 3600) return `${Math.round(seconds / 60)}m`;
  const hours = Math.floor(seconds / 3600);
  const mins = Math.round((seconds % 3600) / 60);
  return mins > 0 ? `${hours}h ${mins}m` : `${hours}h`;
}

/**
 * Quick preview component that shows report details on hover.
 */
function ReportQuickPreview({ reportId }: { reportId: string }) {
  const { data: report, isLoading } = useReport(reportId);

  if (isLoading) {
    return (
      <div className="animate-pulse space-y-2">
        <div className="h-4 bg-neutral-100 rounded w-3/4" />
        <div className="h-4 bg-neutral-100 rounded w-1/2" />
        <div className="h-4 bg-neutral-100 rounded w-2/3" />
      </div>
    );
  }

  if (!report) return null;

  const content = report.content;
  const completedSteps = content.steps.filter((s) => s.status === 'completed').length;
  const totalSteps = content.steps.length;
  const totalActiveSeconds = content.timeSummary?.totalActiveSeconds ?? report.totalTimeSeconds ?? 0;
  const totalEstimatedSeconds = content.timeSummary?.totalEstimatedSeconds ?? 0;

  return (
    <div className="space-y-3">
      <div className="text-sm">
        <span className="font-medium">{totalSteps} steps</span>
        <span className="text-neutral-400 mx-2">•</span>
        <span className="text-success-600">{completedSteps} completed</span>
      </div>

      {totalActiveSeconds > 0 && (
        <div className="text-sm">
          <span className="text-neutral-500">Time: </span>
          <span className="font-medium">{formatDurationSeconds(totalActiveSeconds)}</span>
          {totalEstimatedSeconds > 0 && (
            <>
              <span className="text-neutral-400"> / </span>
              <span>{formatDurationSeconds(totalEstimatedSeconds)} est.</span>
            </>
          )}
        </div>
      )}

      {content.testsCovered && content.testsCovered.length > 0 && (
        <div className="text-sm">
          <span className="text-neutral-500">Tests: </span>
          <span className="font-medium">{content.testsCovered.length}</span>
        </div>
      )}

      <div className="pt-2 border-t border-neutral-100 text-xs text-primary-600">
        Click to view full report →
      </div>
    </div>
  );
}

/**
 * Report list item component with hover preview functionality.
 */
export function ReportListItem({ report }: ReportListItemProps) {
  return (
    <HoverCard.Root openDelay={300}>
      <HoverCard.Trigger asChild>
        <Link
          to={`/reports/${report.id}`}
          className="block p-4 bg-white border border-neutral-200 rounded-lg 
                     hover:border-primary-300 hover:shadow-sm transition-all"
        >
          <div className="flex items-center justify-between">
            <div className="flex items-center gap-3">
              <span className="px-2 py-1 bg-primary-50 text-primary-700 
                             font-mono text-sm rounded">
                {report.ticketKey}
              </span>
              <span className="text-neutral-700 font-medium truncate max-w-md">
                {report.ticketTitle || 'Untitled'}
              </span>
            </div>

            <div className="flex items-center gap-4 text-sm text-neutral-500">
              <span>{report.templateName}</span>
              {report.totalTimeSeconds && (
                <span>{formatDurationSeconds(report.totalTimeSeconds)}</span>
              )}
              <span title={format(new Date(report.createdAt), 'PPpp')}>
                {formatDistanceToNow(new Date(report.createdAt), { addSuffix: true })}
              </span>
            </div>
          </div>
        </Link>
      </HoverCard.Trigger>

      <HoverCard.Portal>
        <HoverCard.Content
          side="right"
          sideOffset={8}
          className="bg-white rounded-lg shadow-xl border border-neutral-200 
                     p-4 w-80 animate-scale-in z-50"
        >
          <ReportQuickPreview reportId={report.id} />
          <HoverCard.Arrow className="fill-white" />
        </HoverCard.Content>
      </HoverCard.Portal>
    </HoverCard.Root>
  );
}