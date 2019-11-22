use chrono::{DateTime, SecondsFormat, Utc};
use serde::de::Error as DeError;
use serde::{Deserialize, Deserializer, Serializer};
use std::error::Error as StdError;
use std::fmt;

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
    let parsed = parse_timestamp(time);

    match parsed {
        Ok(timestamp) => Ok(timestamp),
        Err(_) => Err(DeError::custom(format!(
            "Unable to parse timestamp [{}]",
            time
        ))),
    }
}

pub fn serialize<S>(date: &DateTime<Utc>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let json_date = date.to_rfc3339_opts(SecondsFormat::Millis, true);
    serializer.serialize_str(&json_date)
}

#[derive(Debug, Clone)]
struct TimeStampParseError {
    pub timestamp_input: String,
    pub rfc3339_parse_error: Option<String>,
    pub rfc2822_parse_error: Option<String>,
}

impl fmt::Display for TimeStampParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(error) = &self.rfc2822_parse_error {
            return write!(f, "RFC2822 Parse Error: {}", error);
        }
        if let Some(error) = &self.rfc3339_parse_error {
            return write!(f, "RFC3339 Parse Error: {}", error);
        }

        write!(f, "No errors")
    }
}

impl StdError for TimeStampParseError {
    fn description(&self) -> &str {
        // Note: inverted order compared to the assignment of errors
        if let Some(error) = &self.rfc2822_parse_error {
            return error.as_str();
        }
        if let Some(error) = &self.rfc3339_parse_error {
            return error.as_str();
        }

        "No error"
    }
}

fn parse_timestamp(time: &str) -> Result<DateTime<Utc>, TimeStampParseError> {
    let mut parse_error = TimeStampParseError {
        timestamp_input: time.to_string(),
        rfc3339_parse_error: None,
        rfc2822_parse_error: None,
    };

    match DateTime::parse_from_rfc3339(time) {
        Ok(parsed) => {
            return Ok(parsed.with_timezone(&Utc));
        }
        Err(error) => {
            parse_error.rfc3339_parse_error = Some(error.description().to_string());
        }
    }

    match DateTime::parse_from_rfc2822(time) {
        Ok(parsed) => {
            return Ok(parsed.with_timezone(&Utc));
        }
        Err(error) => {
            parse_error.rfc2822_parse_error = Some(error.description().to_string());
        }
    }

    Err(parse_error)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_timestamp_eq(
        expected_as_epoch: i64,
        parse_result: Result<DateTime<Utc>, TimeStampParseError>,
    ) {
        match parse_result {
            Ok(parsed) => {
                assert_eq!(parsed.timestamp_nanos(), expected_as_epoch);
            }
            Err(error) => {
                eprintln!(
                    "Error parsing [{}]: {}",
                    error.timestamp_input,
                    error.description()
                );
                panic!(error)
            }
        }
    }

    #[test]
    fn can_parse_rfc3339_utc_with_millis() {
        let expected: i64 = 0;
        let input = "1970-01-01T00:00:00.000Z";
        let parse_result = parse_timestamp(input);
        assert_timestamp_eq(expected, parse_result);
    }

    #[test]
    fn can_parse_rfc3339_utc_only_seconds() {
        let expected: i64 = 0;
        let input = "1970-01-01T00:00:00Z";
        let parse_result = parse_timestamp(input);
        assert_timestamp_eq(expected, parse_result);
    }

    #[test]
    fn cant_parse_rfc3339_pacific_time() {
        let expected: i64 = 1328741760000000000;
        let input = "2012-02-08T14:56:00.000-08:00";
        let parse_result = parse_timestamp(input);
        assert_timestamp_eq(expected, parse_result);
    }

    #[test]
    fn can_parse_rfc2822_gmt_only_seconds() {
        let expected: i64 = 0;
        let input = "Thu, 01 Jan 1970 00:00:00 GMT";
        let parse_result = parse_timestamp(input);
        assert_timestamp_eq(expected, parse_result);
    }
}
