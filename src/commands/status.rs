//! `smf status ...`

use anyhow::Result;
use chrono::Utc;
use colored::*;
use libcontract::status::{ContractStatus, Detail};
//use libcontract::status::{ContractStatus, Detail};
//use smf::{Query, QuerySelection, SmfState};
use smf::{Query, QuerySelection};

/*
use util::color_aware_string::ColorAwareString;
*/
use crate::util;
use util::smf::{
    stylize_smf_date, stylize_smf_fmri, stylize_smf_state_full,
    stylize_smf_state_small,
};

use crate::arguments::SubCommandStatus;

pub fn run(cmd: SubCommandStatus) -> Result<()> {
    let select = QuerySelection::ByPattern(&cmd.services);
    let q = Query::new();
    let svcs = q.get_status(select)?;

    let now = Utc::now().naive_utc();

    for svc in svcs {
        let cur_svc = &[&svc.fmri];
        let dependencies: Vec<_> = q.get_dependencies_of(cur_svc)?.collect();
        let dependents: Vec<_> = q.get_dependents_of(cur_svc)?.collect();
        let log_files: Vec<_> = q.get_log_files(cur_svc)?.collect();

        let fmri = stylize_smf_fmri(&svc.fmri)?;
        let state_full = stylize_smf_state_full(&svc.state);
        let state_small = stylize_smf_state_small(&svc.state);
        let time = stylize_smf_date(&now, &svc.service_time)?;

        println!("{}: {} {}", "        fmri".bold(), state_small, fmri);
        println!("{}: {}", "       state".bold(), state_full);
        println!(
            "{}: {}",
            " description".bold(),
            &svc.description.unwrap_or_default()
        );
        println!("{}: {}", "        time".bold(), time);
        println!("{}: {}", "        zone".bold(), &svc.zone.green());

        if let Some(ctid) = svc.contract_id {
            let ctid = ctid as u32;
            println!(
                "{}: {}",
                "    contract".bold(),
                ctid.to_string().magenta()
            );

            match ContractStatus::new(ctid, Detail::All) {
                Ok(status) => {
                    let members = status.get_members().unwrap_or_default();
                    if !members.is_empty() {
                        let pids: Vec<_> = members
                            .iter()
                            .map(|x| x.to_string().cyan().to_string())
                            .collect();
                        println!(
                            "{}: {}",
                            "        pids".bold(),
                            pids.join(", ")
                        );
                    }
                }
                Err(_) => (),
            };
        }

        println!(
            "{}: {}",
            "dependencies".bold(),
            dependencies.len().to_string().magenta()
        );
        for dep in dependencies {
            let dep_fmri = stylize_smf_fmri(&dep.fmri)?;
            let dep_state_small = stylize_smf_state_small(&dep.state);
            println!("              {} {}", dep_state_small, dep_fmri);
        }

        println!(
            "{}: {}",
            "  dependents".bold(),
            dependents.len().to_string().magenta()
        );
        for dep in dependents {
            let dep_fmri = stylize_smf_fmri(&dep.fmri)?;
            let dep_state_small = stylize_smf_state_small(&dep.state);
            println!("              {} {}", dep_state_small, dep_fmri);
        }

        println!(
            "{}: {}",
            "   log files".bold(),
            log_files.len().to_string().magenta()
        );
        for log_file in log_files {
            let name = log_file.into_os_string().into_string().unwrap();
            println!("              - {}", name.cyan());
        }

        println!();
    }

    Ok(())
}
