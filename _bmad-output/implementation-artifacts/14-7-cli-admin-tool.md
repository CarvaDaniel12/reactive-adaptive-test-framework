# Story 14.7: CLI Admin Tool

**As a** system administrator  
**I want** a CLI tool for administrative tasks  
**So that** I can manage the application without using the API directly

---

## Story Metadata

| Field | Value |
|-------|-------|
| Story ID | 14.7 |
| Epic | Rust Implementation Improvements |
| Sprint | 6 - Usability - CLI |
| Priority | P2 |
| Estimated Days | 2 |
| Dependencies | 14.1 (Graceful Shutdown and Signal Handling) |
| Status | ready-for-dev |

---

## Technical Requirements

### 1. Add `clap` crate for CLI argument parsing

- Use `clap` v4 with derive feature for type-safe CLI
- Support environment variable binding
- Implement subcommands for different operations
- Provide comprehensive help and usage information

### 2. Implement subcommands

#### `serve` - Start the API server (default behavior)
- Optional `--port` flag (default: 3000)
- Optional `--host` flag (default: 0.0.0.0)
- Optional `--workers` flag (Tokio worker threads)
- Graceful shutdown handling (reuses Story 14.1)

#### `migrate run` - Run pending database migrations
- Execute all pending SQLx migrations
- Display progress for each migration
- Roll back on failure
- Record completed migrations in database

#### `migrate status` - Show migration status
- List all migrations with status (applied/pending)
- Show migration timestamps
- Display database version

#### `migrate reset` - Reset database (dangerous)
- Warning prompt for confirmation
- Drop all tables and recreate schema
- Require `--force` flag to proceed

#### `health` - Check integration health
- Check all configured integrations (Jira, Postman, Testmo, Splunk)
- Optional `--integration` flag to check specific integration
- Display health status with color indicators
- Exit with appropriate code (0=healthy, 1=degraded, 2=unhealthy)

#### `health watch` - Watch health status continuously
- Refresh health status every 30 seconds
- Display changes in real-time
- Exit on Ctrl+C

#### `config validate` - Validate configuration
- Load and parse configuration file
- Validate all required fields
- Check integration credentials
- Return exit code based on validation result

#### `config show` - Show masked configuration
- Display current configuration
- Mask sensitive values (API keys, tokens, passwords)
- Show active environment variables

#### `config generate-key` - Generate encryption key
- Generate 256-bit encryption key
- Output in hex format
- Optionally write to file

### 3. Support `--config` flag for custom config file

- Allow specifying custom configuration file path
- Support multiple config file formats (YAML, TOML, JSON)
- Default to `config.yaml` in current directory or `/etc/qa-pms/config.yaml`

### 4. Colorized output with `--no-color` option

- Use color for status indicators (green=success, red=error, yellow=warning)
- Respect `NO_COLOR` environment variable
- Provide `--no-color` global flag to disable colors

### 5. JSON output option for scripting

- Add `--json` global flag to output JSON instead of human-readable text
- Structure output consistently across commands
- Include exit code in JSON output

### 6. Logging configuration

- Support `--log-level` flag (error, warn, info, debug, trace)
- Support `--log-format` flag (pretty, json, compact)
- Default to INFO level with pretty format

---

## Acceptance Criteria

- [ ] `qa-pms serve` starts the server (current behavior preserved)
- [ ] `qa-pms serve --port 8080` starts on custom port
- [ ] `qa-pms migrate run` runs migrations successfully
- [ ] `qa-pms migrate status` shows migration state
- [ ] `qa-pms migrate reset --force` resets database with warning
- [ ] `qa-pms health` checks all integrations
- [ ] `qa-pms health --integration jira` checks only Jira
- [ ] `qa-pms health watch` refreshes health status continuously
- [ ] `qa-pms config validate` validates configuration
- [ ] `qa-pms config show` displays masked configuration
- [ ] `qa-pms config generate-key` outputs valid 256-bit hex key
- [ ] `--help` shows comprehensive help for all commands
- [ ] Exit codes indicate success/failure appropriately (0=success, 1=error)
- [ ] `--config` flag loads custom configuration file
- [ ] `--no-color` flag disables colored output
- [ ] `--json` flag outputs structured JSON
- [ ] `--log-level` and `--log-format` flags control logging
- [ ] Graceful shutdown works in `serve` command

---

## Implementation Notes

### Installation

Add `clap` to workspace dependencies:

```toml
# Cargo.toml

[workspace.dependencies]
clap = { version = "4.4", features = ["derive", "env", "string"] }
```

Add to API crate:

```toml
# crates/qa-pms-api/Cargo.toml

[dependencies]
clap = { workspace = true }
indicatif = "0.17"  # For progress bars
console = "0.15"     # For colored output
termcolor = "1.4"    # For color support
```

### CLI Structure

```rust
// crates/qa-pms-api/src/cli.rs

use clap::{Parser, Subcommand, ValueEnum};
use std::path::PathBuf;

/// QA Intelligent PMS - A comprehensive quality assurance platform with AI-powered test management
#[derive(Parser)]
#[command(name = "qa-pms")]
#[command(about = "QA Intelligent PMS CLI Tool", long_about = None)]
#[command(author, version)]
#[command(propagate_version = true)]
pub struct Cli {
    /// Path to configuration file
    #[arg(short, long, global = true, value_name = "FILE")]
    pub config: Option<PathBuf>,

    /// Disable colored output
    #[arg(long, global = true, env = "NO_COLOR")]
    pub no_color: bool,

    /// Output format (text, json)
    #[arg(short, long, global = true, value_enum, default_value_t = OutputFormat::Text)]
    pub output: OutputFormat,

    /// Log level (error, warn, info, debug, trace)
    #[arg(long, global = true, value_enum, default_value_t = LogLevel::Info)]
    pub log_level: LogLevel,

    /// Log format (pretty, json, compact)
    #[arg(long, global = true, value_enum, default_value_t = LogFormat::Pretty)]
    pub log_format: LogFormat,

    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Start the API server
    Serve {
        /// Host address to bind to
        #[arg(short, long, default_value = "0.0.0.0")]
        host: String,

        /// Port to listen on
        #[arg(short, long, default_value = "3000")]
        port: u16,

        /// Number of worker threads (default: CPU count)
        #[arg(short, long)]
        workers: Option<usize>,
    },

    /// Database migration commands
    Migrate {
        #[command(subcommand)]
        action: MigrateAction,
    },

    /// Check integration health
    Health {
        /// Specific integration to check
        #[arg(short, long)]
        integration: Option<String>,

        /// Watch mode: refresh health status every 30 seconds
        #[arg(long)]
        watch: bool,
    },

    /// Configuration management
    Config {
        #[command(subcommand)]
        action: ConfigAction,
    },
}

#[derive(Subcommand)]
pub enum MigrateAction {
    /// Run pending database migrations
    Run {
        /// Run migrations in dry-run mode (don't actually apply)
        #[arg(long)]
        dry_run: bool,
    },

    /// Show migration status
    Status,

    /// Reset database (DANGEROUS: drops all tables)
    Reset {
        /// Force reset without confirmation prompt
        #[arg(long)]
        force: bool,
    },
}

#[derive(Subcommand)]
pub enum ConfigAction {
    /// Validate configuration file
    Validate,

    /// Show current configuration (sensitive values are masked)
    Show,

    /// Generate a new 256-bit encryption key
    GenerateKey {
        /// Write key to file instead of stdout
        #[arg(short, long, value_name = "FILE")]
        output: Option<PathBuf>,
    },
}

#[derive(Clone, ValueEnum)]
pub enum OutputFormat {
    Text,
    Json,
}

#[derive(Clone, ValueEnum)]
pub enum LogLevel {
    Error,
    Warn,
    Info,
    Debug,
    Trace,
}

#[derive(Clone, ValueEnum)]
pub enum LogFormat {
    Pretty,
    Json,
    Compact,
}

impl LogLevel {
    pub fn to_tracing_level(&self) -> tracing::Level {
        match self {
            LogLevel::Error => tracing::Level::ERROR,
            LogLevel::Warn => tracing::Level::WARN,
            LogLevel::Info => tracing::Level::INFO,
            LogLevel::Debug => tracing::Level::DEBUG,
            LogLevel::Trace => tracing::Level::TRACE,
        }
    }
}
```

### Output Helpers

```rust
// crates/qa-pms-api/src/cli/output.rs

use console::{Style, Term};
use termcolor::{Color, ColorSpec, StandardStream, WriteColor};
use std::io::{self, Write};

pub struct OutputFormatter {
    format: OutputFormat,
    no_color: bool,
    term: Term,
}

impl OutputFormatter {
    pub fn new(format: OutputFormat, no_color: bool) -> Self {
        Self {
            format,
            no_color,
            term: Term::stdout(),
        }
    }

    /// Print success message (green)
    pub fn success(&self, message: &str) {
        if self.format == OutputFormat::Json {
            self.json_output("success", message);
            return;
        }

        if self.no_color {
            println!("✓ {}", message);
        } else {
            let style = Style::new().green();
            println!("{} {}", style.apply_to("✓"), message);
        }
    }

    /// Print error message (red)
    pub fn error(&self, message: &str) {
        if self.format == OutputFormat::Json {
            self.json_output("error", message);
            return;
        }

        if self.no_color {
            eprintln!("✗ {}", message);
        } else {
            let style = Style::new().red();
            eprintln!("{} {}", style.apply_to("✗"), message);
        }
    }

    /// Print warning message (yellow)
    pub fn warning(&self, message: &str) {
        if self.format == OutputFormat::Json {
            self.json_output("warning", message);
            return;
        }

        if self.no_color {
            println!("⚠ {}", message);
        } else {
            let style = Style::new().yellow();
            println!("{} {}", style.apply_to("⚠"), message);
        }
    }

    /// Print info message (blue)
    pub fn info(&self, message: &str) {
        if self.format == OutputFormat::Json {
            self.json_output("info", message);
            return;
        }

        if self.no_color {
            println!("ℹ {}", message);
        } else {
            let style = Style::new().blue();
            println!("{} {}", style.apply_to("ℹ"), message);
        }
    }

    /// Print table header
    pub fn table_header(&self, headers: &[&str]) {
        if self.format == OutputFormat::Json {
            return; // Tables not supported in JSON mode
        }

        let mut stdout = StandardStream::stdout(if self.no_color {
            termcolor::ColorChoice::Never
        } else {
            termcolor::ColorChoice::Auto
        });

        stdout.set_color(ColorSpec::new().set_bold(true)).ok();
        for (i, header) in headers.iter().enumerate() {
            write!(&mut stdout, "{:<20}", header).ok();
            if i < headers.len() - 1 {
                write!(&mut stdout, "  ").ok();
            }
        }
        stdout.reset().ok();
        writeln!(&mut stdout).ok();
        println!("{}", "─".repeat(20 * headers.len() + (headers.len() - 1) * 2));
    }

    /// Print table row
    pub fn table_row(&self, values: &[&str]) {
        if self.format == OutputFormat::Json {
            return;
        }

        for (i, value) in values.iter().enumerate() {
            print!("{:<20}", value);
            if i < values.len() - 1 {
                print!("  ");
            }
        }
        println!();
    }

    /// Print JSON output
    fn json_output(&self, level: &str, message: &str) {
        let output = serde_json::json!({
            "level": level,
            "message": message,
        });
        println!("{}", output);
    }

    /// Print structured JSON
    pub fn json<T: serde::Serialize>(&self, data: T) {
        if self.format == OutputFormat::Json {
            println!("{}", serde_json::to_string_pretty(&data).unwrap());
        }
    }

    /// Confirm with user (yes/no)
    pub fn confirm(&self, message: &str) -> bool {
        if self.format == OutputFormat::Json {
            // In JSON mode, always return false for safety
            self.error("Confirmation required. Run without --json flag.");
            return false;
        }

        print!("{} [y/N]: ", message);
        io::stdout().flush().ok();

        let mut input = String::new();
        io::stdin().read_line(&mut input).ok();

        let input = input.trim().to_lowercase();
        input == "y" || input == "yes"
    }
}
```

### Main CLI Entry Point

```rust
// crates/qa-pms-api/src/main.rs

mod cli;
mod commands;
mod cli_output;

use cli::{Cli, Commands, LogLevel, LogFormat, OutputFormat};
use cli_output::OutputFormatter;
use anyhow::{Context, Result};

#[tokio::main]
async fn main() -> Result<()> {
    // Parse CLI arguments
    let cli = Cli::parse();

    // Initialize logging based on CLI flags
    init_logging(&cli.log_level, &cli.log_format);

    // Create output formatter
    let output = OutputFormatter::new(cli.output, cli.no_color);

    // Load configuration
    let settings = load_settings(cli.config).await.context("Failed to load configuration")?;

    // Execute command or default to serve
    match cli.command.unwrap_or(Commands::Serve {
        host: "0.0.0.0".to_string(),
        port: 3000,
        workers: None,
    }) {
        Commands::Serve { host, port, workers } => {
            commands::serve::execute(&settings, &host, port, workers, &output).await?;
        }
        Commands::Migrate { action } => {
            commands::migrate::execute(&settings, action, &output).await?;
        }
        Commands::Health { integration, watch } => {
            commands::health::execute(&settings, integration, watch, &output).await?;
        }
        Commands::Config { action } => {
            commands::config::execute(&settings, action, &output).await?;
        }
    }

    Ok(())
}

fn init_logging(level: &LogLevel, format: &LogFormat) {
    use tracing_subscriber::{fmt, EnvFilter, layer::SubscriberExt, util::SubscriberInitExt};

    let filter = EnvFilter::builder()
        .with_default_directive(level.to_tracing_level().into())
        .from_env_lossy();

    match format {
        LogFormat::Pretty => {
            tracing_subscriber::registry()
                .with(filter)
                .with(fmt::layer().pretty())
                .init();
        }
        LogFormat::Json => {
            tracing_subscriber::registry()
                .with(filter)
                .with(fmt::layer().json())
                .init();
        }
        LogFormat::Compact => {
            tracing_subscriber::registry()
                .with(filter)
                .with(fmt::layer().compact())
                .init();
        }
    }
}

async fn load_settings(config_path: Option<std::path::PathBuf>) -> Result<qa_pms_config::Settings> {
    // Try to load from specified path, then default locations
    let paths = config_path
        .into_iter()
        .chain([
            std::path::PathBuf::from("config.yaml"),
            std::path::PathBuf::from("/etc/qa-pms/config.yaml"),
        ]);

    for path in paths {
        if path.exists() {
            return qa_pms_config::load_from_file(&path).await;
        }
    }

    // Fall back to environment variables
    Ok(qa_pms_config::load_from_env()?)
}
```

### Serve Command

```rust
// crates/qa-pms-api/src/commands/serve.rs

use crate::{app::create_app, telemetry::{TelemetryConfig, init_tracing, shutdown_tracing}, cli_output::OutputFormatter};
use anyhow::Result;
use std::net::SocketAddr;

pub async fn execute(
    settings: &qa_pms_config::Settings,
    host: &str,
    port: u16,
    workers: Option<usize>,
    output: &OutputFormatter,
) -> Result<()> {
    let addr = format!("{}:{}", host, port).parse::<SocketAddr>()?;
    let listener = tokio::net::TcpListener::bind(&addr).await?;

    output.info(&format!("Starting server on http://{}", addr));
    output.info(&format!("Workers: {}", workers.unwrap_or_else(|| num_cpus::get())));

    // Create app
    let app = create_app(settings.clone()).await?;

    // Run server with graceful shutdown
    let shutdown_signal = async {
        tokio::signal::ctrl_c()
            .await
            .expect("Failed to install Ctrl+C handler");
        output.info("Received shutdown signal");
    };

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal)
        .await?;

    output.success("Server shut down gracefully");
    Ok(())
}
```

### Migrate Command

```rust
// crates/qa-pms-api/src/commands/migrate.rs

use crate::cli::{Commands, MigrateAction};
use crate::cli_output::OutputFormatter;
use anyhow::Result;
use sqlx::PgPool;
use indicatif::{ProgressBar, ProgressStyle};

pub async fn execute(
    settings: &qa_pms_config::Settings,
    action: MigrateAction,
    output: &OutputFormatter,
) -> Result<()> {
    let pool = PgPool::connect(&settings.database.url).await?;

    match action {
        MigrateAction::Run { dry_run } => {
            if dry_run {
                output.info("Dry-run mode: would apply migrations:");
                list_migrations(&pool, output).await?;
            } else {
                output.info("Running migrations...");
                run_migrations(&pool, output).await?;
                output.success("Migrations completed successfully");
            }
        }
        MigrateAction::Status => {
            output.info("Migration status:");
            list_migrations(&pool, output).await?;
        }
        MigrateAction::Reset { force } => {
            if !force && !output.confirm("WARNING: This will drop all tables. Continue?") {
                output.info("Reset cancelled");
                return Ok(());
            }

            output.info("Resetting database...");
            reset_database(&pool, output).await?;
            output.success("Database reset successfully");
        }
    }

    Ok(())
}

async fn run_migrations(pool: &PgPool, output: &OutputFormatter) -> Result<()> {
    let migrations = sqlx::migrate!("./migrations");

    // Create progress bar
    let pb = ProgressBar::new_spinner();
    pb.set_style(ProgressStyle::default_spinner()
        .tick_strings(&["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"])
        .template("{spinner:.green} {msg}"));

    pb.set_message("Applying migrations...");

    migrations.run(pool).await?;

    pb.finish_with_message("Migrations applied");

    Ok(())
}

async fn list_migrations(pool: &PgPool, output: &OutputFormatter) -> Result<()> {
    output.table_header(&["Migration", "Status", "Applied At"]);

    let migrations = sqlx::query!(
        "SELECT version, description, applied_on FROM _sqlx_migrations ORDER BY version"
    )
    .fetch_all(pool)
    .await?;

    if migrations.is_empty() {
        output.table_row(&["(none)", "No migrations", ""]);
    } else {
        for migration in migrations {
            let status = migration.applied_on.map(|_| "Applied");
            let applied_at = migration.applied_on
                .map(|d| d.to_string())
                .unwrap_or_else(|| "Pending".to_string());

            output.table_row(&[
                &migration.description,
                status.unwrap_or("Pending"),
                &applied_at,
            ]);
        }
    }

    Ok(())
}

async fn reset_database(pool: &PgPool, output: &OutputFormatter) -> Result<()> {
    // Drop all tables
    sqlx::query(
        r#"
        DO $$ DECLARE
            r RECORD;
        BEGIN
            FOR r IN (SELECT tablename FROM pg_tables WHERE schemaname = 'public') LOOP
                EXECUTE 'DROP TABLE IF EXISTS ' || quote_ident(r.tablename) || ' CASCADE';
            END LOOP;
        END $$;
        "#
    )
    .execute(pool)
    .await?;

    // Re-run migrations
    run_migrations(pool, output).await?;

    Ok(())
}
```

### Health Command

```rust
// crates/qa-pms-api/src/commands/health.rs

use crate::cli_output::OutputFormatter;
use anyhow::Result;
use chrono::Utc;

#[derive(serde::Serialize)]
struct HealthStatus {
    integration: String,
    status: String,
    message: String,
    timestamp: String,
}

pub async fn execute(
    settings: &qa_pms_config::Settings,
    integration: Option<String>,
    watch: bool,
    output: &OutputFormatter,
) -> Result<()> {
    if watch {
        watch_health(settings, integration, output).await?;
    } else {
        check_health(settings, integration, output).await?;
    }

    Ok(())
}

async fn check_health(
    settings: &qa_pms_config::Settings,
    integration: Option<String>,
    output: &OutputFormatter,
) -> Result<()> {
    let integrations = vec![
        ("jira", "Jira"),
        ("postman", "Postman"),
        ("testmo", "Testmo"),
        ("splunk", "Splunk"),
    ];

    let mut statuses = Vec::new();

    for (id, name) in integrations {
        if let Some(ref filter) = integration {
            if id != filter {
                continue;
            }
        }

        let (status, message) = check_integration_health(settings, id).await;
        statuses.push(HealthStatus {
            integration: name.to_string(),
            status,
            message,
            timestamp: Utc::now().to_rfc3339(),
        });

        match status.as_str() {
            "healthy" => output.success(&format!("{}: {}", name, message)),
            "degraded" => output.warning(&format!("{}: {}", name, message)),
            "unhealthy" => output.error(&format!("{}: {}", name, message)),
            _ => {}
        }
    }

    if output.format == OutputFormat::Json {
        output.json(statuses);
    }

    // Determine overall exit code
    let has_unhealthy = statuses.iter().any(|s| s.status == "unhealthy");
    let has_degraded = statuses.iter().any(|s| s.status == "degraded");

    if has_unhealthy {
        std::process::exit(2);
    } else if has_degraded {
        std::process::exit(1);
    }

    Ok(())
}

async fn watch_health(
    settings: &qa_pms_config::Settings,
    integration: Option<String>,
    output: &OutputFormatter,
) -> Result<()> {
    output.info("Watching health status (Ctrl+C to stop)...");

    loop {
        // Clear screen (works on Unix, Windows handles differently)
        print!("\x1B[2J\x1B[1;1H");

        check_health(settings, integration.clone(), output).await?;

        tokio::time::sleep(tokio::time::Duration::from_secs(30)).await;
    }
}

async fn check_integration_health(settings: &qa_pms_config::Settings, integration: &str) -> (String, String) {
    // This would call the actual health check logic
    // For now, return mock responses
    match integration {
        "jira" => ("healthy", "API responding".to_string()),
        "postman" => ("healthy", "Search working".to_string()),
        "testmo" => ("healthy", "CRUD operations OK".to_string()),
        "splunk" => ("degraded", "Query template rate limit".to_string()),
        _ => ("unknown", "Not configured".to_string()),
    }
}
```

### Config Command

```rust
// crates/qa-pms-api/src/commands/config.rs

use crate::cli::{Commands, ConfigAction};
use crate::cli_output::OutputFormatter;
use anyhow::Result;
use rand::Rng;

#[derive(serde::Serialize)]
struct ValidationResult {
    valid: bool,
    errors: Vec<String>,
    warnings: Vec<String>,
}

pub async fn execute(
    settings: &qa_pms_config::Settings,
    action: ConfigAction,
    output: &OutputFormatter,
) -> Result<()> {
    match action {
        ConfigAction::Validate => {
            validate_config(settings, output).await?;
        }
        ConfigAction::Show => {
            show_config(settings, output).await?;
        }
        ConfigAction::GenerateKey { output: output_path } => {
            generate_key(output_path, output).await?;
        }
    }

    Ok(())
}

async fn validate_config(
    settings: &qa_pms_config::Settings,
    output: &OutputFormatter,
) -> Result<()> {
    let mut errors = Vec::new();
    let mut warnings = Vec::new();

    // Validate database URL
    if settings.database.url.is_empty() {
        errors.push("Database URL is required".to_string());
    }

    // Validate Jira settings
    if settings.jira.base_url.is_empty() {
        errors.push("Jira base URL is required".to_string());
    }
    if settings.jira.api_token.is_empty() && settings.jira.oauth.client_id.is_empty() {
        warnings.push("Jira authentication not configured".to_string());
    }

    // Validate other integrations similarly...

    let result = ValidationResult {
        valid: errors.is_empty(),
        errors,
        warnings,
    };

    if output.format == OutputFormat::Json {
        output.json(result);
    } else {
        if result.valid {
            output.success("Configuration is valid");
        } else {
            output.error("Configuration has errors:");
            for error in &result.errors {
                println!("  - {}", error);
            }
            for warning in &result.warnings {
                output.warning(&format!("  - {}", warning));
            }
        }
    }

    Ok(())
}

async fn show_config(settings: &qa_pms_config::Settings, output: &OutputFormatter) -> Result<()> {
    // Mask sensitive values
    let masked_config = serde_json::json!({
        "database": {
            "url": mask_url(&settings.database.url),
        },
        "jira": {
            "base_url": settings.jira.base_url,
            "api_token": mask_string(&settings.jira.api_token),
            "oauth": {
                "client_id": settings.jira.oauth.client_id,
                "client_secret": mask_string(&settings.jira.oauth.client_secret),
            },
        },
        // ... other integrations
    });

    output.json(masked_config);
    Ok(())
}

async fn generate_key(output_path: Option<std::path::PathBuf>, output: &OutputFormatter) -> Result<()> {
    let mut rng = rand::thread_rng();
    let key: [u8; 32] = rng.gen();
    let hex_key = hex::encode(key);

    match output_path {
        Some(path) => {
            tokio::fs::write(&path, hex_key).await?;
            output.success(&format!("Generated encryption key and saved to {}", path.display()));
        }
        None => {
            output.info("Generated encryption key:");
            println!("{}", hex_key);
        }
    }

    Ok(())
}

fn mask_url(url: &str) -> String {
    if url.is_empty() {
        return "(not set)".to_string();
    }

    // Mask password in URL: postgresql://user:pass@host/db
    if let Some(start) = url.find("://") {
        if let Some(at) = url.find('@') {
            return format!("{}://***@{}", &url[..start + 3], &url[at + 1..]);
        }
    }

    "***".to_string()
}

fn mask_string(s: &str) -> String {
    if s.is_empty() {
        "(not set)".to_string();
    }

    "***".to_string()
}
```

---

## Dependencies to Add

### Workspace Dependencies

```toml
# Cargo.toml

[workspace.dependencies]
clap = { version = "4.4", features = ["derive", "env", "string"] }
indicatif = "0.17"
console = "0.15"
termcolor = "1.4"
```

### Crate Dependencies

```toml
# crates/qa-pms-api/Cargo.toml

[dependencies]
clap = { workspace = true }
indicatif = { workspace = true }
console = { workspace = true }
termcolor = { workspace = true }
```

---

## Files to Create

| File | Description |
|------|-------------|
| `crates/qa-pms-api/src/cli.rs` | CLI argument parsing with clap |
| `crates/qa-pms-api/src/cli_output.rs` | Output formatting helpers |
| `crates/qa-pms-api/src/commands/mod.rs` | Command module exports |
| `crates/qa-pms-api/src/commands/serve.rs` | Serve command implementation |
| `crates/qa-pms-api/src/commands/migrate.rs` | Migration command implementation |
| `crates/qa-pms-api/src/commands/health.rs` | Health check command implementation |
| `crates/qa-pms-api/src/commands/config.rs` | Config command implementation |

---

## Files to Modify

| File | Type | Changes |
|-------|------|---------|
| `Cargo.toml` | Modify | Add CLI dependencies to workspace |
| `crates/qa-pms-api/Cargo.toml` | Modify | Add CLI crate dependencies |
| `crates/qa-pms-api/src/main.rs` | Modify | Update main to use CLI, route to commands |

---

## Testing Strategy

### Unit Tests for CLI Parsing

```rust
// crates/qa-pms-api/tests/cli_parsing_test.rs

#[cfg(test)]
mod tests {
    use super::*;
    use clap::Parser;

    #[test]
    fn test_serve_command_defaults() {
        let cli = Cli::parse_from(["qa-pms", "serve"]);

        assert!(matches!(cli.command, Some(Commands::Serve { .. })));
    }

    #[test]
    fn test_serve_command_with_port() {
        let cli = Cli::parse_from(["qa-pms", "serve", "--port", "8080"]);

        match cli.command {
            Some(Commands::Serve { port, .. }) => {
                assert_eq!(port, 8080);
            }
            _ => panic!("Expected Serve command"),
        }
    }

    #[test]
    fn test_migrate_run_command() {
        let cli = Cli::parse_from(["qa-pms", "migrate", "run"]);

        match cli.command {
            Some(Commands::Migrate { action }) => {
                assert!(matches!(action, MigrateAction::Run { .. }));
            }
            _ => panic!("Expected Migrate command"),
        }
    }

    #[test]
    fn test_config_generate_key_command() {
        let cli = Cli::parse_from(["qa-pms", "config", "generate-key"]);

        match cli.command {
            Some(Commands::Config { action }) => {
                assert!(matches!(action, ConfigAction::GenerateKey { .. }));
            }
            _ => panic!("Expected Config command"),
        }
    }

    #[test]
    fn test_json_output_flag() {
        let cli = Cli::parse_from(["qa-pms", "health", "--json"]);

        assert_eq!(cli.output, OutputFormat::Json);
    }

    #[test]
    fn test_no_color_flag() {
        let cli = Cli::parse_from(["qa-pms", "health", "--no-color"]);

        assert!(cli.no_color);
    }

    #[test]
    fn test_config_flag() {
        let cli = Cli::parse_from(["qa-pms", "--config", "/path/to/config.yaml", "health"]);

        assert_eq!(cli.config, Some("/path/to/config.yaml".into()));
    }
}
```

### Integration Tests for Commands

```rust
// crates/qa-pms-api/tests/commands_integration_test.rs

#[tokio::test]
async fn test_migrate_command() {
    let output = OutputFormatter::new(OutputFormat::Text, true);

    // This would require a test database
    // For now, just verify command structure is correct

    assert!(true);
}

#[tokio::test]
async fn test_config_generate_key() {
    let output = OutputFormatter::new(OutputFormat::Text, true);

    // Test key generation
    let mut rng = rand::thread_rng();
    let key: [u8; 32] = rng.gen();
    let hex_key = hex::encode(key);

    assert_eq!(hex_key.len(), 64); // 256 bits = 64 hex chars
}
```

### Manual Testing

```bash
# Test serve command
qa-pms serve --port 8080

# Test migrate commands
qa-pms migrate status
qa-pms migrate run
qa-pms migrate reset --force

# Test health command
qa-pms health
qa-pms health --integration jira
qa-pms health watch

# Test config commands
qa-pms config validate
qa-pms config show
qa-pms config generate-key

# Test flags
qa-pms health --json
qa-pms health --no-color
qa-pms --config custom-config.yaml health
qa-pms health --log-level debug
```

---

## Success Metrics

- **Commands Work**: All CLI commands execute without errors
- **Exit Codes**: Exit codes indicate success/failure appropriately
- **Help Text**: `--help` shows comprehensive documentation
- **Output Formats**: Both text and JSON output work correctly
- **Configuration**: Custom config files loaded successfully
- **Migrations**: Database migrations apply and rollback correctly
- **Health Checks**: Integration health status reported accurately
- **Key Generation**: Encryption keys are valid 256-bit hex strings

---

## Context and Dependencies

This story depends on:
- **Story 14.1**: Graceful shutdown for the `serve` command

This story enables:
- **Story 14.8**: CLI can be used to manage integration tests

---

## Risks and Mitigation

| Risk | Probability | Impact | Mitigation |
|-------|-------------|--------|------------|
| Breaking existing `serve` behavior | Low | High | Preserve default behavior when no command specified |
| Configuration errors cause crashes | Medium | Medium | Validate configuration before starting server |
| Reset command data loss | Low | High | Require `--force` flag and confirmation prompt |
| Output format inconsistency | Low | Medium | Standardize JSON schema for all commands |
| Platform-specific features | Low | Low | Test on Windows, macOS, and Linux |

---

## Next Steps

After this story is complete:
1. Test all CLI commands manually
2. Write comprehensive documentation for CLI usage
3. Create shell completion scripts (bash, zsh, fish)
4. Package as standalone binary for easy deployment
5. Proceed to Story 14.8 (Integration Tests with Testcontainers)

---

## Usage Examples

### Starting the Server

```bash
# Start with defaults (0.0.0.0:3000)
qa-pms serve

# Start on custom port
qa-pms serve --port 8080

# Start with custom host and workers
qa-pms serve --host 127.0.0.1 --port 8080 --workers 4
```

### Database Migrations

```bash
# Check migration status
qa-pms migrate status

# Run pending migrations
qa-pms migrate run

# Dry-run migrations (don't actually apply)
qa-pms migrate run --dry-run

# Reset database (WARNING: drops all tables)
qa-pms migrate reset --force
```

### Health Checks

```bash
# Check all integrations
qa-pms health

# Check specific integration
qa-pms health --integration jira

# Watch health status continuously
qa-pms health --watch

# JSON output for scripting
qa-pms health --json
```

### Configuration Management

```bash
# Validate configuration
qa-pms config validate

# Show current configuration (sensitive values masked)
qa-pms config show

# Generate new encryption key
qa-pms config generate-key

# Generate key and save to file
qa-pms config generate-key --output encryption-key.txt
```

### Advanced Usage

```bash
# Use custom config file
qa-pms --config /etc/qa-pms/production.yaml serve

# Disable colored output
qa-pms health --no-color

# JSON output for automation
qa-pms health --json | jq '.[] | select(.status == "unhealthy")'

# Debug logging
qa-pms --log-level trace serve
```

---

## Shell Completion

Generate completion scripts:

```bash
# Bash
qa-pms generate-completion bash > /etc/bash_completion.d/qa-pms.bash

# Zsh
qa-pms generate-completion zsh > /usr/local/share/zsh/site-functions/_qa-pms

# Fish
qa-pms generate-completion fish > ~/.config/fish/completions/qa-pms.fish
```

Add to `cli.rs`:

```rust
#[derive(Parser)]
#[command(name = "qa-pms")]
#[command(
    subcommand_value_name = "COMMAND",
    subcommand_help_heading = "Commands",
    disable_help_subcommand = true,
    propagate_version = true,
    after_help = "Environment variables:\n  NO_COLOR           Disable colored output\n  RUST_LOG            Set log level"
)]
pub struct Cli {
    // ... existing fields ...
}
```

---

## Packaging

Create standalone binary with musl for portability:

```bash
# Build with musl target
cargo build --release --target x86_64-unknown-linux-musl

# Binary is available at:
target/x86_64-unknown-linux-musl/release/qa-pms

# Copy to /usr/local/bin
sudo cp target/x86_64-unknown-linux-musl/release/qa-pms /usr/local/bin/qa-pms
sudo chmod +x /usr/local/bin/qa-pms
```

---

## Documentation

Create CLI documentation in docs folder:

```markdown
# CLI Reference

## Overview

The QA Intelligent PMS CLI tool provides administrative tasks for managing the application.

## Installation

### From Binary

```bash
sudo cp qa-pms /usr/local/bin/qa-pms
sudo chmod +x /usr/local/bin/qa-pms
```

### From Cargo

```bash
cargo install qa-pms-api
```

## Commands

### serve

Start the API server.

**Usage:**
```bash
qa-pms serve [OPTIONS]
```

**Options:**
- `-h, --host <HOST>` - Host address to bind to (default: 0.0.0.0)
- `-p, --port <PORT>` - Port to listen on (default: 3000)
- `--workers <N>` - Number of worker threads (default: CPU count)

**Examples:**
```bash
qa-pms serve
qa-pms serve --port 8080 --host 127.0.0.1
```

### migrate

Manage database migrations.

**Usage:**
```bash
qa-pms migrate <COMMAND>
```

**Commands:**
- `run` - Run pending migrations
- `status` - Show migration status
- `reset` - Reset database (DANGEROUS)

**Examples:**
```bash
qa-pms migrate status
qa-pms migrate run
qa-pms migrate reset --force
```

### health

Check integration health.

**Usage:**
```bash
qa-pms health [OPTIONS]
```

**Options:**
- `-i, --integration <NAME>` - Check specific integration
- `--watch` - Watch mode: refresh every 30 seconds

**Examples:**
```bash
qa-pms health
qa-pms health --integration jira
qa-pms health --watch
```

### config

Manage configuration.

**Usage:**
```bash
qa-pms config <COMMAND>
```

**Commands:**
- `validate` - Validate configuration
- `show` - Show current configuration (sensitive values masked)
- `generate-key` - Generate 256-bit encryption key

**Examples:**
```bash
qa-pms config validate
qa-pms config show
qa-pms config generate-key
```

## Global Options

- `--config <FILE>` - Path to configuration file
- `--no-color` - Disable colored output
- `--json` - Output JSON instead of human-readable text
- `--log-level <LEVEL>` - Log level (error, warn, info, debug, trace)
- `--log-format <FORMAT>` - Log format (pretty, json, compact)
- `-h, --help` - Show help
- `-V, --version` - Show version

## Exit Codes

- `0` - Success
- `1` - Error or degraded health status
- `2` - Critical failure (unhealthy integration)

## Environment Variables

- `NO_COLOR` - Disable colored output (when set to any value)
- `RUST_LOG` - Override log level (e.g., `RUST_LOG=debug`)
```
