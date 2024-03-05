use std::{
    fmt,
    str::FromStr,
    time::{SystemTime, UNIX_EPOCH},
};

use chrono::DateTime;

#[derive(Debug, PartialEq)]
pub enum InputFormat {
    Json,
    Yaml,
    QueryString,
    UrlEncoded,
    UnixTimestamp,
    MillisTimestamp,
    ISO8601,
    RelativeTime,
    HumanTime,
}

// Error type for parsing InputFormat from a string
#[derive(Debug, Clone, PartialEq)]
pub struct ParseInputFormatError;

impl fmt::Display for ParseInputFormatError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Invalid input format")
    }
}

impl FromStr for InputFormat {
    type Err = ParseInputFormatError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "json" => Ok(InputFormat::Json),
            "yaml" => Ok(InputFormat::Yaml),
            "qs" => Ok(InputFormat::QueryString),
            "querystring" => Ok(InputFormat::QueryString),
            "urlencoded" => Ok(InputFormat::UrlEncoded),
            "unix" => Ok(InputFormat::UnixTimestamp),
            "millis" => Ok(InputFormat::MillisTimestamp),
            "iso8601" => Ok(InputFormat::ISO8601),
            "relative" => Ok(InputFormat::RelativeTime),
            "human" => Ok(InputFormat::HumanTime),
            _ => Err(ParseInputFormatError),
        }
    }
}

pub(crate) fn detect_format(input: &str) -> Option<InputFormat> {
    // TODO: can probably return the parsed value from here in addition to
    //       the format type to avoid parsing twice
    // check if timestamp (seconds or millis)
    if let Some(format) = check_timestamp(input) {
        return Some(format);
    }
    if let Some(format) = check_iso8601(input) {
        return Some(format);
    }
    None
}

fn check_timestamp(input: &str) -> Option<InputFormat> {
    // we could get seconds or milliseconds, and possibly milliseconds w/ a decimal
    match input.parse::<f64>() {
        Ok(number) => {
            // Current time in seconds since the epoch
            let now = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("Time went backwards")
                .as_secs_f64();

            // Define a reasonable range
            let start = 0.0;
            let end = now + 60.0 * 60.0 * 24.0 * 365.0 * 20.0; // 20 years

            let is_in_seconds_range = number > start && number < end;
            let is_in_milliseconds_range = number > start * 1000.0 && number < end * 1000.0;

            if is_in_seconds_range {
                Some(InputFormat::UnixTimestamp)
            } else if is_in_milliseconds_range {
                Some(InputFormat::MillisTimestamp)
            } else {
                None
            }
        }
        Err(_) => None,
    }
}

fn check_iso8601(input: &str) -> Option<InputFormat> {
    match DateTime::parse_from_rfc3339(input) {
        Ok(_) => Some(InputFormat::ISO8601),
        Err(_) => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input_format() {
        assert_eq!("json".parse(), Ok(InputFormat::Json));
        assert_eq!("yaml".parse(), Ok(InputFormat::Yaml));
        assert_eq!("qs".parse(), Ok(InputFormat::QueryString));
        assert_eq!("querystring".parse(), Ok(InputFormat::QueryString));
        assert_eq!("urlencoded".parse(), Ok(InputFormat::UrlEncoded));
        assert_eq!("unix".parse(), Ok(InputFormat::UnixTimestamp));
        assert_eq!("millis".parse(), Ok(InputFormat::MillisTimestamp));
        assert_eq!("iso8601".parse(), Ok(InputFormat::ISO8601));
        assert_eq!("relative".parse(), Ok(InputFormat::RelativeTime));
        assert_eq!("human".parse(), Ok(InputFormat::HumanTime));
    }

    #[test]
    fn test_detect_format() {
        assert_eq!(detect_format("1709522051125"), Some(InputFormat::MillisTimestamp), "Integer millis failed");
        assert_eq!(detect_format("1709522051125.123"), Some(InputFormat::MillisTimestamp), "Decimal millis failed");
        assert_eq!(detect_format("1709522051.125"), Some(InputFormat::UnixTimestamp), "Decimal seconds failed");
        assert_eq!(detect_format("1709522051"), Some(InputFormat::UnixTimestamp), "Integer seconds failed");
        assert_eq!(detect_format("2024-03-04T03:14:11.125Z"), Some(InputFormat::ISO8601));
    }
}
