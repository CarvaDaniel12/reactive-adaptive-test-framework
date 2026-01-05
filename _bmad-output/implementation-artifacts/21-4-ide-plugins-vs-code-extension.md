# Story 21.4: IDE Plugins (VS Code Extension)

Status: ready-for-dev

Epic: 21 - Developer Experience
Priority: P1 (High Value)
Estimated Effort: 3 days
Sprint: 1

## Story

As a **QA Engineer**,
I want to **have a VS Code extension for workflow editing and testing**,
So that **I can work within my familiar IDE instead of switching between tools**.

## Acceptance Criteria

1. **Given** I have the VS Code extension installed
   **When** I open a workflow YAML file
   **Then** I get syntax highlighting for workflow properties
   **And** auto-completion for workflow properties (name, description, steps, etc.)
   **And** inline error checking for required fields
   **And** validation of workflow structure (steps, dependencies, integrations)

2. **Given** I want to execute a workflow
   **When** I right-click on workflow file
   **Then** I see "Run Workflow" option in context menu
   **And** workflow executes in integrated terminal
   **And** results are shown in output channel
   **And** status bar shows workflow execution status

3. **Given** I want to validate a workflow
   **When** I save a workflow file
   **Then** workflow is validated automatically
   **And** errors are shown as diagnostics (red squiggles)
   **And** warnings are shown for potential issues
   **And** hover shows error descriptions and suggestions

4. **Given** I want to create a new workflow
   **When** I use snippet prefix "workflow"
   **Then** workflow template is inserted with placeholders
   **And** I can tab through placeholders to fill in details
   **And** common patterns are available (integration steps, Jira config)

5. **Given** I want to explore workflows
   **When** I open Workflow Explorer in sidebar
   **Then** I see list of all workflow files in workspace
   **And** I can click to open workflow files
   **And** I can filter workflows by name or tags
   **And** I can run workflows from explorer

6. **Given** workflow templates exist
   **When** I use command "QA PMS: Create Workflow from Template"
   **Then** I see list of available templates
   **And** I can select a template (bug, feature, regression, etc.)
   **And** new workflow file is created with template content
   **And** file is opened for editing

## Tasks / Subtasks

- [ ] Task 1: Create VS Code extension project structure (AC: #1, #2, #3, #4, #5, #6)
  - [ ] 1.1: Create `vscode-extension/` directory at project root
  - [ ] 1.2: Initialize VS Code extension project with `npm init` or `yo code`
  - [ ] 1.3: Create `package.json` with extension metadata (name, displayName, description, publisher, version)
  - [ ] 1.4: Configure `activationEvents` (onLanguage: qapms-workflow, onCommand, onStartupFinished)
  - [ ] 1.5: Configure `contributes` section (languages, grammars, snippets, commands, views)
  - [ ] 1.6: Create `src/extension.ts` main entry point
  - [ ] 1.7: Add TypeScript configuration (`tsconfig.json`)
  - [ ] 1.8: Add VS Code extension dependencies (`@types/vscode`, `vscode-languageclient` if needed)

- [ ] Task 2: Implement YAML language support for workflows (AC: #1)
  - [ ] 2.1: Create `syntaxes/qapms-workflow.tmLanguage.json` TextMate grammar
  - [ ] 2.2: Define patterns for workflow keywords (name, description, version, steps, jira, etc.)
  - [ ] 2.3: Define patterns for step types (manual, integration, automated)
  - [ ] 2.4: Define patterns for integration names (jira, postman, testmo, splunk)
  - [ ] 2.5: Define patterns for YAML syntax (keys, values, comments, arrays)
  - [ ] 2.6: Create `language-configuration.json` for language features (brackets, comments, word patterns)
  - [ ] 2.7: Test syntax highlighting with sample workflow files

- [ ] Task 3: Implement auto-completion provider (AC: #1)
  - [ ] 3.1: Create `src/providers/completionProvider.ts`
  - [ ] 3.2: Implement `WorkflowCompletionProvider` class extending `vscode.CompletionItemProvider`
  - [ ] 3.3: Add root-level properties (name, description, version, tags, jira, steps)
  - [ ] 3.4: Add step-level properties (id, name, type, description, estimated_time, depends_on, integration, config)
  - [ ] 3.5: Add step type values (manual, integration, automated)
  - [ ] 3.6: Add integration names (jira, postman, testmo, splunk)
  - [ ] 3.7: Register completion provider for 'qapms-workflow' language
  - [ ] 3.8: Add documentation snippets to completion items
  - [ ] 3.9: Test auto-completion in workflow files

- [ ] Task 4: Implement diagnostic provider for validation (AC: #3)
  - [ ] 4.1: Create `src/providers/diagnosticProvider.ts`
  - [ ] 4.2: Implement `WorkflowDiagnosticProvider` class
  - [ ] 4.3: Parse YAML file using `js-yaml` or `yaml` crate bindings
  - [ ] 4.4: Validate required fields (name, steps array)
  - [ ] 4.5: Validate step structure (id, name, type required)
  - [ ] 4.6: Validate step types (must be manual, integration, or automated)
  - [ ] 4.7: Validate integration steps (integration field required)
  - [ ] 4.8: Validate step dependencies (depends_on steps must exist)
  - [ ] 4.9: Check for circular dependencies in steps
  - [ ] 4.10: Validate Jira configuration (ticket required if enabled)
  - [ ] 4.11: Create diagnostics for errors (vscode.DiagnosticSeverity.Error)
  - [ ] 4.12: Create diagnostics for warnings (missing optional fields, unused configs)
  - [ ] 4.13: Register diagnostic collection with `vscode.languages.createDiagnosticCollection()`
  - [ ] 4.14: Hook into `onDidChangeTextDocument` and `onDidSaveTextDocument` events
  - [ ] 4.15: Test validation with valid and invalid workflow files

- [ ] Task 5: Implement workflow execution command (AC: #2)
  - [ ] 5.1: Create `src/commands/runWorkflow.ts`
  - [ ] 5.2: Implement `registerRunWorkflowCommand()` function
  - [ ] 5.3: Register command `qapms.runWorkflow` in `package.json` contributes.commands
  - [ ] 5.4: Check if active editor has workflow file (.yaml or .yml)
  - [ ] 5.5: Create output channel "QA PMS Workflow" using `vscode.window.createOutputChannel()`
  - [ ] 5.6: Execute CLI command `qapms run <workflow-file>` using `child_process.spawn()`
  - [ ] 5.7: Stream stdout/stderr to output channel
  - [ ] 5.8: Show success/error notification based on exit code
  - [ ] 5.9: Add context menu item "Run Workflow" for .yaml/.yml files
  - [ ] 5.10: Test workflow execution from extension

- [ ] Task 6: Create workflow snippets (AC: #4)
  - [ ] 6.1: Create `snippets/snippets.json` file
  - [ ] 6.2: Add "New Workflow" snippet with prefix "workflow"
  - [ ] 6.3: Include placeholders for name, description, version, tags, jira config, steps
  - [ ] 6.4: Add "Integration Step" snippet with prefix "integration-step"
  - [ ] 6.5: Add "Jira Integration" snippet with prefix "jira"
  - [ ] 6.6: Add "Manual Step" snippet with prefix "manual-step"
  - [ ] 6.7: Add "Postman Collection" snippet with prefix "postman"
  - [ ] 6.8: Test snippets insertion and tab navigation

- [ ] Task 7: Implement Workflow Explorer sidebar view (AC: #5)
  - [ ] 7.1: Create `src/views/workflowExplorer.ts`
  - [ ] 7.2: Implement `WorkflowExplorer` class
  - [ ] 7.3: Create `WorkflowTreeDataProvider` implementing `vscode.TreeDataProvider<WorkflowItem>`
  - [ ] 7.4: Register tree view `qapmsWorkflows` in `package.json` contributes.views
  - [ ] 7.5: Scan workspace for workflow files (`**/*.yaml`, `**/*.yml` excluding node_modules)
  - [ ] 7.6: Create `WorkflowItem` class extending `vscode.TreeItem`
  - [ ] 7.7: Display workflow files in tree (grouped by directory if needed)
  - [ ] 7.8: Add context menu actions (Run Workflow, Open File, Create New)
  - [ ] 7.9: Implement refresh command `qapms.refreshWorkflows`
  - [ ] 7.10: Add filter/search functionality (optional)
  - [ ] 7.11: Test workflow explorer with multiple workflow files

- [ ] Task 8: Implement workflow template creation command (AC: #6)
  - [ ] 8.1: Create `src/commands/createWorkflowFromTemplate.ts`
  - [ ] 8.2: Implement `registerCreateWorkflowCommand()` function
  - [ ] 8.3: Register command `qapms.createWorkflowFromTemplate` in package.json
  - [ ] 8.4: Show quick pick with template options (Bug Verification, Feature Acceptance, Regression Testing, etc.)
  - [ ] 8.5: Prompt for workflow name and file path
  - [ ] 8.6: Load template content from extension resources or API call
  - [ ] 8.7: Replace template placeholders with user input
  - [ ] 8.8: Create new workflow file in workspace
  - [ ] 8.9: Open created file in editor
  - [ ] 8.10: Test template creation flow

- [ ] Task 9: Add status bar integration (AC: #2)
  - [ ] 9.1: Create status bar item in `src/extension.ts`
  - [ ] 9.2: Display "QA PMS" text with icon when workflow file is active
  - [ ] 9.3: Show workflow validation status (valid/invalid)
  - [ ] 9.4: Update status bar on document change
  - [ ] 9.5: Add click action to show workflow status/details

- [ ] Task 10: Add comprehensive tests and documentation (AC: All)
  - [ ] 10.1: Create unit tests for completion provider
  - [ ] 10.2: Create unit tests for diagnostic provider
  - [ ] 10.3: Create integration tests for workflow execution
  - [ ] 10.4: Create README.md with installation and usage instructions
  - [ ] 10.5: Create CHANGELOG.md for version tracking
  - [ ] 10.6: Add extension icon and screenshots
  - [ ] 10.7: Test extension in VS Code development host
  - [ ] 10.8: Package extension for publishing (.vsix file)

## Dev Notes

### Architecture Compliance

**Tech Stack:**
- VS Code Extension API (TypeScript/JavaScript)
- Node.js runtime for extension host
- TypeScript for type safety
- `js-yaml` or `yaml` package for YAML parsing
- `child_process` for CLI execution
- VS Code Language Server Protocol (if needed for advanced features)

**Code Structure:**
- **Extension Root:** `vscode-extension/` directory (separate from main Rust project)
- **Source Code:** `vscode-extension/src/` TypeScript files
- **Grammar:** `vscode-extension/syntaxes/qapms-workflow.tmLanguage.json`
- **Snippets:** `vscode-extension/snippets/snippets.json`
- **Configuration:** `vscode-extension/package.json` (extension manifest)

**VS Code Extension Pattern:**
Following VS Code Extension API patterns:
```typescript
import * as vscode from 'vscode';

export function activate(context: vscode.ExtensionContext) {
    // Register providers, commands, views
}

export function deactivate() {
    // Cleanup
}
```

### Context7 Requirements (MANDATORY)

**CRITICAL:** Before implementing any code, use Context7 MCP to:

1. **Resolve library ID**: Search for "vscode extension api typescript"
2. **Query Context7 for**: "How to create VS Code extension with language support and syntax highlighting"
3. **Query Context7 for**: "How to implement auto-completion and diagnostics in VS Code extension"
4. **Verify patterns for**:
   - `vscode.CompletionItemProvider` - auto-completion
   - `vscode.DiagnosticCollection` - validation/errors
   - `vscode.TreeDataProvider` - sidebar views
   - TextMate grammar syntax highlighting
   - Command registration and execution
5. **Check best practices for**:
   - VS Code extension structure and packaging
   - Language server protocol (if needed)
   - Extension activation events and lifecycle

### Previous Story Intelligence

**From Story 21.2 (Code Generation Templates):**
- Workflow template structure is defined
- Template generation API exists
- This extension can integrate with template generation API
- Extension can use templates for "Create Workflow from Template" feature

**From Story 5.1 (Workflow Templates):**
- Workflow structure is defined (`WorkflowTemplate`, `WorkflowStep`)
- Validation rules are established
- Extension should validate against same structure

**Integration Points:**
- Extension can call CLI tool (`qapms`) for workflow execution
- Extension can call API endpoints for template generation (if API exists)
- Extension validates workflows using same rules as backend

### Project Structure Notes

**Alignment with unified structure:**
- ✅ VS Code extension in separate directory (`vscode-extension/`)
- ✅ Extension is optional tooling (doesn't affect core functionality)
- ✅ Extension can be packaged and published to VS Code Marketplace

**Files to Create:**
- `vscode-extension/package.json` - Extension manifest
- `vscode-extension/src/extension.ts` - Main entry point
- `vscode-extension/src/providers/completionProvider.ts` - Auto-completion
- `vscode-extension/src/providers/diagnosticProvider.ts` - Validation
- `vscode-extension/src/commands/runWorkflow.ts` - Run workflow command
- `vscode-extension/src/commands/createWorkflowFromTemplate.ts` - Template creation
- `vscode-extension/src/views/workflowExplorer.ts` - Sidebar view
- `vscode-extension/syntaxes/qapms-workflow.tmLanguage.json` - Syntax highlighting
- `vscode-extension/snippets/snippets.json` - Code snippets
- `vscode-extension/tsconfig.json` - TypeScript configuration
- `vscode-extension/README.md` - Extension documentation

**Files to Modify:**
- `qa-intelligent-pms/Cargo.toml` - Ensure CLI tool (`qapms`) is buildable for extension to use
- Project documentation - Add extension installation instructions

**Naming Conventions:**
- Extension ID: `qapms.workflow-extension` (or similar)
- Commands: `qapms.runWorkflow`, `qapms.createWorkflowFromTemplate`, `qapms.refreshWorkflows`
- Language ID: `qapms-workflow`

### Testing Standards

**Unit Tests:**
- Test completion provider suggestions
- Test diagnostic provider validation logic
- Test YAML parsing and error handling

**Integration Tests:**
- Test extension activation in VS Code
- Test workflow execution from extension
- Test template creation flow

**Manual Testing:**
- Test syntax highlighting with various workflow files
- Test auto-completion in different contexts
- Test validation with valid/invalid workflows
- Test workflow explorer with multiple files
- Test snippets insertion and navigation

**Test Coverage Target:**
- Minimum 70% coverage for TypeScript code
- All commands should be tested
- All providers should be tested

### References

- **Source: `_bmad-output/planning-artifacts/epics-detailed/epic-21-developer-experience.md#story-21.4`** - Story requirements
- **Source: VS Code Extension API Documentation** - Extension development patterns
- **Source: `_bmad-output/implementation-artifacts/21-2-code-generation-templates-for-workflows.md`** - Template structure reference

## Dev Agent Record

### Agent Model Used

Claude Sonnet 4.5 (via Cursor)

### Debug Log References

(None yet - story not implemented)

### Completion Notes List

(None yet - story not implemented)

### File List

**Created:**
- `vscode-extension/package.json` - Extension manifest
- `vscode-extension/src/extension.ts` - Main entry point
- `vscode-extension/src/providers/completionProvider.ts` - Auto-completion provider
- `vscode-extension/src/providers/diagnosticProvider.ts` - Validation provider
- `vscode-extension/src/commands/runWorkflow.ts` - Run workflow command
- `vscode-extension/src/commands/createWorkflowFromTemplate.ts` - Template creation command
- `vscode-extension/src/views/workflowExplorer.ts` - Sidebar explorer view
- `vscode-extension/syntaxes/qapms-workflow.tmLanguage.json` - Syntax highlighting grammar
- `vscode-extension/snippets/snippets.json` - Code snippets
- `vscode-extension/tsconfig.json` - TypeScript configuration
- `vscode-extension/README.md` - Extension documentation

**Modified:**
- Project documentation - Add extension installation instructions

### Change Log

**2026-01-10 - Story Created:**
- Initial story file created with complete structure following BMAD standard
- Added all required sections: Story, Metadata, Acceptance Criteria (6 ACs), Tasks (10 tasks with subtasks), Dev Notes, Dev Agent Record, File List
- Converted acceptance criteria from epic format to Given/When/Then format
- Added comprehensive dev notes with architecture patterns, Context7 requirements, integration points
