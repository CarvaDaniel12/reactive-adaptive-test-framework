/**
 * Main application layout with collapsible sidebar.
 *
 * Implements the Hybrid Adaptive layout from the UX design:
 * - 64px collapsed sidebar (icons only)
 * - 240px expanded sidebar (icons + labels)
 * - 300ms ease-in-out animations
 * - State persisted to localStorage
 */
import { Outlet } from "react-router-dom";
import { Sidebar } from "./Sidebar";
import { Header } from "./Header";
import { useLayoutStore } from "@/stores/layoutStore";
import { useKeyboardShortcuts } from "@/hooks/useKeyboardShortcuts";
import { AIChatbot } from "@/components/ai";

export function MainLayout() {
  const { sidebarCollapsed } = useLayoutStore();

  // Enable global keyboard shortcuts
  useKeyboardShortcuts();

  return (
    <div className="min-h-screen bg-neutral-50 flex">
      {/* Skip link for accessibility */}
      <a
        href="#main-content"
        className="sr-only focus:not-sr-only focus:absolute focus:top-4 focus:left-4 
                   bg-primary-500 text-white px-4 py-2 rounded-lg z-50"
      >
        Skip to main content
      </a>

      {/* Sidebar - Fixed position */}
      <Sidebar />

      {/* Main Content Area - Offset by sidebar width */}
      <div
        className={`
          flex-1 flex flex-col min-h-screen
          transition-all duration-300 ease-in-out
          ${sidebarCollapsed ? "ml-16" : "ml-60"}
        `}
      >
        {/* Header */}
        <Header />

        {/* Page Content */}
        <main id="main-content" className="flex-1 p-6 overflow-auto">
          <Outlet />
        </main>
      </div>

      {/* AI Chatbot */}
      <AIChatbot />
    </div>
  );
}
