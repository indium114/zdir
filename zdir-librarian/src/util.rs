use std::time::{SystemTime, UNIX_EPOCH};

pub const SECOND: u64 = 1;
pub const MINUTE: u64 = 60 * SECOND;
pub const HOUR: u64 = 60 * MINUTE;
pub const DAY: u64 = 24 * HOUR;
pub const WEEK: u64 = 7 * DAY;
pub const MONTH: u64 = 30 * DAY;

pub struct Entry {
    path: String,
    frecency: f64,
    last_accessed: u64,
}

pub fn rank(entry: Entry) -> Entry {
    let mut entry = entry;

    // thanks to zoxide for this ranking method
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("You've travelled back to... before 1969? How do you even have a computer?")
        .as_secs();

    let duration = now.saturating_sub(entry.last_accessed);
    if duration < HOUR {
        entry.frecency *= 4.0
    } else if duration < DAY {
        entry.frecency *= 2.0
    } else if duration < WEEK {
        entry.frecency *= 0.5
    } else {
        entry.frecency *= 0.25
    }

    entry
}
