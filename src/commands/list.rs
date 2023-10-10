use anyhow::{ensure, Context, Result};
use chrono::Utc;
use colored::*;
use libcontract::status::{ContractStatus, Detail};
use regex::Regex;
use smf::{Query, SmfState};

use crate::util;
use util::color_aware_string::ColorAwareString;
use util::smf::parse_smf_date;

pub fn run(all: bool) -> Result<()> {
    let q = Query::new();
    let mut svcs: Vec<_> = q.get_status_all().unwrap().collect();

    svcs.sort_by_key(|svc| svc.fmri.to_string());

    let now = Utc::now().naive_utc();

    println!();
    println!(
        "{}",
        format_output_line(&[
            "".bold().to_string(),
            "SERVICE".bold().to_string(),
            "CTID".bold().to_string(),
            "#PIDS".bold().to_string(),
            "TIME".bold().to_string(),
        ])
    );

    for svc in svcs {
        // skip legacy services
        if svc.state == SmfState::Legacy {
            continue;
        }

        if !all && svc.state == SmfState::Disabled {
            continue;
        }

        /*
        if let Some(f) = filter {
            if !svc.fmri.contains(f) {
                continue;
            }
        }
        */

        //println!("{} {:?}", svc.fmri, svc.description);
        let state = stylize_smf_state(&svc.state);
        let fmri = stylize_fmri(&svc.fmri)?;
        let ctid = stylize_contract_id(&svc.contract_id);
        let pids = stylize_pids(&svc.contract_id);
        let time = {
            let then = parse_smf_date(&now, &svc.service_time)?;
            let dur = (now - then).to_std()?;
            let s = util::relative_duration(&dur);

            match dur.as_secs() {
                n if n < 60 => s.to_string().red(),
                n if n < 24 * 60 * 60 => s.to_string().yellow(),
                _ => s.to_string().black().bold(),
            }
            .to_string()
        };

        println!("{}", format_output_line(&[state, fmri, ctid, pids, time]));
    }

    println!();

    Ok(())
}

/// Get a suitable char for the state (as a `String`).
fn stylize_smf_state(state: &SmfState) -> String {
    let s = match state {
        SmfState::Online => "✔".green(),
        SmfState::Disabled => "✖".black().bold(),
        SmfState::Degraded => "✖".red(),
        SmfState::Maintenance => "*".red().bold(),
        SmfState::Offline => "*".yellow(),
        SmfState::Legacy => "L".green(),
        SmfState::Uninitialized => "?".yellow(),
    };

    s.to_string()
}

fn stylize_contract_id(ctid: &Option<usize>) -> String {
    match ctid {
        Some(ctid) => ctid.to_string().magenta(),
        None => "-".yellow(),
    }
    .to_string()
}

fn stylize_pids(ctid: &Option<usize>) -> String {
    match ctid {
        Some(ctid) => {
            let ctid = *ctid as u32;
            let status = ContractStatus::new(ctid, Detail::All).unwrap();
            let members = status.get_members().unwrap();
            match members.len() {
                0 => "0".yellow(),
                n => n.to_string().green(),
            }
        }
        None => "-".yellow(),
    }
    .to_string()
}

/**
 * Style an FMRI
 *
 * Expects a string that looks like:
 *
 * <type>:/<name>:<instance>
 *
 * For example:
 *
 * svc:/milestone/single-user:default
 */
fn stylize_fmri(fmri: &str) -> Result<String> {
    let fmri_re = Regex::new(r"^([a-z]+):/(.*)/(.*):(.*)$").unwrap();

    let caps = fmri_re
        .captures(fmri)
        .with_context(|| format!("cannot parse fmri: {}", fmri))?;

    ensure!(caps.len() == 5, "invalid caps len");

    let mut out = format!(
        "{}{}{}{}{}",
        caps[1].cyan(),
        ":/".black().bold(),
        caps[2].black().bold(),
        ":".black().bold(),
        caps[3].green(),
    );

    match &caps[4] {
        "default" => (),
        inst => out = format!("{}:{}", out, inst.magenta()),
    };

    Ok(out)
}

fn format_output_line<T: AsRef<str>>(cols: &[T]) -> String {
    let data = [
        (cols[0].as_ref(), 1, "..."),
        (cols[1].as_ref(), 40, ""),
        (cols[2].as_ref(), 7, ""),
        (cols[3].as_ref(), 7, ""),
        (cols[4].as_ref(), 10, ""),
    ];

    let mut line = String::new();

    for (text, max, _suffix) in data {
        let cas = ColorAwareString::with_string(text.into());

        line.push(' ');
        let padded = cas.pad_end(max);
        line.push_str(&padded);
    }

    line
}
