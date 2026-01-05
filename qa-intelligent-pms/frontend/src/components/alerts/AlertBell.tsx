/**
 * Alert bell icon with unread count badge.
 * Story 9.4: Shows unread alert count in header.
 */
import { useUnreadAlertCount } from "@/hooks/useAlerts";

interface AlertBellProps {
  onClick: () => void;
}

export function AlertBell({ onClick }: AlertBellProps) {
  const { data: count = 0 } = useUnreadAlertCount();

  return (
    <button
      type="button"
      onClick={onClick}
      className="relative p-2 text-neutral-500 hover:text-neutral-700 hover:bg-neutral-100 rounded-lg transition-colors"
      title={count > 0 ? `${count} unread alerts` : "No alerts"}
    >
      <BellIcon className="w-5 h-5" />
      {count > 0 && (
        <span className="absolute -top-0.5 -right-0.5 flex items-center justify-center min-w-[18px] h-[18px] px-1 text-xs font-bold text-white bg-red-500 rounded-full">
          {count > 99 ? "99+" : count}
        </span>
      )}
    </button>
  );
}

function BellIcon({ className }: { className?: string }) {
  return (
    <svg className={className} viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth={1.5}>
      <path strokeLinecap="round" strokeLinejoin="round" d="M14.857 17.082a23.848 23.848 0 005.454-1.31A8.967 8.967 0 0118 9.75v-.7V9A6 6 0 006 9v.75a8.967 8.967 0 01-2.312 6.022c1.733.64 3.56 1.085 5.455 1.31m5.714 0a24.255 24.255 0 01-5.714 0m5.714 0a3 3 0 11-5.714 0" />
    </svg>
  );
}
