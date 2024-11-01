//! Argument parsing logic (via `clap`) for smf.

use clap::{ArgEnum, Parser, Subcommand};

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

    /// Sort output based on the given fields
    #[clap(
        short,
        long,
        value_enum,
        value_delimiter = ',',
        default_value = "time,fmri"
    )]
    pub sort: Vec<ListSortItems>,

    /// String to filter services on
    pub filter: Option<String>,
}

/// `smf status ...`
#[derive(Debug, Parser)]
pub struct SubCommandStatus {
    /// Show more output (include multiline values)
    #[clap(short, long)]
    pub long: bool,

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

#[derive(Debug, Clone, ArgEnum)]
#[clap(rename_all = "lowercase")]
pub enum ListSortItems {
    Fmri,
    State,
    Time,
    Contract,
}

pub fn parse() -> Args {
    Args::parse()
}
