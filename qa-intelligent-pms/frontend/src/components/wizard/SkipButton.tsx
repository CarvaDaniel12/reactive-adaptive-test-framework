/**
 * Skip button with tooltip warning.
 *
 * Used for optional steps to allow users to skip configuration.
 */
import * as Tooltip from "@radix-ui/react-tooltip";

interface SkipButtonProps {
  /** Callback when skip is clicked */
  onSkip: () => void;
  /** Whether the button is disabled */
  disabled?: boolean;
}

export function SkipButton({ onSkip, disabled = false }: SkipButtonProps) {
  return (
    <Tooltip.Provider delayDuration={200}>
      <Tooltip.Root>
        <Tooltip.Trigger asChild>
          <button
            type="button"
            onClick={onSkip}
            disabled={disabled}
            className="text-neutral-500 hover:text-neutral-700 underline text-sm
              disabled:opacity-50 disabled:cursor-not-allowed
              transition-colors duration-150"
          >
            Skip this step
          </button>
        </Tooltip.Trigger>
        <Tooltip.Portal>
          <Tooltip.Content
            className="bg-neutral-900 text-white text-sm px-3 py-2 rounded shadow-lg
              max-w-xs animate-in fade-in-0 zoom-in-95"
            sideOffset={5}
          >
            You can configure this later in Settings
            <Tooltip.Arrow className="fill-neutral-900" />
          </Tooltip.Content>
        </Tooltip.Portal>
      </Tooltip.Root>
    </Tooltip.Provider>
  );
}
