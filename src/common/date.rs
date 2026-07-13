/// Microseconds since January 1, 1601 UTC
pub fn chromium_timestamp(timestamp: i64) -> Option<i64> {
    if timestamp < 0 {
        return None;
    }

    // Microseconds to seconds; Change epoch from Jan 1, 1601 -> Jan 1, 1970
    unix_timestamp(timestamp / 1_000_000 - 11_644_473_600)
}

/// Milliseconds since January 1, 1970 UTC
pub fn mozilla_timestamp(timestamp: i64) -> Option<i64> {
    // Milliseconds to seconds
    unix_timestamp(timestamp / 1_000)
}

fn unix_timestamp(timestamp: i64) -> Option<i64> {
    if timestamp < 0 {
        return None;
    }

    Some(timestamp)
}

#[cfg(target_os = "macos")]
pub fn safari_timestamp(timestamp: i64) -> Option<i64> {
    if timestamp < 0 {
        return None;
    }

    // Nanoseconds to seconds, change epoch from Jan 1, 2001 -> Jan 1, 1970
    Some(timestamp / 1_000_000_000 + 978_307_200)
}
