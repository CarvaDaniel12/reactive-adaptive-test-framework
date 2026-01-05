# Story 7.2: Report Content: Tests Covered and Strategies

Status: ready-for-dev

## Story

As a QA (Ana),
I want my report to include tests and strategies,
So that stakeholders see the testing scope.

## Acceptance Criteria

1. **Given** report is being generated
   **When** content is compiled
   **Then** report includes Tests Covered section (from search results)

2. **Given** Tests Covered section exists
   **When** content is shown
   **Then** Postman collections used are listed

3. **Given** Tests Covered section exists
   **When** content is shown
   **Then** Testmo test cases linked are listed

4. **Given** report is being generated
   **When** content is compiled
   **Then** report includes Strategies Used section

5. **Given** Strategies Used section exists
   **When** content is shown
   **Then** test approach notes are included

6. **Given** Strategies Used section exists
   **When** content is shown
   **Then** edge cases considered are listed

7. **Given** report is being generated
   **When** content is compiled
   **Then** report includes Gap Analysis section

8. **Given** Gap Analysis exists
   **When** content is shown
   **Then** time actual vs estimated is displayed

9. **Given** Gap Analysis exists
   **When** content is shown
   **Then** steps that took longer with reasons are highlighted

10. **Given** section has no data
    **When** report renders
    **Then** sections are optional if no data exists

11. **Given** report is generated
    **When** viewed
    **Then** format is clean and professional

## Tasks

- [ ] Task 1: Create TestsCoveredSection types
- [ ] Task 2: Integrate search results into report
- [ ] Task 3: Create StrategiesSection with notes extraction
- [ ] Task 4: Create GapAnalysisSection
- [ ] Task 5: Update report generation to include all sections
- [ ] Task 6: Create report section components

## Dev Notes

### Extended Report Content Types

```rust
// crates/qa-pms-workflow/src/report/models.rs (additions)

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TestsCoveredSection {
    pub postman_collections: Vec<PostmanTestReference>,
    pub testmo_cases: Vec<TestmoTestReference>,
    pub total_tests: i32,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PostmanTestReference {
    pub collection_name: String,
    pub collection_id: String,
    pub requests_count: i32,
    pub url: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TestmoTestReference {
    pub case_id: i64,
    pub case_title: String,
    pub suite_name: Option<String>,
    pub url: Option<String>,
    pub run_id: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StrategiesSection {
    pub test_approach: Option<String>,
    pub edge_cases: Vec<String>,
    pub additional_considerations: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GapAnalysisSection {
    pub overall: GapSummary,
    pub steps_over_estimate: Vec<StepGap>,
    pub recommendations: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GapSummary {
    pub actual_seconds: i32,
    pub estimated_seconds: i32,
    pub gap_percent: f64,
    pub level: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StepGap {
    pub step_index: i32,
    pub step_name: String,
    pub actual_seconds: i32,
    pub estimated_seconds: i32,
    pub gap_percent: f64,
    pub possible_reason: Option<String>,
}
```

### Report Content Builder

```rust
impl ReportService {
    fn build_tests_covered(
        &self,
        search_results: Option<&SearchResults>,
        step_links: &[StepLink],
    ) -> Option<TestsCoveredSection> {
        let postman: Vec<PostmanTestReference> = search_results
            .map(|r| r.postman.iter().map(|p| PostmanTestReference {
                collection_name: p.collection_name.clone(),
                collection_id: p.collection_id.clone(),
                requests_count: p.requests.len() as i32,
                url: p.url.clone(),
            }).collect())
            .unwrap_or_default();

        let testmo: Vec<TestmoTestReference> = search_results
            .map(|r| r.testmo.iter().map(|t| TestmoTestReference {
                case_id: t.case_id,
                case_title: t.title.clone(),
                suite_name: t.suite_name.clone(),
                url: t.url.clone(),
                run_id: None,
            }).collect())
            .unwrap_or_default();

        // Also include links from step notes
        let from_links: Vec<TestmoTestReference> = step_links.iter()
            .filter(|l| l.link_type == "test")
            .map(|l| TestmoTestReference {
                case_id: 0,
                case_title: l.label.clone(),
                suite_name: None,
                url: Some(l.url.clone()),
                run_id: None,
            })
            .collect();

        let total = postman.len() + testmo.len() + from_links.len();

        if total == 0 {
            return None;
        }

        Some(TestsCoveredSection {
            postman_collections: postman,
            testmo_cases: [testmo, from_links].concat(),
            total_tests: total as i32,
        })
    }

    fn build_strategies(
        &self,
        step_results: &[WorkflowStepResult],
    ) -> Option<StrategiesSection> {
        // Extract from notes using patterns
        let mut edge_cases: Vec<String> = Vec::new();
        let mut approach_notes: Vec<String> = Vec::new();

        for result in step_results {
            if let Some(notes) = &result.notes {
                // Extract edge cases (lines starting with "Edge case:" or in lists after "Edge cases:")
                let edge_case_regex = regex::Regex::new(r"(?i)edge\s*case[s]?[:\-]\s*(.+)").unwrap();
                for cap in edge_case_regex.captures_iter(notes) {
                    if let Some(m) = cap.get(1) {
                        edge_cases.push(m.as_str().trim().to_string());
                    }
                }

                // Extract approach notes
                if notes.to_lowercase().contains("approach") || 
                   notes.to_lowercase().contains("strategy") {
                    approach_notes.push(notes.clone());
                }
            }
        }

        if edge_cases.is_empty() && approach_notes.is_empty() {
            return None;
        }

        Some(StrategiesSection {
            test_approach: approach_notes.first().cloned(),
            edge_cases,
            additional_considerations: Vec::new(),
        })
    }

    fn build_gap_analysis(
        &self,
        steps: &[StepSection],
        time_summary: &TimeSummarySection,
    ) -> GapAnalysisSection {
        let gap_percent = ((time_summary.efficiency_ratio - 1.0) * 100.0).round();
        let level = if gap_percent <= 0.0 {
            "on_target"
        } else if gap_percent <= 20.0 {
            "warning"
        } else {
            "critical"
        };

        let steps_over: Vec<StepGap> = steps.iter()
            .filter_map(|s| {
                let actual = s.time_seconds?;
                let estimated = s.estimated_seconds;
                let ratio = actual as f64 / estimated as f64;
                
                if ratio > 1.2 {
                    Some(StepGap {
                        step_index: s.index,
                        step_name: s.name.clone(),
                        actual_seconds: actual,
                        estimated_seconds: estimated,
                        gap_percent: ((ratio - 1.0) * 100.0).round(),
                        possible_reason: None,
                    })
                } else {
                    None
                }
            })
            .collect();

        let recommendations = if gap_percent > 20.0 {
            vec![
                "Consider reviewing time estimates for this workflow type".into(),
                "Check if additional requirements were discovered during testing".into(),
            ]
        } else {
            Vec::new()
        };

        GapAnalysisSection {
            overall: GapSummary {
                actual_seconds: time_summary.total_active_seconds,
                estimated_seconds: time_summary.total_estimated_seconds,
                gap_percent,
                level: level.into(),
            },
            steps_over_estimate: steps_over,
            recommendations,
        }
    }
}
```

### Frontend Components

```tsx
// frontend/src/components/reports/TestsCoveredSection.tsx
interface TestsCoveredSectionProps {
  data: TestsCoveredSection;
}

export function TestsCoveredSection({ data }: TestsCoveredSectionProps) {
  if (data.totalTests === 0) return null;

  return (
    <section className="space-y-4">
      <h3 className="text-lg font-semibold flex items-center gap-2">
        <CheckCircledIcon className="w-5 h-5 text-success-500" />
        Tests Covered ({data.totalTests})
      </h3>

      {data.postmanCollections.length > 0 && (
        <div>
          <h4 className="text-sm font-medium text-neutral-600 mb-2">
            Postman Collections
          </h4>
          <div className="space-y-2">
            {data.postmanCollections.map((col, i) => (
              <div key={i} className="flex items-center justify-between p-2 bg-neutral-50 rounded">
                <span className="text-sm">{col.collectionName}</span>
                <span className="text-xs text-neutral-400">
                  {col.requestsCount} requests
                </span>
              </div>
            ))}
          </div>
        </div>
      )}

      {data.testmoCases.length > 0 && (
        <div>
          <h4 className="text-sm font-medium text-neutral-600 mb-2">
            Testmo Test Cases
          </h4>
          <div className="space-y-2">
            {data.testmoCases.map((tc, i) => (
              <a
                key={i}
                href={tc.url || '#'}
                target="_blank"
                rel="noopener noreferrer"
                className="flex items-center gap-2 p-2 bg-neutral-50 rounded hover:bg-neutral-100"
              >
                <span className="text-sm">{tc.caseTitle}</span>
                {tc.suiteName && (
                  <span className="text-xs text-neutral-400">({tc.suiteName})</span>
                )}
              </a>
            ))}
          </div>
        </div>
      )}
    </section>
  );
}

// frontend/src/components/reports/GapAnalysisSection.tsx
export function GapAnalysisSection({ data }: GapAnalysisSectionProps) {
  const levelColors = {
    on_target: "text-success-600 bg-success-50",
    warning: "text-warning-600 bg-warning-50",
    critical: "text-error-600 bg-error-50",
  };

  return (
    <section className="space-y-4">
      <h3 className="text-lg font-semibold flex items-center gap-2">
        <TimerIcon className="w-5 h-5" />
        Gap Analysis
      </h3>

      {/* Overall */}
      <div className={cn(
        "p-4 rounded-lg",
        levelColors[data.overall.level as keyof typeof levelColors]
      )}>
        <div className="flex justify-between items-center">
          <span>Overall Efficiency</span>
          <span className="font-bold">
            {data.overall.gapPercent > 0 ? '+' : ''}{data.overall.gapPercent}%
          </span>
        </div>
      </div>

      {/* Steps over estimate */}
      {data.stepsOverEstimate.length > 0 && (
        <div>
          <h4 className="text-sm font-medium text-neutral-600 mb-2">
            Steps Over Estimate
          </h4>
          {data.stepsOverEstimate.map((step) => (
            <div key={step.stepIndex} className="p-3 border border-warning-200 rounded-lg mb-2">
              <div className="flex justify-between">
                <span className="font-medium">{step.stepName}</span>
                <span className="text-warning-600">+{step.gapPercent}%</span>
              </div>
              <div className="text-sm text-neutral-500 mt-1">
                {formatTime(step.actualSeconds)} actual vs {formatTime(step.estimatedSeconds)} estimated
              </div>
            </div>
          ))}
        </div>
      )}

      {/* Recommendations */}
      {data.recommendations.length > 0 && (
        <div className="bg-primary-50 p-4 rounded-lg">
          <h4 className="text-sm font-medium text-primary-700 mb-2">Recommendations</h4>
          <ul className="list-disc list-inside text-sm text-primary-600 space-y-1">
            {data.recommendations.map((rec, i) => (
              <li key={i}>{rec}</li>
            ))}
          </ul>
        </div>
      )}
    </section>
  );
}
```

### References

- [Source: epics.md#Story 7.2]
- [Dependency: Story 7.1 - Report Generation]
- [Dependency: Epic 4 - Search Integration]
