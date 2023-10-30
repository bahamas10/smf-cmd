//! Argument parsing logic (via `clap`) for smf.

use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[clap(author, version, about, verbatim_doc_comment, long_about = None)]
pub struct Args {
    /// Subcommand.
    #[clap(subcommand)]
    pub command: SubCommands,
}

#[derive(Debug, Subcommand)]
pub enum SubCommands {
    /// List services
    List(SubCommandList),

    /// Tail service logs
    #[clap(alias = "logs")]
    Log(SubCommandLog),

    /// Get service status
    Status(SubCommandStatus),

    /// Enable service(s).
    Enable { services: Vec<String> },

    /// Disable service(s).
    Disable { services: Vec<String> },
}

/// `smf list ...`
#[derive(Debug, Parser)]
pub struct SubCommandList {
    /// Show all services (including disabled)
    #[clap(short, long)]
    pub all: bool,

    /// Show only services with a contract
    #[clap(short, long)]
    pub contract: bool,

    /// Show `ptree` output for services with a contract
    #[clap(short, long)]
    pub tree: bool,

    /// String to filter services on
    pub filter: Option<String>,
}

/// `smf status ...`
#[derive(Debug, Parser)]
pub struct SubCommandStatus {
    /// Services to process
    #[clap(required = true)]
    pub services: Vec<String>,
}

/// `smf log ...`
#[derive(Debug, Parser)]
pub struct SubCommandLog {
    /// Follow the log file (passes `-F` to `tail`)
    #[clap(short, long)]
    pub follow: bool,

    /// Number of lines to view (passes `-n` to `tail`)
    #[clap(short, long)]
    pub number: Option<u32>,

    /// Services to process
    #[clap(required = true)]
    pub services: Vec<String>,
}

pub fn parse() -> Args {
    Args::parse()
}
