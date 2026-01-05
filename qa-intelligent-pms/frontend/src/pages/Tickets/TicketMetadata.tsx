import { formatDistanceToNow, format } from "date-fns";
import { StatusSelector } from "./StatusSelector";
import type { TicketDetail } from "./types";

interface TicketMetadataProps {
  ticket: TicketDetail;
  /** Callback when status changes */
  onStatusChange?: (newStatus: string, newStatusColor: string) => void;
}

const PRIORITY_COLORS: Record<string, string> = {
  error: "bg-error-100 text-error-700",
  warning: "bg-warning-100 text-warning-700",
  primary: "bg-primary-100 text-primary-700",
  neutral: "bg-neutral-100 text-neutral-600",
};

function formatDate(dateString: string): { relative: string; full: string } {
  const date = new Date(dateString);
  return {
    relative: formatDistanceToNow(date, { addSuffix: true }),
    full: format(date, "PPpp"), // e.g., "Jan 4, 2026 at 3:30 PM"
  };
}

export function TicketMetadata({ ticket, onStatusChange }: TicketMetadataProps) {
  const createdDate = formatDate(ticket.createdAt);
  const updatedDate = formatDate(ticket.updatedAt);

  return (
    <div className="bg-neutral-50 rounded-xl p-5 border border-neutral-200">
      <div className="grid grid-cols-2 md:grid-cols-3 lg:grid-cols-6 gap-5">
        {/* Status - with interactive selector */}
        <div className="space-y-1">
          <span className="text-xs font-medium text-neutral-500 uppercase tracking-wider">
            Status
          </span>
          <div>
            <StatusSelector
              ticketKey={ticket.key}
              currentStatus={ticket.status}
              currentStatusColor={ticket.statusColor}
              onStatusChange={onStatusChange}
            />
          </div>
        </div>

        {/* Priority */}
        <div className="space-y-1">
          <span className="text-xs font-medium text-neutral-500 uppercase tracking-wider">
            Priority
          </span>
          <div>
            {ticket.priority ? (
              <span
                className={`inline-flex px-2.5 py-1 rounded-md text-sm font-medium ${
                  PRIORITY_COLORS[ticket.priorityColor] || PRIORITY_COLORS.neutral
                }`}
              >
                {ticket.priority}
              </span>
            ) : (
              <span className="text-sm text-neutral-400">Not set</span>
            )}
          </div>
        </div>

        {/* Assignee */}
        <div className="space-y-1">
          <span className="text-xs font-medium text-neutral-500 uppercase tracking-wider">
            Assignee
          </span>
          {ticket.assignee ? (
            <div className="flex items-center gap-2">
              {ticket.assignee.avatarUrl ? (
                <img
                  src={ticket.assignee.avatarUrl}
                  alt=""
                  className="w-6 h-6 rounded-full ring-2 ring-white"
                />
              ) : (
                <div className="w-6 h-6 rounded-full bg-primary-100 flex items-center justify-center ring-2 ring-white">
                  <span className="text-xs font-semibold text-primary-700">
                    {ticket.assignee.name.charAt(0).toUpperCase()}
                  </span>
                </div>
              )}
              <span className="text-sm font-medium text-neutral-900 truncate">
                {ticket.assignee.name}
              </span>
            </div>
          ) : (
            <span className="text-sm text-neutral-400">Unassigned</span>
          )}
        </div>

        {/* Reporter */}
        <div className="space-y-1">
          <span className="text-xs font-medium text-neutral-500 uppercase tracking-wider">
            Reporter
          </span>
          {ticket.reporter ? (
            <div className="flex items-center gap-2">
              {ticket.reporter.avatarUrl ? (
                <img
                  src={ticket.reporter.avatarUrl}
                  alt=""
                  className="w-6 h-6 rounded-full ring-2 ring-white"
                />
              ) : (
                <div className="w-6 h-6 rounded-full bg-neutral-200 flex items-center justify-center ring-2 ring-white">
                  <span className="text-xs font-semibold text-neutral-600">
                    {ticket.reporter.name.charAt(0).toUpperCase()}
                  </span>
                </div>
              )}
              <span className="text-sm font-medium text-neutral-900 truncate">
                {ticket.reporter.name}
              </span>
            </div>
          ) : (
            <span className="text-sm text-neutral-400">Unknown</span>
          )}
        </div>

        {/* Created */}
        <div className="space-y-1">
          <span className="text-xs font-medium text-neutral-500 uppercase tracking-wider">
            Created
          </span>
          <div title={createdDate.full}>
            <span className="text-sm font-medium text-neutral-900">
              {createdDate.relative}
            </span>
          </div>
        </div>

        {/* Updated */}
        <div className="space-y-1">
          <span className="text-xs font-medium text-neutral-500 uppercase tracking-wider">
            Updated
          </span>
          <div title={updatedDate.full}>
            <span className="text-sm font-medium text-neutral-900">
              {updatedDate.relative}
            </span>
          </div>
        </div>
      </div>
    </div>
  );
}
