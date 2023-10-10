use std::time::Duration;

pub mod color_aware_string;
pub mod smf;

/**
 * Convert a duration to a human-readable string like "5 minutes", "2 hours",
 * etc.
 *
 * # Example
 *
 * Duration for 5 seconds ago:
 *
 * ```
 * use std::time::Duration;
 * let dur = Duration::new(5, 0);
 * assert_eq!(relative_duration(&dur), "5 seconds".to_string());
 * ```
 */
pub fn relative_duration(t: &Duration) -> String {
    let secs = t.as_secs();

    let v = [
        (secs / 60 / 60 / 24 / 365, "year"),
        (secs / 60 / 60 / 24 / 30, "month"),
        (secs / 60 / 60 / 24 / 7, "week"),
        (secs / 60 / 60 / 24, "day"),
        (secs / 60 / 60, "hour"),
        (secs / 60, "minute"),
        (secs, "second"),
    ];

    let mut plural = "";
    for (num, name) in v {
        if num > 1 {
            plural = "s"
        }

        if num > 0 {
            return format!("{} {}{}", num, name, plural);
        }
    }

    String::from("0 seconds")
}
