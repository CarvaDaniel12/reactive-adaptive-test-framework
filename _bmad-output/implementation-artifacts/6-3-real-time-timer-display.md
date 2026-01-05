# Story 6.3: Real-Time Timer Display

Status: ready-for-dev

## Story

As a QA (Ana),
I want to see the timer counting in real-time,
So that I'm aware of time spent.

## Acceptance Criteria

1. **Given** user has an active workflow with timer running
   **When** the UI renders
   **Then** header displays current step timer: "Step: 00:15:32"

2. **Given** user has an active workflow with timer running
   **When** the UI renders
   **Then** header displays total workflow timer: "Total: 01:23:45"

3. **Given** timer is running
   **When** UI updates
   **Then** visual indicator shows timer is running (pulsing dot)

4. **Given** timer display exists
   **When** time passes
   **Then** timer updates every second

5. **Given** timer component exists
   **When** implemented
   **Then** timer uses efficient React state (no unnecessary re-renders)

6. **Given** workflow is active
   **When** UI renders
   **Then** timer display is always visible during workflow

## Tasks

- [ ] Task 1: Create TimerDisplay component
- [ ] Task 2: Implement useTimer hook with interval
- [ ] Task 3: Create pulsing indicator animation
- [ ] Task 4: Optimize re-renders with useMemo
- [ ] Task 5: Add timer to WorkflowHeader
- [ ] Task 6: Format time as HH:MM:SS

## Dev Notes

### useTimer Hook

```tsx
// frontend/src/hooks/useTimer.ts
import { useState, useEffect, useRef, useCallback } from "react";

interface UseTimerOptions {
  startTime: Date | null;
  pausedAt?: Date | null;
  totalPausedSeconds?: number;
  isRunning: boolean;
}

export function useTimer({
  startTime,
  pausedAt,
  totalPausedSeconds = 0,
  isRunning,
}: UseTimerOptions) {
  const [elapsed, setElapsed] = useState(0);
  const intervalRef = useRef<NodeJS.Timeout | null>(null);

  const calculateElapsed = useCallback(() => {
    if (!startTime) return 0;
    
    const now = pausedAt || new Date();
    const totalMs = now.getTime() - startTime.getTime();
    const totalSeconds = Math.floor(totalMs / 1000);
    
    return Math.max(0, totalSeconds - totalPausedSeconds);
  }, [startTime, pausedAt, totalPausedSeconds]);

  useEffect(() => {
    if (isRunning && startTime) {
      // Initial calculation
      setElapsed(calculateElapsed());
      
      // Update every second
      intervalRef.current = setInterval(() => {
        setElapsed(calculateElapsed());
      }, 1000);
    }

    return () => {
      if (intervalRef.current) {
        clearInterval(intervalRef.current);
      }
    };
  }, [isRunning, startTime, calculateElapsed]);

  return elapsed;
}

export function formatTime(seconds: number): string {
  const hours = Math.floor(seconds / 3600);
  const minutes = Math.floor((seconds % 3600) / 60);
  const secs = seconds % 60;

  const pad = (n: number) => n.toString().padStart(2, "0");

  if (hours > 0) {
    return `${pad(hours)}:${pad(minutes)}:${pad(secs)}`;
  }
  return `${pad(minutes)}:${pad(secs)}`;
}
```

### TimerDisplay Component

```tsx
// frontend/src/components/workflow/TimerDisplay.tsx
import { useTimer, formatTime } from "@/hooks/useTimer";
import { cn } from "@/lib/utils";

interface TimerDisplayProps {
  label: string;
  startTime: Date | null;
  pausedAt?: Date | null;
  totalPausedSeconds?: number;
  isRunning: boolean;
  size?: "sm" | "md" | "lg";
}

export function TimerDisplay({
  label,
  startTime,
  pausedAt,
  totalPausedSeconds,
  isRunning,
  size = "md",
}: TimerDisplayProps) {
  const elapsed = useTimer({
    startTime,
    pausedAt,
    totalPausedSeconds,
    isRunning,
  });

  const sizeClasses = {
    sm: "text-sm",
    md: "text-base",
    lg: "text-lg",
  };

  return (
    <div className={cn("flex items-center gap-2", sizeClasses[size])}>
      {/* Pulsing indicator */}
      {isRunning && (
        <span className="relative flex h-2 w-2">
          <span className="animate-ping absolute inline-flex h-full w-full rounded-full bg-success-400 opacity-75" />
          <span className="relative inline-flex rounded-full h-2 w-2 bg-success-500" />
        </span>
      )}
      
      {!isRunning && pausedAt && (
        <span className="h-2 w-2 rounded-full bg-warning-500" />
      )}

      <span className="text-neutral-500">{label}:</span>
      <span className={cn(
        "font-mono font-medium",
        isRunning ? "text-neutral-900" : "text-neutral-500"
      )}>
        {formatTime(elapsed)}
      </span>
    </div>
  );
}
```

### WorkflowHeader with Timers

```tsx
// frontend/src/components/workflow/WorkflowHeader.tsx (updated)
export function WorkflowHeader({ onExit }: WorkflowHeaderProps) {
  const { stepTimer, totalTimer, status } = useWorkflowStore();
  const isRunning = status === "active";

  return (
    <div className="bg-white border-b border-neutral-200 px-4 py-3">
      <div className="flex items-center justify-between">
        {/* Left: Ticket Info */}
        {/* ... */}

        {/* Center: Timers */}
        <div className="flex items-center gap-6">
          <TimerDisplay
            label="Step"
            startTime={stepTimer.startTime}
            pausedAt={stepTimer.pausedAt}
            totalPausedSeconds={stepTimer.totalPausedSeconds}
            isRunning={isRunning}
          />
          <TimerDisplay
            label="Total"
            startTime={totalTimer.startTime}
            pausedAt={totalTimer.pausedAt}
            totalPausedSeconds={totalTimer.totalPausedSeconds}
            isRunning={isRunning}
          />
        </div>

        {/* Right: Actions */}
        {/* ... */}
      </div>
    </div>
  );
}
```

### CSS Animation

```css
/* Already using Tailwind's animate-ping, but custom if needed */
@keyframes pulse-dot {
  0%, 100% { opacity: 1; }
  50% { opacity: 0.5; }
}
```

### Performance Optimization

- Use `useCallback` for calculation functions
- Memoize formatted time with `useMemo`
- Single interval per timer component
- Clean up intervals on unmount

### References

- [Source: epics.md#Story 6.3]
