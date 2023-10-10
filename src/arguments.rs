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
    List {
        /// Show all services (including disabled)
        #[clap(short, long)]
        all: bool,
    },

    /// Tail service logs
    #[clap(alias = "logs")]
    Log {
        /// Follow the log file
        #[clap(short, long)]
        follow: bool,
    },

    /// Enable service(s).
    Enable { services: Vec<String> },

    /// Disable service(s).
    Disable { services: Vec<String> },
}

pub fn parse() -> Args {
    Args::parse()
}
