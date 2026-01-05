/**
 * Application header component.
 *
 * Displays current page context, timer slot, alerts, and user menu.
 * Story 9.4: Alert bell with unread count badge.
 */
import { useState } from "react";
import * as DropdownMenu from "@radix-ui/react-dropdown-menu";
import * as Popover from "@radix-ui/react-popover";
import { useLayoutStore } from "@/stores/layoutStore";
import { Link } from "react-router-dom";
import { AlertBell } from "@/components/alerts/AlertBell";
import { AlertList } from "@/components/alerts/AlertList";

export function Header() {
  const { pageTitle, pageSubtitle } = useLayoutStore();
  const [alertsOpen, setAlertsOpen] = useState(false);

  return (
    <header className="h-16 bg-white border-b border-neutral-200 px-6 flex items-center justify-between">
      {/* Left: Page Context */}
      <div>
        <h1 className="text-lg font-semibold text-neutral-900">{pageTitle}</h1>
        {pageSubtitle && (
          <p className="text-sm text-neutral-500">{pageSubtitle}</p>
        )}
      </div>

      {/* Center: Timer Slot (for future workflow timer) */}
      <div id="header-timer-slot" className="flex items-center" />

      {/* Right: Alerts + User Menu */}
      <div className="flex items-center gap-2">
        {/* Alert Bell with Popover (Story 9.4) */}
        <Popover.Root open={alertsOpen} onOpenChange={setAlertsOpen}>
          <Popover.Trigger asChild>
            <div>
              <AlertBell onClick={() => setAlertsOpen(!alertsOpen)} />
            </div>
          </Popover.Trigger>
          <Popover.Portal>
            <Popover.Content
              className="bg-white rounded-lg shadow-xl border border-neutral-200 w-96 max-h-[500px] overflow-hidden z-50"
              sideOffset={8}
              align="end"
            >
              <div className="p-3 border-b border-neutral-100 flex items-center justify-between">
                <h3 className="font-semibold text-neutral-900">Alerts</h3>
                <Link
                  to="/patterns"
                  className="text-xs text-primary-600 hover:underline"
                  onClick={() => setAlertsOpen(false)}
                >
                  View History
                </Link>
              </div>
              <AlertList />
              <Popover.Arrow className="fill-white" />
            </Popover.Content>
          </Popover.Portal>
        </Popover.Root>

        {/* User Menu */}
        <DropdownMenu.Root>
        <DropdownMenu.Trigger asChild>
          <button
            type="button"
            className="flex items-center gap-2 p-2 rounded-lg hover:bg-neutral-100 transition-colors"
          >
            <div className="w-8 h-8 bg-primary-100 text-primary-600 rounded-full flex items-center justify-center">
              <PersonIcon className="w-4 h-4" />
            </div>
          </button>
        </DropdownMenu.Trigger>

        <DropdownMenu.Portal>
          <DropdownMenu.Content
            className="bg-white rounded-lg shadow-lg border border-neutral-200 py-1 min-w-[160px] z-50"
            sideOffset={8}
            align="end"
          >
            <DropdownMenu.Item asChild>
              <Link
                to="/settings"
                className="px-3 py-2 text-sm text-neutral-600 hover:bg-neutral-100 cursor-pointer flex items-center gap-2 outline-none"
              >
                <SettingsIcon className="w-4 h-4" />
                Settings
              </Link>
            </DropdownMenu.Item>
            <DropdownMenu.Separator className="h-px bg-neutral-200 my-1" />
            <DropdownMenu.Item
              className="px-3 py-2 text-sm text-error-500 hover:bg-error-50 cursor-pointer flex items-center gap-2 outline-none"
              onSelect={() => {
                // Clear setup and redirect to setup wizard
                localStorage.removeItem("qa-pms-wizard");
                localStorage.removeItem("qa-pms-layout");
                window.location.href = "/setup";
              }}
            >
              <ExitIcon className="w-4 h-4" />
              Reset Setup
            </DropdownMenu.Item>
          </DropdownMenu.Content>
        </DropdownMenu.Portal>
      </DropdownMenu.Root>
      </div>
    </header>
  );
}

// Icons
function PersonIcon({ className }: { className?: string }) {
  return (
    <svg className={className} viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth={1.5}>
      <path strokeLinecap="round" strokeLinejoin="round" d="M15.75 6a3.75 3.75 0 11-7.5 0 3.75 3.75 0 017.5 0zM4.501 20.118a7.5 7.5 0 0114.998 0A17.933 17.933 0 0112 21.75c-2.676 0-5.216-.584-7.499-1.632z" />
    </svg>
  );
}

function SettingsIcon({ className }: { className?: string }) {
  return (
    <svg className={className} viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth={1.5}>
      <path strokeLinecap="round" strokeLinejoin="round" d="M9.594 3.94c.09-.542.56-.94 1.11-.94h2.593c.55 0 1.02.398 1.11.94l.213 1.281c.063.374.313.686.645.87.074.04.147.083.22.127.324.196.72.257 1.075.124l1.217-.456a1.125 1.125 0 011.37.49l1.296 2.247a1.125 1.125 0 01-.26 1.431l-1.003.827c-.293.24-.438.613-.431.992a6.759 6.759 0 010 .255c-.007.378.138.75.43.99l1.005.828c.424.35.534.954.26 1.43l-1.298 2.247a1.125 1.125 0 01-1.369.491l-1.217-.456c-.355-.133-.75-.072-1.076.124a6.57 6.57 0 01-.22.128c-.331.183-.581.495-.644.869l-.213 1.28c-.09.543-.56.941-1.11.941h-2.594c-.55 0-1.02-.398-1.11-.94l-.213-1.281c-.062-.374-.312-.686-.644-.87a6.52 6.52 0 01-.22-.127c-.325-.196-.72-.257-1.076-.124l-1.217.456a1.125 1.125 0 01-1.369-.49l-1.297-2.247a1.125 1.125 0 01.26-1.431l1.004-.827c.292-.24.437-.613.43-.992a6.932 6.932 0 010-.255c.007-.378-.138-.75-.43-.99l-1.004-.828a1.125 1.125 0 01-.26-1.43l1.297-2.247a1.125 1.125 0 011.37-.491l1.216.456c.356.133.751.072 1.076-.124.072-.044.146-.087.22-.128.332-.183.582-.495.644-.869l.214-1.281z" />
      <path strokeLinecap="round" strokeLinejoin="round" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z" />
    </svg>
  );
}

function ExitIcon({ className }: { className?: string }) {
  return (
    <svg className={className} viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth={1.5}>
      <path strokeLinecap="round" strokeLinejoin="round" d="M15.75 9V5.25A2.25 2.25 0 0013.5 3h-6a2.25 2.25 0 00-2.25 2.25v13.5A2.25 2.25 0 007.5 21h6a2.25 2.25 0 002.25-2.25V15M12 9l-3 3m0 0l3 3m-3-3h12.75" />
    </svg>
  );
}
