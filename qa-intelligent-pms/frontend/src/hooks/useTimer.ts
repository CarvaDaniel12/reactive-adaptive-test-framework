import { useState, useEffect, useRef, useCallback } from "react";
import { useQuery, useMutation, useQueryClient } from "@tanstack/react-query";

// ============================================================================
// Types
// ============================================================================

export interface TimeSession {
  id: string;
  workflowInstanceId: string;
  stepIndex: number;
  startedAt: string;
  pausedAt: string | null;
  endedAt: string | null;
  totalSeconds: number;
  isActive: boolean;
}

export interface TimerState {
  stepSeconds: number;
  totalSeconds: number;
  isRunning: boolean;
  isPaused: boolean;
  sessionId: string | null;
}

// ============================================================================
// Timer Hook
// ============================================================================

/**
 * Hook for managing real-time timer display.
 */
export function useTimer(workflowId: string | undefined) {
  const queryClient = useQueryClient();
  const intervalRef = useRef<ReturnType<typeof setInterval> | null>(null);
  const startTimeRef = useRef<number>(0);

  const [timerState, setTimerState] = useState<TimerState>({
    stepSeconds: 0,
    totalSeconds: 0,
    isRunning: false,
    isPaused: false,
    sessionId: null,
  });

  // Fetch active session
  const { data: activeSession } = useQuery({
    queryKey: ["timeSession", workflowId, "active"],
    queryFn: async (): Promise<TimeSession | null> => {
      if (!workflowId) return null;
      const response = await fetch(`/api/v1/time/sessions/${workflowId}/active`);
      if (!response.ok) return null;
      return response.json();
    },
    enabled: !!workflowId,
    refetchInterval: 30000, // Refresh every 30s for sync
  });

  // Fetch all sessions for total time
  const { data: allSessions } = useQuery({
    queryKey: ["timeSessions", workflowId],
    queryFn: async (): Promise<{ sessions: TimeSession[]; totalSeconds: number }> => {
      if (!workflowId) return { sessions: [], totalSeconds: 0 };
      const response = await fetch(`/api/v1/time/sessions/${workflowId}`);
      if (!response.ok) return { sessions: [], totalSeconds: 0 };
      return response.json();
    },
    enabled: !!workflowId,
  });

  // Start timer mutation
  const startMutation = useMutation({
    mutationFn: async ({ stepIndex }: { stepIndex: number }) => {
      const response = await fetch(`/api/v1/time/sessions/${workflowId}/start/${stepIndex}`, {
        method: "POST",
      });
      if (!response.ok) throw new Error("Failed to start timer");
      return response.json() as Promise<TimeSession>;
    },
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ["timeSession", workflowId] });
      queryClient.invalidateQueries({ queryKey: ["timeSessions", workflowId] });
    },
  });

  // End timer mutation
  const endMutation = useMutation({
    mutationFn: async (sessionId: string) => {
      const response = await fetch(`/api/v1/time/sessions/${sessionId}/end`, {
        method: "POST",
      });
      if (!response.ok) throw new Error("Failed to end timer");
      return response.json();
    },
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ["timeSession", workflowId] });
      queryClient.invalidateQueries({ queryKey: ["timeSessions", workflowId] });
    },
  });

  // Pause timer mutation
  const pauseMutation = useMutation({
    mutationFn: async (sessionId: string) => {
      const response = await fetch(`/api/v1/time/sessions/${sessionId}/pause`, {
        method: "POST",
      });
      if (!response.ok) throw new Error("Failed to pause timer");
      return response.json();
    },
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ["timeSession", workflowId] });
    },
  });

  // Resume timer mutation
  const resumeMutation = useMutation({
    mutationFn: async (sessionId: string) => {
      const response = await fetch(`/api/v1/time/sessions/${sessionId}/resume`, {
        method: "POST",
      });
      if (!response.ok) throw new Error("Failed to resume timer");
      return response.json();
    },
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ["timeSession", workflowId] });
    },
  });

  // Update timer state from active session
  useEffect(() => {
    if (activeSession) {
      const isPaused = !!activeSession.pausedAt;
      const isRunning = activeSession.isActive && !isPaused;

      // Calculate elapsed time
      const startTime = new Date(activeSession.startedAt).getTime();
      const now = Date.now();
      const elapsedMs = isPaused
        ? new Date(activeSession.pausedAt!).getTime() - startTime
        : now - startTime;
      const elapsedSeconds = Math.floor(elapsedMs / 1000);

      startTimeRef.current = startTime;
      setTimerState((prev) => ({
        ...prev,
        stepSeconds: elapsedSeconds,
        isRunning,
        isPaused,
        sessionId: activeSession.id,
      }));
    } else {
      setTimerState((prev) => ({
        ...prev,
        stepSeconds: 0,
        isRunning: false,
        isPaused: false,
        sessionId: null,
      }));
    }
  }, [activeSession]);

  // Update total seconds from all sessions
  useEffect(() => {
    if (allSessions) {
      setTimerState((prev) => ({
        ...prev,
        totalSeconds: allSessions.totalSeconds,
      }));
    }
  }, [allSessions]);

  // Real-time timer tick
  useEffect(() => {
    if (timerState.isRunning && !timerState.isPaused) {
      intervalRef.current = setInterval(() => {
        const now = Date.now();
        const elapsed = Math.floor((now - startTimeRef.current) / 1000);
        setTimerState((prev) => ({
          ...prev,
          stepSeconds: elapsed,
        }));
      }, 1000);
    } else if (intervalRef.current) {
      clearInterval(intervalRef.current);
      intervalRef.current = null;
    }

    return () => {
      if (intervalRef.current) {
        clearInterval(intervalRef.current);
      }
    };
  }, [timerState.isRunning, timerState.isPaused]);

  // Actions
  const startTimer = useCallback(
    (stepIndex: number) => {
      startMutation.mutate({ stepIndex });
    },
    [startMutation]
  );

  const endTimer = useCallback(() => {
    if (timerState.sessionId) {
      endMutation.mutate(timerState.sessionId);
    }
  }, [endMutation, timerState.sessionId]);

  const pauseTimer = useCallback(() => {
    if (timerState.sessionId) {
      pauseMutation.mutate(timerState.sessionId);
    }
  }, [pauseMutation, timerState.sessionId]);

  const resumeTimer = useCallback(() => {
    if (timerState.sessionId) {
      resumeMutation.mutate(timerState.sessionId);
    }
  }, [resumeMutation, timerState.sessionId]);

  return {
    ...timerState,
    startTimer,
    endTimer,
    pauseTimer,
    resumeTimer,
    isLoading:
      startMutation.isPending ||
      endMutation.isPending ||
      pauseMutation.isPending ||
      resumeMutation.isPending,
  };
}

// ============================================================================
// Utility Functions
// ============================================================================

/**
 * Format seconds to HH:MM:SS display.
 */
export function formatTime(seconds: number): string {
  const hrs = Math.floor(seconds / 3600);
  const mins = Math.floor((seconds % 3600) / 60);
  const secs = seconds % 60;

  if (hrs > 0) {
    return `${hrs.toString().padStart(2, "0")}:${mins.toString().padStart(2, "0")}:${secs.toString().padStart(2, "0")}`;
  }
  return `${mins.toString().padStart(2, "0")}:${secs.toString().padStart(2, "0")}`;
}

/**
 * Format seconds to human-readable duration.
 */
export function formatDurationLong(seconds: number): string {
  const hrs = Math.floor(seconds / 3600);
  const mins = Math.floor((seconds % 3600) / 60);

  if (hrs > 0 && mins > 0) {
    return `${hrs}h ${mins}m`;
  }
  if (hrs > 0) {
    return `${hrs}h`;
  }
  if (mins > 0) {
    return `${mins}m`;
  }
  return `${seconds}s`;
}
