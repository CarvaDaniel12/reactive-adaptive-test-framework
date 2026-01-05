import * as Toast from "@radix-ui/react-toast";
import { useToast, type ToastVariant } from "@/hooks/useToast";

const VARIANT_STYLES: Record<ToastVariant, string> = {
  success: "bg-success-50 border-success-200 text-success-900",
  error: "bg-error-50 border-error-200 text-error-900",
  warning: "bg-warning-50 border-warning-200 text-warning-900",
  info: "bg-primary-50 border-primary-200 text-primary-900",
};

const VARIANT_ICONS: Record<ToastVariant, React.ReactNode> = {
  success: (
    <svg className="w-5 h-5 text-success-600" fill="none" viewBox="0 0 24 24" stroke="currentColor">
      <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M5 13l4 4L19 7" />
    </svg>
  ),
  error: (
    <svg className="w-5 h-5 text-error-600" fill="none" viewBox="0 0 24 24" stroke="currentColor">
      <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M6 18L18 6M6 6l12 12" />
    </svg>
  ),
  warning: (
    <svg className="w-5 h-5 text-warning-600" fill="none" viewBox="0 0 24 24" stroke="currentColor">
      <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z" />
    </svg>
  ),
  info: (
    <svg className="w-5 h-5 text-primary-600" fill="none" viewBox="0 0 24 24" stroke="currentColor">
      <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z" />
    </svg>
  ),
};

export function ToastProvider({ children }: { children: React.ReactNode }) {
  const { toasts, dismiss } = useToast();

  return (
    <Toast.Provider swipeDirection="right">
      {children}

      {toasts.map((toast) => (
        <Toast.Root
          key={toast.id}
          className={`
            flex items-start gap-3 p-4 rounded-lg border shadow-lg
            data-[state=open]:animate-slide-in-up
            data-[state=closed]:animate-fade-out
            data-[swipe=move]:translate-x-[var(--radix-toast-swipe-move-x)]
            data-[swipe=cancel]:translate-x-0 data-[swipe=cancel]:transition-transform
            data-[swipe=end]:animate-swipe-out
            ${VARIANT_STYLES[toast.variant]}
          `}
          onOpenChange={(open) => {
            if (!open) dismiss(toast.id);
          }}
        >
          {/* Icon */}
          <div className="flex-shrink-0 mt-0.5">
            {VARIANT_ICONS[toast.variant]}
          </div>

          {/* Content */}
          <div className="flex-1 min-w-0">
            <Toast.Title className="font-medium text-sm">
              {toast.title}
            </Toast.Title>
            {toast.description && (
              <Toast.Description className="text-sm opacity-80 mt-1">
                {toast.description}
              </Toast.Description>
            )}
            {toast.action && (
              <Toast.Action
                altText={toast.action.label}
                className="mt-2 text-sm font-medium underline underline-offset-2 cursor-pointer hover:opacity-80"
                onClick={toast.action.onClick}
              >
                {toast.action.label}
              </Toast.Action>
            )}
          </div>

          {/* Close button */}
          <Toast.Close
            className="flex-shrink-0 p-1 rounded hover:bg-black/5 transition-colors"
            aria-label="Close"
          >
            <svg className="w-4 h-4 opacity-60" fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M6 18L18 6M6 6l12 12" />
            </svg>
          </Toast.Close>
        </Toast.Root>
      ))}

      <Toast.Viewport className="fixed bottom-4 right-4 flex flex-col gap-2 w-96 max-w-[calc(100vw-2rem)] z-50 outline-none" />
    </Toast.Provider>
  );
}
