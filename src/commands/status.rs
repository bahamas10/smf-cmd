use anyhow::Result;
use chrono::Utc;
use itertools::izip;
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
    let statuses = Query::new().get_status(select)?;

    let dependencies = Query::new().get_dependencies_of(&cmd.services)?;
    println!("1");
    let dependents = Query::new().get_dependents_of(&cmd.services)?;
    println!("1");
    let log_files = Query::new().get_log_files(&cmd.services)?;
    println!("1");

    let _now = Utc::now().naive_utc();

    let iter = izip!(statuses, dependencies, dependents, log_files);
    for (status, dependency, dependent, log_file) in iter {
        /*
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
        */

        println!("status = {:#?}", status);
        println!("dependency = {:#?}", dependency);
        println!("dependent = {:#?}", dependent);
        println!("log_file = {:#?}", log_file);
        //println!("time = {}", time);
    }

    println!();

    Ok(())
}
