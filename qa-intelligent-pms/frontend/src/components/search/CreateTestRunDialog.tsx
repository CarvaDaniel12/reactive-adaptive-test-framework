import { useState } from "react";
import * as Dialog from "@radix-ui/react-dialog";
import { useCreateTestRun } from "@/hooks/useCreateTestRun";

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

function CheckIcon({ className }: { className?: string }) {
  return (
    <svg className={className} viewBox="0 0 15 15" fill="none" xmlns="http://www.w3.org/2000/svg">
      <path
        d="M11.4669 3.72684C11.7558 3.91574 11.8369 4.30308 11.648 4.59198L7.39799 11.092C7.29783 11.2452 7.13556 11.3467 6.95402 11.3699C6.77247 11.3931 6.58989 11.3355 6.45446 11.2124L3.70446 8.71241C3.44905 8.48022 3.43023 8.08494 3.66242 7.82953C3.89461 7.57412 4.28989 7.5553 4.5453 7.78749L6.75292 9.79441L10.6018 3.90792C10.7907 3.61902 11.178 3.53795 11.4669 3.72684Z"
        fill="currentColor"
        fillRule="evenodd"
        clipRule="evenodd"
      />
    </svg>
  );
}

function RocketIcon({ className }: { className?: string }) {
  return (
    <svg className={className} viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2">
      <path strokeLinecap="round" strokeLinejoin="round" d="M15.59 14.37a6 6 0 01-5.84 7.38v-4.8m5.84-2.58a14.98 14.98 0 006.16-12.12A14.98 14.98 0 009.631 8.41m5.96 5.96a14.926 14.926 0 01-5.841 2.58m-.119-8.54a6 6 0 00-7.381 5.84h4.8m2.581-5.84a14.927 14.927 0 00-2.58 5.84m2.699 2.7c-.103.021-.207.041-.311.06a15.09 15.09 0 01-2.448-2.448 14.9 14.9 0 01.06-.312m-2.24 2.39a4.493 4.493 0 00-1.757 4.306 4.493 4.493 0 004.306-1.758M16.5 9a1.5 1.5 0 11-3 0 1.5 1.5 0 013 0z" />
    </svg>
  );
}

interface SelectedTestCase {
  id: number;
  name: string;
}

interface CreateTestRunDialogProps {
  open: boolean;
  onOpenChange: (open: boolean) => void;
  ticketKey: string;
  selectedCases: SelectedTestCase[];
  onSuccess?: () => void;
}

/**
 * Dialog for creating a new Testmo test run.
 *
 * Shows selected test cases and allows customizing the run name.
 * Uses Radix UI Dialog for accessibility.
 */
export function CreateTestRunDialog({
  open,
  onOpenChange,
  ticketKey,
  selectedCases,
  onSuccess,
}: CreateTestRunDialogProps) {
  const [customName, setCustomName] = useState("");
  const createTestRun = useCreateTestRun();

  // Generate default name: QA-{ticket}-{YYYY-MM-DD}
  const defaultName = `QA-${ticketKey}-${new Date().toISOString().split("T")[0]}`;

  const handleCreate = async () => {
    try {
      await createTestRun.mutateAsync({
        ticketKey,
        caseIds: selectedCases.map((c) => c.id),
        customName: customName || undefined,
      });

      onOpenChange(false);
      setCustomName("");
      onSuccess?.();
    } catch {
      // Error handled by mutation
    }
  };

  return (
    <Dialog.Root open={open} onOpenChange={onOpenChange}>
      <Dialog.Portal>
        <Dialog.Overlay className="fixed inset-0 bg-black/50 animate-fade-in z-50" />
        <Dialog.Content
          className="fixed top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2
                     bg-white rounded-lg shadow-xl w-full max-w-md p-6 z-50
                     animate-scale-in focus:outline-none"
        >
          <Dialog.Title className="text-lg font-semibold text-neutral-900 mb-2">
            Create Test Run
          </Dialog.Title>

          <Dialog.Description className="text-sm text-neutral-500 mb-4">
            Create a new test run in Testmo with the selected test cases.
          </Dialog.Description>

          {/* Run Name Input */}
          <div className="mb-4">
            <label
              htmlFor="runName"
              className="block text-sm font-medium text-neutral-700 mb-1"
            >
              Run Name
            </label>
            <input
              id="runName"
              type="text"
              value={customName}
              onChange={(e) => setCustomName(e.target.value)}
              placeholder={defaultName}
              className="w-full px-3 py-2 border border-neutral-300 rounded-lg
                         focus:outline-none focus:ring-2 focus:ring-blue-500
                         focus:border-transparent"
            />
            <p className="text-xs text-neutral-400 mt-1">
              Leave empty to use default: {defaultName}
            </p>
          </div>

          {/* Selected Cases */}
          <div className="mb-6">
            <h4 className="text-sm font-medium text-neutral-700 mb-2">
              Selected Test Cases ({selectedCases.length})
            </h4>
            <div className="max-h-40 overflow-y-auto border border-neutral-200 rounded-lg">
              {selectedCases.map((testCase) => (
                <div
                  key={testCase.id}
                  className="flex items-center gap-2 px-3 py-2 border-b border-neutral-100 
                             last:border-b-0 text-sm"
                >
                  <CheckIcon className="w-4 h-4 text-green-500 flex-shrink-0" />
                  <span className="truncate">{testCase.name}</span>
                </div>
              ))}
            </div>
          </div>

          {/* Actions */}
          <div className="flex justify-end gap-3">
            <Dialog.Close asChild>
              <button
                className="px-4 py-2 text-sm font-medium text-neutral-600 
                           hover:text-neutral-800 transition-colors"
              >
                Cancel
              </button>
            </Dialog.Close>
            <button
              onClick={handleCreate}
              disabled={createTestRun.isPending || selectedCases.length === 0}
              className="flex items-center gap-2 px-4 py-2 bg-blue-500 text-white 
                         font-medium rounded-lg hover:bg-blue-600 transition-colors
                         disabled:opacity-50 disabled:cursor-not-allowed"
            >
              {createTestRun.isPending ? (
                <>
                  <div className="w-4 h-4 border-2 border-white/30 border-t-white rounded-full animate-spin" />
                  Creating...
                </>
              ) : (
                <>
                  <RocketIcon className="w-4 h-4" />
                  Create Run
                </>
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
