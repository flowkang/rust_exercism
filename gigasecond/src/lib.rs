use std::ops::Add;
use time::{Duration, PrimitiveDateTime as DateTime};

/// Create a datetime from the given numeric point in time.
///
/// Panics if any field is invalid.

// Returns a DateTime one billion seconds after start.
pub fn after(start: DateTime) -> DateTime {
    start.add(Duration::seconds(1_000_000_000))
}
