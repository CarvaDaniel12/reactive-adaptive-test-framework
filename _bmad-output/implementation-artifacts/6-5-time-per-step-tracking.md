# Story 6.5: Time Per Step Tracking

Status: ready-for-dev

## Story

As a QA (Ana),
I want time tracked per workflow step,
So that I can see where I spend the most time.

## Acceptance Criteria

1. **Given** user moves between workflow steps
   **When** user completes a step
   **Then** time session for current step is finalized

2. **Given** step is completed
   **When** time is calculated
   **Then** total time is calculated excluding pauses

3. **Given** step is completed
   **When** time is saved
   **Then** time is stored in `time_sessions` table

4. **Given** step is completed
   **When** next step begins
   **Then** new session starts for next step

5. **Given** step is completed
   **When** UI updates
   **Then** step completion UI shows time spent on that step

6. **Given** workflow has history
   **When** user views it
   **Then** historical view shows time breakdown by step

## Tasks

- [ ] Task 1: Integrate step transition with time session end
- [ ] Task 2: Calculate and store total_seconds on step complete
- [ ] Task 3: Auto-start new session for next step
- [ ] Task 4: Create StepTimeBreakdown component
- [ ] Task 5: Add time to step completion UI
- [ ] Task 6: Create step time summary in workflow summary

## Dev Notes

### Backend - Step Transition

```rust
// crates/qa-pms-api/src/services/time.rs
impl TimeService {
    /// Called when completing a step
    pub async fn complete_step_timing(
        &self,
        instance_id: Uuid,
        step_index: i32,
    ) -> Result<StepTimeSummary> {
        // End current session
        let session = self.repo
            .get_active_session_for_step(instance_id, step_index)
            .await?
            .ok_or_else(|| anyhow!("No active session for step"))?;
        
        let ended_session = self.repo.end_session(session.id).await?;
        
        Ok(StepTimeSummary {
            step_index,
            total_seconds: ended_session.total_seconds.unwrap_or(0),
            pause_count: ended_session.pause_count,
            total_paused_seconds: ended_session.total_paused_seconds,
        })
    }

    /// Called when advancing to next step
    pub async fn start_next_step(
        &self,
        instance_id: Uuid,
        next_step_index: i32,
    ) -> Result<TimeSession> {
        self.repo.start_session(instance_id, next_step_index).await
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StepTimeSummary {
    pub step_index: i32,
    pub total_seconds: i32,
    pub pause_count: i32,
    pub total_paused_seconds: i32,
}
```

### Repository - End Session

```rust
impl TimeRepository {
    pub async fn end_session(&self, id: Uuid) -> Result<TimeSession> {
        sqlx::query_as::<_, TimeSession>(
            r#"
            UPDATE time_sessions
            SET ended_at = NOW(),
                is_active = false,
                total_seconds = EXTRACT(EPOCH FROM (NOW() - started_at))::INTEGER 
                              - total_paused_seconds
            WHERE id = $1
            RETURNING *
            "#,
        )
        .bind(id)
        .fetch_one(&self.pool)
        .await
        .map_err(Into::into)
    }

    pub async fn get_sessions_for_workflow(
        &self,
        instance_id: Uuid,
    ) -> Result<Vec<TimeSession>> {
        sqlx::query_as::<_, TimeSession>(
            r#"
            SELECT * FROM time_sessions
            WHERE workflow_instance_id = $1
            ORDER BY step_index
            "#,
        )
        .bind(instance_id)
        .fetch_all(&self.pool)
        .await
        .map_err(Into::into)
    }
}
```

### Update Complete Step API

```rust
// In complete_step endpoint
pub async fn complete_step(/* ... */) -> Result<Json<CompleteStepResponse>, ApiError> {
    // ... existing logic ...
    
    // Complete step timing
    let time_service = TimeService::new(state.db_pool.clone());
    let step_time = time_service
        .complete_step_timing(instance_id, step_index)
        .await
        .map_err(|e| ApiError::Internal(e.to_string()))?;

    // Start next step timing if not last step
    if !workflow_completed {
        time_service
            .start_next_step(instance_id, next_step_index)
            .await
            .map_err(|e| ApiError::Internal(e.to_string()))?;
    }

    Ok(Json(CompleteStepResponse {
        step_result,
        step_time: Some(step_time),
        next_step_index,
        workflow_completed,
    }))
}
```

### Frontend - StepTimeBreakdown

```tsx
// frontend/src/components/workflow/StepTimeBreakdown.tsx
interface StepTimeBreakdownProps {
  steps: Array<{
    name: string;
    estimatedMinutes: number;
  }>;
  timeSessions: Array<{
    stepIndex: number;
    totalSeconds: number;
    pauseCount: number;
  }>;
}

export function StepTimeBreakdown({ steps, timeSessions }: StepTimeBreakdownProps) {
  return (
    <div className="space-y-2">
      {steps.map((step, index) => {
        const session = timeSessions.find(s => s.stepIndex === index);
        const actualMinutes = session 
          ? Math.round(session.totalSeconds / 60) 
          : null;
        const estimatedMinutes = step.estimatedMinutes;
        
        const ratio = actualMinutes 
          ? actualMinutes / estimatedMinutes 
          : null;

        return (
          <div
            key={index}
            className="flex items-center justify-between p-3 bg-neutral-50 rounded-lg"
          >
            <div className="flex items-center gap-3">
              <span className="w-6 h-6 rounded-full bg-neutral-200 flex items-center justify-center text-xs">
                {index + 1}
              </span>
              <span className="text-sm font-medium">{step.name}</span>
            </div>
            
            <div className="flex items-center gap-4 text-sm">
              {actualMinutes !== null ? (
                <>
                  <span className={cn(
                    "font-medium",
                    ratio && ratio <= 1 ? "text-success-600" : 
                    ratio && ratio <= 1.2 ? "text-warning-600" : "text-error-600"
                  )}>
                    {actualMinutes}m
                  </span>
                  <span className="text-neutral-400">/</span>
                  <span className="text-neutral-500">{estimatedMinutes}m</span>
                  {session?.pauseCount > 0 && (
                    <span className="text-xs text-neutral-400">
                      ({session.pauseCount} pause{session.pauseCount > 1 ? 's' : ''})
                    </span>
                  )}
                </>
              ) : (
                <span className="text-neutral-400">—</span>
              )}
            </div>
          </div>
        );
      })}
    </div>
  );
}
```

### Step Completion UI Update

```tsx
// Show time in step completion toast
toast.success(
  `Step completed in ${formatTime(stepTime.totalSeconds)}!`,
  {
    description: stepTime.totalSeconds < step.estimatedMinutes * 60
      ? "Great job! Under estimate."
      : "Step took longer than estimated.",
  }
);
```

### References

- [Source: epics.md#Story 6.5]
- [Dependency: Story 5.5 - Complete Workflow Step]
