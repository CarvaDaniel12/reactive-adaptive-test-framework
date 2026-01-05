# Story 7.1: Report Generation on Workflow Completion

Status: ready-for-dev

## Story

As a QA (Ana),
I want a report generated when I complete a workflow,
So that I have documentation of my testing.

## Acceptance Criteria

1. **Given** user completes all workflow steps
   **When** user clicks "Generate Report"
   **Then** report is created containing ticket information (key, title, description)

2. **Given** report is created
   **When** content is populated
   **Then** report includes workflow template used

3. **Given** report is created
   **When** content is populated
   **Then** report includes all steps with completion status

4. **Given** report is created
   **When** content is populated
   **Then** report includes notes per step

5. **Given** report is created
   **When** content is populated
   **Then** report includes total time and time per step

6. **Given** report is created
   **When** content is populated
   **Then** report includes timestamp of completion

7. **Given** report is generated
   **When** saved
   **Then** report is stored in database

8. **Given** report generation
   **When** performance is measured
   **Then** report generation completes in < 5s (NFR-PERF-02)

## Tasks

- [ ] Task 1: Create reports database table
- [ ] Task 2: Create ReportService with generation logic
- [ ] Task 3: Create Report model and types
- [ ] Task 4: Create POST /api/v1/reports endpoint
- [ ] Task 5: Integrate with workflow completion flow
- [ ] Task 6: Create useGenerateReport hook

## Dev Notes

### Database Schema

```sql
-- migrations/20260103_create_reports.sql
CREATE TABLE reports (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    workflow_instance_id UUID REFERENCES workflow_instances(id),
    ticket_id VARCHAR(50) NOT NULL,
    ticket_key VARCHAR(50) NOT NULL,
    ticket_title VARCHAR(500),
    ticket_description TEXT,
    template_name VARCHAR(255) NOT NULL,
    content_json JSONB NOT NULL,
    total_time_seconds INTEGER,
    status VARCHAR(50) DEFAULT 'draft',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    finalized_at TIMESTAMPTZ
);

CREATE INDEX idx_reports_ticket ON reports(ticket_key);
CREATE INDEX idx_reports_created ON reports(created_at DESC);
```

### Report Types

```rust
// crates/qa-pms-workflow/src/report/models.rs
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Report {
    pub id: Uuid,
    pub workflow_instance_id: Uuid,
    pub ticket_id: String,
    pub ticket_key: String,
    pub ticket_title: Option<String>,
    pub ticket_description: Option<String>,
    pub template_name: String,
    pub content: ReportContent,
    pub total_time_seconds: Option<i32>,
    pub status: ReportStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub finalized_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReportContent {
    pub workflow: WorkflowSection,
    pub steps: Vec<StepSection>,
    pub time_summary: TimeSummarySection,
    pub tests_covered: Option<TestsCoveredSection>,
    pub additional_notes: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkflowSection {
    pub template_name: String,
    pub template_description: Option<String>,
    pub started_at: DateTime<Utc>,
    pub completed_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StepSection {
    pub index: i32,
    pub name: String,
    pub description: String,
    pub status: String,
    pub notes: Option<String>,
    pub links: Vec<StepLink>,
    pub time_seconds: Option<i32>,
    pub estimated_seconds: i32,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TimeSummarySection {
    pub total_active_seconds: i32,
    pub total_estimated_seconds: i32,
    pub total_paused_seconds: i32,
    pub efficiency_ratio: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ReportStatus {
    Draft,
    Finalized,
}
```

### Report Service

```rust
// crates/qa-pms-workflow/src/report/service.rs
pub struct ReportService {
    pool: PgPool,
}

impl ReportService {
    pub async fn generate_report(
        &self,
        instance_id: Uuid,
    ) -> Result<Report> {
        // Fetch all required data
        let instance = self.get_workflow_instance(instance_id).await?;
        let template = self.get_template(instance.template_id).await?;
        let step_results = self.get_step_results(instance_id).await?;
        let time_sessions = self.get_time_sessions(instance_id).await?;
        
        // Build content
        let content = self.build_report_content(
            &instance,
            &template,
            &step_results,
            &time_sessions,
        )?;
        
        // Calculate totals
        let total_time: i32 = time_sessions.iter()
            .filter_map(|s| s.total_seconds)
            .sum();
        
        // Create report
        let report = sqlx::query_as::<_, Report>(
            r#"
            INSERT INTO reports 
                (workflow_instance_id, ticket_id, ticket_key, ticket_title,
                 ticket_description, template_name, content_json, total_time_seconds)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            RETURNING *
            "#,
        )
        .bind(instance_id)
        .bind(&instance.ticket_id)
        .bind(&instance.ticket_key)
        .bind(&instance.ticket_title)
        .bind::<Option<&str>>(None) // ticket_description from Jira
        .bind(&template.name)
        .bind(sqlx::types::Json(&content))
        .bind(total_time)
        .fetch_one(&self.pool)
        .await?;
        
        Ok(report)
    }

    fn build_report_content(
        &self,
        instance: &WorkflowInstance,
        template: &WorkflowTemplate,
        step_results: &[WorkflowStepResult],
        time_sessions: &[TimeSession],
    ) -> Result<ReportContent> {
        let steps: Vec<StepSection> = template.steps.iter().enumerate()
            .map(|(i, step)| {
                let result = step_results.iter().find(|r| r.step_index == i as i32);
                let session = time_sessions.iter().find(|s| s.step_index == i as i32);
                
                StepSection {
                    index: i as i32,
                    name: step.name.clone(),
                    description: step.description.clone(),
                    status: result.map(|r| format!("{:?}", r.status))
                        .unwrap_or("pending".into()),
                    notes: result.and_then(|r| r.notes.clone()),
                    links: result.map(|r| r.links.clone()).unwrap_or_default(),
                    time_seconds: session.and_then(|s| s.total_seconds),
                    estimated_seconds: (step.estimated_minutes * 60) as i32,
                }
            })
            .collect();
        
        let total_active: i32 = time_sessions.iter()
            .filter_map(|s| s.total_seconds)
            .sum();
        let total_estimated: i32 = template.steps.iter()
            .map(|s| (s.estimated_minutes * 60) as i32)
            .sum();
        let total_paused: i32 = time_sessions.iter()
            .map(|s| s.total_paused_seconds)
            .sum();
        
        Ok(ReportContent {
            workflow: WorkflowSection {
                template_name: template.name.clone(),
                template_description: template.description.clone(),
                started_at: instance.started_at,
                completed_at: instance.completed_at.unwrap_or_else(Utc::now),
            },
            steps,
            time_summary: TimeSummarySection {
                total_active_seconds: total_active,
                total_estimated_seconds: total_estimated,
                total_paused_seconds: total_paused,
                efficiency_ratio: if total_estimated > 0 {
                    total_active as f64 / total_estimated as f64
                } else { 1.0 },
            },
            tests_covered: None, // Populated in Story 7.2
            additional_notes: None,
        })
    }
}
```

### API Endpoint

```rust
// POST /api/v1/reports
#[utoipa::path(
    post,
    path = "/api/v1/reports",
    request_body = GenerateReportRequest,
    responses(
        (status = 201, description = "Report generated", body = Report),
        (status = 404, description = "Workflow not found"),
    ),
    tag = "reports"
)]
pub async fn generate_report(
    State(state): State<Arc<AppState>>,
    Json(request): Json<GenerateReportRequest>,
) -> Result<(StatusCode, Json<Report>), ApiError> {
    let service = ReportService::new(state.db_pool.clone());
    let report = service.generate_report(request.workflow_instance_id).await
        .map_err(|e| ApiError::Internal(e.to_string()))?;
    
    Ok((StatusCode::CREATED, Json(report)))
}
```

### Frontend Hook

```tsx
// frontend/src/hooks/useGenerateReport.ts
export function useGenerateReport() {
  const navigate = useNavigate();

  return useMutation({
    mutationFn: async (instanceId: string) => {
      const response = await fetch("/api/v1/reports", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ workflowInstanceId: instanceId }),
      });
      if (!response.ok) throw new Error("Failed to generate report");
      return response.json();
    },
    onSuccess: (report) => {
      toast.success("Report generated!");
      navigate(`/reports/${report.id}`);
    },
  });
}
```

### References

- [Source: epics.md#Story 7.1]
- [NFR: NFR-PERF-02 - <5s report generation]
