import { useState } from "react";
import * as Dialog from "@radix-ui/react-dialog";
import { useWorkflowTemplates, formatDuration, type WorkflowTemplate } from "@/hooks/useWorkflow";

// SVG Icons
function CrossIcon({ className }: { className?: string }) {
  return (
    <svg className={className} viewBox="0 0 15 15" fill="none" xmlns="http://www.w3.org/2000/svg">
      <path
        d="M11.7816 4.03157C12.0062 3.80702 12.0062 3.44295 11.7816 3.2184C11.5571 2.99385 11.193 2.99385 10.9685 3.2184L7.50005 6.68682L4.03164 3.2184C3.80708 2.99385 3.44301 2.99385 3.21846 3.2184C2.99391 3.44295 2.99391 3.80702 3.21846 4.03157L6.68688 7.49999L3.21846 10.9684C2.99391 11.193 2.99391 11.557 3.21846 11.7816C3.44301 12.0061 3.80708 12.0061 4.03164 11.7816L7.50005 8.31316L10.9685 11.7816C11.193 12.0061 11.5571 12.0061 11.7816 11.7816C12.0062 11.557 12.0062 11.193 11.7816 10.9684L8.31322 7.49999L11.7816 4.03157Z"
        fill="currentColor"
        fillRule="evenodd"
        clipRule="evenodd"
      />
    </svg>
  );
}

function BugIcon({ className }: { className?: string }) {
  return (
    <svg className={className} viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2">
      <path strokeLinecap="round" strokeLinejoin="round" d="M12 12.75c1.148 0 2.278.08 3.383.237 1.037.146 1.866.966 1.866 2.013 0 3.728-2.35 6.75-5.25 6.75S6.75 18.728 6.75 15c0-1.046.83-1.867 1.866-2.013A24.204 24.204 0 0112 12.75zm0 0c2.883 0 5.647.508 8.207 1.44a23.91 23.91 0 01-1.152 6.06M12 12.75c-2.883 0-5.647.508-8.208 1.44.125 2.104.52 4.136 1.153 6.06M12 12.75a2.25 2.25 0 002.248-2.354M12 12.75a2.25 2.25 0 01-2.248-2.354M12 8.25c.995 0 1.971-.08 2.922-.236.403-.066.74-.358.795-.762a3.778 3.778 0 00-.399-2.25M12 8.25c-.995 0-1.97-.08-2.922-.236-.402-.066-.74-.358-.795-.762a3.734 3.734 0 01.4-2.253M12 8.25a2.25 2.25 0 00-2.248 2.146M12 8.25a2.25 2.25 0 012.248 2.146M8.683 5a6.032 6.032 0 01-1.155-1.002c.07-.63.27-1.222.574-1.747m.581 2.749A3.75 3.75 0 0115.318 5m0 0c.427-.283.815-.62 1.155-.999a4.471 4.471 0 00-.575-1.752M4.921 6a24.048 24.048 0 00-.392 3.314c1.668.546 3.416.914 5.223 1.082M19.08 6c.205 1.08.337 2.187.392 3.314a23.882 23.882 0 01-5.223 1.082" />
    </svg>
  );
}

function SparklesIcon({ className }: { className?: string }) {
  return (
    <svg className={className} viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2">
      <path strokeLinecap="round" strokeLinejoin="round" d="M9.813 15.904L9 18.75l-.813-2.846a4.5 4.5 0 00-3.09-3.09L2.25 12l2.846-.813a4.5 4.5 0 003.09-3.09L9 5.25l.813 2.846a4.5 4.5 0 003.09 3.09L15.75 12l-2.846.813a4.5 4.5 0 00-3.09 3.09zM18.259 8.715L18 9.75l-.259-1.035a3.375 3.375 0 00-2.455-2.456L14.25 6l1.036-.259a3.375 3.375 0 002.455-2.456L18 2.25l.259 1.035a3.375 3.375 0 002.456 2.456L21.75 6l-1.035.259a3.375 3.375 0 00-2.456 2.456zM16.894 20.567L16.5 21.75l-.394-1.183a2.25 2.25 0 00-1.423-1.423L13.5 18.75l1.183-.394a2.25 2.25 0 001.423-1.423l.394-1.183.394 1.183a2.25 2.25 0 001.423 1.423l1.183.394-1.183.394a2.25 2.25 0 00-1.423 1.423z" />
    </svg>
  );
}

function ArrowPathIcon({ className }: { className?: string }) {
  return (
    <svg className={className} viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2">
      <path strokeLinecap="round" strokeLinejoin="round" d="M16.023 9.348h4.992v-.001M2.985 19.644v-4.992m0 0h4.992m-4.993 0l3.181 3.183a8.25 8.25 0 0013.803-3.7M4.031 9.865a8.25 8.25 0 0113.803-3.7l3.181 3.182m0-4.991v4.99" />
    </svg>
  );
}

function ClockIcon({ className }: { className?: string }) {
  return (
    <svg className={className} viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2">
      <path strokeLinecap="round" strokeLinejoin="round" d="M12 6v6h4.5m4.5 0a9 9 0 11-18 0 9 9 0 0118 0z" />
    </svg>
  );
}

interface TemplateSelectionDialogProps {
  open: boolean;
  onOpenChange: (open: boolean) => void;
  onSelect: (template: WorkflowTemplate) => void;
  loading?: boolean;
}

/**
 * Dialog for selecting a workflow template.
 */
export function TemplateSelectionDialog({
  open,
  onOpenChange,
  onSelect,
  loading = false,
}: TemplateSelectionDialogProps) {
  const { data: templates, isLoading } = useWorkflowTemplates();
  const [selectedId, setSelectedId] = useState<string | null>(null);

  const handleConfirm = () => {
    const template = templates?.find((t) => t.id === selectedId);
    if (template) {
      onSelect(template);
    }
  };

  const getIcon = (ticketType: string) => {
    switch (ticketType) {
      case "bug":
        return <BugIcon className="w-6 h-6 text-red-500" />;
      case "feature":
        return <SparklesIcon className="w-6 h-6 text-blue-500" />;
      case "regression":
        return <ArrowPathIcon className="w-6 h-6 text-amber-500" />;
      default:
        return <ClockIcon className="w-6 h-6 text-neutral-500" />;
    }
  };

  return (
    <Dialog.Root open={open} onOpenChange={onOpenChange}>
      <Dialog.Portal>
        <Dialog.Overlay className="fixed inset-0 bg-black/50 animate-fade-in z-50" />
        <Dialog.Content
          className="fixed top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2
                     bg-white rounded-lg shadow-xl w-full max-w-lg p-6 z-50
                     animate-scale-in focus:outline-none"
        >
          <Dialog.Title className="text-lg font-semibold text-neutral-900 mb-2">
            Start Workflow
          </Dialog.Title>

          <Dialog.Description className="text-sm text-neutral-500 mb-4">
            Select a workflow template to guide your testing process.
          </Dialog.Description>

          {/* Templates List */}
          {isLoading ? (
            <div className="space-y-3">
              {[1, 2, 3].map((i) => (
                <div key={i} className="h-24 bg-neutral-100 rounded-lg animate-pulse" />
              ))}
            </div>
          ) : (
            <div className="space-y-3 max-h-80 overflow-y-auto">
              {templates?.map((template) => (
                <button
                  key={template.id}
                  onClick={() => setSelectedId(template.id)}
                  className={`w-full text-left p-4 border rounded-lg transition-colors
                    ${selectedId === template.id
                      ? "border-blue-500 bg-blue-50"
                      : "border-neutral-200 hover:border-blue-300"
                    }`}
                >
                  <div className="flex items-start gap-3">
                    <div className="flex-shrink-0 mt-0.5">
                      {getIcon(template.ticketType)}
                    </div>
                    <div className="flex-1 min-w-0">
                      <h4 className="font-medium text-neutral-900">{template.name}</h4>
                      {template.description && (
                        <p className="text-sm text-neutral-500 mt-1 line-clamp-2">
                          {template.description}
                        </p>
                      )}
                      <div className="flex items-center gap-4 mt-2 text-xs text-neutral-400">
                        <span>{template.stepCount} steps</span>
                        <span className="flex items-center gap-1">
                          <ClockIcon className="w-3 h-3" />
                          {formatDuration(template.estimatedMinutes)}
                        </span>
                      </div>
                    </div>
                  </div>
                </button>
              ))}
            </div>
          )}

          {/* Actions */}
          <div className="flex justify-end gap-3 mt-6">
            <Dialog.Close asChild>
              <button
                className="px-4 py-2 text-sm font-medium text-neutral-600 
                           hover:text-neutral-800 transition-colors"
              >
                Cancel
              </button>
            </Dialog.Close>
            <button
              onClick={handleConfirm}
              disabled={!selectedId || loading}
              className="flex items-center gap-2 px-4 py-2 bg-blue-500 text-white 
                         font-medium rounded-lg hover:bg-blue-600 transition-colors
                         disabled:opacity-50 disabled:cursor-not-allowed"
            >
              {loading ? (
                <>
                  <div className="w-4 h-4 border-2 border-white/30 border-t-white rounded-full animate-spin" />
                  Starting...
                </>
              ) : (
                "Start Workflow"
              )}
            </button>
          </div>

          {/* Close Button */}
          <Dialog.Close asChild>
            <button
              className="absolute top-4 right-4 p-1 text-neutral-400 
                         hover:text-neutral-600 transition-colors"
              aria-label="Close"
            >
              <CrossIcon className="w-5 h-5" />
            </button>
          </Dialog.Close>
        </Dialog.Content>
      </Dialog.Portal>
    </Dialog.Root>
  );
}
