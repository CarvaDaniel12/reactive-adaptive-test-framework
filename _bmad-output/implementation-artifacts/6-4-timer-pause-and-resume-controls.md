# Story 6.4: Timer Pause and Resume Controls

Status: ready-for-dev

## Story

As a QA (Ana),
I want to pause the timer during interruptions,
So that break time isn't counted.

## Acceptance Criteria

1. **Given** timer is running
   **When** user clicks "Pause"
   **Then** `paused_at` timestamp is recorded

2. **Given** timer is paused
   **When** display updates
   **Then** timer display stops incrementing

3. **Given** timer is paused
   **When** UI shows status
   **Then** "Paused" indicator is shown

4. **Given** timer is paused
   **When** button renders
   **Then** button changes to "Resume"

5. **Given** timer is paused and user clicks "Resume"
   **When** timer resumes
   **Then** `resumed_at` timestamp is recorded

6. **Given** timer resumes
   **When** time is calculated
   **Then** timer continues from paused value

7. **Given** pause/resume occurs
   **When** total is calculated
   **Then** paused duration is excluded from total

8. **Given** pause/resume events occur
   **When** database is updated
   **Then** pause/resume events are logged in database

## Tasks

- [ ] Task 1: Create pause timer API endpoint
- [ ] Task 2: Create resume timer API endpoint
- [ ] Task 3: Update TimeSession with pause tracking
- [ ] Task 4: Create PauseResumeButton component
- [ ] Task 5: Update timer display for paused state
- [ ] Task 6: Track pause count and total paused time

## Dev Notes

### Backend API

```rust
// crates/qa-pms-api/src/routes/time.rs

/// PUT /api/v1/time/sessions/:session_id/pause
#[utoipa::path(
    put,
    path = "/api/v1/time/sessions/{session_id}/pause",
    responses(
        (status = 200, description = "Timer paused", body = TimeSession),
        (status = 400, description = "Timer not running"),
    ),
    tag = "time"
)]
pub async fn pause_timer(
    State(state): State<Arc<AppState>>,
    Path(session_id): Path<Uuid>,
) -> Result<Json<TimeSession>, ApiError> {
    let repo = TimeRepository::new(state.db_pool.clone());
    
    let session = repo.get_session(session_id).await?
        .ok_or_else(|| ApiError::NotFound("Session not found".into()))?;
    
    if session.paused_at.is_some() {
        return Err(ApiError::BadRequest("Timer already paused".into()));
    }
    
    let updated = repo.pause_session(session_id).await?;
    Ok(Json(updated))
}

/// PUT /api/v1/time/sessions/:session_id/resume
#[utoipa::path(
    put,
    path = "/api/v1/time/sessions/{session_id}/resume",
    responses(
        (status = 200, description = "Timer resumed", body = TimeSession),
        (status = 400, description = "Timer not paused"),
    ),
    tag = "time"
)]
pub async fn resume_timer(
    State(state): State<Arc<AppState>>,
    Path(session_id): Path<Uuid>,
) -> Result<Json<TimeSession>, ApiError> {
    let repo = TimeRepository::new(state.db_pool.clone());
    
    let session = repo.get_session(session_id).await?
        .ok_or_else(|| ApiError::NotFound("Session not found".into()))?;
    
    if session.paused_at.is_none() {
        return Err(ApiError::BadRequest("Timer not paused".into()));
    }
    
    let updated = repo.resume_session(session_id).await?;
    Ok(Json(updated))
}
```

### Repository Methods

```rust
impl TimeRepository {
    pub async fn pause_session(&self, id: Uuid) -> Result<TimeSession> {
        sqlx::query_as::<_, TimeSession>(
            r#"
            UPDATE time_sessions
            SET paused_at = NOW(),
                pause_count = pause_count + 1
            WHERE id = $1
            RETURNING *
            "#,
        )
        .bind(id)
        .fetch_one(&self.pool)
        .await
        .map_err(Into::into)
    }

    pub async fn resume_session(&self, id: Uuid) -> Result<TimeSession> {
        sqlx::query_as::<_, TimeSession>(
            r#"
            UPDATE time_sessions
            SET total_paused_seconds = total_paused_seconds + 
                EXTRACT(EPOCH FROM (NOW() - paused_at))::INTEGER,
                paused_at = NULL,
                resumed_at = NOW()
            WHERE id = $1
            RETURNING *
            "#,
        )
        .bind(id)
        .fetch_one(&self.pool)
        .await
        .map_err(Into::into)
    }
}
```

### Frontend Hook

```tsx
// frontend/src/hooks/usePauseTimer.ts
import { useMutation } from "@tanstack/react-query";
import { useWorkflowStore } from "@/stores/workflowStore";

export function usePauseTimer() {
  const { setTimerPaused } = useWorkflowStore();

  return useMutation({
    mutationFn: async (sessionId: string) => {
      const response = await fetch(
        `/api/v1/time/sessions/${sessionId}/pause`,
        { method: "PUT" }
      );
      if (!response.ok) throw new Error("Failed to pause timer");
      return response.json();
    },
    onSuccess: (data) => {
      setTimerPaused(data.pausedAt);
    },
  });
}

export function useResumeTimer() {
  const { setTimerResumed } = useWorkflowStore();

  return useMutation({
    mutationFn: async (sessionId: string) => {
      const response = await fetch(
        `/api/v1/time/sessions/${sessionId}/resume`,
        { method: "PUT" }
      );
      if (!response.ok) throw new Error("Failed to resume timer");
      return response.json();
    },
    onSuccess: (data) => {
      setTimerResumed(data.totalPausedSeconds);
    },
  });
}
```

### PauseResumeButton Component

```tsx
// frontend/src/components/workflow/PauseResumeButton.tsx
import { PauseIcon, PlayIcon } from "@radix-ui/react-icons";
import { usePauseTimer, useResumeTimer } from "@/hooks/usePauseTimer";
import { useWorkflowStore } from "@/stores/workflowStore";

export function PauseResumeButton() {
  const { timerSessionId, status } = useWorkflowStore();
  const { mutate: pause, isPending: isPausing } = usePauseTimer();
  const { mutate: resume, isPending: isResuming } = useResumeTimer();

  const isPaused = status === "paused";
  const isLoading = isPausing || isResuming;

  const handleClick = () => {
    if (!timerSessionId) return;
    if (isPaused) {
      resume(timerSessionId);
    } else {
      pause(timerSessionId);
    }
  };

  return (
    <button
      onClick={handleClick}
      disabled={isLoading}
      className={cn(
        "flex items-center gap-2 px-3 py-2 rounded-lg font-medium transition-colors",
        isPaused
          ? "bg-success-500 text-white hover:bg-success-600"
          : "bg-warning-500 text-white hover:bg-warning-600",
        isLoading && "opacity-50 cursor-not-allowed"
      )}
    >
      {isPaused ? (
        <>
          <PlayIcon className="w-4 h-4" />
          Resume
        </>
      ) : (
        <>
          <PauseIcon className="w-4 h-4" />
          Pause
        </>
      )}
    </button>
  );
}
```

### References

- [Source: epics.md#Story 6.4]
- [Dependency: Story 6.2 - Automatic Timer Start]
