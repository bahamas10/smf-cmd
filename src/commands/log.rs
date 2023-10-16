//! `smf log ...`

use anyhow::{Context, Result};
use colored::*;
use exec::Command;
use smf::Query;

use crate::arguments::SubCommandLog;

pub fn run(cmd: SubCommandLog) -> Result<()> {
    // get log files for the service arguments
    let log_files: Vec<String> = Query::new()
        .get_log_files(&cmd.services)
        .with_context(|| {
            format!(
                "failed to get log files for service(s): {:?}",
                cmd.services
            )
        })?
        .map(|p| p.into_os_string().into_string().unwrap())
        .collect();

    // construct arguments for `tail`
    let mut args: Vec<String> = vec!["tail".into()];
    if cmd.follow {
        args.push("-F".into());
    }
    if let Some(number) = cmd.number {
        args.push("-n".into());
        args.push(number.to_string());
    }
    args.extend_from_slice(&log_files);

    for log_file in log_files {
        eprintln!("- {}", log_file.cyan());
    }

    // fork `tail`
    let err = Command::new(&args[0]).args(&args[1..]).exec();

    // exec failed if we are here
    Err(err).with_context(|| format!("failed to exec: {}", args.join(" ")))
}
