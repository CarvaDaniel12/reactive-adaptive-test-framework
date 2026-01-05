import { create } from "zustand";

/** Toast variant types */
export type ToastVariant = "success" | "error" | "warning" | "info";

/** Toast action button */
export interface ToastAction {
  label: string;
  onClick: () => void;
}

/** Toast notification */
export interface Toast {
  id: string;
  title: string;
  description?: string;
  variant: ToastVariant;
  action?: ToastAction;
}

/** Toast input (without id) */
export type ToastInput = Omit<Toast, "id">;

/** Toast store state */
interface ToastStore {
  toasts: Toast[];
  toast: (input: ToastInput) => string;
  dismiss: (id: string) => void;
  dismissAll: () => void;
}

/** Auto-dismiss timeout in milliseconds */
const AUTO_DISMISS_TIMEOUT = 5000;

/** Generate unique ID */
function generateId(): string {
  return Math.random().toString(36).slice(2, 11);
}

/**
 * Toast store for managing notifications.
 * 
 * Usage:
 * ```tsx
 * const { toast, dismiss } = useToast();
 * 
 * // Success toast
 * toast({ title: "Success!", variant: "success" });
 * 
 * // Error toast with action
 * toast({
 *   title: "Failed",
 *   description: "Something went wrong",
 *   variant: "error",
 *   action: { label: "Retry", onClick: () => retry() }
 * });
 * ```
 */
export const useToast = create<ToastStore>((set, get) => ({
  toasts: [],

  toast: (input: ToastInput) => {
    const id = generateId();
    const newToast: Toast = { ...input, id };

    set((state) => ({
      toasts: [...state.toasts, newToast],
    }));

    // Auto-dismiss after timeout (unless it has an action)
    if (!input.action) {
      setTimeout(() => {
        get().dismiss(id);
      }, AUTO_DISMISS_TIMEOUT);
    }

    return id;
  },

  dismiss: (id: string) => {
    set((state) => ({
      toasts: state.toasts.filter((t) => t.id !== id),
    }));
  },

  dismissAll: () => {
    set({ toasts: [] });
  },
}));

/** Convenience functions for common toast types */
export const toastSuccess = (title: string, description?: string) =>
  useToast.getState().toast({ title, description, variant: "success" });

export const toastError = (title: string, description?: string, action?: ToastAction) =>
  useToast.getState().toast({ title, description, variant: "error", action });

export const toastWarning = (title: string, description?: string) =>
  useToast.getState().toast({ title, description, variant: "warning" });

export const toastInfo = (title: string, description?: string) =>
  useToast.getState().toast({ title, description, variant: "info" });
