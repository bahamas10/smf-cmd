use anyhow::{ensure, Context, Result};
use chrono::{Datelike, Days, Months, NaiveDate, NaiveDateTime};

/**
 * Parse a date as seen by `svcs`.
 *
 * It's unfortunate this function exists - `svcs` should probably be updated to
 * output the time in epoch seconds if a machine-parseable option is given.
 *
 * But for now...
 *
 * from `svcs.c`:
 *
 * > Print time if started within the past 24 hours, print date
 * > if within the past 12 months or, finally, print year if
 * > start
 */
pub fn parse_smf_date(
    now: &NaiveDateTime,
    date: &str,
) -> Result<NaiveDateTime> {
    let d = if date.contains(':') {
        // less than 24 hours old
        let spl: Vec<_> = date.split(':').collect();
        ensure!(spl.len() == 3, "invalid smf date");

        let hours: u32 = spl[0]
            .parse()
            .with_context(|| format!("failed to parse hours from: {}", date))?;
        let minutes: u32 = spl[1].parse().with_context(|| {
            format!("failed to parse minutes from: {}", date)
        })?;
        let seconds: u32 = spl[2].parse().with_context(|| {
            format!("failed to parse seconds from: {}", date)
        })?;

        // use current date
        let mut d = now
            .date()
            .and_hms_opt(hours, minutes, seconds)
            .with_context(|| format!("failed to parse time: {}", date))?;

        // if the date is the future set it back 1 day
        if d > *now {
            let one_day = Days::new(1);
            d = d.checked_sub_days(one_day).with_context(|| {
                format!("failed to subtract a day from {}", d)
            })?;
        }

        d
    } else if date.contains('_') {
        // between 24 hours and 1 year
        let spl: Vec<_> = date.split('_').collect();
        ensure!(spl.len() == 2, "invalid smf date");

        // use current year
        let new_date_str = format!("{}-{}-{}", now.year(), spl[0], spl[1]);
        let mut d = NaiveDate::parse_from_str(&new_date_str, "%Y-%b-%d")?
            .and_hms_opt(0, 0, 0)
            .unwrap();

        // if the date is in the future set it back 1 year
        if d > *now {
            let one_year = Months::new(12);
            d = d.checked_sub_months(one_year).with_context(|| {
                format!("failed to subtract a year from {}", d)
            })?;
        }

        d
    } else {
        // over a year old
        let then_year: i32 = date
            .parse()
            .with_context(|| format!("failed to parse year from: {}", date))?;

        // select jan 1 of the year
        NaiveDate::from_ymd_opt(then_year, 1, 1)
            .with_context(|| {
                format!("parse_smf_date invalid year: {}", then_year)
            })?
            .and_hms_opt(0, 0, 0)
            .unwrap()
    };

    Ok(d)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_under_a_day() {
        let now = NaiveDate::from_ymd_opt(2023, 10, 9)
            .unwrap()
            .and_hms_opt(12, 30, 0)
            .unwrap();

        let dates = [
            // same day
            ("07:11:22", 1696835482),
            ("12:29:00", 1696854540),
            // day before
            ("12:31:00", 1696768260),
            ("22:00:05", 1696802405),
        ];

        for (smf_date, want_timestamp) in dates {
            let date = parse_smf_date(&now, smf_date).unwrap();
            assert_eq!(date.timestamp(), want_timestamp);
        }
    }

    #[test]
    fn test_over_a_day() {
        let now = NaiveDate::from_ymd_opt(2023, 10, 9)
            .unwrap()
            .and_hms_opt(0, 0, 0)
            .unwrap();

        let dates = [
            // same year
            ("Jan_1", 1672531200),
            ("Feb_27", 1677456000),
            ("Mar_4", 1677888000),
            ("Aug_9", 1691539200),
            ("Oct_8", 1696723200),
            // year before
            ("Oct_10", 1665360000),
            ("Oct_28", 1666915200),
            ("Dec_25", 1671926400),
        ];

        for (smf_date, want_timestamp) in dates {
            let date = parse_smf_date(&now, smf_date).unwrap();
            assert_eq!(date.timestamp(), want_timestamp);
        }
    }

    #[test]
    fn test_over_a_year() {
        let now = NaiveDate::from_ymd_opt(2023, 10, 9)
            .unwrap()
            .and_hms_opt(0, 0, 0)
            .unwrap();

        let dates = [("2021", 1609459200), ("2022", 1640995200)];

        for (smf_date, want_timestamp) in dates {
            let date = parse_smf_date(&now, smf_date).unwrap();
            assert_eq!(date.timestamp(), want_timestamp);
        }
    }

    #[test]
    fn test_invalid() {
        let now = NaiveDate::from_ymd_opt(2023, 10, 9)
            .unwrap()
            .and_hms_opt(0, 0, 0)
            .unwrap();

        let invalid = ["foo", "00:11:97", "Aug_58"];

        for s in invalid {
            let err = parse_smf_date(&now, s).unwrap_err();
            println!("{} = {:#?}", s, err);
        }
    }
}
