use chrono::{DateTime, Utc};
use serde::de::Error as DeError;
use serde::{Deserialize, Deserializer};

/// Deserializes a timestamp string into a chrono timezone UTC coded
/// data type. Initially, we attempt to deserialize assuming a RFC339
/// format timestamp. If that fails, then we attempt to parse the
/// timestamp as a RFC2822 compatible timestamp.
///
pub fn deserialize<'de, D: Deserializer<'de>>(deserializer: D) -> Result<DateTime<Utc>, D::Error> {
    let deserialize_result = Deserialize::deserialize(deserializer);
    if let Err(err) = deserialize_result {
        return Err(err);
    }

    let time = deserialize_result.unwrap();

    if let Ok(parsed) = DateTime::parse_from_rfc3339(time) {
        return Ok(parsed.with_timezone(&Utc));
    }

    if let Ok(parsed) = DateTime::parse_from_rfc2822(time) {
        return Ok(parsed.with_timezone(&Utc));
    }

    return Err(DeError::custom(format!(
        "Unable to parse timestamp [{}]",
        time
    )));
}
