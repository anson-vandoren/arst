use std::{
    fmt,
    str::FromStr,
    time::{SystemTime, UNIX_EPOCH},
};

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
    // check if timestamp
    if let Some(format) = check_timestamp(input) {
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
