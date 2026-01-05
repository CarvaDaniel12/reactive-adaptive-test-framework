# Story 7.4: Report Export to Markdown and HTML

Status: ready-for-dev

## Story

As a QA (Ana),
I want to export my report in different formats,
So that I can share it appropriately.

## Acceptance Criteria

1. **Given** user has a finalized report
   **When** user clicks "Export"
   **Then** export options are shown

2. **Given** export options shown
   **When** Markdown selected
   **Then** .md file is generated for technical sharing

3. **Given** export options shown
   **When** HTML selected
   **Then** .html file is generated for browser viewing

4. **Given** export options shown
   **When** Copy to clipboard selected
   **Then** content is copied for pasting

5. **Given** Markdown export
   **When** file is generated
   **Then** produces valid, formatted markdown

6. **Given** HTML export
   **When** file is generated
   **Then** includes professional styling

7. **Given** export occurs
   **When** file is ready
   **Then** file is downloaded to user's computer

8. **Given** export filename
   **When** generated
   **Then** follows pattern: `QA-{ticket-key}-report-{date}.{ext}`

## Tasks

- [ ] Task 1: Create MarkdownExporter service
- [ ] Task 2: Create HTMLExporter service
- [ ] Task 3: Create ExportDialog component
- [ ] Task 4: Implement copy to clipboard
- [ ] Task 5: Create download utility
- [ ] Task 6: Add export styling for HTML

## Dev Notes

### Markdown Exporter

```rust
// crates/qa-pms-workflow/src/report/export/markdown.rs
pub struct MarkdownExporter;

impl MarkdownExporter {
    pub fn export(report: &Report) -> String {
        let mut md = String::new();
        
        // Header
        md.push_str(&format!("# QA Report: {}\n\n", report.ticket_key));
        md.push_str(&format!("**Ticket:** {} - {}\n\n", 
            report.ticket_key, 
            report.ticket_title.as_deref().unwrap_or("N/A")
        ));
        md.push_str(&format!("**Template:** {}\n\n", report.template_name));
        md.push_str(&format!("**Generated:** {}\n\n", 
            report.created_at.format("%Y-%m-%d %H:%M UTC")
        ));
        
        md.push_str("---\n\n");
        
        // Workflow Summary
        md.push_str("## Workflow Summary\n\n");
        let content = &report.content;
        md.push_str(&format!("- **Started:** {}\n", 
            content.workflow.started_at.format("%Y-%m-%d %H:%M")
        ));
        md.push_str(&format!("- **Completed:** {}\n", 
            content.workflow.completed_at.format("%Y-%m-%d %H:%M")
        ));
        md.push_str(&format!("- **Total Time:** {}\n", 
            Self::format_duration(content.time_summary.total_active_seconds)
        ));
        md.push_str(&format!("- **Estimated:** {}\n\n", 
            Self::format_duration(content.time_summary.total_estimated_seconds)
        ));
        
        // Steps
        md.push_str("## Testing Steps\n\n");
        for step in &content.steps {
            let status_icon = match step.status.as_str() {
                "completed" => "✅",
                "skipped" => "⏭️",
                _ => "⬜",
            };
            
            md.push_str(&format!("### {} Step {}: {}\n\n", 
                status_icon, step.index + 1, step.name
            ));
            md.push_str(&format!("{}\n\n", step.description));
            
            if let Some(time) = step.time_seconds {
                md.push_str(&format!("**Time:** {} (estimated: {})\n\n",
                    Self::format_duration(time),
                    Self::format_duration(step.estimated_seconds)
                ));
            }
            
            if let Some(notes) = &step.notes {
                md.push_str("**Notes:**\n\n");
                md.push_str(&format!("> {}\n\n", notes.replace('\n', "\n> ")));
            }
            
            if !step.links.is_empty() {
                md.push_str("**Links:**\n\n");
                for link in &step.links {
                    md.push_str(&format!("- [{}]({}) _{}_\n", 
                        link.label, link.url, link.link_type
                    ));
                }
                md.push_str("\n");
            }
        }
        
        // Tests Covered
        if let Some(tests) = &content.tests_covered {
            md.push_str("## Tests Covered\n\n");
            
            if !tests.postman_collections.is_empty() {
                md.push_str("### Postman Collections\n\n");
                for col in &tests.postman_collections {
                    md.push_str(&format!("- **{}** ({} requests)\n", 
                        col.collection_name, col.requests_count
                    ));
                }
                md.push_str("\n");
            }
            
            if !tests.testmo_cases.is_empty() {
                md.push_str("### Testmo Test Cases\n\n");
                for tc in &tests.testmo_cases {
                    if let Some(url) = &tc.url {
                        md.push_str(&format!("- [{}]({})\n", tc.case_title, url));
                    } else {
                        md.push_str(&format!("- {}\n", tc.case_title));
                    }
                }
                md.push_str("\n");
            }
        }
        
        // Additional Notes
        if let Some(notes) = &content.additional_notes {
            md.push_str("## Additional Notes\n\n");
            md.push_str(notes);
            md.push_str("\n\n");
        }
        
        // Footer
        md.push_str("---\n\n");
        md.push_str("_Generated by QA Intelligent PMS_\n");
        
        md
    }
    
    fn format_duration(seconds: i32) -> String {
        let hours = seconds / 3600;
        let minutes = (seconds % 3600) / 60;
        
        if hours > 0 {
            format!("{}h {}m", hours, minutes)
        } else {
            format!("{}m", minutes)
        }
    }
}
```

### HTML Exporter

```rust
// crates/qa-pms-workflow/src/report/export/html.rs
pub struct HtmlExporter;

impl HtmlExporter {
    pub fn export(report: &Report) -> String {
        let styles = include_str!("report-styles.css");
        let content = &report.content;
        
        format!(r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>QA Report: {ticket_key}</title>
    <style>{styles}</style>
</head>
<body>
    <div class="report">
        <header class="report-header">
            <h1>QA Report: {ticket_key}</h1>
            <div class="meta">
                <span class="ticket">{ticket_key}</span>
                <span class="title">{ticket_title}</span>
            </div>
            <div class="info">
                <span>Template: {template_name}</span>
                <span>Generated: {date}</span>
            </div>
        </header>
        
        <section class="summary">
            <h2>Workflow Summary</h2>
            <div class="metrics">
                <div class="metric">
                    <span class="label">Total Time</span>
                    <span class="value">{total_time}</span>
                </div>
                <div class="metric">
                    <span class="label">Estimated</span>
                    <span class="value">{estimated_time}</span>
                </div>
                <div class="metric">
                    <span class="label">Efficiency</span>
                    <span class="value {efficiency_class}">{efficiency}%</span>
                </div>
            </div>
        </section>
        
        <section class="steps">
            <h2>Testing Steps</h2>
            {steps_html}
        </section>
        
        {tests_section}
        {notes_section}
        
        <footer>
            <p>Generated by QA Intelligent PMS</p>
        </footer>
    </div>
</body>
</html>"#,
            ticket_key = report.ticket_key,
            ticket_title = report.ticket_title.as_deref().unwrap_or(""),
            template_name = report.template_name,
            date = report.created_at.format("%Y-%m-%d %H:%M UTC"),
            styles = styles,
            total_time = Self::format_duration(content.time_summary.total_active_seconds),
            estimated_time = Self::format_duration(content.time_summary.total_estimated_seconds),
            efficiency = ((content.time_summary.efficiency_ratio) * 100.0).round() as i32,
            efficiency_class = Self::efficiency_class(content.time_summary.efficiency_ratio),
            steps_html = Self::render_steps(&content.steps),
            tests_section = Self::render_tests_section(&content.tests_covered),
            notes_section = Self::render_notes_section(&content.additional_notes),
        )
    }
    
    fn render_steps(steps: &[StepSection]) -> String {
        steps.iter().map(|step| {
            let status_class = match step.status.as_str() {
                "completed" => "completed",
                "skipped" => "skipped",
                _ => "pending",
            };
            
            format!(r#"
            <div class="step {status_class}">
                <div class="step-header">
                    <span class="step-number">{num}</span>
                    <h3>{name}</h3>
                    <span class="step-time">{time}</span>
                </div>
                <p class="description">{desc}</p>
                {notes}
                {links}
            </div>"#,
                status_class = status_class,
                num = step.index + 1,
                name = step.name,
                time = step.time_seconds
                    .map(|t| Self::format_duration(t))
                    .unwrap_or_default(),
                desc = step.description,
                notes = step.notes.as_ref()
                    .map(|n| format!(r#"<div class="notes"><strong>Notes:</strong><p>{}</p></div>"#, n))
                    .unwrap_or_default(),
                links = if step.links.is_empty() { String::new() } else {
                    format!(r#"<div class="links">{}</div>"#,
                        step.links.iter()
                            .map(|l| format!(r#"<a href="{}" target="_blank">{}</a>"#, l.url, l.label))
                            .collect::<Vec<_>>()
                            .join(" ")
                    )
                },
            )
        }).collect()
    }
    
    fn efficiency_class(ratio: f64) -> &'static str {
        if ratio <= 1.0 { "good" }
        else if ratio <= 1.2 { "warning" }
        else { "bad" }
    }
    
    fn format_duration(seconds: i32) -> String {
        let hours = seconds / 3600;
        let minutes = (seconds % 3600) / 60;
        if hours > 0 { format!("{}h {}m", hours, minutes) }
        else { format!("{}m", minutes) }
    }
}
```

### Frontend Export Dialog

```tsx
// frontend/src/components/reports/ExportDialog.tsx
interface ExportDialogProps {
  report: Report;
  open: boolean;
  onOpenChange: (open: boolean) => void;
}

export function ExportDialog({ report, open, onOpenChange }: ExportDialogProps) {
  const [exporting, setExporting] = useState<string | null>(null);

  const filename = `QA-${report.ticketKey}-report-${format(new Date(), 'yyyy-MM-dd')}`;

  const handleExport = async (format: "md" | "html" | "clipboard") => {
    setExporting(format);
    
    try {
      const response = await fetch(
        `/api/v1/reports/${report.id}/export?format=${format}`
      );
      
      if (format === "clipboard") {
        const text = await response.text();
        await navigator.clipboard.writeText(text);
        toast.success("Copied to clipboard!");
      } else {
        const blob = await response.blob();
        const url = URL.createObjectURL(blob);
        const a = document.createElement("a");
        a.href = url;
        a.download = `${filename}.${format}`;
        a.click();
        URL.revokeObjectURL(url);
        toast.success(`Downloaded ${filename}.${format}`);
      }
      
      onOpenChange(false);
    } catch (error) {
      toast.error("Export failed");
    } finally {
      setExporting(null);
    }
  };

  return (
    <Dialog.Root open={open} onOpenChange={onOpenChange}>
      <Dialog.Portal>
        <Dialog.Overlay className="fixed inset-0 bg-black/50" />
        <Dialog.Content className="fixed top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2 bg-white rounded-xl p-6 w-full max-w-md">
          <Dialog.Title className="text-xl font-semibold mb-4">
            Export Report
          </Dialog.Title>

          <div className="space-y-3">
            <button
              onClick={() => handleExport("md")}
              disabled={!!exporting}
              className="w-full flex items-center gap-3 p-4 border rounded-lg hover:bg-neutral-50"
            >
              <FileTextIcon className="w-6 h-6 text-neutral-500" />
              <div className="text-left">
                <p className="font-medium">Markdown (.md)</p>
                <p className="text-sm text-neutral-500">For technical documentation</p>
              </div>
              {exporting === "md" && <Spinner className="ml-auto" />}
            </button>

            <button
              onClick={() => handleExport("html")}
              disabled={!!exporting}
              className="w-full flex items-center gap-3 p-4 border rounded-lg hover:bg-neutral-50"
            >
              <GlobeIcon className="w-6 h-6 text-neutral-500" />
              <div className="text-left">
                <p className="font-medium">HTML</p>
                <p className="text-sm text-neutral-500">For browser viewing</p>
              </div>
              {exporting === "html" && <Spinner className="ml-auto" />}
            </button>

            <button
              onClick={() => handleExport("clipboard")}
              disabled={!!exporting}
              className="w-full flex items-center gap-3 p-4 border rounded-lg hover:bg-neutral-50"
            >
              <ClipboardIcon className="w-6 h-6 text-neutral-500" />
              <div className="text-left">
                <p className="font-medium">Copy to Clipboard</p>
                <p className="text-sm text-neutral-500">Markdown format</p>
              </div>
              {exporting === "clipboard" && <Spinner className="ml-auto" />}
            </button>
          </div>

          <Dialog.Close asChild>
            <button className="absolute top-4 right-4 p-1 text-neutral-400 hover:text-neutral-600">
              <Cross2Icon className="w-5 h-5" />
            </button>
          </Dialog.Close>
        </Dialog.Content>
      </Dialog.Portal>
    </Dialog.Root>
  );
}
```

### Export API Endpoint

```rust
// GET /api/v1/reports/:id/export
pub async fn export_report(
    State(state): State<Arc<AppState>>,
    Path(report_id): Path<Uuid>,
    Query(query): Query<ExportQuery>,
) -> Result<Response, ApiError> {
    let report = get_report(&state.db_pool, report_id).await?;
    
    let (content, content_type, extension) = match query.format.as_str() {
        "md" => (
            MarkdownExporter::export(&report),
            "text/markdown",
            "md",
        ),
        "html" => (
            HtmlExporter::export(&report),
            "text/html",
            "html",
        ),
        _ => return Err(ApiError::BadRequest("Invalid format".into())),
    };
    
    let filename = format!("QA-{}-report-{}.{}", 
        report.ticket_key,
        Utc::now().format("%Y-%m-%d"),
        extension
    );
    
    Ok(Response::builder()
        .header("Content-Type", content_type)
        .header("Content-Disposition", format!("attachment; filename=\"{}\"", filename))
        .body(content.into())
        .unwrap())
}
```

### References

- [Source: epics.md#Story 7.4]
- [Dependency: Story 7.3 - Report Preview]
