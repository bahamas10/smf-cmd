//! `smf status ...`

use anyhow::Result;
use chrono::Utc;
//use libcontract::status::{ContractStatus, Detail};
//use smf::{Query, QuerySelection, SmfState};
use smf::{Query, QuerySelection};

/*
use crate::util;
use util::color_aware_string::ColorAwareString;
use util::smf::parse_smf_date;
*/

use crate::arguments::SubCommandStatus;

pub fn run(cmd: SubCommandStatus) -> Result<()> {
    let select = QuerySelection::ByPattern(&cmd.services);
    let q = Query::new();
    let svcs = q.get_status(select)?;

    let _now = Utc::now().naive_utc();

    for svc in svcs {
        let dependencies: Vec<_> = q.get_dependencies_of(&cmd.services)?.collect();
        let dependents: Vec<_> = q.get_dependents_of(&cmd.services)?.collect();
        let log_files: Vec<_> = q.get_log_files(&cmd.services)?.collect();

        println!("{}", svc.fmri);
        println!("dependencies ({})", dependencies.len());
        for dep in dependencies {
            println!(" - {}", dep.fmri);
        }

        println!("dependents ({})", dependents.len());
        for dep in dependents {
            println!(" - {}", dep.fmri);
        }

        println!("log files ({})", log_files.len());
        for log_file in log_files {
            let name = log_file.into_os_string().into_string().unwrap();
            println!(" - {}", name);
        }
    }

    println!();

    Ok(())
}
