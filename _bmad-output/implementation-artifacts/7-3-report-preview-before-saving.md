# Story 7.3: Report Preview Before Saving

Status: ready-for-dev

## Story

As a QA (Ana),
I want to preview my report before finalizing,
So that I can make corrections.

## Acceptance Criteria

1. **Given** report has been generated
   **When** preview is displayed
   **Then** user sees full report rendered (read-only)

2. **Given** preview is displayed
   **When** user wants to edit
   **Then** Edit button is available to modify notes/sections

3. **Given** preview is displayed
   **When** user wants to cancel
   **Then** Cancel button returns to previous view

4. **Given** preview is displayed
   **When** user wants to save
   **Then** Save & Export buttons are available

5. **Given** user is editing
   **When** changes are made
   **Then** edits update the report in real-time

6. **Given** preview is rendered
   **When** styling is applied
   **Then** preview uses same styling as final export

7. **Given** preview is displayed
   **When** user wants to add info
   **Then** user can add additional notes at this stage

## Tasks

- [ ] Task 1: Create ReportPreview component
- [ ] Task 2: Create ReportEditor for editing mode
- [ ] Task 3: Implement real-time edit updates
- [ ] Task 4: Create preview/edit mode toggle
- [ ] Task 5: Add additional notes field
- [ ] Task 6: Create PUT /api/v1/reports/:id endpoint for updates
- [ ] Task 7: Match preview styling with export

## Dev Notes

### ReportPreview Component

```tsx
// frontend/src/components/reports/ReportPreview.tsx
import { useState } from "react";
import { Report } from "@/types/report";
import { ReportHeader } from "./ReportHeader";
import { StepsSection } from "./StepsSection";
import { TestsCoveredSection } from "./TestsCoveredSection";
import { GapAnalysisSection } from "./GapAnalysisSection";
import { AdditionalNotesSection } from "./AdditionalNotesSection";

interface ReportPreviewProps {
  report: Report;
  onSave: (updates: Partial<Report>) => void;
  onExport: (format: "md" | "html") => void;
  onCancel: () => void;
}

export function ReportPreview({
  report,
  onSave,
  onExport,
  onCancel,
}: ReportPreviewProps) {
  const [isEditing, setIsEditing] = useState(false);
  const [editedContent, setEditedContent] = useState(report.content);

  const handleSave = () => {
    onSave({ content: editedContent });
    setIsEditing(false);
  };

  const handleContentChange = (updates: Partial<typeof editedContent>) => {
    setEditedContent({ ...editedContent, ...updates });
  };

  return (
    <div className="max-w-4xl mx-auto">
      {/* Toolbar */}
      <div className="sticky top-0 bg-white border-b border-neutral-200 py-3 px-4 flex items-center justify-between z-10">
        <div className="flex items-center gap-3">
          <h1 className="text-lg font-semibold">Report Preview</h1>
          {report.status === "draft" && (
            <span className="px-2 py-0.5 bg-warning-100 text-warning-700 text-xs rounded">
              Draft
            </span>
          )}
        </div>

        <div className="flex items-center gap-2">
          {isEditing ? (
            <>
              <button
                onClick={() => setIsEditing(false)}
                className="px-3 py-2 text-sm text-neutral-600 hover:text-neutral-800"
              >
                Cancel Edit
              </button>
              <button
                onClick={handleSave}
                className="px-3 py-2 text-sm bg-primary-500 text-white rounded-lg hover:bg-primary-600"
              >
                Save Changes
              </button>
            </>
          ) : (
            <>
              <button
                onClick={onCancel}
                className="px-3 py-2 text-sm text-neutral-600 hover:text-neutral-800"
              >
                Cancel
              </button>
              <button
                onClick={() => setIsEditing(true)}
                className="px-3 py-2 text-sm border border-neutral-300 rounded-lg hover:bg-neutral-50"
              >
                Edit
              </button>
              <button
                onClick={() => onExport("md")}
                className="px-3 py-2 text-sm border border-neutral-300 rounded-lg hover:bg-neutral-50"
              >
                Export .md
              </button>
              <button
                onClick={() => onExport("html")}
                className="px-3 py-2 text-sm bg-primary-500 text-white rounded-lg hover:bg-primary-600"
              >
                Export HTML
              </button>
            </>
          )}
        </div>
      </div>

      {/* Report Content */}
      <div className="report-preview p-8 bg-white shadow-sm mt-4 rounded-lg">
        <ReportHeader
          ticketKey={report.ticketKey}
          ticketTitle={report.ticketTitle}
          templateName={report.templateName}
          completedAt={report.content.workflow.completedAt}
        />

        <hr className="my-6" />

        {/* Workflow Summary */}
        <section className="mb-8">
          <h2 className="text-xl font-semibold mb-4">Workflow Summary</h2>
          <div className="grid grid-cols-2 gap-4 text-sm">
            <div>
              <span className="text-neutral-500">Template:</span>
              <span className="ml-2 font-medium">{editedContent.workflow.templateName}</span>
            </div>
            <div>
              <span className="text-neutral-500">Duration:</span>
              <span className="ml-2 font-medium">
                {formatDuration(editedContent.timeSummary.totalActiveSeconds)}
              </span>
            </div>
          </div>
        </section>

        {/* Steps */}
        <StepsSection
          steps={editedContent.steps}
          isEditing={isEditing}
          onStepNotesChange={(index, notes) => {
            const updatedSteps = [...editedContent.steps];
            updatedSteps[index] = { ...updatedSteps[index], notes };
            handleContentChange({ steps: updatedSteps });
          }}
        />

        {/* Tests Covered */}
        {editedContent.testsCovered && (
          <TestsCoveredSection data={editedContent.testsCovered} />
        )}

        {/* Gap Analysis */}
        {editedContent.gapAnalysis && (
          <GapAnalysisSection data={editedContent.gapAnalysis} />
        )}

        {/* Additional Notes */}
        <AdditionalNotesSection
          notes={editedContent.additionalNotes}
          isEditing={isEditing}
          onChange={(notes) => handleContentChange({ additionalNotes: notes })}
        />
      </div>
    </div>
  );
}
```

### Editable Steps Section

```tsx
// frontend/src/components/reports/StepsSection.tsx
interface StepsSectionProps {
  steps: StepSection[];
  isEditing: boolean;
  onStepNotesChange: (index: number, notes: string) => void;
}

export function StepsSection({ steps, isEditing, onStepNotesChange }: StepsSectionProps) {
  return (
    <section className="mb-8">
      <h2 className="text-xl font-semibold mb-4">Testing Steps</h2>
      
      <div className="space-y-4">
        {steps.map((step, index) => (
          <div key={index} className="border border-neutral-200 rounded-lg p-4">
            <div className="flex items-center justify-between mb-2">
              <div className="flex items-center gap-2">
                <span className={cn(
                  "w-6 h-6 rounded-full flex items-center justify-center text-xs",
                  step.status === "completed" 
                    ? "bg-success-100 text-success-700"
                    : "bg-neutral-100 text-neutral-500"
                )}>
                  {index + 1}
                </span>
                <h4 className="font-medium">{step.name}</h4>
              </div>
              
              {step.timeSeconds && (
                <TimeComparison
                  actualSeconds={step.timeSeconds}
                  estimatedSeconds={step.estimatedSeconds}
                  showAlert={false}
                />
              )}
            </div>

            <p className="text-sm text-neutral-600 mb-3">{step.description}</p>

            {/* Notes */}
            {isEditing ? (
              <textarea
                value={step.notes || ""}
                onChange={(e) => onStepNotesChange(index, e.target.value)}
                placeholder="Add notes for this step..."
                className="w-full p-2 text-sm border border-neutral-300 rounded-lg resize-none"
                rows={3}
              />
            ) : step.notes ? (
              <div className="bg-neutral-50 rounded-lg p-3 text-sm">
                <span className="text-xs text-neutral-500 uppercase mb-1 block">Notes</span>
                <p className="whitespace-pre-wrap">{step.notes}</p>
              </div>
            ) : null}

            {/* Links */}
            {step.links && step.links.length > 0 && (
              <div className="mt-2 flex flex-wrap gap-2">
                {step.links.map((link, i) => (
                  <a
                    key={i}
                    href={link.url}
                    target="_blank"
                    rel="noopener noreferrer"
                    className="text-xs text-primary-600 hover:underline flex items-center gap-1"
                  >
                    <Link2Icon className="w-3 h-3" />
                    {link.label}
                  </a>
                ))}
              </div>
            )}
          </div>
        ))}
      </div>
    </section>
  );
}
```

### Additional Notes Section

```tsx
// frontend/src/components/reports/AdditionalNotesSection.tsx
interface AdditionalNotesSectionProps {
  notes: string | null;
  isEditing: boolean;
  onChange: (notes: string) => void;
}

export function AdditionalNotesSection({
  notes,
  isEditing,
  onChange,
}: AdditionalNotesSectionProps) {
  if (!isEditing && !notes) return null;

  return (
    <section className="mt-8 pt-6 border-t border-neutral-200">
      <h2 className="text-xl font-semibold mb-4">Additional Notes</h2>
      
      {isEditing ? (
        <textarea
          value={notes || ""}
          onChange={(e) => onChange(e.target.value)}
          placeholder="Add any additional notes, observations, or recommendations..."
          className="w-full p-4 text-sm border border-neutral-300 rounded-lg resize-none"
          rows={6}
        />
      ) : notes ? (
        <div className="prose prose-sm max-w-none">
          <p className="whitespace-pre-wrap">{notes}</p>
        </div>
      ) : null}
    </section>
  );
}
```

### Update Report API

```rust
// PUT /api/v1/reports/:id
#[utoipa::path(
    put,
    path = "/api/v1/reports/{report_id}",
    request_body = UpdateReportRequest,
    responses(
        (status = 200, description = "Report updated", body = Report),
        (status = 404, description = "Report not found"),
    ),
    tag = "reports"
)]
pub async fn update_report(
    State(state): State<Arc<AppState>>,
    Path(report_id): Path<Uuid>,
    Json(request): Json<UpdateReportRequest>,
) -> Result<Json<Report>, ApiError> {
    let report = sqlx::query_as::<_, Report>(
        r#"
        UPDATE reports
        SET content_json = $2,
            updated_at = NOW()
        WHERE id = $1
        RETURNING *
        "#,
    )
    .bind(report_id)
    .bind(sqlx::types::Json(&request.content))
    .fetch_one(&state.db_pool)
    .await
    .map_err(|_| ApiError::NotFound("Report not found".into()))?;

    Ok(Json(report))
}
```

### Preview Styles (matching export)

```css
/* frontend/src/styles/report-preview.css */
.report-preview {
  font-family: 'Inter', system-ui, sans-serif;
  line-height: 1.6;
  color: #1a1a1a;
}

.report-preview h1 { font-size: 1.5rem; font-weight: 700; }
.report-preview h2 { font-size: 1.25rem; font-weight: 600; margin-top: 1.5rem; }
.report-preview h3 { font-size: 1rem; font-weight: 600; }

.report-preview hr {
  border: none;
  border-top: 1px solid #e5e5e5;
  margin: 1.5rem 0;
}

/* Print styles for consistent export */
@media print {
  .report-preview { padding: 0; box-shadow: none; }
  .sticky { position: static; }
}
```

### References

- [Source: epics.md#Story 7.3]
- [Dependency: Story 7.1 - Report Generation]
