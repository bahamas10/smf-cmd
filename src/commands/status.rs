//! `smf status ...`

use std::cmp;

use anyhow::{Context, Result};
use chrono::Utc;
use colored::*;
use indexmap::map::IndexMap;
use libcontract::status::{ContractStatus, Detail};
use smf::{Query, QuerySelection};

use crate::util;
use util::color_aware_string::ColorAwareString;
use util::smf::{
    stylize_smf_date, stylize_smf_fmri, stylize_smf_state_full,
    stylize_smf_state_small,
};

use crate::arguments::SubCommandStatus;

pub fn run(cmd: SubCommandStatus) -> Result<()> {
    let select = QuerySelection::ByPattern(&cmd.services);
    let q = Query::new();
    let svcs = q
        .get_status(select)
        .with_context(|| format!("failed to get_status: {:?}", cmd.services))?;

    let now = Utc::now().naive_utc();

    // loop each service to process
    for (i, svc) in svcs.enumerate() {
        if i > 0 {
            println!();
        }

        let mut map = IndexMap::new();
        let cur_svc = &[&svc.fmri];

        // gather service data
        let dependencies: Vec<_> = q
            .get_dependencies_of(cur_svc)
            .with_context(|| {
                format!("failed to get_dependencies_of: {}", &svc.fmri)
            })?
            .collect();
        let dependents: Vec<_> = q
            .get_dependents_of(cur_svc)
            .with_context(|| {
                format!("failed to get_dependents_of: {}", &svc.fmri)
            })?
            .collect();
        let log_files: Vec<_> = q
            .get_log_files(cur_svc)
            .with_context(|| format!("failed to get_log_files: {}", &svc.fmri))?
            .collect();

        // format and store output
        let fmri = stylize_smf_fmri(&svc.fmri)?;
        let state_full = stylize_smf_state_full(&svc.state);
        let state_small = stylize_smf_state_small(&svc.state);
        let time = stylize_smf_date(&now, &svc.service_time)?;

        map.insert("fmri", format!("{} {}", state_small, fmri));
        map.insert("state", state_full);
        map.insert("description", svc.description.unwrap_or_default());
        map.insert("time", time);
        map.insert("zone", svc.zone.green().to_string());

        // gather contract data (if applicable)
        if let Some(ctid) = svc.contract_id {
            let ctid = ctid as u32;
            map.insert("contract", ctid.to_string().magenta().to_string());

            if let Ok(status) = ContractStatus::new(ctid, Detail::All) {
                let members = status.get_members().unwrap_or_default();
                if !members.is_empty() {
                    let pids: Vec<_> = members
                        .iter()
                        .map(|x| x.to_string().cyan().to_string())
                        .collect();
                    map.insert("pids", pids.join(", "));
                }
            };
        }

        // format deps and other multi-line data
        {
            let mut s = vec![];
            s.push(dependencies.len().to_string().magenta().to_string());

            if cmd.long {
                for dep in dependencies {
                    let dep_fmri = stylize_smf_fmri(&dep.fmri)?;
                    let dep_state_small = stylize_smf_state_small(&dep.state);
                    s.push(format!("{} {}", dep_state_small, dep_fmri));
                }
            }
            map.insert("dependencies", s.join("\n"));
        }

        {
            let mut s = vec![];
            s.push(dependents.len().to_string().magenta().to_string());

            if cmd.long {
                for dep in dependents {
                    let dep_fmri = stylize_smf_fmri(&dep.fmri)?;
                    let dep_state_small = stylize_smf_state_small(&dep.state);
                    s.push(format!("{} {}", dep_state_small, dep_fmri));
                }
            }
            map.insert("dependents", s.join("\n"));
        }

        {
            let mut s = vec![];
            s.push(log_files.len().to_string().magenta().to_string());

            if cmd.long {
                for log_file in log_files {
                    let name = log_file.into_os_string().into_string().unwrap();
                    s.push(name.cyan().to_string());
                }
            }
            map.insert("log files", s.join("\n"));
        }

        // format the output for the current service and print it
        let s = format_status_map(&map);
        println!("{}", s);
    }

    Ok(())
}

fn format_status_map(map: &IndexMap<&str, String>) -> String {
    // figure out which key has the max length
    let max_key = map.keys().map(|k| k.len()).reduce(cmp::max).unwrap();

    // loop each map item
    let mut s = vec![];
    for (key, value) in map {
        // loop over values (possibly separated by newlines)
        for (i, line) in value.lines().enumerate() {
            let key: String = match i {
                0 => {
                    // we are printing the first value, format the key name
                    let key =
                        ColorAwareString::with_string(key.bold().to_string())
                            .pad_start(max_key);
                    format!("{}:", key)
                }
                _ => {
                    // we are printing a subsequent value, just pad with blank
                    // spaces
                    ColorAwareString::with_string("".into())
                        .pad_start(max_key + 1)
                }
            };

            s.push(format!("{} {}", key, line));
        }
    }
    s.join("\n")
}
