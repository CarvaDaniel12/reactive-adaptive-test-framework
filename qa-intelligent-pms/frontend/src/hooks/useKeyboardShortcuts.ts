/**
 * Global keyboard shortcuts hook.
 *
 * Provides keyboard shortcuts for common actions.
 */
import { useEffect } from "react";
import { useLayoutStore } from "@/stores/layoutStore";

/**
 * Hook to enable global keyboard shortcuts.
 *
 * Shortcuts:
 * - Ctrl+Shift+M: Toggle sidebar
 */
export function useKeyboardShortcuts() {
  const toggleSidebar = useLayoutStore((state) => state.toggleSidebar);

  useEffect(() => {
    const handleKeyDown = (e: KeyboardEvent) => {
      // Ignore if user is typing in an input
      const target = e.target as HTMLElement;
      if (
        target.tagName === "INPUT" ||
        target.tagName === "TEXTAREA" ||
        target.isContentEditable
      ) {
        return;
      }

      // Ctrl+Shift+M: Toggle sidebar
      if (e.ctrlKey && e.shiftKey && e.key.toLowerCase() === "m") {
        e.preventDefault();
        toggleSidebar();
      }
    };

    window.addEventListener("keydown", handleKeyDown);
    return () => window.removeEventListener("keydown", handleKeyDown);
  }, [toggleSidebar]);
}
