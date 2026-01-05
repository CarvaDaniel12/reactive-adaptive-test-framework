/**
 * Layout state management store.
 *
 * Manages sidebar collapse state with persistence to localStorage.
 */
import { create } from "zustand";
import { persist } from "zustand/middleware";

interface LayoutState {
  /** Whether the sidebar is collapsed (64px) or expanded (240px) */
  sidebarCollapsed: boolean;
  /** Current page title shown in header */
  pageTitle: string;
  /** Current page subtitle shown in header */
  pageSubtitle: string;
}

interface LayoutActions {
  /** Toggle sidebar between collapsed and expanded */
  toggleSidebar: () => void;
  /** Set sidebar collapsed state directly */
  setSidebarCollapsed: (collapsed: boolean) => void;
  /** Set the current page title */
  setPageTitle: (title: string, subtitle?: string) => void;
}

export const useLayoutStore = create<LayoutState & LayoutActions>()(
  persist(
    (set) => ({
      // Initial state
      sidebarCollapsed: false,
      pageTitle: "Dashboard",
      pageSubtitle: "",

      // Actions
      toggleSidebar: () =>
        set((state) => ({
          sidebarCollapsed: !state.sidebarCollapsed,
        })),

      setSidebarCollapsed: (collapsed) =>
        set({ sidebarCollapsed: collapsed }),

      setPageTitle: (title, subtitle = "") =>
        set({ pageTitle: title, pageSubtitle: subtitle }),
    }),
    {
      name: "qa-pms-layout",
      // Only persist sidebar state, not page title
      partialize: (state) => ({ sidebarCollapsed: state.sidebarCollapsed }),
    }
  )
);
