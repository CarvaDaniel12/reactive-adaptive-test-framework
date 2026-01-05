# Story 10.5: Problematic Endpoints Display

Status: ready-for-dev

## Story

As a PM (Carlos),
I want to see which endpoints have the most issues,
So that I can focus technical improvements.

## Acceptance Criteria

1. **Given** tickets reference specific endpoints
   **When** endpoints section renders
   **Then** top 10 problematic endpoints are displayed

2. **Given** endpoint is displayed
   **When** count shown
   **Then** issue count per endpoint is visible

3. **Given** endpoint is displayed
   **When** types shown
   **Then** common issue types are listed

4. **Given** endpoint is displayed
   **When** trend shown
   **Then** trend over time is visible

5. **Given** endpoint data extraction
   **When** processed
   **Then** endpoint data is extracted from ticket descriptions/notes

6. **Given** endpoint is interactive
   **When** clicked
   **Then** clicking endpoint shows related tickets

7. **Given** endpoint data
   **When** export needed
   **Then** can export endpoint report

## Tasks

- [ ] Task 1: Create endpoint extraction logic
- [ ] Task 2: Implement issue type categorization
- [ ] Task 3: Create ProblematicEndpointsSection component
- [ ] Task 4: Create EndpointCard component
- [ ] Task 5: Create EndpointDetailDialog
- [ ] Task 6: Implement endpoint report export

## Dev Notes

### Endpoint Extraction Service

```rust
// crates/qa-pms-analytics/src/endpoints.rs
use regex::Regex;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProblematicEndpoint {
    pub endpoint: String,
    pub method: Option<String>,
    pub issue_count: i64,
    pub common_issues: Vec<IssueType>,
    pub trend: TrendDirection,
    pub affected_tickets: Vec<String>,
    pub last_issue_date: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct IssueType {
    pub category: String, // "timeout", "error_500", "validation", etc.
    pub count: i64,
}

impl EndpointAnalysisService {
    pub async fn get_problematic_endpoints(&self, period: &str, limit: usize) -> Result<Vec<ProblematicEndpoint>> {
        let (start, end, _, _) = Self::calculate_period_ranges(period);

        // Get all ticket descriptions and notes with potential endpoints
        let raw_data = self.get_raw_endpoint_data(start, end).await?;

        // Extract endpoints
        let mut endpoint_issues: HashMap<String, EndpointData> = HashMap::new();

        for (ticket_key, text, completed_at) in raw_data {
            let endpoints = self.extract_endpoints(&text);
            let issues = self.extract_issue_types(&text);

            for endpoint in endpoints {
                let entry = endpoint_issues.entry(endpoint.clone()).or_insert_with(|| EndpointData {
                    endpoint,
                    method: None,
                    tickets: Vec::new(),
                    issues: Vec::new(),
                    last_date: completed_at,
                });
                
                entry.tickets.push(ticket_key.clone());
                entry.issues.extend(issues.clone());
                if completed_at > entry.last_date {
                    entry.last_date = completed_at;
                }
            }
        }

        // Convert to ProblematicEndpoint and sort
        let mut endpoints: Vec<ProblematicEndpoint> = endpoint_issues
            .into_values()
            .map(|data| ProblematicEndpoint {
                endpoint: data.endpoint,
                method: data.method,
                issue_count: data.tickets.len() as i64,
                common_issues: Self::aggregate_issues(&data.issues),
                trend: TrendDirection::Stable, // Would calculate from historical data
                affected_tickets: data.tickets,
                last_issue_date: data.last_date,
            })
            .collect();

        endpoints.sort_by(|a, b| b.issue_count.cmp(&a.issue_count));
        endpoints.truncate(limit);

        Ok(endpoints)
    }

    fn extract_endpoints(&self, text: &str) -> Vec<String> {
        let patterns = [
            // REST-style paths
            Regex::new(r"(?i)/api/v\d+/[a-z][a-z0-9/_-]+").unwrap(),
            // Generic API paths
            Regex::new(r"(?i)/[a-z]+/[a-z][a-z0-9/_-]+").unwrap(),
            // Full URLs
            Regex::new(r"https?://[^\s]+/api/[^\s]+").unwrap(),
        ];

        let mut endpoints: HashSet<String> = HashSet::new();

        for pattern in &patterns {
            for cap in pattern.find_iter(text) {
                let endpoint = cap.as_str()
                    .split('?').next().unwrap() // Remove query params
                    .to_lowercase();
                endpoints.insert(endpoint);
            }
        }

        endpoints.into_iter().collect()
    }

    fn extract_issue_types(&self, text: &str) -> Vec<String> {
        let text_lower = text.to_lowercase();
        let mut issues = Vec::new();

        let patterns = [
            ("timeout", &["timeout", "timed out", "time out"]),
            ("500_error", &["500", "internal server error", "server error"]),
            ("400_error", &["400", "bad request"]),
            ("404_error", &["404", "not found"]),
            ("validation", &["validation", "invalid"]),
            ("authentication", &["401", "403", "unauthorized", "forbidden"]),
            ("performance", &["slow", "performance", "latency"]),
        ];

        for (category, keywords) in patterns {
            if keywords.iter().any(|kw| text_lower.contains(kw)) {
                issues.push(category.to_string());
            }
        }

        issues
    }

    fn aggregate_issues(issues: &[String]) -> Vec<IssueType> {
        let mut counts: HashMap<String, i64> = HashMap::new();
        for issue in issues {
            *counts.entry(issue.clone()).or_insert(0) += 1;
        }

        let mut result: Vec<IssueType> = counts
            .into_iter()
            .map(|(category, count)| IssueType { category, count })
            .collect();

        result.sort_by(|a, b| b.count.cmp(&a.count));
        result.truncate(3); // Top 3 issue types
        result
    }
}
```

### Problematic Endpoints Section

```tsx
// frontend/src/components/pm-dashboard/ProblematicEndpointsSection.tsx
interface ProblematicEndpointsSectionProps {
  data?: ProblematicEndpoint[];
  isLoading: boolean;
}

export function ProblematicEndpointsSection({ data, isLoading }: ProblematicEndpointsSectionProps) {
  const [selectedEndpoint, setSelectedEndpoint] = useState<string | null>(null);

  if (isLoading) {
    return (
      <div className="bg-white rounded-xl border border-neutral-200 p-6">
        <div className="h-64 bg-neutral-100 animate-pulse rounded-lg" />
      </div>
    );
  }

  return (
    <div className="bg-white rounded-xl border border-neutral-200">
      <div className="p-4 border-b border-neutral-200 flex items-center justify-between">
        <div>
          <h3 className="font-semibold text-neutral-900">Problematic Endpoints</h3>
          <p className="text-sm text-neutral-500">Top 10 by issue frequency</p>
        </div>
        <button className="text-sm text-primary-600 hover:text-primary-700">
          Export Report
        </button>
      </div>

      <div className="divide-y divide-neutral-100 max-h-96 overflow-y-auto">
        {data?.length === 0 ? (
          <div className="p-8 text-center text-neutral-500">
            No endpoint issues detected
          </div>
        ) : (
          data?.map((endpoint, index) => (
            <EndpointCard
              key={endpoint.endpoint}
              rank={index + 1}
              data={endpoint}
              onClick={() => setSelectedEndpoint(endpoint.endpoint)}
            />
          ))
        )}
      </div>

      {/* Detail Dialog */}
      {selectedEndpoint && (
        <EndpointDetailDialog
          endpoint={selectedEndpoint}
          open={!!selectedEndpoint}
          onOpenChange={() => setSelectedEndpoint(null)}
        />
      )}
    </div>
  );
}
```

### Endpoint Card

```tsx
// frontend/src/components/pm-dashboard/EndpointCard.tsx
interface EndpointCardProps {
  rank: number;
  data: ProblematicEndpoint;
  onClick: () => void;
}

export function EndpointCard({ rank, data, onClick }: EndpointCardProps) {
  const issueColors: Record<string, string> = {
    timeout: "bg-warning-100 text-warning-700",
    "500_error": "bg-error-100 text-error-700",
    "400_error": "bg-orange-100 text-orange-700",
    "404_error": "bg-neutral-100 text-neutral-700",
    validation: "bg-purple-100 text-purple-700",
    authentication: "bg-red-100 text-red-700",
    performance: "bg-yellow-100 text-yellow-700",
  };

  return (
    <button
      onClick={onClick}
      className="w-full text-left p-4 hover:bg-neutral-50 transition-colors"
    >
      <div className="flex items-start gap-3">
        <span className={cn(
          "w-6 h-6 rounded-full flex items-center justify-center text-xs font-bold",
          rank <= 3 ? "bg-error-100 text-error-700" : "bg-neutral-100 text-neutral-600"
        )}>
          {rank}
        </span>

        <div className="flex-1 min-w-0">
          <div className="flex items-center gap-2">
            {data.method && (
              <span className="px-1.5 py-0.5 text-xs font-mono bg-primary-100 text-primary-700 rounded">
                {data.method}
              </span>
            )}
            <code className="text-sm font-mono text-neutral-900 truncate">
              {data.endpoint}
            </code>
          </div>

          <div className="flex items-center gap-2 mt-2">
            <span className="text-sm text-neutral-500">
              {data.issueCount} issues
            </span>
            <span className="text-neutral-300">•</span>
            <span className="text-sm text-neutral-400">
              {formatDistanceToNow(new Date(data.lastIssueDate), { addSuffix: true })}
            </span>
          </div>

          {/* Issue Types */}
          {data.commonIssues.length > 0 && (
            <div className="flex flex-wrap gap-1 mt-2">
              {data.commonIssues.map((issue) => (
                <span
                  key={issue.category}
                  className={cn(
                    "px-2 py-0.5 text-xs rounded-full",
                    issueColors[issue.category] || "bg-neutral-100 text-neutral-600"
                  )}
                >
                  {issue.category.replace("_", " ")} ({issue.count})
                </span>
              ))}
            </div>
          )}
        </div>

        <ChevronRightIcon className="w-5 h-5 text-neutral-400 mt-1" />
      </div>
    </button>
  );
}
```

### Endpoint Detail Dialog

```tsx
// frontend/src/components/pm-dashboard/EndpointDetailDialog.tsx
export function EndpointDetailDialog({ endpoint, open, onOpenChange }: EndpointDetailDialogProps) {
  const { data, isLoading } = useEndpointDetails(endpoint);

  return (
    <Dialog.Root open={open} onOpenChange={onOpenChange}>
      <Dialog.Content className="fixed top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2 
                                 bg-white rounded-xl p-6 w-full max-w-2xl max-h-[80vh] overflow-y-auto">
        <Dialog.Title className="flex items-center gap-2 text-xl font-semibold mb-2">
          <CodeIcon className="w-5 h-5" />
          Endpoint Details
        </Dialog.Title>
        
        <code className="block p-3 bg-neutral-100 rounded-lg text-sm font-mono mb-6">
          {endpoint}
        </code>

        {/* Issue Timeline */}
        <section className="mb-6">
          <h4 className="font-medium text-neutral-700 mb-3">Issue Timeline</h4>
          <div className="h-32">
            <EndpointIssueChart data={data?.timeline} />
          </div>
        </section>

        {/* Related Tickets */}
        <section>
          <h4 className="font-medium text-neutral-700 mb-3">
            Affected Tickets ({data?.tickets.length || 0})
          </h4>
          <div className="space-y-2 max-h-48 overflow-y-auto">
            {data?.tickets.map((ticket) => (
              <Link
                key={ticket.key}
                to={`/tickets/${ticket.key}`}
                className="flex items-center justify-between p-3 bg-neutral-50 rounded-lg hover:bg-neutral-100"
              >
                <div className="flex items-center gap-2">
                  <span className="font-mono text-sm text-primary-600">{ticket.key}</span>
                  <span className="text-sm text-neutral-600 truncate">{ticket.title}</span>
                </div>
                <span className="text-xs text-neutral-400">
                  {format(new Date(ticket.date), "MMM d")}
                </span>
              </Link>
            ))}
          </div>
        </section>
      </Dialog.Content>
    </Dialog.Root>
  );
}
```

### References

- [Source: epics.md#Story 10.5]
