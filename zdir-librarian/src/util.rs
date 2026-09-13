use std::{
    path::Path,
    time::{SystemTime, UNIX_EPOCH},
};

pub const SECOND: u64 = 1;
pub const MINUTE: u64 = 60 * SECOND;
pub const HOUR: u64 = 60 * MINUTE;
pub const DAY: u64 = 24 * HOUR;
pub const WEEK: u64 = 7 * DAY;
pub const MONTH: u64 = 30 * DAY;

pub struct Entry {
    pub path: String,
    pub frecency: f64,
    pub last_accessed: u64,
}

pub fn rank(entry: Entry) -> Entry {
    let mut entry = entry;

    // thanks to zoxide for this ranking method
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("You've travelled back to... before 1970? How do you even have a computer?")
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

pub fn matches(path: &str, args: Vec<&str>) -> bool {
    if args.is_empty() {
        return true;
    }

    let path_lower = path.to_lowercase();
    let terms: Vec<String> = args.iter().map(|s| s.to_lowercase()).collect();

    // ensure that terms are in order
    let mut cursor = 0usize;

    for term in &terms {
        let Some(relative_pos) = path_lower[cursor..].find(term) else {
            return false;
        };

        cursor += relative_pos + term.len();
    }

    // final term of query must match final directory in path
    let final_term = terms.last().unwrap();
    let final_term_component =
        final_term.rsplit('/').next().unwrap_or(final_term);

    let stored_final_component = Path::new(&path_lower)
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("");

    stored_final_component.starts_with(final_term_component)
}
