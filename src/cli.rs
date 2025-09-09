// Shared CLI definition for both runtime (src/main.rs) and build script (build.rs).
// Keep this file self-contained: only depend on `clap` and avoid referencing other crate-local types.

use clap::{Args, Parser, Subcommand, ValueEnum};

// Longer description used for the top-level man page section.
const LONG_ABOUT: &str = r#"
myapp is a tiny example CLI demonstrating auto-generated man pages with clap and clap_mangen.

It showcases:
    - Nested subcommands (e.g., `config get`, `config set`)
    - Rich help/usage text derived from a single source of truth
    - Build-time man page generation to `target/man/`

Top-level commands:
    - config: manage configuration values (get/set)
    - server: run a demo server (addr/port/verbosity)
    - remote: add or remove a remote by name
"#;

/// myapp — a tiny example CLI used to demonstrate auto-generated man pages.
#[derive(Debug, Parser)]
#[command(
    name = "myapp",
    about = "Example CLI with nested subcommands and man page generation",
    long_about = LONG_ABOUT,
    version
)]
pub struct Cli {
    /// Top-level subcommand to execute
    #[command(subcommand)]
    pub command: Commands,
}

/// All top-level subcommands.
#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Manage configuration values
    Config(ConfigCmd),

    /// Run the server
    Server(ServerCmd),

    /// Interact with remotes
    Remote(RemoteCmd),
}

/// `config` command with nested subcommands.
#[derive(Debug, Args)]
pub struct ConfigCmd {
    /// Action to perform on configuration (get/set)
    #[command(subcommand)]
    pub action: ConfigAction,
}

/// Second-level subcommands for `config`.
#[derive(Debug, Subcommand)]
pub enum ConfigAction {
    /// Get a configuration value
    Get(ConfigGet),

    /// Set a configuration value
    Set(ConfigSet),
}

/// Arguments for `config get`.
#[derive(Debug, Args)]
pub struct ConfigGet {
    /// Configuration key to read, e.g. "core.editor"
    pub key: String,

    /// Optional output format
    #[arg(long, value_enum, default_value_t = OutputFormat::Plain, help = "Output format for the value")]
    pub format: OutputFormat,
}

/// Arguments for `config set`.
#[derive(Debug, Args)]
pub struct ConfigSet {
    /// Configuration key to write, e.g. "core.editor"
    pub key: String,

    /// Value to assign to the key
    pub value: String,

    /// Write to the global scope instead of local
    #[arg(long, help = "Write to the global config scope")] 
    pub global: bool,
}

/// Output format for config get.
#[derive(Copy, Clone, Debug, Eq, PartialEq, ValueEnum)]
pub enum OutputFormat {
    /// Plain text output (default)
    Plain,
    /// JSON output
    Json,
}

/// Arguments for `server` command.
#[derive(Debug, Args)]
pub struct ServerCmd {
    /// Port to listen on
    #[arg(short, long, default_value_t = 8080, help = "Port to listen on")]
    pub port: u16,

    /// Bind address
    #[arg(long, default_value = "127.0.0.1", help = "Bind address")]
    pub addr: String,

    /// Increase output verbosity (-v, -vv)
    #[arg(short, long, action = clap::ArgAction::Count, help = "Increase verbosity (-v, -vv)")]
    pub verbose: u8,
}

/// Arguments for `remote` command.
#[derive(Debug, Args)]
pub struct RemoteCmd {
    /// Remote name
    pub name: String,

    /// Remote URL
    #[arg(long, help = "Remote URL (e.g., https://example.com/repo.git)")]
    pub url: Option<String>,

    /// Remove the remote instead of adding
    #[arg(long, help = "Remove the remote instead of adding")]
    pub remove: bool,
}
