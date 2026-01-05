# Story 11.2: Splunk Query Templates

Status: ready-for-dev

## Story

As a QA (Ana),
I want pre-built query templates,
So that I don't need to write SPL from scratch.

## Acceptance Criteria

1. **Given** user opens Splunk panel
   **When** user views templates
   **Then** "Error logs for date range" template is available

2. **Given** templates available
   **When** viewed
   **Then** "Logs for specific user ID" template is available

3. **Given** templates available
   **When** viewed
   **Then** "Logs containing ticket key" template is available

4. **Given** templates available
   **When** viewed
   **Then** "Performance metrics" template is available

5. **Given** templates available
   **When** viewed
   **Then** "Custom (user can save own)" template option is available

6. **Given** template is selected
   **When** applied
   **Then** selecting template populates query field

7. **Given** template has placeholders
   **When** applied
   **Then** placeholders (e.g., {TICKET_KEY}) are auto-filled

8. **Given** custom templates
   **When** user saves
   **Then** user can save custom templates

## Tasks

- [ ] Task 1: Create default templates
- [ ] Task 2: Create templates API endpoint
- [ ] Task 3: Implement template selection UI
- [ ] Task 4: Create placeholder replacement logic
- [ ] Task 5: Create save custom template dialog
- [ ] Task 6: Persist custom templates in database

## Dev Notes

### Default Templates

```rust
// crates/qa-pms-api/src/splunk/templates.rs
pub fn get_default_templates() -> Vec<SplunkTemplate> {
    vec![
        SplunkTemplate {
            id: "error_logs".into(),
            name: "Error Logs".into(),
            description: "All error and exception logs".into(),
            query: r#"index=main (level=ERROR OR level=FATAL) 
| table _time, level, message, source, host
| sort -_time"#.into(),
            is_default: true,
            placeholders: vec![],
        },
        SplunkTemplate {
            id: "ticket_logs".into(),
            name: "Logs for Ticket".into(),
            description: "Find logs mentioning the ticket key".into(),
            query: r#"index=main "{TICKET_KEY}"
| table _time, level, message, source
| sort -_time"#.into(),
            is_default: true,
            placeholders: vec!["TICKET_KEY".into()],
        },
        SplunkTemplate {
            id: "user_activity".into(),
            name: "User Activity".into(),
            description: "Logs for a specific user".into(),
            query: r#"index=main user_id="{USER_ID}"
| table _time, action, details, source
| sort -_time"#.into(),
            is_default: true,
            placeholders: vec!["USER_ID".into()],
        },
        SplunkTemplate {
            id: "performance".into(),
            name: "Performance Metrics".into(),
            description: "API response times and performance".into(),
            query: r#"index=main sourcetype=api_access
| stats avg(response_time) as avg_ms, 
        max(response_time) as max_ms, 
        count as requests 
        by endpoint
| sort -avg_ms"#.into(),
            is_default: true,
            placeholders: vec![],
        },
        SplunkTemplate {
            id: "exceptions".into(),
            name: "Exceptions & Stack Traces".into(),
            description: "Application exceptions with full stack traces".into(),
            query: r#"index=main (exception OR stacktrace OR "at " "Exception")
| rex field=_raw "(?<exception_type>\w+Exception)"
| table _time, exception_type, message, _raw
| sort -_time"#.into(),
            is_default: true,
            placeholders: vec![],
        },
    ]
}
```

### Template Types

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SplunkTemplate {
    pub id: String,
    pub name: String,
    pub description: String,
    pub query: String,
    pub is_default: bool,
    pub placeholders: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct CustomTemplate {
    pub id: Uuid,
    pub user_id: String,
    pub name: String,
    pub description: Option<String>,
    pub query: String,
    pub placeholders: Vec<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
```

### Template API Endpoints

```rust
// GET /api/v1/splunk/templates
pub async fn list_templates(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<SplunkTemplate>>, ApiError> {
    let mut templates = get_default_templates();
    
    // Add custom templates
    let custom = sqlx::query_as::<_, CustomTemplate>(
        "SELECT * FROM splunk_templates WHERE user_id = $1 ORDER BY name"
    )
    .bind("current_user")
    .fetch_all(&state.db_pool)
    .await?;

    for ct in custom {
        templates.push(SplunkTemplate {
            id: ct.id.to_string(),
            name: ct.name,
            description: ct.description.unwrap_or_default(),
            query: ct.query,
            is_default: false,
            placeholders: ct.placeholders,
        });
    }

    Ok(Json(templates))
}

// POST /api/v1/splunk/templates
pub async fn save_template(
    State(state): State<Arc<AppState>>,
    Json(request): Json<SaveTemplateRequest>,
) -> Result<(StatusCode, Json<SplunkTemplate>), ApiError> {
    let id = Uuid::new_v4();
    let placeholders = extract_placeholders(&request.query);

    sqlx::query(
        r#"
        INSERT INTO splunk_templates (id, user_id, name, description, query, placeholders)
        VALUES ($1, $2, $3, $4, $5, $6)
        "#,
    )
    .bind(id)
    .bind("current_user")
    .bind(&request.name)
    .bind(&request.description)
    .bind(&request.query)
    .bind(&placeholders)
    .execute(&state.db_pool)
    .await?;

    Ok((StatusCode::CREATED, Json(SplunkTemplate {
        id: id.to_string(),
        name: request.name,
        description: request.description.unwrap_or_default(),
        query: request.query,
        is_default: false,
        placeholders,
    })))
}

fn extract_placeholders(query: &str) -> Vec<String> {
    let re = regex::Regex::new(r"\{([A-Z_]+)\}").unwrap();
    re.captures_iter(query)
        .map(|cap| cap[1].to_string())
        .collect::<HashSet<_>>()
        .into_iter()
        .collect()
}
```

### Frontend - Template UI

```tsx
// frontend/src/components/splunk/TemplateSelector.tsx
interface TemplateSelectorProps {
  templates: SplunkTemplate[];
  onSelect: (template: SplunkTemplate) => void;
  onSaveNew: () => void;
}

export function TemplateSelector({ templates, onSelect, onSaveNew }: TemplateSelectorProps) {
  const defaultTemplates = templates.filter(t => t.isDefault);
  const customTemplates = templates.filter(t => !t.isDefault);

  return (
    <div className="space-y-2">
      <div className="flex items-center justify-between">
        <label className="text-sm font-medium text-neutral-700">Templates</label>
        <button
          onClick={onSaveNew}
          className="text-sm text-primary-600 hover:text-primary-700"
        >
          + Save Current
        </button>
      </div>

      <select
        onChange={(e) => {
          const template = templates.find(t => t.id === e.target.value);
          if (template) onSelect(template);
        }}
        className="w-full px-3 py-2 border border-neutral-300 rounded-lg text-sm"
      >
        <option value="">Select a template...</option>
        
        <optgroup label="Default Templates">
          {defaultTemplates.map(t => (
            <option key={t.id} value={t.id}>{t.name}</option>
          ))}
        </optgroup>

        {customTemplates.length > 0 && (
          <optgroup label="My Templates">
            {customTemplates.map(t => (
              <option key={t.id} value={t.id}>{t.name}</option>
            ))}
          </optgroup>
        )}
      </select>
    </div>
  );
}
```

### Save Template Dialog

```tsx
// frontend/src/components/splunk/SaveTemplateDialog.tsx
interface SaveTemplateDialogProps {
  query: string;
  open: boolean;
  onOpenChange: (open: boolean) => void;
}

export function SaveTemplateDialog({ query, open, onOpenChange }: SaveTemplateDialogProps) {
  const [name, setName] = useState("");
  const [description, setDescription] = useState("");
  const { mutate: save, isPending } = useSaveTemplate();

  const handleSave = () => {
    save(
      { name, description, query },
      { onSuccess: () => onOpenChange(false) }
    );
  };

  return (
    <Dialog.Root open={open} onOpenChange={onOpenChange}>
      <Dialog.Content className="fixed top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2 bg-white rounded-xl p-6 w-full max-w-md">
        <Dialog.Title className="text-xl font-semibold mb-4">Save Template</Dialog.Title>

        <div className="space-y-4">
          <div>
            <label className="block text-sm font-medium text-neutral-700 mb-1">
              Template Name
            </label>
            <input
              type="text"
              value={name}
              onChange={(e) => setName(e.target.value)}
              placeholder="My Custom Query"
              className="w-full px-3 py-2 border border-neutral-300 rounded-lg"
            />
          </div>

          <div>
            <label className="block text-sm font-medium text-neutral-700 mb-1">
              Description (optional)
            </label>
            <input
              type="text"
              value={description}
              onChange={(e) => setDescription(e.target.value)}
              placeholder="What does this query do?"
              className="w-full px-3 py-2 border border-neutral-300 rounded-lg"
            />
          </div>

          <div className="p-3 bg-neutral-50 rounded-lg">
            <p className="text-xs text-neutral-500 mb-1">Query Preview:</p>
            <code className="text-xs text-neutral-700 break-all">{query.slice(0, 100)}...</code>
          </div>
        </div>

        <div className="flex justify-end gap-2 mt-6">
          <Dialog.Close asChild>
            <button className="px-4 py-2 text-neutral-600">Cancel</button>
          </Dialog.Close>
          <button
            onClick={handleSave}
            disabled={!name.trim() || isPending}
            className="px-4 py-2 bg-primary-500 text-white rounded-lg disabled:opacity-50"
          >
            {isPending ? "Saving..." : "Save Template"}
          </button>
        </div>
      </Dialog.Content>
    </Dialog.Root>
  );
}
```

### References

- [Source: epics.md#Story 11.2]
