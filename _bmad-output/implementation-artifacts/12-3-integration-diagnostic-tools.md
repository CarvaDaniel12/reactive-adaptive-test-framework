# Story 12.3: Integration Diagnostic Tools

Status: ready-for-dev

## Story

As a support person (Sofia),
I want to run diagnostics on integrations,
So that I can quickly identify connection issues.

## Acceptance Criteria

1. **Given** support person views an issue
   **When** they click "Run Diagnostics"
   **Then** health check runs on all integrations

2. **Given** diagnostics run
   **When** credentials checked
   **Then** credential validation is performed

3. **Given** diagnostics run
   **When** latency measured
   **Then** latency measurement is displayed

4. **Given** diagnostics run
   **When** errors counted
   **Then** recent error count per integration is shown

5. **Given** diagnostics complete
   **When** results displayed
   **Then** results show pass/fail indicators

6. **Given** results displayed
   **When** issues found
   **Then** suggestions for common fixes are shown

7. **Given** user-specific issue
   **When** diagnostics run
   **Then** can be run for specific user's config

## Tasks

- [ ] Task 1: Create DiagnosticsPanel component
- [ ] Task 2: Create integration health check service
- [ ] Task 3: Implement latency measurement
- [ ] Task 4: Create error count queries
- [ ] Task 5: Build suggestions engine
- [ ] Task 6: Create user-specific diagnostics

## Dev Notes

### Diagnostics API

```rust
// crates/qa-pms-api/src/services/diagnostics.rs
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DiagnosticsResult {
    pub integrations: Vec<IntegrationDiagnostic>,
    pub overall_status: OverallStatus,
    pub run_at: DateTime<Utc>,
    pub duration_ms: u64,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct IntegrationDiagnostic {
    pub name: String,
    pub status: DiagnosticStatus,
    pub checks: Vec<DiagnosticCheck>,
    pub latency_ms: Option<u64>,
    pub recent_errors: i64,
    pub suggestions: Vec<String>,
}

#[derive(Debug, Clone, Copy, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum DiagnosticStatus {
    Healthy,
    Degraded,
    Unhealthy,
    Unknown,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DiagnosticCheck {
    pub name: String,
    pub passed: bool,
    pub message: String,
    pub duration_ms: u64,
}

pub struct DiagnosticsService {
    pool: PgPool,
    config: Arc<AppConfig>,
}

impl DiagnosticsService {
    pub async fn run_diagnostics(&self, user_id: Option<&str>) -> Result<DiagnosticsResult> {
        let start = std::time::Instant::now();
        let mut integrations = Vec::new();

        // Jira diagnostics
        integrations.push(self.check_jira(user_id).await);
        
        // Postman diagnostics
        integrations.push(self.check_postman(user_id).await);
        
        // Testmo diagnostics
        integrations.push(self.check_testmo(user_id).await);
        
        // Splunk diagnostics (if configured)
        if self.config.splunk.is_some() {
            integrations.push(self.check_splunk().await);
        }

        // Database diagnostics
        integrations.push(self.check_database().await);

        let overall = Self::calculate_overall_status(&integrations);

        Ok(DiagnosticsResult {
            integrations,
            overall_status: overall,
            run_at: Utc::now(),
            duration_ms: start.elapsed().as_millis() as u64,
        })
    }

    async fn check_jira(&self, user_id: Option<&str>) -> IntegrationDiagnostic {
        let mut checks = Vec::new();
        let mut suggestions = Vec::new();
        let start = std::time::Instant::now();

        // Check 1: Credentials exist
        let creds_check = self.check_jira_credentials(user_id).await;
        checks.push(creds_check.clone());
        if !creds_check.passed {
            suggestions.push("Configure Jira credentials in Settings".into());
        }

        // Check 2: API connectivity
        if creds_check.passed {
            let api_check = self.check_jira_api_connectivity().await;
            checks.push(api_check.clone());
            if !api_check.passed {
                suggestions.push("Check Jira URL is accessible from this network".into());
                suggestions.push("Verify API token hasn't expired".into());
            }
        }

        // Check 3: Permissions
        if checks.iter().all(|c| c.passed) {
            let perms_check = self.check_jira_permissions().await;
            checks.push(perms_check.clone());
            if !perms_check.passed {
                suggestions.push("Ensure API token has read permissions for projects".into());
            }
        }

        let latency = start.elapsed().as_millis() as u64;
        let status = Self::checks_to_status(&checks);
        let recent_errors = self.count_recent_errors("jira").await.unwrap_or(0);

        IntegrationDiagnostic {
            name: "Jira".into(),
            status,
            checks,
            latency_ms: Some(latency),
            recent_errors,
            suggestions,
        }
    }

    async fn check_jira_credentials(&self, _user_id: Option<&str>) -> DiagnosticCheck {
        let start = std::time::Instant::now();
        // Check if credentials are configured
        let exists = self.config.jira.is_some();
        
        DiagnosticCheck {
            name: "Credentials Configured".into(),
            passed: exists,
            message: if exists { "Jira credentials found" } else { "No Jira credentials configured" }.into(),
            duration_ms: start.elapsed().as_millis() as u64,
        }
    }

    async fn check_jira_api_connectivity(&self) -> DiagnosticCheck {
        let start = std::time::Instant::now();
        
        let result = tokio::time::timeout(
            std::time::Duration::from_secs(10),
            self.jira_client.get_server_info()
        ).await;

        let (passed, message) = match result {
            Ok(Ok(_)) => (true, "Successfully connected to Jira API".into()),
            Ok(Err(e)) => (false, format!("API error: {}", e)),
            Err(_) => (false, "Connection timed out after 10s".into()),
        };

        DiagnosticCheck {
            name: "API Connectivity".into(),
            passed,
            message,
            duration_ms: start.elapsed().as_millis() as u64,
        }
    }

    fn checks_to_status(checks: &[DiagnosticCheck]) -> DiagnosticStatus {
        let passed = checks.iter().filter(|c| c.passed).count();
        let total = checks.len();
        
        if passed == total {
            DiagnosticStatus::Healthy
        } else if passed == 0 {
            DiagnosticStatus::Unhealthy
        } else {
            DiagnosticStatus::Degraded
        }
    }

    async fn count_recent_errors(&self, integration: &str) -> Result<i64> {
        sqlx::query_scalar(
            r#"
            SELECT COUNT(*) FROM error_logs
            WHERE message ILIKE '%' || $1 || '%'
              AND created_at > NOW() - INTERVAL '24 hours'
            "#,
        )
        .bind(integration)
        .fetch_one(&self.pool)
        .await
        .map_err(Into::into)
    }
}
```

### Diagnostics Panel Component

```tsx
// frontend/src/components/support/DiagnosticsPanel.tsx
import { useState } from "react";
import { useRunDiagnostics } from "@/hooks/useDiagnostics";

interface DiagnosticsPanelProps {
  userId?: string;
}

export function DiagnosticsPanel({ userId }: DiagnosticsPanelProps) {
  const { mutate: runDiagnostics, data: results, isPending } = useRunDiagnostics();

  const handleRun = () => {
    runDiagnostics({ userId });
  };

  return (
    <div className="bg-white rounded-xl border border-neutral-200">
      <div className="p-4 border-b border-neutral-200 flex items-center justify-between">
        <div>
          <h3 className="font-semibold">Integration Diagnostics</h3>
          {userId && (
            <p className="text-sm text-neutral-500">For user: {userId}</p>
          )}
        </div>
        <button
          onClick={handleRun}
          disabled={isPending}
          className="px-4 py-2 bg-primary-500 text-white rounded-lg hover:bg-primary-600 disabled:opacity-50"
        >
          {isPending ? "Running..." : "Run Diagnostics"}
        </button>
      </div>

      {results && (
        <div className="p-4 space-y-4">
          {/* Overall Status */}
          <div className={cn(
            "p-4 rounded-lg flex items-center justify-between",
            results.overallStatus === "healthy" ? "bg-success-50" :
            results.overallStatus === "degraded" ? "bg-warning-50" : "bg-error-50"
          )}>
            <div className="flex items-center gap-3">
              <StatusIcon status={results.overallStatus} />
              <div>
                <p className="font-medium">Overall Status: {results.overallStatus}</p>
                <p className="text-sm text-neutral-600">
                  Completed in {results.durationMs}ms
                </p>
              </div>
            </div>
          </div>

          {/* Integration Results */}
          <div className="space-y-3">
            {results.integrations.map((integration) => (
              <IntegrationResult key={integration.name} data={integration} />
            ))}
          </div>
        </div>
      )}
    </div>
  );
}

function IntegrationResult({ data }: { data: IntegrationDiagnostic }) {
  const [expanded, setExpanded] = useState(false);

  const statusIcon = {
    healthy: <CheckCircledIcon className="w-5 h-5 text-success-500" />,
    degraded: <ExclamationTriangleIcon className="w-5 h-5 text-warning-500" />,
    unhealthy: <CrossCircledIcon className="w-5 h-5 text-error-500" />,
    unknown: <QuestionMarkCircledIcon className="w-5 h-5 text-neutral-400" />,
  };

  return (
    <div className="border border-neutral-200 rounded-lg">
      <button
        onClick={() => setExpanded(!expanded)}
        className="w-full p-4 flex items-center justify-between hover:bg-neutral-50"
      >
        <div className="flex items-center gap-3">
          {statusIcon[data.status]}
          <span className="font-medium">{data.name}</span>
        </div>
        <div className="flex items-center gap-4 text-sm text-neutral