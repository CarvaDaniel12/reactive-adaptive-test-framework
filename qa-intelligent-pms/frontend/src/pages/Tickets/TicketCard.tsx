import { Link } from "react-router-dom";
import type { TicketSummary } from "./types";

interface TicketCardProps {
  ticket: TicketSummary;
}

const PRIORITY_COLORS: Record<string, string> = {
  error: "bg-error-100 text-error-700",
  warning: "bg-warning-100 text-warning-700",
  primary: "bg-primary-100 text-primary-700",
  neutral: "bg-neutral-100 text-neutral-700",
};

const STATUS_COLORS: Record<string, string> = {
  blue: "bg-primary-100 text-primary-700",
  yellow: "bg-warning-100 text-warning-700",
  green: "bg-success-100 text-success-700",
  default: "bg-neutral-100 text-neutral-600",
};

function getStatusColorClass(colorName: string): string {
  return STATUS_COLORS[colorName.toLowerCase()] || STATUS_COLORS.default;
}

function formatRelativeTime(dateString: string): string {
  const date = new Date(dateString);
  const now = new Date();
  const diffMs = now.getTime() - date.getTime();
  const diffMins = Math.floor(diffMs / 60000);
  const diffHours = Math.floor(diffMins / 60);
  const diffDays = Math.floor(diffHours / 24);

  if (diffMins < 1) return "just now";
  if (diffMins < 60) return `${diffMins}m ago`;
  if (diffHours < 24) return `${diffHours}h ago`;
  if (diffDays < 7) return `${diffDays}d ago`;

  return date.toLocaleDateString("en-US", {
    month: "short",
    day: "numeric",
  });
}

export function TicketCard({ ticket }: TicketCardProps) {
  return (
    <Link
      to={`/tickets/${ticket.key}`}
      className="block p-4 bg-white border border-neutral-200 rounded-lg 
                 hover:border-primary-300 hover:shadow-md transition-all duration-200
                 focus:outline-none focus:ring-2 focus:ring-primary-500 focus:ring-offset-2"
    >
      {/* Header */}
      <div className="flex items-start justify-between mb-2">
        <span className="text-sm font-mono font-medium text-primary-600">
          {ticket.key}
        </span>
        {ticket.priority && (
          <span
            className={`text-xs px-2 py-0.5 rounded font-medium ${
              PRIORITY_COLORS[ticket.priorityColor] || PRIORITY_COLORS.neutral
            }`}
          >
            {ticket.priority}
          </span>
        )}
      </div>

      {/* Title */}
      <h3 className="font-medium text-neutral-900 line-clamp-2 mb-3 min-h-[2.5rem]">
        {ticket.title}
      </h3>

      {/* Footer */}
      <div className="flex items-center justify-between text-sm">
        <span
          className={`px-2 py-1 rounded text-xs font-medium ${getStatusColorClass(
            ticket.statusColor
          )}`}
        >
          {ticket.status}
        </span>

        <div className="flex items-center gap-2 text-neutral-500">
          {ticket.assigneeName && (
            <div className="flex items-center gap-1.5">
              {ticket.assigneeAvatar ? (
                <img
                  src={ticket.assigneeAvatar}
                  alt=""
                  className="w-5 h-5 rounded-full"
                />
              ) : (
                <div className="w-5 h-5 rounded-full bg-neutral-200 flex items-center justify-center">
                  <span className="text-[10px] font-medium text-neutral-600">
                    {ticket.assigneeName.charAt(0).toUpperCase()}
                  </span>
                </div>
              )}
              <span className="text-xs truncate max-w-[80px]">
                {ticket.assigneeName.split(" ")[0]}
              </span>
            </div>
          )}
          <span className="text-xs text-neutral-400">
            {formatRelativeTime(ticket.updatedAt)}
          </span>
        </div>
      </div>
    </Link>
  );
}
