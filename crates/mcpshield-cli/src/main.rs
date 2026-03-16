mod commands;

use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(
    name = "mcpshield",
    version,
    about = "AI Agent Security Platform — Runtime firewall for MCP servers",
    long_about = "MCPShield protects Claude Desktop, Cursor, and Claude Code from tool poisoning, \
                  credential exfiltration, and rug-pull attacks. Fully offline — no API keys required."
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,

    #[arg(long, global = true)]
    pub config: Option<PathBuf>,

    #[arg(long, short, global = true)]
    pub verbose: bool,

    #[arg(long, global = true)]
    pub json: bool,

    #[arg(long, global = true)]
    pub no_color: bool,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Scan MCP configurations for vulnerabilities
    Scan {
        path: Option<PathBuf>,
        #[arg(long, default_value = "low")]
        severity: String,
    },
    /// Start runtime proxy
    Proxy {
        #[arg(long)]
        block_writes: bool,
        #[arg(long)]
        server_cmd: Option<String>,
    },
    /// Manage tool pins
    Pin {
        #[command(subcommand)]
        action: PinAction,
    },
    /// View audit log
    Audit {
        #[arg(long)]
        last: Option<String>,
        #[arg(long)]
        severity: Option<String>,
    },
    /// Generate example config
    Init,
    /// Start honeypot MCP server
    Honeypot {
        #[command(subcommand)]
        action: HoneypotAction,
    },
    /// Launch web dashboard
    Dashboard {
        #[arg(long, default_value = "8080")]
        port: u16,
    },
    /// Query server trust registry
    Registry {
        #[command(subcommand)]
        action: RegistryAction,
    },
    /// Generate OWASP compliance report
    Report {
        #[arg(long, default_value = "text")]
        format: String,
    },
}

#[derive(Subcommand, Debug)]
pub enum PinAction {
    List,
    Verify,
    Reset,
}

#[derive(Subcommand, Debug)]
pub enum HoneypotAction {
    /// Start the honeypot server
    Start,
    /// View recent attacks
    Attacks {
        #[arg(long, default_value = "24h")]
        last: String,
    },
    /// Generate attack report
    Report {
        #[arg(long, default_value = "text")]
        format: String,
    },
}

#[derive(Subcommand, Debug)]
pub enum RegistryAction {
    /// Check a server's trust score
    Check { server_name: String },
    /// Search the registry
    Search { query: String },
}

fn main() {
    let cli = Cli::parse();

    if cli.no_color {
        colored::control::set_override(false);
    }

    // Initialize tracing
    let level = if cli.verbose {
        tracing::Level::DEBUG
    } else {
        tracing::Level::WARN
    };
    tracing_subscriber::fmt()
        .with_max_level(level)
        .with_target(false)
        .init();

    let config = mcpshield_core::config::Config::load(cli.config.as_deref());

    match cli.command {
        Commands::Scan { path, severity } => {
            commands::scan::run(&config, path, &severity, cli.verbose, cli.json);
        }
        Commands::Proxy {
            block_writes,
            server_cmd,
        } => {
            commands::proxy::run(&config, block_writes, server_cmd);
        }
        Commands::Pin { action } => {
            commands::pin::run(&config, action);
        }
        Commands::Audit { last, severity } => {
            commands::audit::run(&config, last, severity);
        }
        Commands::Init => {
            commands::init::run();
        }
        Commands::Honeypot { action } => {
            commands::honeypot::run(&config, action);
        }
        Commands::Dashboard { port } => {
            commands::dashboard::run(port);
        }
        Commands::Registry { action } => {
            commands::registry::run(&config, action);
        }
        Commands::Report { format } => {
            commands::report::run(&config, &format);
        }
    }
}
