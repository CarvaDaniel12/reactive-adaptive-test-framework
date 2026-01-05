# Story 6.2: Automatic Timer Start on Workflow Begin

Status: ready-for-dev

## Story

As a QA (Ana),
I want time tracking to start automatically,
So that I don't need to remember to start a timer.

## Acceptance Criteria

1. **Given** user starts a workflow (Story 5.3)
   **When** the workflow begins
   **Then** a time session is created automatically

2. **Given** time session is created
   **When** session is initialized
   **Then** `started_at` is set to current timestamp

3. **Given** time session is created
   **When** session is initialized
   **Then** `is_active` is set to true

4. **Given** time session is created
   **When** session is initialized
   **Then** session is linked to workflow instance and step

5. **Given** workflow starts
   **When** timer begins
   **Then** no user action is required to start timer

6. **Given** timer session is started
   **When** database is checked
   **Then** timer session persists to database

## Tasks

- [ ] Task 1: Integrate timer start with workflow creation API
- [ ] Task 2: Create TimeService with auto-start logic
- [ ] Task 3: Update workflow start to call TimeService
- [ ] Task 4: Add timer state to workflow response
- [ ] Task 5: Create frontend timer initialization hook
- [ ] Task 6: Test automatic timer start flow

## Dev Notes

### Backend Integration

```rust
// crates/qa-pms-api/src/services/time.rs
pub struct TimeService {
    repo: TimeRepository,
}

impl TimeService {
    /// Start timer automatically when workflow begins
    pub async fn start_workflow_timer(
        &self,
        instance_id: Uuid,
        step_index: i32,
    ) -> Result<TimeSession> {
        // Check if session already exists
        if let Some(session) = self.repo.get_active_session(instance_id).await? {
            return Ok(session);
        }

        // Create new session
        self.repo.start_session(instance_id, step_index).await
    }

    /// Called when step changes
    pub async fn transition_to_step(
        &self,
        instance_id: Uuid,
        from_step: i32,
        to_step: i32,
    ) -> Result<TimeSession> {
        // End current session
        if let Some(session) = self.repo.get_active_session(instance_id).await? {
            self.repo.end_session(session.id).await?;
        }

        // Start new session
        self.repo.start_session(instance_id, to_step).await
    }
}
```

### API Integration

```rust
// Update create_instance in workflow.rs
pub async fn create_instance(/* ... */) -> Result<...> {
    // ... create instance ...
    
    // Auto-start timer for first step
    let time_service = TimeService::new(state.db_pool.clone());
    let timer_session = time_service
        .start_workflow_timer(instance.id, 0)
        .await?;

    Ok((StatusCode::CREATED, Json(WorkflowInstanceResponse {
        instance,
        template,
        step_results,
        active_timer: Some(timer_session),
    })))
}
```

### Frontend Hook

```tsx
// frontend/src/hooks/useWorkflowTimer.ts
export function useWorkflowTimer() {
  const { instanceId, currentStep } = useWorkflowStore();
  const [startTime, setStartTime] = useState<Date | null>(null);
  
  // Timer starts automatically when workflow starts
  useEffect(() => {
    if (instanceId) {
      setStartTime(new Date());
    }
  }, [instanceId]);

  return { startTime, isRunning: !!startTime };
}
```

### References

- [Source: epics.md#Story 6.2]
- [Dependency: Story 5.3 - Start Workflow from Ticket]
