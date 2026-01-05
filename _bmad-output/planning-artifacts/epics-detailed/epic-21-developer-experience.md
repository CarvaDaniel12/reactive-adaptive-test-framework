# Project Management
qapms init <project-name>              # Initialize new project
qapms init --template <template>       # Use specific template
qapms status                           # Show project status
qapms doctor                           # Diagnose common issues

# Workflow Commands
qapms workflow list                    # List all workflows
qapms workflow create <name>           # Create new workflow
qapms workflow generate <ticket-id>    # Generate workflow from ticket
qapms workflow validate <file>         # Validate workflow YAML
qapms workflow export <name>           # Export workflow as JSON/Markdown

# Integration Commands
qapms integration list                 # List configured integrations
qapms integration add <provider>        # Add integration (interactive)
qapms integration test <provider>       # Test connection
qapms integration remove <provider>    # Remove integration
qapms integration sync <provider>      # Sync data from provider

# Execution Commands
qapms run <workflow-name>              # Execute workflow
qapms run --interactive                # Run with interactive prompts
qapms run --dry-run                    # Validate without executing
qapms run --continue                   # Continue paused workflow

# Development Commands
qapms dev                              # Start dev server with hot reload
qapms dev --port 3000                  # Start on custom port
qapms dev --debug                      # Enable debug logging
qapms dev --profile                    # Enable profiling

# Data Commands
qapms data mock                        # Generate mock test data
qapms data import <file>               # Import tickets from file
qapms data export <format>             # Export data (JSON, CSV, PDF)
qapms data seed                        # Seed database with sample data

# Utility Commands
qapms config get <key>                 # Get config value
qapms config set <key> <value>         # Set config value
qapms config list                      # List all config
qapms config validate                  # Validate configuration
qapms logs --tail                      # Tail application logs
qapms logs --filter <keyword>          # Filter logs

# Help & Docs
qapms help                             # Show all commands
qapms help <command>                   # Show command help
qapms docs                             # Open documentation in browser
qapms examples                         # List code examples
```

### Technical Requirements

- CLI built with `clap` crate
- Interactive prompts with `dialoguer` crate
- Color output with `colored` or `termcolor`
- Progress bars for long operations
- Auto-completion support (bash, zsh, fish, PowerShell)
- Configuration file management (YAML/TOML)
- Secret management (encrypted storage)
- Cross-platform support (Windows, macOS, Linux)

### Implementation Example

```rust
use clap::{Parser, Subcommand};
use dialoguer::{Input, Password, Select, Confirm};
use colored::*;

#[derive(Parser)]
#[command(name = "qapms")]
#[command(about = "QA Intelligent PMS CLI", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize a new QA PMS project
    Init {
        /// Project name
        name: String,
        /// Template to use
        #[arg(short, long, default_value = "standard")]
        template: String,
    },
    /// Create a new workflow
    Workflow {
        #[command(subcommand)]
        action: WorkflowCommands,
    },
    /// Add integration
    Integration {
        #[command(subcommand)]
        action: IntegrationCommands,
    },
    /// Execute workflow
    Run {
        /// Workflow name or ID
        name: String,
        /// Interactive mode
        #[arg(short, long)]
        interactive: bool,
        /// Dry run (validate only)
        #[arg(long)]
        dry_run: bool,
    },
    /// Start development server
    Dev {
        /// Port
        #[arg(short, long, default_value_t = 3000)]
        port: u16,
        /// Enable debug mode
        #[arg(short, long)]
        debug: bool,
    },
}

// Workflow subcommands
#[derive(Subcommand)]
enum WorkflowCommands {
    Create {
        name: String,
        #[arg(short, long, default_value = "standard")]
        template: String,
    },
    Generate {
        ticket_id: String,
    },
    Validate {
        file: String,
    },
}

// Integration subcommands
#[derive(Subcommand)]
enum IntegrationCommands {
    Add {
        provider: String,
    },
    Test {
        provider: String,
    },
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Init { name, template } => init_project(name, template)?,
        Commands::Workflow { action } => handle_workflow(action)?,
        Commands::Integration { action } => handle_integration(action)?,
        Commands::Run { name, interactive, dry_run } => {
            run_workflow(name, interactive, dry_run)?
        }
        Commands::Dev { port, debug } => start_dev_server(port, debug)?,
    }

    Ok(())
}

fn init_project(name: String, template: String) -> Result<(), Box<dyn std::error::Error>> {
    println!("{} Creating new project: {}", "[INFO]".cyan(), name);
    
    // Create directory structure
    let project_dir = std::path::PathBuf::from(&name);
    std::fs::create_dir_all(&project_dir)?;
    
    let workflows_dir = project_dir.join("workflows");
    std::fs::create_dir_all(&workflows_dir)?;
    
    let config_dir = project_dir.join("config");
    std::fs::create_dir_all(&config_dir)?;
    
    // Generate configuration file
    let config_content = format!(
        r#"# QA Intelligent PMS Configuration
# Generated automatically by qapms init

project:
  name: {}
  created_at: {}
  version: 1.0

database:
  url: postgresql://localhost/qapms_{}
  
integrations:
  jira:
    enabled: false
    base_url: ""
    email: ""
  postman:
    enabled: false
    api_key: ""
  testmo:
    enabled: false
    url: ""
    
workflows:
  auto_save: true
  templates_dir: ./templates
"#,
        name,
        chrono::Utc::now().to_rfc3339(),
        name.to_lowercase().replace(" ", "_")
    );
    
    std::fs::write(project_dir.join("config").join("qapms.yml"), config_content)?;
    
    println!("{} Project created successfully!", "[SUCCESS]".green());
    println!();
    println!("Next steps:");
    println!("  1. cd {}", name);
    println!("  2. qapms integration add jira");
    println!("  3. qapms workflow create my-first-test");
    println!("  4. qapms dev");
    
    Ok(())
}

fn handle_integration(action: IntegrationCommands) -> Result<(), Box<dyn std::error::Error>> {
    match action {
        IntegrationCommands::Add { provider } => {
            println!("{} Adding integration: {}", "[INFO]".cyan(), provider);
            
            match provider.as_str() {
                "jira" => add_jira_integration()?,
                "postman" => add_postman_integration()?,
                "testmo" => add_testmo_integration()?,
                _ => eprintln!("{} Unknown provider: {}", "[ERROR]".red(), provider),
            }
        }
        IntegrationCommands::Test { provider } => {
            println!("{} Testing integration: {}", "[INFO]".cyan(), provider);
            // Test implementation
        }
    }
    Ok(())
}

fn add_jira_integration() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n{} Jira Integration Setup", "=".cyan());
    
    // Base URL
    let base_url = Input::new()
        .with_prompt("Jira Base URL")
        .with_initial_text("https://yourcompany.atlassian.net")
        .interact()?;
    
    // Email
    let email = Input::new()
        .with_prompt("Jira Email")
        .interact()?;
    
    // API Token (password)
    let api_token = Password::new()
        .with_prompt("Jira API Token")
        .with_confirmation("Confirm API Token", "Passwords do not match")
        .interact()?;
    
    // Test connection
    println!("\n{} Testing connection...", "[INFO]".cyan());
    // Simulated connection test
    println!("{} Connection successful!", "[SUCCESS]".green());
    
    // Save configuration (encrypted)
    let config = JiraConfig {
        base_url,
        email,
        api_token: encrypt(&api_token)?,
        enabled: true,
    };
    
    save_integration_config("jira", &config)?;
    
    println!("{} Jira integration configured successfully!", "[SUCCESS]".green());
    Ok(())
}
```

### Interactive Prompt Example

```rust
use dialoguer::{theme::ColorfulTheme, MultiSelect, Select};

fn create_workflow_interactive() -> Result<(), Box<dyn std::error::Error>> {
    let theme = ColorfulTheme::default();
    
    // Select template
    let templates = vec![
        "Standard Bug Verification",
        "Regression Testing",
        "Smoke Testing",
        "Integration Testing",
        "Performance Testing",
    ];
    
    let selection = Select::with_theme(&theme)
        .with_prompt("Select workflow template")
        .items(&templates)
        .interact()?;
    
    // Select integrations to include
    let integrations = vec![
        "Jira Ticket Sync",
        "Postman Collection",
        "Testmo Test Run",
        "Splunk Log Query",
    ];
    
    let selected_integrations = MultiSelect::with_theme(&theme)
        .with_prompt("Select integrations to include")
        .items(&integrations)
        .interact()?;
    
    // Workflow name
    let name = Input::with_theme(&theme)
        .with_prompt("Workflow name")
        .interact()?;
    
    // Description
    let description = Input::with_theme(&theme)
        .with_prompt("Description")
        .allow_empty(true)
        .interact()?;
    
    // Generate workflow
    generate_workflow(&name, &description, &templates[selection], &selected_integrations)?;
    
    Ok(())
}
```

### Auto-completion Scripts

Generate shell auto-completion:

```bash
# Generate bash completion
qapms completions bash > /etc/bash_completion.d/qapms

# Generate zsh completion
qapms completions zsh > ~/.zfunc/_qapms

# Generate fish completion
qapms completions fish > ~/.config/fish/completions/qapms.fish

# PowerShell
qapms completions powershell | Out-File -Encoding UTF8 qapms.ps1
```

### Implementation Notes

- Use `clap` crate for command-line parsing
- Use `dialoguer` for interactive prompts
- Use `indicatif` for progress bars
- Use `serde` and `serde_yaml` for config serialization
- Implement config file validation
- Add color-coded output for better UX
- Support dry-run mode for all commands
- Implement undo capability where possible
- Add comprehensive help text for all commands
- Support configuration profiles (dev, staging, prod)

---

## Story 21.2: Code Generation Templates for Workflows

As a **QA Engineer**,
I want to **generate workflow templates based on ticket type or testing scenario**,
So that **I don't have to manually configure every workflow and can follow best practices**.

### Acceptance Criteria

**Given** I select a ticket type (bug, feature, etc.)  
**When** I generate a workflow template  
**Then** it includes appropriate steps for that ticket type  
**And** pre-configures integration points  
**And** adds inline documentation  
**And** includes validation rules

**Given** I want to create a custom template  
**When** I save a workflow as template  
**Then** I can name and describe it  
**And** it appears in template list  
**And** I can share it with team  

### Predefined Templates

#### 1. Bug Verification Template

```yaml
name: "Bug Verification"
description: "Standard workflow for verifying bug fixes"
version: "1.0"
tags: [bug, verification, standard]

# Jira integration
jira:
  ticket_required: true
  sync_status: true
  update_on_complete: "Verified"
  
# Workflow steps
steps:
  - id: "setup"
    name: "Environment Setup"
    type: "manual"
    description: "Prepare test environment"
    estimated_time: 15
    checklists:
      - "Deploy latest build"
      - "Clear cache"
      - "Login to system"
    
  - id: "reproduce"
    name: "Reproduce Bug"
    type: "manual"
    description: "Reproduce the original bug"
    estimated_time: 30
    depends_on: ["setup"]
    checklists:
      - "Follow reproduction steps"
      - "Document actual behavior"
      - "Compare with expected behavior"
    validation:
      - "Bug must be reproducible"
    
  - id: "verify_fix"
    name: "Verify Fix"
    type: "manual"
    description: "Verify the bug is fixed"
    estimated_time: 45
    depends_on: ["reproduce"]
    checklists:
      - "Test the fix"
      - "Verify no regression"
      - "Test edge cases"
    validation:
      - "Bug must NOT be reproducible"
      - "Related features must work"
    
  - id: "automated_tests"
    name: "Run Automated Tests"
    type: "integration"
    integration: "postman"
    description: "Run automated test collection"
    estimated_time: 20
    depends_on: ["verify_fix"]
    config:
      collection: "bug-regression-tests"
      environment: "staging"
    
  - id: "documentation"
    name: "Documentation"
    type: "manual"
    description: "Document verification results"
    estimated_time: 15
    depends_on: ["automated_tests"]
    checklists:
      - "Update test cases"
      - "Document findings"
      - "Add notes to ticket"
```

#### 2. Regression Testing Template

```yaml
name: "Regression Testing"
description: "Comprehensive regression testing after changes"
version: "1.0"
tags: [regression, comprehensive]

steps:
  - id: "smoke_test"
    name: "Smoke Tests"
    type: "integration"
    integration: "postman"
    description: "Quick smoke tests"
    estimated_time: 30
    config:
      collection: "smoke-tests"
      environment: "staging"
    
  - id: "core_features"
    name: "Core Features"
    type: "manual"
    description: "Test core application features"
    estimated_time: 120
    depends_on: ["smoke_test"]
    checklists:
      - "User authentication"
      - "Data CRUD operations"
      - "Search functionality"
      - "Notification system"
    
  - id: "automated_suite"
    name: "Automated Test Suite"
    type: "integration"
    integration: "testmo"
    description: "Run full regression suite"
    estimated_time: 180
    depends_on: ["core_features"]
    config:
      test_run_name: "Regression - {{date}}"
      suite_id: "regression-suite"
    
  - id: "log_analysis"
    name: "Log Analysis"
    type: "integration"
    integration: "splunk"
    description: "Check for errors in logs"
    estimated_time: 30
    depends_on: ["automated_suite"]
    config:
      query: "error OR exception | stats count by service"
      time_range: "1h"
```

#### 3. Feature Acceptance Template

```yaml
name: "Feature Acceptance"
description: "Acceptance testing for new features"
version: "1.0"
tags: [feature, acceptance, uat]

steps:
  - id: "requirement_review"
    name: "Review Requirements"
    type: "manual"
    description: "Review feature requirements"
    estimated_time: 30
    checklists:
      - "Read user stories"
      - "Understand acceptance criteria"
      - "Identify test scenarios"
    
  - id: "positive_tests"
    name: "Positive Test Cases"
    type: "manual"
    description: "Test happy path scenarios"
    estimated_time: 90
    depends_on: ["requirement_review"]
    checklists:
      - "Test all acceptance criteria"
      - "Verify success messages"
      - "Check data persistence"
    
  - id: "negative_tests"
    name: "Negative Test Cases"
    type: "manual"
    description: "Test error handling"
    estimated_time: 60
    depends_on: ["positive_tests"]
    checklists:
      - "Test invalid inputs"
      - "Test boundary conditions"
      - "Test error messages"
    
  - id: "edge_cases"
    name: "Edge Cases"
    type: "manual"
    description: "Test edge cases and corner scenarios"
    estimated_time: 45
    depends_on: ["negative_tests"]
    checklists:
      - "Test empty values"
      - "Test large datasets"
      - "Test special characters"
    
  - id: "performance"
    name: "Performance Testing"
    type: "manual"
    description: "Basic performance checks"
    estimated_time: 30
    depends_on: ["edge_cases"]
    checklists:
      - "Page load time"
      - "API response time"
      - "Database query performance"
```

### Template Generator API

```rust
// Template generator
pub struct TemplateGenerator {
    templates: HashMap<String, Template>,
}

impl TemplateGenerator {
    pub fn new() -> Self {
        let mut templates = HashMap::new();
        templates.insert("bug".to_string(), Template::bug_verification());
        templates.insert("regression".to_string(), Template::regression_testing());
        templates.insert("feature".to_string(), Template::feature_acceptance());
        templates.insert("smoke".to_string(), Template::smoke_testing());
        templates.insert("integration".to_string(), Template::integration_testing());
        
        Self { templates }
    }
    
    pub fn generate(&self, template_type: &str, config: &TemplateConfig) -> Result<Workflow> {
        let template = self.templates.get(template_type)
            .ok_or_else(|| anyhow!("Template not found: {}", template_type))?;
        
        let mut workflow = template.clone().into_workflow();
        
        // Apply customizations
        if let Some(name) = &config.name {
            workflow.name = name.clone();
        }
        
        if let Some(ticket) = &config.ticket {
            workflow.add_metadata("ticket", ticket);
            workflow.update_jira_ticket(Some(ticket.clone()));
        }
        
        if let Some(integrations) = &config.integrations {
            workflow.add_integrations(integrations.clone());
        }
        
        Ok(workflow)
    }
    
    pub fn list(&self) -> Vec<&str> {
        self.templates.keys().map(|k| k.as_str()).collect()
    }
}

#[derive(Debug, Clone)]
pub struct TemplateConfig {
    pub name: Option<String>,
    pub ticket: Option<String>,
    pub integrations: Option<Vec<String>>,
    pub estimated_time_multiplier: Option<f64>,
}

impl Template {
    pub fn bug_verification() -> Self {
        Template {
            name: "Bug Verification".to_string(),
            description: "Standard workflow for verifying bug fixes".to_string(),
            version: "1.0".to_string(),
            steps: vec![
                Step {
                    id: "setup".to_string(),
                    name: "Environment Setup".to_string(),
                    r#type: StepType::Manual,
                    description: "Prepare test environment".to_string(),
                    estimated_time: 15,
                    checklists: vec![
                        "Deploy latest build".to_string(),
                        "Clear cache".to_string(),
                        "Login to system".to_string(),
                    ],
                    ..Default::default()
                },
                // ... more steps
            ],
            ..Default::default()
        }
    }
}
```

### Interactive Template Selection

```rust
fn select_template_interactive() -> Result<Workflow> {
    let templates = vec![
        ("bug", "Bug Verification - Verify bug fixes"),
        ("regression", "Regression Testing - Comprehensive testing"),
        ("feature", "Feature Acceptance - UAT for new features"),
        ("smoke", "Smoke Testing - Quick sanity checks"),
        ("integration", "Integration Testing - Test integrations"),
    ];
    
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select workflow template")
        .items(&templates.iter().map(|(_, desc)| *desc).collect::<Vec<_>>())
        .interact()?;
    
    let (template_type, _) = templates[selection];
    
    let config = TemplateConfig {
        name: None,
        ticket: None,
        integrations: None,
        estimated_time_multiplier: None,
    };
    
    let generator = TemplateGenerator::new();
    let workflow = generator.generate(template_type, &config)?;
    
    Ok(workflow)
}
```

### Implementation Notes

- Store templates as YAML files in `templates/` directory
- Support custom templates from user project
- Implement template validation
- Add template versioning
- Support template inheritance (extend existing templates)
- Generate inline documentation in templates
- Add template marketplace (community contributions)
- Implement template testing framework

---

## Story 21.3: Mock Data Generators for Testing

As a **QA Engineer**,
I want to **generate realistic mock data for testing**,
So that **I can test with comprehensive datasets without manually creating test data**.

### Acceptance Criteria

**Given** I need test data for tickets  
**When** I run `qapms data mock --type tickets --count 10`  
**Then** it generates 10 realistic Jira tickets  
**And** includes varied statuses, priorities, and types  
**And** saves to database or exports to file

**Given** I need test users  
**When** I generate mock users  
**Then** it creates users with realistic names and emails  
**And** assigns appropriate roles  
**And** includes user metadata

### Mock Data Types

#### 1. Jira Tickets

```rust
use fake::{Fake, Faker};
use fake::locales::EN;
use rand::Rng;

#[derive(Debug, Clone)]
pub struct MockTicket {
    pub key: String,
    pub summary: String,
    pub description: String,
    pub status: String,
    pub priority: String,
    pub issue_type: String,
    pub assignee: Option<String>,
    pub reporter: String,
    pub created: DateTime<Utc>,
    pub updated: DateTime<Utc>,
    pub components: Vec<String>,
    pub labels: Vec<String>,
    pub story_points: Option<i32>,
}

impl MockTicket {
    pub fn generate<R: Rng>(rng: &mut R) -> Self {
        let statuses = vec![
            "Open", "In Progress", "In Review", "Testing", "Done", "Closed"
        ];
        let priorities = vec![
            "Highest", "High", "Medium", "Low", "Lowest"
        ];
        let issue_types = vec![
            "Bug", "Story", "Task", "Epic", "Improvement"
        ];
        let components = vec![
            "Frontend", "Backend", "API", "Database", "Authentication", 
            "Reporting", "Integration"
        ];
        let labels = vec![
            "urgent", "regression", "performance", "security", "ui", "api"
        ];
        
        let status = statuses[rng.gen_range(0..statuses.len())].to_string();
        let issue_type = issue_types[rng.gen_range(0..issue_types.len())].to_string();
        
        let summary = match issue_type.as_str() {
            "Bug" => Self::generate_bug_summary(rng),
            "Story" => Self::generate_feature_summary(rng),
            "Task" => Self::generate_task_summary(rng),
            _ => Self::generate_generic_summary(rng),
        };
        
        let story_points = if issue_type == "Story" || issue_type == "Epic" {
            Some(rng.gen_range(1..14))
        } else {
            None
        };
        
        Self {
            key: format!("PROJ-{}", rng.gen_range(1000..9999)),
            summary,
            description: Self::generate_description(rng),
            status,
            priority: priorities[rng.gen_range(0..priorities.len())].to_string(),
            issue_type,
            assignee: if rng.gen_bool(0.8) {
                Some(format!("user{}", rng.gen_range(1..100)))
            } else {
                None
            },
            reporter: format!("user{}", rng.gen_range(1..100)),
            created: Utc::now() - chrono::Duration::days(rng.gen_range(1..90)),
            updated: Utc::now() - chrono::Duration::days(rng.gen_range(0..30)),
            components: (0..rng.gen_range(1..3))
                .map(|_| components[rng.gen_range(0..components.len())].to_string())
                .collect(),
            labels: (0..rng.gen_range(0..4))
                .map(|_| labels[rng.gen_range(0..labels.len())].to_string())
                .collect(),
            story_points,
        }
    }
    
    fn generate_bug_summary<R: Rng>(rng: &mut R) -> String {
        let areas = vec![
            "Login", "API", "UI", "Dashboard", "Reports", "Integration",
            "Authentication", "Database", "Search", "Notifications"
        ];
        let issues = vec![
            "crashes when", "fails to load", "displays incorrect", "slow response",
            "doesn't save", "shows error", "not working", "unexpected behavior"
        ];
        
        let area = areas[rng.gen_range(0..areas.len())];
        let issue = issues[rng.gen_range(0..issues.len())];
        
        format!("{} {}", area, issue)
    }
    
    fn generate_feature_summary<R: Rng>(rng: &mut R) -> String {
        let actions = vec![
            "Add", "Implement", "Create", "Build", "Develop", "Enhance"
        ];
        let features = vec![
            "user profile page", "export functionality", "search filters",
            "dashboard widgets", "integration with Jira", "API endpoints",
            "notification system", "report generation"
        ];
        
        let action = actions[rng.gen_range(0..actions.len())];
        let feature = features[rng.gen_range(0..features.len())];
        
        format!("{} {}", action, feature)
    }
}
```

#### 2. Mock Users

```rust
#[derive(Debug, Clone)]
pub struct MockUser {
    pub id: String,
    pub email: String,
    pub name: String,
    pub role: String,
    pub department: String,
    pub created_at: DateTime<Utc>,
    pub last_login: DateTime<Utc>,
    pub active: bool,
}

impl MockUser {
    pub fn generate<R: Rng>(rng: &mut R) -> Self {
        let first_names = vec![
            "John", "Jane", "Mike", "Sarah", "David", "Emily", 
            "Chris", "Lisa", "Ryan", "Jessica", "Alex", "Megan"
        ];
        let last_names = vec![
            "Smith", "Johnson", "Williams", "Brown", "Jones", "Garcia",
            "Miller", "Davis", "Rodriguez", "Martinez", "Wilson", "Taylor"
        ];
        let roles = vec![
            "admin", "qa_lead", "qa_engineer", "pm_po", "viewer"
        ];
        let departments = vec![
            "QA", "Engineering", "Product", "Operations", "Support"
        ];
        
        let first_name = first_names[rng.gen_range(0..first_names.len())];
        let last_name = last_names[rng.gen_range(0..last_names.len())];
        let email = format!("{}.{}@company.com", 
            first_name.to_lowercase(), 
            last_name.to_lowercase()
        );
        
        Self {
            id: format!("user-{}", uuid::Uuid::new_v4()),
            email,
            name: format!("{} {}", first_name, last_name),
            role: roles[rng.gen_range(0..roles.len())].to_string(),
            department: departments[rng.gen_range(0..departments.len())].to_string(),
            created_at: Utc::now() - chrono::Duration::days(rng.gen_range(30..365)),
            last_login: Utc::now() - chrono::Duration::hours(rng.gen_range(1..72)),
            active: rng.gen_bool(0.9),
        }
    }
}
```

#### 3. Mock Workflows

```rust
#[derive(Debug, Clone)]
pub struct MockWorkflow {
    pub id: String,
    pub name: String,
    pub description: String,
    pub status: String,
    pub ticket_key: Option<String>,
    pub created_at: DateTime<Utc>,
    pub steps: Vec<MockWorkflowStep>,
}

#[derive(Debug, Clone)]
pub struct MockWorkflowStep {
    pub id: String,
    pub name: String,
    pub status: String,
    pub time_spent: u32,
    pub notes: Option<String>,
}

impl MockWorkflow {
    pub fn generate<R: Rng>(rng: &mut R, ticket: Option<&MockTicket>) -> Self {
        let statuses = vec![
            "not_started", "in_progress", "paused", "completed", "cancelled"
        ];
        let status = statuses[rng.gen_range(0..statuses.len())].to_string();
        
        let step_statuses = vec![
            "not_started", "in_progress", "completed", "skipped"
        ];
        
        let num_steps = rng.gen_range(3..8);
        let steps: Vec<MockWorkflowStep> = (0..num_steps)
            .map(|i| MockWorkflowStep {
                id: format!("step-{}", i),
                name: format!("Step {}", i + 1),
                status: step_statuses[rng.gen_range(0..step_statuses.len())].to_string(),
                time_spent: rng.gen_range(5..60),
                notes: if rng.gen_bool(0.3) {
                    Some("Notes from testing".to_string())
                } else {
                    None
                },
            })
            .collect();
        
        Self {
            id: format!("workflow-{}", uuid::Uuid::new_v4()),
            name: ticket.map(|t| format!("Workflow for {}", t.key))
                .unwrap_or_else(|| format!("Test Workflow {}", rng.gen_range(1..100))),
            description: "Generated mock workflow for testing".to_string(),
            status,
            ticket_key: ticket.map(|t| t.key.clone()),
            created_at: Utc::now() - chrono::Duration::days(rng.gen_range(1..30)),
            steps,
        }
    }
}
```

### CLI Command for Mock Data

```rust
// CLI command
pub fn mock_data(
    data_type: String,
    count: usize,
    output: Option<String>,
    format: Option<String>,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut rng = rand::thread_rng();
    
    let data = match data_type.as_str() {
        "tickets" => {
            let tickets: Vec<MockTicket> = (0..count)
                .map(|_| MockTicket::generate(&mut rng))
                .collect();
            serde_json::to_string_pretty(&tickets)?
        }
        "users" => {
            let users: Vec<MockUser> = (0..count)
                .map(|_| MockUser::generate(&mut rng))
                .collect();
            serde_json::to_string_pretty(&users)?
        }
        "workflows" => {
            let workflows: Vec<MockWorkflow> = (0..count)
                .map(|_| MockWorkflow::generate(&mut rng, None))
                .collect();
            serde_json::to_string_pretty(&workflows)?
        }
        _ => return Err(anyhow!("Unknown data type: {}", data_type)),
    };
    
    match output {
        Some(path) => {
            std::fs::write(&path, data)?;
            println!("{} Generated {} {} items and saved to {}", 
                "[SUCCESS]".green(), count, data_type, path);
        }
        None => {
            println!("{}", data);
        }
    }
    
    Ok(())
}
```

### Mock Data with Relationships

```rust
pub fn generate_complete_mock_dataset<R: Rng>(
    rng: &mut R,
    config: &MockConfig,
) -> MockDataset {
    // Generate users
    let users: Vec<MockUser> = (0..config.num_users)
        .map(|_| MockUser::generate(rng))
        .collect();
    
    // Generate tickets
    let tickets: Vec<MockTicket> = (0..config.num_tickets)
        .map(|_| MockTicket::generate(rng))
        .collect();
    
    // Generate workflows linked to tickets
    let workflows: Vec<MockWorkflow> = (0..config.num_workflows)
        .map(|i| {
            let ticket = if i < tickets.len() {
                Some(&tickets[i])
            } else {
                None
            };
            MockWorkflow::generate(rng, ticket)
        })
        .collect();
    
    // Generate time entries
    let time_entries: Vec<MockTimeEntry> = workflows.iter()
        .flat_map(|workflow| {
            workflow.steps.iter()
                .filter(|step| step.status == "completed")
                .map(|step| MockTimeEntry {
                    id: format!("time-{}", uuid::Uuid::new_v4()),
                    workflow_id: workflow.id.clone(),
                    step_id: step.id.clone(),
                    user_id: users[rng.gen_range(0..users.len())].id.clone(),
                    time_spent: step.time_spent,
                    created_at: workflow.created_at,
                })
        })
        .collect();
    
    MockDataset {
        users,
        tickets,
        workflows,
        time_entries,
    }
}

pub struct MockConfig {
    pub num_users: usize,
    pub num_tickets: usize,
    pub num_workflows: usize,
}

pub struct MockDataset {
    pub users: Vec<MockUser>,
    pub tickets: Vec<MockTicket>,
    pub workflows: Vec<MockWorkflow>,
    pub time_entries: Vec<MockTimeEntry>,
}
```

### Implementation Notes

- Use `fake` crate for realistic data generation
- Use `rand` for randomization
- Support multiple output formats (JSON, CSV, YAML)
- Generate realistic relationships between entities
- Support custom mock data profiles
- Implement data validation for generated data
- Add database seeding capability
- Support deterministic generation (seeded RNG)

---

## Story 21.4: IDE Plugins (VS Code Extension)

As a **QA Engineer**,
I want to **have a VS Code extension for workflow editing and testing**,
So that **I can work within my familiar IDE instead of switching between tools**.

### Acceptance Criteria

**Given** I have the VS Code extension installed  
**When** I open a workflow YAML file  
**Then** I get syntax highlighting  
**And** auto-completion for workflow properties  
**And** inline error checking  
**And** validation of workflow structure

**Given** I want to execute a workflow  
**When** I right-click on workflow file  
**Then** I see "Run Workflow" option  
**And** workflow executes in integrated terminal  
**And** results shown in sidebar

### VS Code Extension Features

#### 1. Language Support for Workflow YAML

```json
// package.json (VS Code extension)
{
  "contributes": {
    "languages": [{
      "id": "qapms-workflow",
      "aliases": ["QA PMS Workflow", "workflow"],
      "extensions": [".yaml", ".yml"],
      "configuration": "./language-configuration.json"
    }],
    "grammars": [{
      "language": "qapms-workflow",
      "scopeName": "source.qapms-workflow",
      "path": "./syntaxes/qapms-workflow.tmLanguage.json"
    }],
    "snippets": [{
      "language": "qapms-workflow",
      "path": "./snippets/snippets.json"
    }]
  }
}
```

#### 2. Syntax Highlighting

```json
// syntaxes/qapms-workflow.tmLanguage.json
{
  "scopeName": "source.qapms-workflow",
  "patterns": [
    {
      "match": "\\b(name|description|version|status|type|estimated_time)\\b:",
      "name": "keyword.other.key.qapms-workflow"
    },
    {
      "match": "\\b(manual|integration|automated)\\b",
      "name": "constant.language.qapms-workflow"
    },
    {
      "match": "\\bjira:|postman:|testmo:|splunk:\\b",
      "name": "support.function.integration.qapms-workflow"
    },
    {
      "match": "#.*$",
      "name": "comment.line.number-sign.qapms-workflow"
    }
  ]
}
```

#### 3. Auto-Completion

```typescript
// src/providers/completionProvider.ts
import * as vscode from 'vscode';

export class WorkflowCompletionProvider implements vscode.CompletionItemProvider {
    provideCompletionItems(
        document: vscode.TextDocument,
        position: vscode.Position
    ): vscode.CompletionItem[] {
        const items: vscode.CompletionItem[] = [];
        
        // Root level properties
        items.push(
            new vscode.CompletionItem('name', vscode.CompletionItemKind.Field),
            new vscode.CompletionItem('description', vscode.CompletionItemKind.Field),
            new vscode.CompletionItem('version', vscode.CompletionItemKind.Field),
            new vscode.CompletionItem('status', vscode.CompletionItemKind.Field)
        );
        
        // Step types
        items.push(
            new vscode.CompletionItem('type: manual', vscode.CompletionItemKind.Value),
            new vscode.CompletionItem('type: integration', vscode.CompletionItemKind.Value),
            new vscode.CompletionItem('type: automated', vscode.CompletionItemKind.Value)
        );
        
        // Integrations
        items.push(
            new vscode.CompletionItem('jira:', vscode.CompletionItemKind.Module),
            new vscode.CompletionItem('postman:', vscode.CompletionItemKind.Module),
            new vscode.CompletionItem('testmo:', vscode.CompletionItemKind.Module),
            new vscode.CompletionItem('splunk:', vscode.CompletionItemKind.Module)
        );
        
        return items;
    }
}
```

#### 4. Validation & Diagnostics

```typescript
// src/providers/diagnosticProvider.ts
import * as vscode from 'vscode';
import * as yaml from 'js-yaml';

export class WorkflowDiagnosticProvider {
    private diagnostics: vscode.DiagnosticCollection;
    
    constructor() {
        this.diagnostics = vscode.languages.createDiagnosticCollection('qapms-workflow');
    }
    
    validate(document: vscode.TextDocument): void {
        const diagnostics: vscode.Diagnostic[] = [];
        
        try {
            const workflow = yaml.load(document.getText()) as any;
            
            // Validate required fields
            if (!workflow.name) {
                const range = this.findLineForKey(document, 'name');
                diagnostics.push(new vscode.Diagnostic(
                    range,
                    'Missing required field: name',
                    vscode.DiagnosticSeverity.Error
                ));
            }
            
            // Validate steps
            if (workflow.steps && Array.isArray(workflow.steps)) {
                workflow.steps.forEach((step: any, index: number) => {
                    if (!step.id) {
                        const range = this.findStepLine(document, index, 'id');
                        diagnostics.push(new vscode.Diagnostic(
                            range,
                            `Step ${index}: Missing required field: id`,
                            vscode.DiagnosticSeverity.Error
                        ));
                    }
                    
                    if (!step.type || !['manual', 'integration', 'automated'].includes(step.type)) {
                        const range = this.findStepLine(document, index, 'type');
                        diagnostics.push(new vscode.Diagnostic(
                            range,
                            `Step ${index}: Invalid type. Must be: manual, integration, or automated`,
                            vscode.DiagnosticSeverity.Error
                        ));
                    }
                    
                    if (step.type === 'integration' && !step.integration) {
                        const range = this.findStepLine(document, index, 'integration');
                        diagnostics.push(new vscode.Diagnostic(
                            range,
                            `Step ${index}: Integration steps require 'integration' field`,
                            vscode.DiagnosticSeverity.Error
                        ));
                    }
                });
            }
            
            // Validate Jira configuration
            if (workflow.jira && workflow.jira.enabled && !workflow.jira.ticket) {
                const range = this.findLineForKey(document, 'jira.ticket');
                diagnostics.push(new vscode.Diagnostic(
                    range,
                    'Jira integration enabled but no ticket specified',
                    vscode.DiagnosticSeverity.Warning
                ));
            }
            
        } catch (error) {
            // YAML syntax error
            const range = new vscode.Range(
                new vscode.Position(0, 0),
                new vscode.Position(document.lineCount, 0)
            );
            diagnostics.push(new vscode.Diagnostic(
                range,
                `YAML syntax error: ${error}`,
                vscode.DiagnosticSeverity.Error
            ));
        }
        
        this.diagnostics.set(document.uri, diagnostics);
    }
    
    private findLineForKey(document: vscode.TextDocument, key: string): vscode.Range {
        const text = document.getText();
        const match = new RegExp(`^${key}:`, 'm').exec(text);
        if (match) {
            const pos = document.positionAt(match.index);
            return new vscode.Range(pos, new vscode.Position(pos.line, 100));
        }
        return new vscode.Range(0, 0, 0, 0);
    }
    
    private findStepLine(document: vscode.TextDocument, stepIndex: number, key: string): vscode.Range {
        // Find the specific step and key
        // Implementation depends on YAML structure
        return new vscode.Range(0, 0, 0, 0);
    }
}
```

#### 5. Run Workflow Command

```typescript
// src/commands/runWorkflow.ts
import * as vscode from 'vscode';
import * as cp from 'child_process';
import { spawn } from 'child_process';

export function registerRunWorkflowCommand(context: vscode.ExtensionContext): void {
    const command = vscode.commands.registerCommand('qapms.runWorkflow', async () => {
        const editor = vscode.window.activeTextEditor;
        if (!editor) {
            vscode.window.showErrorMessage('No workflow file open');
            return;
        }
        
        const document = editor.document;
        if (!document.fileName.endsWith('.yaml') && !document.fileName.endsWith('.yml')) {
            vscode.window.showErrorMessage('Not a workflow file');
            return;
        }
        
        // Create output channel
        const output = vscode.window.createOutputChannel('QA PMS Workflow');
        output.show();
        
        output.appendLine(`Running workflow: ${document.fileName}`);
        output.appendLine('---');
        
        // Run workflow using CLI
        const cli = spawn('qapms', ['run', document.fileName], {
            cwd: vscode.workspace.rootPath
        });
        
        cli.stdout.on('data', (data) => {
            output.append(data.toString());
        });
        
        cli.stderr.on('data', (data) => {
            output.appendLine(`[ERROR] ${data.toString()}`);
        });
        
        cli.on('close', (code) => {
            output.appendLine(`---`);
            output.appendLine(`Workflow completed with code: ${code}`);
            if (code === 0) {
                vscode.window.showInformationMessage('Workflow completed successfully');
            } else {
                vscode.window.showErrorMessage(`Workflow failed with code ${code}`);
            }
        });
    });
    
    context.subscriptions.push(command);
}
```

#### 6. Workflow Explorer View

```typescript
// src/views/workflowExplorer.ts
import * as vscode from 'vscode';
import * as path from 'path';

export class WorkflowExplorer {
    private treeDataProvider: WorkflowTreeDataProvider;
    
    constructor(context: vscode.ExtensionContext) {
        this.treeDataProvider = new WorkflowTreeDataProvider(context);
        
        vscode.window.registerTreeDataProvider('qapmsWorkflows', this.treeDataProvider);
        
        // Refresh command
        vscode.commands.registerCommand('qapms.refreshWorkflows', () => {
            this.treeDataProvider.refresh();
        });
    }
}

class WorkflowTreeDataProvider implements vscode.TreeDataProvider<WorkflowItem> {
    private _onDidChangeTreeData = new vscode.EventEmitter<WorkflowItem | undefined | void>();
    readonly onDidChangeTreeData = this._onDidChangeTreeData.event;
    
    constructor(private context: vscode.ExtensionContext) {}
    
    refresh(): void {
        this._onDidChangeTreeData.fire();
    }
    
    getTreeItem(element: WorkflowItem): vscode.TreeItem {
        return element;
    }
    
    async getChildren(element?: WorkflowItem): Promise<WorkflowItem[]> {
        if (!element) {
            // Root level - show workflow directories
            const workflowsPath = path.join(vscode.workspace.rootPath || '', 'workflows');
            const items = await vscode.workspace.findFiles(
                path.join('workflows', '**', '*.yaml'),
                '**/node_modules/**'
            );
            
            return items.map(uri => new WorkflowItem(
                path.basename(uri.fsPath),
                vscode.TreeItemCollapsibleState.None,
                uri
            ));
        }
        return [];
    }
}

class WorkflowItem extends vscode.TreeItem {
    constructor(
        public readonly label: string,
        public readonly collapsibleState: vscode.TreeItemCollapsibleState,
        public readonly uri: vscode.Uri
    ) {
        super(label, collapsibleState);
        this.tooltip = label;
        this.contextValue = 'workflow';
        this.command = {
            command: 'vscode.open',
            title: 'Open Workflow',
            arguments: [uri]
        };
    }
}
```

#### 7. Snippets

```json
// snippets/snippets.json
{
  "New Workflow": {
    "prefix": "workflow",
    "body": [
      "name: \"${1:Workflow Name}\"",
      "description: \"${2:Workflow description}\"",
      "version: \"1.0\"",
      "tags: [${3:tag1}, ${4:tag2}]",
      "",
      "jira:",
      "  ticket_required: ${5:true}",
      "",
      "steps:",
      "  - id: \"${6:step-1}\"",
      "    name: \"${7:Step Name}\"",
      "    type: manual",
      "    description: \"${8:Step description}\"",
      "    estimated_time: ${9:30}",
      "    checklists:",
      "      - \"${10:Checklist item}\"",
      "$0"
    ],
    "description": "Create a new workflow template"
  },
  
  "Integration Step": {
    "prefix": "integration-step",
    "body": [
      "  - id: \"${1:integration-step}\"",
      "    name: \"${2:Integration Step}\"",
      "    type: integration",
      "    integration: \"${3:postman}\"",
      "    description: \"${4:Description}\"",
      "    estimated_time: ${5:20}",
      "    config:",
      "      ${6:key}: \"${7:value}\"",
      "$0"
    ],
    "description": "Create an integration step"
  },
  
  "Jira Integration": {
    "prefix": "jira",
    "body": [
      "jira:",
      "  ticket_required: true",
      "  ticket: \"${1:PROJ-123}\"",
      "  sync_status: true",
      "  update_on_complete: \"${2:Verified}\"",
      "$0"
    ],
    "description": "Add Jira integration configuration"
  }
}
```

### Extension Commands

```typescript
// src/extension.ts
export function activate(context: vscode.ExtensionContext) {
    // Register commands
    registerRunWorkflowCommand(context);
    registerValidateWorkflowCommand(context);
    registerCreateWorkflowCommand(context);
    
    // Register providers
    const completionProvider = vscode.languages.registerCompletionItemProvider(
        'qapms-workflow',
        new WorkflowCompletionProvider()
    );
    
    const diagnosticProvider = new WorkflowDiagnosticProvider();
    vscode.workspace.onDidChangeTextDocument(event => {
        diagnosticProvider.validate(event.document);
    });
    
    // Register view
    new WorkflowExplorer(context);
    
    // Status bar
    const statusBarItem = vscode.window.createStatusBarItem(
        vscode.StatusBarAlignment.Right,
        100
    );
    statusBarItem.text = '$(beaker) QA PMS';
    statusBarItem.command = 'qapms.showStatus';
    statusBarItem.show();
}
```

### Implementation Notes

- Use VS Code Extension API
- Implement YAML language support
- Add IntelliSense for workflow schemas
- Create workflow validation
- Add workflow execution in integrated terminal
- Implement workflow explorer sidebar
- Add status bar integration
- Support hot reload of workflows
- Add snippets for common patterns
- Implement workflow templates
- Add workflow preview panel

---

## Story 21.5: Interactive API Playground

As a **QA Engineer**,
I want to **explore and test the API endpoints interactively**,
So that **I can understand how to use the API and test my integrations**.

### Acceptance Criteria

**Given** I access the API Playground  
**When** I select an endpoint  
**Then** I see the endpoint documentation  
**And** I can fill in parameters  
**And** I can execute the request  
**And** I see the response

**Given** I want to save a request  
**When** I click "Save"  
**Then** it's saved to my collection  
**And** I can load it later  
**And** I can share it with team

### API Playground Features

#### 1. Endpoint Explorer

```typescript
// frontend/components/api-playground/EndpointExplorer.tsx
interface Endpoint {
  method: string;
  path: string;
  description: string;
  parameters: Parameter[];
  requestBody?: RequestBody;
  responses: Response[];
}

interface Parameter {
  name: string;
  type: string;
  required: boolean;
  description: string;
  default?: string;
}

export const EndpointExplorer: React.FC = () => {
  const [endpoints, setEndpoints] = useState<Endpoint[]>([]);
  const [selectedEndpoint, setSelectedEndpoint] = useState<Endpoint | null>(null);
  const [response, setResponse] = useState<any>(null);
  const [loading, setLoading] = useState(false);
  const [params, setParams] = useState<Record<string, string>>({});
  
  useEffect(() => {
    // Fetch OpenAPI spec
    fetch('/api/openapi.json')
      .then(res => res.json())
      .then(spec => {
        const endpoints = parseOpenApiSpec(spec);
        setEndpoints(endpoints);
      });
  }, []);
  
  const executeRequest = async () => {
    if (!selectedEndpoint) return;
    
    setLoading(true);
    try {
      const response = await fetch(`/api${selectedEndpoint.path}`, {
        method: selectedEndpoint.method,
        headers: {
          'Content-Type': 'application/json',
        },
        body: params ? JSON.stringify(params) : undefined,
      });
      
      const data = await response.json();
      setResponse(data);
    } catch (error) {
      setResponse({ error: error.message });
    } finally {
      setLoading(false);
    }
  };
  
  return (
    <div className="api-playground">
      <div className="endpoint-list">
        <h3>Endpoints</h3>
        {endpoints.map(endpoint => (
          <button
            key={endpoint.path}
            className={`endpoint-item ${selectedEndpoint?.path === endpoint.path ? 'active' : ''}`}
            onClick={() => setSelectedEndpoint(endpoint)}
          >
            <span className={`method ${endpoint.method.toLowerCase()}`}>
              {endpoint.method}
            </span>
            <span className="path">{endpoint.path}</span>
          </button>
        ))}
      </div>
      
      {selectedEndpoint && (
        <div className="endpoint-details">
          <h3>
            {selectedEndpoint.method} {selectedEndpoint.path}
          </h3>
          <p>{selectedEndpoint.description}</p>
          
          <div className="parameters">
            <h4>Parameters</h4>
            {selectedEndpoint.parameters.map(param => (
              <div key={param.name} className="parameter">
                <label>{param.name} ({param.type})</label>
                <input
                  type="text"
                  value={params[param.name] || param.default || ''}
                  onChange={(e) => setParams({
                    ...params,
                    [param.name]: e.target.value
                  })}
                  placeholder={param.description}
                />
              </div>
            ))}
          </div>
          
          <button 
            onClick={executeRequest}
            disabled={loading}
          >
            {loading ? 'Executing...' : 'Execute'}
          </button>
          
          {response && (
            <div className="response">
              <h4>Response</h4>
              <pre>{JSON.stringify(response, null, 2)}</pre>
            </div>
          )}
        </div>
      )}
    </div>
  );
};
```

#### 2. Request Builder

```typescript
// frontend/components/api-playground/RequestBuilder.tsx
export const RequestBuilder: React.FC = () => {
  const [method, setMethod] = useState<'GET' | 'POST' | 'PUT' | 'DELETE' | 'PATCH'>('GET');
  const [url, setUrl] = useState('/api/workflows');
  const [headers, setHeaders] = useState<Record<string, string>>({
    'Content-Type': 'application/json',
  });
  const [body, setBody] = useState<string>('');
  const [response, setResponse] = useState<any>(null);
  
  const executeRequest = async () => {
    try {
      const res = await fetch(url, {
        method,
        headers,
        body: ['POST', 'PUT', 'PATCH'].includes(method) ? body : undefined,
      });
      
      const data = await res.json();
      setResponse({
        status: res.status,
        statusText: res.statusText,
        data,
      });
    } catch (error) {
      setResponse({ error: error.message });
    }
  };
  
  return (
    <div className="request-builder">
      <div className="request-line">
        <select 
          value={method} 
          onChange={(e) => setMethod(e.target.value as any)}
        >
          <option value="GET">GET</option>
          <option value="POST">POST</option>
          <option value="PUT">PUT</option>
          <option value="DELETE">DELETE</option>
          <option value="PATCH">PATCH</option>
        </select>
        
        <input 
          type="text" 
          value={url} 
          onChange={(e) => setUrl(e.target.value)}
          placeholder="/api/endpoint"
        />
        
        <button onClick={executeRequest}>Send</button>
      </div>
      
      <div className="headers">
        <h4>Headers</h4>
        {Object.entries(headers).map(([key, value]) => (
          <div key={key} className="header-row">
            <input 
              type="text" 
              value={key} 
              readOnly
            />
            <input 
              type="text" 
              value={value}
              onChange={(e) => setHeaders({
                ...headers,
                [key]: e.target.value
              })}
            />
            <button onClick={() => {
              const newHeaders = { ...headers };
              delete newHeaders[key];
              setHeaders(newHeaders);
            }}>×</button>
          </div>
        ))}
        <button onClick={() => {
          const key = prompt('Header name');
          if (key) {
            setHeaders({ ...headers, [key]: '' });
          }
        }}>+ Add Header</button>
      </div>
      
      {['POST', 'PUT', 'PATCH'].includes(method) && (
        <div className="body">
          <h4>Request Body</h4>
          <textarea
            value={body}
            onChange={(e) => setBody(e.target.value)}
            placeholder="Enter JSON body"
            rows={10}
          />
        </div>
      )}
      
      {response && (
        <div className="response">
          <div className="response-status">
            Status: {response.status} {response.statusText}
          </div>
          <pre>{JSON.stringify(response.data, null, 2)}</pre>
        </div>
      )}
    </div>
  );
};
```

#### 3. Saved Requests

```typescript
// frontend/components/api-playground/SavedRequests.tsx
interface SavedRequest {
  id: string;
  name: string;
  method: string;
  url: string;
  headers: Record<string, string>;
  body?: string;
  createdAt: Date;
}

export const SavedRequests: React.FC = () => {
  const [savedRequests, setSavedRequests] = useState<SavedRequest[]>([]);
  const [selectedRequest, setSelectedRequest] = useState<SavedRequest | null>(null);
  
  const saveRequest = (request: Omit<SavedRequest, 'id' | 'createdAt'>) => {
    const newRequest: SavedRequest = {
      ...request,
      id: Date.now().toString(),
      createdAt: new Date(),
    };
    
    setSavedRequests([...savedRequests, newRequest]);
    localStorage.setItem('api-playground-requests', JSON.stringify([...savedRequests, newRequest]));
  };
  
  const loadRequest = (request: SavedRequest) => {
    setSelectedRequest(request);
  };
  
  const deleteRequest = (id: string) => {
    setSavedRequests(savedRequests.filter(r => r.id !== id));
    localStorage.setItem('api-playground-requests', 
      JSON.stringify(savedRequests.filter(r => r.id !== id))
    );
  };
  
  const exportRequests = () => {
    const data = JSON.stringify(savedRequests, null, 2);
    const blob = new Blob([data], { type: 'application/json' });
    const url = URL.createObjectURL(blob);
    const a = document.createElement('a');
    a.href = url;
    a.download = 'api-playground-requests.json';
    a.click();
  };
  
  useEffect(() => {
    const saved = localStorage.getItem('api-playground-requests');
    if (saved) {
      setSavedRequests(JSON.parse(saved));
    }
  }, []);
  
  return (
    <div className="saved-requests">
      <div className="header">
        <h3>Saved Requests</h3>
        <button onClick={exportRequests}>Export</button>
      </div>
      
      <div className="request-list">
        {savedRequests.map(request => (
          <div key={request.id} className="request-item">
            <div className="request-info">
              <span className={`method ${request.method.toLowerCase()}`}>
                {request.method}
              </span>
              <span className="name">{request.name}</span>
              <span className="url">{request.url}</span>
            </div>
            
            <div className="actions">
              <button onClick={() => loadRequest(request)}>Load</button>
              <button onClick={() => deleteRequest(request.id)}>Delete</button>
            </div>
          </div>
        ))}
      </div>
      
      {selectedRequest && (
        <div className="request-details">
          <RequestBuilder 
            initialMethod={selectedRequest.method as any}
            initialUrl={selectedRequest.url}
            initialHeaders={selectedRequest.headers}
            initialBody={selectedRequest.body}
          />
        </div>
      )}
    </div>
  );
};
```

### API Playground Route

```typescript
// frontend/pages/ApiPlayground.tsx
export const ApiPlayground: React.FC = () => {
  const [activeTab, setActiveTab] = useState<'explorer' | 'builder' | 'saved'>('explorer');
  
  return (
    <div className="api-playground-page">
      <div className="tabs">
        <button 
          className={activeTab === 'explorer' ? 'active' : ''}
          onClick={() => setActiveTab('explorer')}
        >
          Endpoint Explorer
        </button>
        <button 
          className={activeTab === 'builder' ? 'active' : ''}
          onClick={() => setActiveTab('builder')}
        >
          Request Builder
        </button>
        <button 
          className={activeTab === 'saved' ? 'active' : ''}
          onClick={() => setActiveTab('saved')}
        >
          Saved Requests
        </button>
      </div>
      
      <div className="tab-content">
        {activeTab === 'explorer' && <EndpointExplorer />}
        {activeTab === 'builder' && <RequestBuilder />}
        {activeTab === 'saved' && <SavedRequests />}
      </div>
    </div>
  );
};
```

### Implementation Notes

- Use OpenAPI/Swagger spec for auto-documentation
- Implement syntax highlighting for JSON
- Add request history
- Support environment variables
- Add request/response transformation
- Implement batch requests
- Support WebSocket connections
- Add request/export functionality
- Implement request validation
- Add response preview with formatting

---

## Story 21.6: Hot Reload for Configuration Changes

As a **QA Engineer**,
I want to **see configuration changes applied immediately without restarting the server**,
So that **I can iterate quickly on workflow definitions and integrations**.

### Acceptance Criteria

**Given** the dev server is running  
**When** I modify a workflow YAML file  
**Then** the server detects the change  
**And** reloads the configuration  
**And** shows a notification  
**And** updates the UI without page refresh

**Given** I modify integration settings  
**When** I save the configuration file  
**Then** the integration is reconfigured  
**And** connection is tested automatically  
**And** any errors are shown

### Hot Reload Implementation

```rust
// File watcher for hot reload
use notify::{Watcher, RecursiveMode, watcher, DebouncedEvent};
use std::sync::mpsc::channel;
use std::time::Duration;

pub struct ConfigReloader {
    config_path: PathBuf,
    config: Arc<RwLock<Config>>,
    tx: tokio::sync::broadcast::Sender<ConfigUpdate>,
}

impl ConfigReloader {
    pub fn new(config_path: PathBuf) -> Result<Self> {
        let (tx, _) = tokio::sync::broadcast::channel(100);
        let config = Arc::new(RwLock::new(Config::load(&config_path)?));
        
        Ok(Self {
            config_path,
            config,
            tx,
        })
    }
    
    pub fn start(&self) -> Result<()> {
        let (watcher_tx, watcher_rx) = channel();
        let mut watcher: RecommendedWatcher = watcher(watcher_tx, Duration::from_millis(200))?;
        
        watcher.watch(&self.config_path, RecursiveMode::NonRecursive)?;
        
        let config = self.config.clone();
        let tx = self.tx.clone();
        
        tokio::spawn(async move {
            loop {
                match watcher_rx.recv() {
                    Ok(DebouncedEvent::Write(_)) |
                    Ok(DebouncedEvent::NoticeRemove(_)) |
                    Ok(DebouncedEvent::NoticeWrite(_)) => {
                        // Wait a bit for file write to complete
                        tokio::time::sleep(Duration::from_millis(100)).await;
                        
                        match Config::load(&config_path) {
                            Ok(new_config) => {
                                info!("Configuration reloaded successfully");
                                
                                // Update shared config
                                {
                                    let mut config_guard = config.write().await;
                                    *config_guard = new_config;
                                }
                                
                                // Notify listeners
                                let _ = tx.send(ConfigUpdate::Reloaded);
                            }
                            Err(e) => {
                                error!("Failed to reload configuration: {}", e);
                                let _ = tx.send(ConfigUpdate::Error(e.to_string()));
                            }
                        }
                    }
                    Ok(event) => {
                        debug!("Ignored file event: {:?}", event);
                    }
                    Err(e) => {
                        error!("Watch error: {:?}", e);
                    }
                }
            }
        });
        
        Ok(())
    }
    
    pub fn subscribe(&self) -> tokio::sync::broadcast::Receiver<ConfigUpdate> {
        self.tx.subscribe()
    }
}

#[derive(Debug, Clone)]
pub enum ConfigUpdate {
    Reloaded,
    Error(String),
}

// Hot reload for workflows
pub struct WorkflowReloader {
    workflows_dir: PathBuf,
    workflows: Arc<RwLock<HashMap<String, Workflow>>>,
    tx: tokio::sync::broadcast::Sender<WorkflowUpdate>,
}

impl WorkflowReloader {
    pub fn new(workflows_dir: PathBuf) -> Result<Self> {
        let (tx, _) = tokio::sync::broadcast::channel(100);
        let workflows = Arc::new(RwLock::new(Self::load_workflows(&workflows_dir)?));
        
        Ok(Self {
            workflows_dir,
            workflows,
            tx,
        })
    }
    
    fn load_workflows(dir: &PathBuf) -> Result<HashMap<String, Workflow>> {
        let mut workflows = HashMap::new();
        
        for entry in std::fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            
            if path.extension().and_then(|s| s.to_str()) == Some("yaml") || 
               path.extension().and_then(|s| s.to_str()) == Some("yml") {
                let workflow = Workflow::from_file(&path)?;
                workflows.insert(workflow.id.clone(), workflow);
            }
        }
        
        Ok(workflows)
    }
    
    pub fn start(&self) -> Result<()> {
        let (watcher_tx, watcher_rx) = channel();
        let mut watcher: RecommendedWatcher = watcher(watcher_tx, Duration::from_millis(200))?;
        
        watcher.watch(&self.workflows_dir, RecursiveMode::Recursive)?;
        
        let workflows_dir = self.workflows_dir.clone();
        let workflows = self.workflows.clone();
        let tx = self.tx.clone();
        
        tokio::spawn(async move {
            loop {
                match watcher_rx.recv() {
                    Ok(DebouncedEvent::Create(path)) |
                    Ok(DebouncedEvent::Write(path)) |
                    Ok(DebouncedEvent::Remove(path)) => {
                        if path.extension().and_then(|s| s.to_str()) == Some("yaml") ||
                           path.extension().and_then(|s| s.to_str()) == Some("yml") {
                            
                            tokio::time::sleep(Duration::from_millis(100)).await;
                            
                            match Workflow::from_file(&path) {
                                Ok(workflow) => {
                                    info!("Workflow loaded: {}", workflow.name);
                                    
                                    {
                                        let mut workflows_guard = workflows.write().await;
                                        workflows_guard.insert(workflow.id.clone(), workflow);
                                    }
                                    
                                    let _ = tx.send(WorkflowUpdate::Updated {
                                        id: path.file_stem()
                                            .and_then(|s| s.to_str())
                                            .unwrap_or("unknown")
                                            .to_string(),
                                    });
                                }
                                Err(e) => {
                                    error!("Failed to load workflow {}: {}", path.display(), e);
                                    
                                    // Remove workflow if it was deleted
                                    {
                                        let mut workflows_guard = workflows.write().await;
                                        let id = path.file_stem()
                                            .and_then(|s| s.to_str())
                                            .unwrap_or("unknown");
                                        workflows_guard.remove(id);
                                    }
                                    
                                    let _ = tx.send(WorkflowUpdate::Removed {
                                        id: path.file_stem()
                                            .and_then(|s| s.to_str())
                                            .unwrap_or("unknown")
                                            .to_string(),
                                    });
                                }
                            }
                        }
                    }
                    Ok(_) => {}
                    Err(e) => error!("Watch error: {:?}", e),
                }
            }
        });
        
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub enum WorkflowUpdate {
    Updated { id: String },
    Removed { id: String },
}
```

### Frontend Hot Reload with HMR

```typescript
// frontend/vite.config.ts
import { defineConfig } from 'vite';
import react from '@vitejs/plugin-react';

export default defineConfig({
  plugins: [react()],
  server: {
    hmr: {
      protocol: 'ws',
      host: 'localhost',
      port: 24678,
    },
    watch: {
      usePolling: true,
      interval: 100,
    },
  },
});
```

### WebSocket Notification for Config Changes

```rust
// WebSocket endpoint for hot reload notifications
pub async fn ws_config_updates(
    ws: WebSocketUpgrade,
    State(reloader): State<Arc<ConfigReloader>>,
) -> Response {
    ws.on_upgrade(|mut socket| async move {
        let mut rx = reloader.subscribe();
        
        while let Ok(update) = rx.recv().await {
            let message = match update {
                ConfigUpdate::Reloaded => {
                    serde_json::json!({
                        "type": "config_reloaded",
                        "timestamp": chrono::Utc::now().to_rfc3339()
                    })
                }
                ConfigUpdate::Error(msg) => {
                    serde_json::json!({
                        "type": "config_error",
                        "message": msg,
                        "timestamp": chrono::Utc::now().to_rfc3339()
                    })
                }
            };
            
            if socket.send(Message::Text(message.to_string())).await.is_err() {
                break;
            }
        }
    })
}

// Router setup
let app = Router::new()
    .route("/ws/config", get(ws_config_updates))
    .layer(FromExtractorState::<Arc<ConfigReloader>>::new(reloader));
```

### Frontend WebSocket Connection

```typescript
// frontend/hooks/useConfigUpdates.ts
export function useConfigUpdates() {
  const [configStatus, setConfigStatus] = useState<{
    reloaded: boolean;
    error: string | null;
  }>({
    reloaded: false,
    error: null,
  });
  
  useEffect(() => {
    const ws = new WebSocket('ws://localhost:3000/ws/config');
    
    ws.onmessage = (event) => {
      const data = JSON.parse(event.data);
      
      if (data.type === 'config_reloaded') {
        setConfigStatus({
          reloaded: true,
          error: null,
        });
        
        toast.success('Configuration reloaded successfully');
        
        // Re-fetch data from API
        window.location.reload();
      } else if (data.type === 'config_error') {
        setConfigStatus({
          reloaded: false,
          error: data.message,
        });
        
        toast.error(`Configuration error: ${data.message}`);
      }
    };
    
    ws.onerror = () => {
      toast.error('WebSocket connection error');
    };
    
    return () => {
      ws.close();
    };
  }, []);
  
  return configStatus;
}
```

### Implementation Notes

- Use `notify` crate for file watching
- Debounce file events to avoid multiple reloads
- Implement config validation on reload
- Show toast notifications for reload events
- Support hot reload of workflows, config, and templates
- Implement safe reload (rollback on error)
- Add reload history for debugging
- Support partial reloads (only changed workflows)

---

## Story 21.7: Development Mode with Enhanced Logging

As a **QA Engineer**,
I want to **have a development mode with detailed logging**,
So that **I can debug workflows and integrations easily**.

### Acceptance Criteria

**Given** I start the dev server with debug mode  
**When** I execute workflows  
**Then** all API calls are logged  
**And** detailed integration responses are shown  
**And** timing information is displayed  
**And** errors show full stack traces

**Given** I'm troubleshooting an issue  
**When** I need to see what happened  
**Then** I can filter logs by workflow or step  
**And** I can export logs for analysis  
**And** I can replay failed steps

### Enhanced Logging System

```rust
use tracing::{info, warn, error, debug, instrument};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

pub fn setup_logging(debug: bool) {
    let env_filter = if debug {
        "debug,hyper=info,tower_http=info"
    } else {
        "info"
    };
    
    let fmt_layer = tracing_subscriber::fmt::layer()
        .with_target(false)
        .with_ansi(true)
        .with_file(true)
        .with_line_number(true);
    
    let json_layer = tracing_subscriber::fmt::layer()
        .json()
        .with_target(true)
        .with_span_list(true);
    
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(env_filter))
        .with(fmt_layer)
        .with(json_layer)
        .init();
}

// Structured logging for workflows
#[derive(Debug, Clone)]
pub struct WorkflowLogger {
    workflow_id: String,
}

impl WorkflowLogger {
    pub fn new(workflow_id: String) -> Self {
        Self { workflow_id }
    }
    
    #[instrument(skip(self))]
    pub fn log_step_start(&self, step_id: String, step_name: String) {
        info!(
            workflow_id = %self.workflow_id,
            step_id = %step_id,
            step_name = %step_name,
            "Starting workflow step"
        );
    }
    
    #[instrument(skip(self))]
    pub fn log_step_complete(&self, step_id: String, duration_ms: u64) {
        info!(
            workflow_id = %self.workflow_id,
            step_id = %step_id,
            duration_ms = duration_ms,
            "Workflow step completed"
        );
    }
    
    #[instrument(skip(self))]
    pub fn log_step_error(&self, step_id: String, error: &str) {
        error!(
            workflow_id = %self.workflow_id,
            step_id = %step_id,
            error = %error,
            "Workflow step failed"
        );
    }
    
    #[instrument(skip(self))]
    pub fn log_api_call(&self, method: &str, url: &str, status: u16, duration_ms: u64) {
        debug!(
            workflow_id = %self.workflow_id,
            method = %method,
            url = %url,
            status = status,
            duration_ms = duration_ms,
            "API call completed"
        );
    }
    
    #[instrument(skip(self))]
    pub fn log_integration_response(&self, integration: &str, response: &str) {
        trace!(
            workflow_id = %self.workflow_id,
            integration = %integration,
            response = %response,
            "Integration response"
        );
    }
}

// Usage in workflow execution
#[instrument(skip(config, logger))]
pub async fn execute_step(
    step: &WorkflowStep,
    config: &Config,
    logger: &WorkflowLogger,
) -> Result<StepResult> {
    logger.log_step_start(step.id.clone(), step.name.clone());
    
    let start = std::time::Instant::now();
    
    let result = match &step.r#type {
        StepType::Manual => {
            // Manual step
            Ok(StepResult::manual_step())
        }
        StepType::Integration => {
            execute_integration_step(step, config, logger).await
        }
        StepType::Automated => {
            execute_automated_step(step, config, logger).await
        }
    };
    
    let duration = start.elapsed().as_millis() as u64;
    
    match &result {
        Ok(_) => {
            logger.log_step_complete(step.id.clone(), duration);
        }
        Err(e) => {
            logger.log_step_error(step.id.clone(), &e.to_string());
        }
    }
    
    result
}

#[instrument(skip(logger))]
async fn execute_integration_step(
    step: &WorkflowStep,
    config: &Config,
    logger: &WorkflowLogger,
) -> Result<StepResult> {
    let integration = step.integration.as_ref()
        .ok_or_else(|| anyhow!("Integration step missing integration field"))?;
    
    info!(
        integration = %integration,
        step_id = %step.id,
        "Executing integration step"
    );
    
    let start = std::time::Instant::now();
    
    let response = match integration.as_str() {
        "postman" => {
            execute_postman_collection(step, config).await?
        }
        "testmo" => {
            execute_testmo_run(step, config).await?
        }
        "splunk" => {
            execute_splunk_query(step, config).await?
        }
        _ => return Err(anyhow!("Unknown integration: {}", integration)),
    };
    
    let duration = start.elapsed().as_millis() as u64;
    
    logger.log_integration_response(integration, &serde_json::to_string(&response)?);
    
    Ok(StepResult::integration_step(response, duration))
}
```

### Log Viewer Component

```typescript
// frontend/components/development/LogViewer.tsx
interface LogEntry {
  timestamp: string;
  level: 'debug' | 'info' | 'warn' | 'error';
  message: string;
  workflow_id?: string;
  step_id?: string;
  metadata?: Record<string, any>;
}

export const LogViewer: React.FC = () => {
  const [logs, setLogs] = useState<LogEntry[]>([]);
  const [filter, setFilter] = useState<{
    level?: string;
    workflow_id?: string;
    step_id?: string;
    search?: string;
  }>({});
  
  useEffect(() => {
    // Connect to WebSocket for real-time logs
    const ws = new WebSocket('ws://localhost:3000/ws/logs');
    
    ws.onmessage = (event) => {
      const log: LogEntry = JSON.parse(event.data);
      setLogs(prev => [...prev, log]);
    };
    
    return () => ws.close();
  }, []);
  
  const filteredLogs = logs.filter(log => {
    if (filter.level && log.level !== filter.level) return false;
    if (filter.workflow_id && log.workflow_id !== filter.workflow_id) return false;
    if (filter.step_id && log.step_id !== filter.step_id) return false;
    if (filter.search && !log.message.toLowerCase().includes(filter.search.toLowerCase())) {
      return false;
    }
    return true;
  });
  
  const exportLogs = () => {
    const data = JSON.stringify(filteredLogs, null, 2);
    const blob = new Blob([data], { type: 'application/json' });
    const url = URL.createObjectURL(blob);
    const a = document.createElement('a');
    a.href = url;
    a.download = `logs-${Date.now()}.json`;
    a.click();
  };
  
  return (
    <div className="log-viewer">
      <div className="controls">
        <select 
          value={filter.level || ''}
          onChange={(e) => setFilter({ ...filter, level: e.target.value })}
        >
          <option value="">All Levels</option>
          <option value="debug">Debug</option>
          <option value="info">Info</option>
          <option value="warn">Warning</option>
          <option value="error">Error</option>
        </select>
        
        <input 
          type="text"
          placeholder="Filter by workflow ID"
          value={filter.workflow_id || ''}
          onChange={(e) => setFilter({ ...filter, workflow_id: e.target.value })}
        />
        
        <input 
          type="text"
          placeholder="Search logs"
          value={filter.search || ''}
          onChange={(e) => setFilter({ ...filter, search: e.target.value })}
        />
        
        <button onClick={exportLogs}>Export Logs</button>
        <button onClick={() => setLogs([])}>Clear</button>
      </div>
      
      <div className="log-container">
        {filteredLogs.map((log, index) => (
          <div key={index} className={`log-entry ${log.level}`}>
            <span className="timestamp">{log.timestamp}</span>
            <span className={`level ${log.level}`}>{log.level.toUpperCase()}</span>
            <span className="message">{log.message}</span>
            {log.metadata && (
              <details>
                <summary>Metadata</summary>
                <pre>{JSON.stringify(log.metadata, null, 2)}</pre>
              </details>
            )}
          </div>
        ))}
      </div>
    </div>
  );
};
```

### Implementation Notes

- Use `tracing` crate for structured logging
- Implement log levels (debug, info, warn, error)
- Add request/response logging for integrations
- Include timing information for performance analysis
- Support log filtering and search
- Implement log export functionality
- Add WebSocket for real-time log streaming
- Support log aggregation (send to external service)

---

## Story 21.8: Debug Tools and Profiling Helpers

As a **QA Engineer**,
I want to **have debugging tools to understand workflow execution**,
So that **I can identify bottlenecks and fix issues quickly**.

### Acceptance Criteria

**Given** I'm debugging a workflow  
**When** I enable debug mode  
**Then** I see step-by-step execution  
**And** I can pause at any step  
**And** I can inspect variables  
**And** I can replay steps

**Given** I need to analyze performance  
**When** I view profiling data  
**Then** I see time spent per step  
**And** I can identify slow operations  
**And** I can compare with historical data

### Profiling Middleware

```rust
use std::time::Instant;
use axum::{extract::Request, middleware::Next, response::Response};

pub struct ProfilingData {
    pub request_id: String,
    pub path: String,
    pub method: String,
    pub duration_ms: u64,
    pub db_queries: Vec<DbQuery>,
    pub external_calls: Vec<ExternalCall>,
}

#[derive(Debug, Clone)]
pub struct DbQuery {
    pub query: String,
    pub duration_ms: u64,
}

#[derive(Debug, Clone)]
pub struct ExternalCall {
    pub url: String,
    pub method: String,
    pub duration_ms: u64,
    pub status: u16,
}

// Profiling middleware
pub async fn profiling_middleware(
    req: Request,
    next: Next,
) -> Response {
    let start = Instant::now();
    
    // Create profiling data
    let request_id = uuid::Uuid::new_v4().to_string();
    let path = req.uri().path().to_string();
    let method = req.method().to_string();
    
    let profiling_data = Arc::new(RwLock::new(ProfilingData {
        request_id: request_id.clone(),
        path,
        method,
        duration_ms: 0,
        db_queries: Vec::new(),
        external_calls: Vec::new(),
    }));
    
    // Add profiling data to request extensions
    let mut req = req;
    req.extensions_mut().insert(profiling_data.clone());
    
    let response = next.run(req).await;
    
    let duration = start.elapsed().as_millis() as u64;
    
    // Update profiling data
    {
        profiling_data.write().await.duration_ms = duration;
    }
    
    // Log profiling data in debug mode
    debug!("Request profiling: {:?}", profiling_data.read().await);
    
    // Add profiling header
    let mut response = response;
    response.headers_mut().insert(
        "X-Request-ID",
        HeaderValue::from_str(&request_id).unwrap(),
    );
    response.headers_mut().insert(
        "X-Request-Duration",
        HeaderValue::from_str(&duration.to_string()).unwrap(),
    );
    
    response
}

// Database query profiling
pub struct DbProfiler {
    queries: Arc<RwLock<Vec<DbQuery>>>,
}

impl DbProfiler {
    pub fn new() -> Self {
        Self {
            queries: Arc::new(RwLock::new(Vec::new())),
        }
    }
    
    pub async fn record_query(&self, query: &str, duration_ms: u64) {
        self.queries.write().await.push(DbQuery {
            query: query.to_string(),
            duration_ms,
        });
    }
    
    pub async fn get_queries(&self) -> Vec<DbQuery> {
        self.queries.read().await.clone()
    }
}

// External call profiling
pub struct ExternalCallProfiler {
    calls: Arc<RwLock<Vec<ExternalCall>>>,
}

impl ExternalCallProfiler {
    pub fn new() -> Self {
        Self {
            calls: Arc::new(RwLock::new(Vec::new())),
        }
    }
    
    pub async fn record_call(&self, url: &str, method: &str, duration_ms: u64, status: u16) {
        self.calls.write().await.push(ExternalCall {
            url: url.to_string(),
            method: method.to_string(),
            duration_ms,
            status,
        });
    }
    
    pub async fn get_calls(&self) -> Vec<ExternalCall> {
        self.calls.read().await.clone()
    }
}
```

### Debug Mode Execution

```rust
#[derive(Debug, Clone)]
pub enum DebugMode {
    Normal,
    StepByStep {
        current_step: usize,
        paused: bool,
    },
    Breakpoint {
        step_id: String,
    },
}

pub struct DebugWorkflowExecutor {
    debug_mode: Arc<Mutex<DebugMode>>,
    breakpoints: Arc<RwLock<HashSet<String>>>,
}

impl DebugWorkflowExecutor {
    pub fn new() -> Self {
        Self {
            debug_mode: Arc::new(Mutex::new(DebugMode::Normal)),
            breakpoints: Arc::new(RwLock::new(HashSet::new())),
        }
    }
    
    pub fn set_debug_mode(&self, mode: DebugMode) {
        *self.debug_mode.lock().unwrap() = mode;
    }
    
    pub fn add_breakpoint(&self, step_id: String) {
        self.breakpoints.write().unwrap().insert(step_id);
    }
    
    pub fn remove_breakpoint(&self, step_id: &String) {
        self.breakpoints.write().unwrap().remove(step_id);
    }
    
    pub async fn execute_workflow(
        &self,
        workflow: &Workflow,
        config: &Config,
    ) -> Result<WorkflowExecution> {
        let mut execution = WorkflowExecution::new(workflow.id.clone());
        
        for (index, step) in workflow.steps.iter().enumerate() {
            // Check if paused
            loop {
                let mode = self.debug_mode.lock().unwrap().clone();
                match mode {
                    DebugMode::Normal => break,
                    DebugMode::StepByStep { paused: true, .. } => {
                        debug!("Paused at step {}", step.name);
                        tokio::time::sleep(Duration::from_millis(100)).await;
                    }
                    DebugMode::StepByStep { current_step, paused: false } => {
                        if current_step == index {
                            break;
                        } else {
                            debug!("Waiting to reach step {}", current_step);
                            tokio::time::sleep(Duration::from_millis(100)).await;
                        }
                    }
                    DebugMode::Breakpoint { step_id } => {
                        if step.id == step_id {
                            debug!("Breakpoint hit at step {}", step.name);
                            *self.debug_mode.lock().unwrap() = DebugMode::StepByStep {
                                current_step: index,
                                paused: true,
                            };
                            break;
                        } else {
                            break;
                        }
                    }
                }
            }
            
            // Execute step
            let start = Instant::now();
            let result = self.execute_step(step, config).await;
            let duration = start.elapsed().as_millis() as u64;
            
            execution.add_step_result(StepResult {
                step_id: step.id.clone(),
                status: result.is_ok().then_some("completed").unwrap_or("failed"),
                duration_ms: duration,
                output: result.ok(),
                error: result.err().map(|e| e.to_string()),
            });
            
            if result.is_err() {
                break;
            }
        }
        
        Ok(execution)
    }
    
    async fn execute_step(&self, step: &WorkflowStep, config: &Config) -> Result<StepOutput> {
        // Step execution logic...
        Ok(StepOutput::default())
    }
}
```

### Profiling Dashboard Component

```typescript
// frontend/components/development/ProfilingDashboard.tsx
interface ProfilingData {
  request_id: string;
  path: string;
  method: string;
  duration_ms: number;
  db_queries: DbQuery[];
  external_calls: ExternalCall[];
}

export const ProfilingDashboard: React.FC = () => {
  const [profilingData, setProfilingData] = useState<ProfilingData[]>([]);
  const [selectedRequest, setSelectedRequest] = useState<ProfilingData | null>(null);
  
  useEffect(() => {
    // Connect to WebSocket for profiling data
    const ws = new WebSocket('ws://localhost:3000/ws/profiling');
    
    ws.onmessage = (event) => {
      const data: ProfilingData = JSON.parse(event.data);
      setProfilingData(prev => [...prev, data]);
    };
    
    return () => ws.close();
  }, []);
  
  const totalDuration = profilingData.reduce((sum, d) => sum + d.duration_ms, 0);
  const avgDuration = profilingData.length > 0 ? totalDuration / profilingData.length : 0;
  const slowestRequest = profilingData.reduce((max, d) => 
    d.duration_ms > max.duration_ms ? d : max, 
    profilingData[0] || { duration_ms: 0 }
  );
  
  return (
    <div className="profiling-dashboard">
      <div className="summary">
        <div className="card">
          <h3>Total Requests</h3>
          <p>{profilingData.length}</p>
        </div>
        <div className="card">
          <h3>Total Duration</h3>
          <p>{totalDuration}ms</p>
        </div>
        <div className="card">
          <h3>Average Duration</h3>
          <p>{avgDuration.toFixed(2)}ms</p>
        </div>
        <div className="card">
          <h3>Slowest Request</h3>
          <p>{slowestRequest.duration_ms}ms</p>
        </div>
      </div>
      
      <div className="requests-list">
        <h3>Recent Requests</h3>
        <table>
          <thead>
            <tr>
              <th>Path</th>
              <th>Method</th>
              <th>Duration</th>
              <th>DB Queries</th>
              <th>External Calls</th>
            </tr>
          </thead>
          <tbody>
            {profilingData.map((data, index) => (
              <tr 
                key={index}
                className={selectedRequest?.request_id === data.request_id ? 'selected' : ''}
                onClick={() => setSelectedRequest(data)}
              >
                <td>{data.path}</td>
                <td>{data.method}</td>
                <td>{data.duration_ms}ms</td>
                <td>{data.db_queries.length}</td>
                <td>{data.external_calls.length}</td>
              </tr>
            ))}
          </tbody>
        </table>
      </div>
      
      {selectedRequest && (
        <div className="request-details">
          <h3>Request Details</h3>
          
          <div className="section">
            <h4>Database Queries</h4>
            {selectedRequest.db_queries.map((query, index) => (
              <div key={index} className="db-query">
                <code>{query.query}</code>
                <span>{query.duration_ms}ms</span>
              </div>
            ))}
          </div>
          
          <div className="section">
            <h4>External Calls</h4>
            {selectedRequest.external_calls.map((call, index) => (
              <div key={index} className="external-call">
                <span>{call.method} {call.url}</span>
                <span>{call.duration_ms}ms</span>
                <span className={`status ${call.status >= 200 && call.status < 300 ? 'success' : 'error'}`}>
                  {call.status}
                </span>
              </div>
            ))}
          </div>
        </div>
      )}
    </div>
  );
};
```

### Implementation Notes

- Implement request profiling middleware
- Track database query times
- Track external API call times
- Add step-by-step execution mode
- Support breakpoints in workflows
- Implement variable inspection
- Add performance visualization
- Support replay of failed steps

---

## Story 21.9: Code Snippet Library

As a **QA Engineer**,
I want to **have a library of reusable code snippets**,
So that **I can quickly implement common patterns without writing everything from scratch**.

### Acceptance Criteria

**Given** I need to implement a workflow step  
**When** I search the snippet library  
**Then** I find relevant snippets  
**And** I can insert them into my workflow  
**And** I can customize parameters

**Given** I create a useful snippet  
**When** I save it to the library  
**Then** it's available to my team  
**And** I can add description and tags  
**And** I can categorize it

### Snippet Categories

```typescript
interface CodeSnippet {
  id: string;
  name: string;
  description: string;
  category: SnippetCategory;
  language: 'yaml' | 'json' | 'javascript' | 'python';
  code: string;
  parameters: SnippetParameter[];
  tags: string[];
  created_by: string;
  created_at: Date;
}

interface SnippetParameter {
  name: string;
  type: string;
  required: boolean;
  default?: string;
  description: string;
}

enum SnippetCategory {
  JiraIntegration = 'Jira Integration',
  PostmanCollection = 'Postman Collection',
  TestmoRun = 'Testmo Run',
  SplunkQuery = 'Splunk Query',
  WorkflowSteps = 'Workflow Steps',
  Validation = 'Validation',
  ErrorHandling = 'Error Handling',
}
```

### Predefined Snippets

#### Jira Integration Snippets

```yaml
# Sync ticket status
name: "Jira Status Sync"
category: "Jira Integration"
description: "Sync workflow status to Jira ticket"
code: |
  jira:
    ticket_required: true
    ticket: "{{TICKET_KEY}}"
    sync_status: true
    status_mapping:
      in_progress: "In Progress"
      completed: "Ready for QA"
      failed: "Failed QA"
    update_on_complete: "Verified"
parameters:
  - name: TICKET_KEY
    type: string
    required: true
    description: "Jira ticket key (e.g., PROJ-123)"
tags:
  - jira
  - status
  - sync
```

```yaml
# Create Jira comment
name: "Jira Comment"
category: "Jira Integration"
description: "Add comment to Jira ticket with workflow results"
code: |
  jira:
    ticket_required: true
    add_comment: true
    comment_template: |
      Workflow {{WORKFLOW_NAME}} completed
      
      Steps completed: {{STEPS_COMPLETED}}
      Duration: {{DURATION}}
      
      Results:
      {{RESULTS}}
parameters:
  - name: WORKFLOW_NAME
    type: string
    required: true
    description: "Workflow name"
  - name: STEPS_COMPLETED
    type: number
    required: true
    description: "Number of steps completed"
  - name: DURATION
    type: string
    required: true
    description: "Workflow duration"
  - name: RESULTS
    type: string
    required: true
    description: "Workflow results"
tags:
  - jira
  - comment
  - reporting
```

#### Postman Integration Snippets

```yaml
# Run Postman collection
name: "Postman Collection Run"
category: "Postman Collection"
description: "Execute a Postman collection as workflow step"
code: |
  steps:
    - id: "postman-run"
      name: "Run API Tests"
      type: integration
      integration: postman
      description: "Execute Postman collection"
      estimated_time: 15
      config:
        collection: "{{COLLECTION_NAME}}"
        environment: "{{ENVIRONMENT}}"
        folder: "{{FOLDER}}"
        iteration_count: 1
        delay: 0
      validation:
        all_tests_passed: true
parameters:
  - name: COLLECTION_NAME
    type: string
    required: true
    description: "Postman collection name"
  - name: ENVIRONMENT
    type: string
    required: false
    default: "staging"
    description: "Postman environment"
  - name: FOLDER
    type: string
    required: false
    description: "Specific folder in collection"
tags:
  - postman
  - api
  - testing
```

```yaml
# API health check
name: "API Health Check"
category: "Postman Collection"
description: "Quick health check for API endpoints"
code: |
  steps:
    - id: "health-check"
      name: "API Health Check"
      type: integration
      integration: postman
      description: "Check API endpoints health"
      estimated_time: 5
      config:
        collection: "health-checks"
        endpoints:
          - "{{BASE_URL}}/health"
          - "{{BASE_URL}}/api/status"
          - "{{BASE_URL}}/api/v1/version"
      validation:
        all_status_codes_200: true
        response_time_threshold: 500
parameters:
  - name: BASE_URL
    type: string
    required: true
    description: "API base URL"
tags:
  - postman
  - health
  - monitoring
```

#### Validation Snippets

```yaml
# Step validation with checklists
name: "Step with Checklist"
category: "Validation"
description: "Create a manual step with required checklist items"
code: |
  steps:
    - id: "{{STEP_ID}}"
      name: "{{STEP_NAME}}"
      type: manual
      description: "{{DESCRIPTION}}"
      estimated_time: {{ESTIMATED_TIME}}
      checklists:
        - "{{CHECKLIST_ITEM_1}}"
        - "{{CHECKLIST_ITEM_2}}"
        - "{{CHECKLIST_ITEM_3}}"
      validation:
        all_checklists_completed: true
        notes_required: false
parameters:
  - name: STEP_ID
    type: string
    required: true
    description: "Step ID"
  - name: STEP_NAME
    type: string
    required: true
    description: "Step name"
  - name: DESCRIPTION
    type: string
    required: true
    description: "Step description"
  - name: ESTIMATED_TIME
    type: number
    required: true
    description: "Estimated time in minutes"
  - name: CHECKLIST_ITEM_1
    type: string
    required: true
    description: "First checklist item"
  - name: CHECKLIST_ITEM_2
    type: string
    required: true
    description: "Second checklist item"
  - name: CHECKLIST_ITEM_3
    type: string
    required: true
    description: "Third checklist item"
tags:
  - validation
  - checklist
  - manual
```

### Snippet Library Component

```typescript
// frontend/components/snippets/SnippetLibrary.tsx
export const SnippetLibrary: React.FC = () => {
  const [snippets, setSnippets] = useState<CodeSnippet[]>([]);
  const [selectedCategory, setSelectedCategory] = useState<SnippetCategory | null>(null);
  const [searchQuery, setSearchQuery] = useState('');
  const [selectedSnippet, setSelectedSnippet] = useState<CodeSnippet | null>(null);
  const [showCreateModal, setShowCreateModal] = useState(false);
  
  useEffect(() => {
    // Load snippets from API
    fetch('/api/snippets')
      .then(res => res.json())
      .then(setSnippets);
  }, []);
  
  const filteredSnippets = snippets.filter(snippet => {
    const matchesCategory = !selectedCategory || snippet.category === selectedCategory;
    const matchesSearch = !searchQuery || 
      snippet.name.toLowerCase().includes(searchQuery.toLowerCase()) ||
      snippet.description.toLowerCase().includes(searchQuery.toLowerCase()) ||
      snippet.tags.some(tag => tag.toLowerCase().includes(searchQuery.toLowerCase()));
    
    return matchesCategory && matchesSearch;
  });
  
  const insertSnippet = (snippet: CodeSnippet) => {
    // TODO: Insert snippet into workflow editor
    console.log('Inserting snippet:', snippet);
    toast.success(`Snippet "${snippet.name}" inserted`);
  };
  
  const createSnippet = (snippet: Omit<CodeSnippet, 'id' | 'created_at' | 'created_by'>) => {
    const newSnippet: CodeSnippet = {
      ...snippet,
      id: Date.now().toString(),
      created_at: new Date(),
      created_by: 'current-user',
    };
    
    setSnippets([...snippets, newSnippet]);
    toast.success('Snippet created successfully');
  };
  
  return (
    <div className="snippet-library">
      <div className="sidebar">
        <h3>Categories</h3>
        <ul>
          <li 
            className={!selectedCategory ? 'active' : ''}
            onClick={() => setSelectedCategory(null)}
          >
            All Snippets
          </li>
          {Object.values(SnippetCategory).map(category => (
            <li 
              key={category}
              className={selectedCategory === category ? 'active' : ''}
              onClick={() => setSelectedCategory(category)}
            >
              {category}
            </li>
          ))}
        </ul>
      </div>
      
      <div className="main">
        <div className="header">
          <input 
            type="text"
            placeholder="Search snippets..."
            value={searchQuery}
            onChange={(e) => setSearchQuery(e.target.value)}
          />
          <button onClick={() => setShowCreateModal(true)}>+ Create Snippet</button>
        </div>
        
        <div className="snippets-grid">
          {filteredSnippets.map(snippet => (
            <div 
              key={snippet.id}
              className="snippet-card"
              onClick={() => setSelectedSnippet(snippet)}
            >
              <h4>{snippet.name}</h4>
              <p>{snippet.description}</p>
              <div className="tags">
                {snippet.tags.map(tag => (
                  <span key={tag} className="tag">{tag}</span>
                ))}
              </div>
            </div>
          ))}
        </div>
      </div>
      
      {selectedSnippet && (
        <SnippetDetail
          snippet={selectedSnippet}
          onClose={() => setSelectedSnippet(null)}
          onInsert={() => insertSnippet(selectedSnippet)}
        />
      )}
      
      {showCreateModal && (
        <CreateSnippetModal
          onClose={() => setShowCreateModal(false)}
          onCreate={createSnippet}
        />
      )}
    </div>
  );
};
```

### Implementation Notes

- Create predefined snippets for common patterns
- Support custom snippets from team
- Implement snippet search and filtering
- Add snippet categories and tags
- Support parameter substitution
- Implement snippet versioning
- Add snippet sharing and collaboration
- Support snippet export/import

---

## Story 21.10: Developer Portal with Interactive Docs

As a **QA Engineer**,
I want to **have interactive documentation**,
So that **I can learn the framework quickly and find answers to my questions**.

### Acceptance Criteria

**Given** I access the developer portal  
**When** I search for a topic  
**Then** I see relevant documentation  
**And** I can run code examples inline  
**And** I can copy examples to clipboard

**Given** I'm learning a new feature  
**When** I read the docs  
**Then** I see interactive examples  
**And** I can see expected outputs  
**And** I can navigate to related topics

### Developer Portal Structure

```typescript
// Documentation structure
interface DocumentationSection {
  id: string;
  title: string;
  description: string;
  order: number;
  subsections: DocumentationSubsection[];
  codeExamples: CodeExample[];
}

interface DocumentationSubsection {
  id: string;
  title: string;
  content: string;
  codeExamples: CodeExample[];
}

interface CodeExample {
  id: string;
  title: string;
  description: string;
  code: string;
  language: 'yaml' | 'json' | 'bash' | 'typescript' | 'python';
  runnable?: boolean;
  expectedOutput?: string;
}
```

### Interactive Documentation Component

```typescript
// frontend/components/developer-portal/Documentation.tsx
export const Documentation: React.FC = () => {
  const [activeSection, setActiveSection] = useState<string>('getting-started');
  const [activeSubsection, setActiveSubsection] = useState<string>('installation');
  const [searchQuery, setSearchQuery] = useState('');
  
  const documentation = useDocumentation();
  
  const currentSection = documentation.sections.find(s => s.id === activeSection);
  const currentSubsection = currentSection?.subsections.find(s => s.id === activeSubsection);
  
  return (
    <div className="documentation">
      <div className="sidebar">
        <input 
          type="text"
          placeholder="Search documentation..."
          value={searchQuery}
          onChange={(e) => setSearchQuery(e.target.value)}
        />
        
        <nav>
          {documentation.sections.map(section => (
            <div key={section.id} className="section">
              <h3 
                className={activeSection === section.id ? 'active' : ''}
                onClick={() => {
                  setActiveSection(section.id);
                  setActiveSubsection(section.subsections[0]?.id || '');
                }}
              >
                {section.title}
              </h3>
              
              {activeSection === section.id && (
                <ul>
                  {section.subsections.map(sub => (
                    <li
                      key={sub.id}
                      className={activeSubsection === sub.id ? 'active' : ''}
                      onClick={() => setActiveSubsection(sub.id)}
                    >
                      {sub.title}
                    </li>
                  ))}
                </ul>
              )}
            </div>
          ))}
        </nav>
      </div>
      
      <div className="content">
        {currentSubsection && (
          <>
            <h1>{currentSubsection.title}</h1>
            <div 
              dangerouslySetInnerHTML={{ __html: renderMarkdown(currentSubsection.content) }}
            />
            
            {currentSubsection.codeExamples.map(example => (
              <CodeExampleViewer 
                key={example.id}
                example={example}
              />
            ))}
          </>
        )}
      </div>
      
      <div className="toc">
        <h3>On This Page</h3>
        <TableOfContents content={currentSubsection?.content} />
      </div>
    </div>
  );
};
```

### Interactive Code Example Viewer

```typescript
// frontend/components/developer-portal/CodeExampleViewer.tsx
export const CodeExampleViewer: React.FC<{ example: CodeExample }> = ({ example }) => {
  const [output, setOutput] = useState<string>('');
  const [loading, setLoading] = useState(false);
  const [copied, setCopied] = useState(false);
  
  const runExample = async () => {
    if (!example.runnable) return;
    
    setLoading(true);
    try {
      const response = await fetch('/api/execute-example', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ code: example.code, language: example.language }),
      });
      
      const result = await response.json();
      setOutput(result.output);
    } catch (error) {
      setOutput(error.message);
    } finally {
      setLoading(false);
    }
  };
  
  const copyToClipboard = () => {
    navigator.clipboard.writeText(example.code);
    setCopied(true);
    setTimeout(() => setCopied(false), 2000);
  };
  
  return (
    <div className="code-example">
      <h4>{example.title}</h4>
      <p>{example.description}</p>
      
      <div className="code-block">
        <div className="header">
          <span className="language">{example.language}</span>
          <button onClick={copyToClipboard}>
            {copied ? '✓ Copied' : '📋 Copy'}
          </button>
          {example.runnable && (
            <button onClick={runExample} disabled={loading}>
              {loading ? 'Running...' : ▶ Run}
            </button>
          )}
        </div>
        
        <SyntaxHighlighter language={example.language}>
          {example.code}
        </SyntaxHighlighter>
      </div>
      
      {(output || example.expectedOutput) && (
        <div className="output">
          <h5>Output:</h5>
          <pre>
            {output || example.expectedOutput}
          </pre>
        </div>
      )}
    </div>
  );
};
```

### Documentation Content

```markdown
# Getting Started

## Installation

The easiest way to install QA Intelligent PMS is using the CLI:

\`\`\`bash
# Using npm
npm install -g @qapms/cli

# Using cargo
cargo install qapms-cli

# Or download binary
# Visit https://github.com/yourorg/qapms/releases
\`\`\`

## Quick Start

Initialize a new project:

\`\`\`bash
qapms init my-qa-project
cd my-qa-project
qapms dev
\`\`\`

## Your First Workflow

Create a simple workflow:

\`\`\`yaml
name: "My First Workflow"
description: "A simple test workflow"
version: "1.0"

steps:
  - id: "step-1"
    name: "Setup Environment"
    type: manual
    description: "Prepare test environment"
    estimated_time: 15
    
  - id: "step-2"
    name: "Run Tests"
    type: integration
    integration: postman
    description: "Execute test collection"
    estimated_time: 30
    config:
      collection: "my-tests"
\`\`\`

## Next Steps

- [Configuring Integrations](./integrations)
- [Workflow Templates](./templates)
- [CLI Reference](./cli-reference)
```

### Implementation Notes

- Use interactive documentation framework
- Support Markdown rendering
- Add code syntax highlighting
- Implement runnable code examples
- Add search functionality
- Support documentation versioning
- Implement table of contents
- Add feedback system for docs
- Support multiple languages
- Add API reference with try-it-out

---

## Dependencies

### New Crate Dependencies

```toml
[workspace.dependencies]
# CLI
clap = { version = "4", features = ["derive"] }
dialoguer = "0.11"
colored = "2"
indicatif = "0.17"

# File watching
notify = "6"

# Logging
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter", "json"] }
tracing-appender = "0.2"

# Data generation
fake = { version = "2.9", features = ["derive"] }
rand = "0.8"
uuid = { version = "1.6", features = ["v4"] }

# Profiling
pprof = { version = "0.13", features = ["flamegraph"] }

# Serialization
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
serde_yaml = "0.9"
```

### Frontend Dependencies

```json
{
  "dependencies": {
    "react": "^19",
    "react-dom": "^19",
    "react-router-dom": "^7",
    "axios": "^1.6",
    "zustand": "^5",
    "@tanstack/react-query": "^5",
    "recharts": "^3",
    "react-syntax-highlighter": "^15",
    "marked": "^11",
    "lucide-react": "^0.300"
  },
  "devDependencies": {
    "@types/react": "^19",
    "@types/react-dom": "^19",
    "typescript": "^5",
    "vite": "^7",
    "tailwindcss": "^4"
  }
}
```

---

## Timeline

| Week | Stories | Deliverables |
|------|---------|--------------|
| 1 | 21.1, 21.2 | Enhanced CLI, workflow templates |
| 2 | 21.3, 21.4 | Mock data generators, VS Code extension alpha |
| 3 | 21.5, 21.6 | API playground, hot reload |
| 4 | 21.7, 21.8 | Enhanced logging, debug tools |
| 5 | 21.9, 21.10 | Snippet library, developer portal |
| 6 | Testing & Documentation | Comprehensive DX testing |

---

## Success Metrics

| Metric | Target | Measurement |
|--------|--------|-------------|
| CLI adoption rate | > 90% of QAs | CLI usage analytics |
| Time to first workflow | < 15 minutes | Onboarding surveys |
| Template usage | > 70% of workflows | Template analytics |
| Developer satisfaction | > 4.5/5 | NPS surveys |
| Documentation page views | > 10,000/month | Analytics |
| Snippet reuse | > 50% of workflows | Snippet analytics |
| Debug mode usage | > 60% of dev sessions | Usage analytics |

---

## Next Steps

1. **CLI Foundation**: Implement core CLI structure with clap
2. **Template System**: Create template generator and predefined templates
3. **VS Code Extension**: Start with language support and validation
4. **API Playground**: Build interactive API explorer
5. **Hot Reload**: Implement file watching and WebSocket notifications
6. **Documentation**: Create interactive docs with runnable examples
7. **Testing**: Comprehensive testing of all DX features
8. **Documentation**: Write user guides and tutorials

---

*Last Updated: 2025-01-16*  
*Author: AI Assistant*  
*Version: 1.0*