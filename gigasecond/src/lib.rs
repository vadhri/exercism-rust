use chrono::offset::TimeZone;
use chrono::{DateTime, Utc};

// Returns a Utc DateTime one billion seconds after start.
pub fn after(start: DateTime<Utc>) -> DateTime<Utc> {
    Utc.timestamp(start.timestamp() + 1000000000, 0)
}
