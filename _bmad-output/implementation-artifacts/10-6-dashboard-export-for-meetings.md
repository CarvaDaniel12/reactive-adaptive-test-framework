# Story 10.6: Dashboard Export for Meetings

Status: ready-for-dev

## Story

As a PM (Carlos),
I want to export dashboard data,
So that I can share in stakeholder meetings.

## Acceptance Criteria

1. **Given** user is viewing PM Dashboard
   **When** user clicks "Export"
   **Then** export options are shown

2. **Given** export options shown
   **When** PDF selected
   **Then** formatted report with charts is exported

3. **Given** export options shown
   **When** HTML selected
   **Then** interactive version is exported

4. **Given** export options shown
   **When** CSV selected
   **Then** raw data for spreadsheets is exported

5. **Given** export is generated
   **When** content included
   **Then** export includes current period's data

6. **Given** export is generated
   **When** metadata included
   **Then** export includes timestamp and filters applied

7. **Given** export is generated
   **When** filename created
   **Then** filename follows: `QA-Metrics-{period}-{date}.{ext}`

## Tasks

- [ ] Task 1: Create PDF export service
- [ ] Task 2: Create HTML export service
- [ ] Task 3: Create CSV export service
- [ ] Task 4: Create ExportDialog component
- [ ] Task 5: Add charts to PDF export
- [ ] Task 6: Handle large data exports

## Dev Notes

### Export Service

```rust
// crates/qa-pms-api/src/services/export.rs
use std::io::Cursor;

pub struct PMDashboardExporter {
    pool: PgPool,
}

impl PMDashboardExporter {
    pub async fn export_pdf(&self, data: &PMDashboardData, period: &str) -> Result<Vec<u8>> {
        // Using printpdf or similar crate
        let doc = PdfDocument::empty("QA Metrics Report");
        let (page1, layer1) = doc.add_page(Mm(210.0), Mm(297.0), "Page 1");
        let current_layer = doc.get_page(page1).get_layer(layer1);

        // Header
        let font = doc.add_builtin_font(BuiltinFont::HelveticaBold)?;
        current_layer.use_text("QA Metrics Report", 24.0, Mm(20.0), Mm(277.0), &font);
        current_layer.use_text(
            &format!("Period: {} | Generated: {}", period, Utc::now().format("%Y-%m-%d %H:%M")),
            10.0, Mm(20.0), Mm(267.0), &font
        );

        // KPIs Section
        self.add_kpi_section(&current_layer, &data.kpis, Mm(250.0))?;

        // Component Health Section
        self.add_component_section(&current_layer, &data.componentHealth, Mm(180.0))?;

        // Economy Section
        self.add_economy_section(&current_layer, &data.economy, Mm(110.0))?;

        doc.save_to_bytes().map_err(Into::into)
    }

    pub async fn export_html(&self, data: &PMDashboardData, period: &str) -> Result<String> {
        let template = include_str!("templates/pm_dashboard_export.html");
        
        // Render with data
        let html = template
            .replace("{{period}}", period)
            .replace("{{generated_at}}", &Utc::now().format("%Y-%m-%d %H:%M UTC").to_string())
            .replace("{{bugs_discovered}}", &data.kpis.bugs_discovered.value.to_string())
            .replace("{{bugs_prevented}}", &data.kpis.bugs_prevented.value.to_string())
            .replace("{{prevention_rate}}", &format!("{:.0}%", data.kpis.prevention_rate * 100.0))
            .replace("{{economy_total}}", &format_currency(data.economy.total))
            .replace("{{component_health_json}}", &serde_json::to_string(&data.component_health)?)
            .replace("{{endpoints_json}}", &serde_json::to_string(&data.endpoints)?);

        Ok(html)
    }

    pub async fn export_csv(&self, data: &PMDashboardData, period: &str) -> Result<String> {
        let mut csv = String::new();
        
        // Header info
        csv.push_str(&format!("# QA Metrics Export\n"));
        csv.push_str(&format!("# Period: {}\n", period));
        csv.push_str(&format!("# Generated: {}\n\n", Utc::now().format("%Y-%m-%d %H:%M")));

        // KPIs
        csv.push_str("## Key Metrics\n");
        csv.push_str("Metric,Value,Change\n");
        csv.push_str(&format!(
            "Bugs Discovered,{},{:.1}%\n",
            data.kpis.bugs_discovered.value,
            data.kpis.bugs_discovered.change_percent
        ));
        csv.push_str(&format!(
            "Bugs Prevented,{},{:.1}%\n",
            data.kpis.bugs_prevented.value,
            data.kpis.bugs_prevented.change_percent
        ));
        csv.push_str(&format!(
            "Prevention Rate,{:.1}%,{:.1}%\n",
            data.kpis.prevention_rate * 100.0,
            data.kpis.prevention_rate_change
        ));
        csv.push_str(&format!(
            "Economy Estimate,${:.0},{:.1}%\n\n",
            data.economy.total,
            data.economy.change_percent
        ));

        // Component Health
        csv.push_str("## Component Health\n");
        csv.push_str("Component,Health,Bug Count,Ticket Count,Trend\n");
        for comp in &data.component_health {
            csv.push_str(&format!(
                "{},{},{},{},{}\n",
                comp.component, comp.health, comp.bug_count, comp.ticket_count, comp.trend
            ));
        }
        csv.push_str("\n");

        // Endpoints
        csv.push_str("## Problematic Endpoints\n");
        csv.push_str("Endpoint,Issue Count,Common Issues,Last Issue\n");
        for ep in &data.endpoints {
            let issues: Vec<&str> = ep.common_issues.iter().map(|i| i.category.as_str()).collect();
            csv.push_str(&format!(
                "{},{},\"{}\",{}\n",
                ep.endpoint,
                ep.issue_count,
                issues.join(", "),
                ep.last_issue_date.format("%Y-%m-%d")
            ));
        }

        Ok(csv)
    }
}
```

### HTML Export Template

```html
<!-- templates/pm_dashboard_export.html -->
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <title>QA Metrics Report - {{period}}</title>
    <style>
        * { box-sizing: border-box; margin: 0; padding: 0; }
        body { font-family: 'Inter', -apple-system, sans-serif; padding: 40px; max-width: 1200px; margin: 0 auto; }
        .header { margin-bottom: 40px; }
        .header h1 { font-size: 28px; margin-bottom: 8px; }
        .header .meta { color: #666; font-size: 14px; }
        .kpis { display: grid; grid-template-columns: repeat(4, 1fr); gap: 20px; margin-bottom: 40px; }
        .kpi-card { background: #f9fafb; border-radius: 12px; padding: 24px; }
        .kpi-card .value { font-size: 36px; font-weight: 700; }
        .kpi-card .label { color: #666; margin-top: 4px; }
        .section { margin-bottom: 40px; }
        .section h2 { font-size: 20px; margin-bottom: 16px; }
        table { width: 100%; border-collapse: collapse; }
        th, td { padding: 12px; text-align: left; border-bottom: 1px solid #e5e5e5; }
        th { background: #f9fafb; font-weight: 600; }
        .health-good { color: #16a34a; }
        .health-warning { color: #ca8a04; }
        .health-critical { color: #dc2626; }
        @media print { body { padding: 20px; } }
    </style>
</head>
<body>
    <div class="header">
        <h1>QA Metrics Report</h1>
        <p class="meta">Period: {{period}} | Generated: {{generated_at}}</p>
    </div>

    <div class="kpis">
        <div class="kpi-card">
            <div class="value">{{bugs_discovered}}</div>
            <div class="label">Bugs Discovered</div>
        </div>
        <div class="kpi-card">
            <div class="value">{{bugs_prevented}}</div>
            <div class="label">Bugs Prevented</div>
        </div>
        <div class="kpi-card">
            <div class="value">{{prevention_rate}}</div>
            <div class="label">Prevention Rate</div>
        </div>
        <div class="kpi-card" style="background: #dcfce7;">
            <div class="value" style="color: #16a34a;">{{economy_total}}</div>
            <div class="label">Economy Estimate</div>
        </div>
    </div>

    <div class="section">
        <h2>Component Health</h2>
        <table>
            <thead>
                <tr><th>Component</th><th>Health</th><th>Bugs</th><th>Tickets</th><th>Trend</th></tr>
            </thead>
            <tbody id="component-table"></tbody>
        </table>
    </div>

    <div class="section">
        <h2>Problematic Endpoints</h2>
        <table>
            <thead>
                <tr><th>Endpoint</th><th>Issues</th><th>Types</th><th>Last Issue</th></tr>
            </thead>
            <tbody id="endpoints-table"></tbody>
        </table>
    </div>

    <script>
        const componentData = {{component_health_json}};
        const endpointsData = {{endpoints_json}};
        // Populate tables...
    </script>
</body>
</html>
```

### API Endpoint

```rust
// GET /api/v1/pm-dashboard/export
#[derive(Debug, Deserialize)]
pub struct ExportQuery {
    format: String, // "pdf", "html", "csv"
    period: String,
}

pub async fn export_pm_dashboard(
    State(state): State<Arc<AppState>>,
    Query(query): Query<ExportQuery>,
) -> Result<Response, ApiError> {
    // Get dashboard data
    let service = PMDashboardService::new(state.db_pool.clone());
    let data = service.get_dashboard_data(&query.period).await?;

    let exporter = PMDashboardExporter::new(state.db_pool.clone());
    
    let (content, content_type, extension) = match query.format.as_str() {
        "pdf" => {
            let bytes = exporter.export_pdf(&data, &query.period).await?;
            (bytes, "application/pdf", "pdf")
        }
        "html" => {
            let html = exporter.export_html(&data, &query.period).await?;
            (html.into_bytes(), "text/html", "html")
        }
        "csv" => {
            let csv = exporter.export_csv(&data, &query.period).await?;
            (csv.into_bytes(), "text/csv", "csv")
        }
        _ => return Err(ApiError::BadRequest("Invalid format".into())),
    };

    let filename = format!(
        "QA-Metrics-{}-{}.{}",
        query.period,
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

- [Source: epics.md#Story 10.6]
