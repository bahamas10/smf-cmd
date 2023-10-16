//! `smf list ...`

use anyhow::Result;
use chrono::Utc;
use colored::*;
use libcontract::status::{ContractStatus, Detail};
use smf::{Query, SmfState};

use crate::util;
use util::color_aware_string::ColorAwareString;
use util::smf::{
    get_ptree_for_fmri, stylize_smf_date, stylize_smf_fmri,
    stylize_smf_state_small,
};

use crate::arguments::SubCommandList;

pub fn run(cmd: SubCommandList) -> Result<()> {
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

        if !cmd.all && svc.state == SmfState::Disabled {
            continue;
        }

        if cmd.contract && svc.contract_id.is_none() {
            continue;
        }

        if let Some(ref f) = cmd.filter {
            if !svc.fmri.contains(f) {
                continue;
            }
        }

        let state = stylize_smf_state_small(&svc.state);
        let fmri = stylize_smf_fmri(&svc.fmri)?;
        let ctid = stylize_contract_id(&svc.contract_id);
        let pids = stylize_pids(&svc.contract_id);
        let time = stylize_smf_date(&now, &svc.service_time)?;

        println!("{}", format_output_line(&[state, fmri, ctid, pids, time]));

        if cmd.tree && svc.contract_id.is_some() {
            let ptree = get_ptree_for_fmri(&svc.fmri)?;
            println!("\n{}\n", ptree.bold().black());
        }
    }

    println!();

    Ok(())
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
