use chrono::{Local, LocalResult, TimeZone, Utc};
use chrono_humanize::{Accuracy, HumanTime, Tense};

#[derive(Debug, PartialEq, Eq)]
pub enum OutputFormat {
    UnixTimestamp,
    Local,
    ISO8601,
    LocalHuman,
    Relative,
}

pub(crate) fn output_from_unix_timestamp(
    timestamp: f64,
    include_opt: Option<Vec<OutputFormat>>,
    exclude_opt: Option<Vec<OutputFormat>>,
) {
    let seconds = timestamp.trunc() as i64;
    let nanos = (timestamp.fract() * 1_000_000_000.0) as u32;

    let datetime_utc_result = Utc.timestamp_opt(seconds, nanos);

    match datetime_utc_result {
        LocalResult::Single(datetime_utc) => {
            // Conversion is valid and unambiguous
            let include = include_opt.unwrap_or_default();
            let exclude = exclude_opt.unwrap_or_default();

            // Check each format to decide if it should be printed
            if (include.is_empty() && !exclude.contains(&OutputFormat::UnixTimestamp))
                || include.contains(&OutputFormat::UnixTimestamp)
            {
                // Unix timestamp with up to nanoseconds
                println!("Unix timestamp: {}", timestamp);
            }
            if (include.is_empty() && !exclude.contains(&OutputFormat::Local))
                || include.contains(&OutputFormat::Local)
            {
                println!("Local: {}", datetime_utc.with_timezone(&Local));
            }
            if (include.is_empty() && !exclude.contains(&OutputFormat::ISO8601))
                || include.contains(&OutputFormat::ISO8601)
            {
                // ISO8601 format
                let iso8601 = datetime_utc.to_rfc3339_opts(chrono::SecondsFormat::Millis, true);
                println!("ISO 8601: {}", iso8601);
            }
            if (include.is_empty() && !exclude.contains(&OutputFormat::LocalHuman))
                || include.contains(&OutputFormat::LocalHuman)
            {
                // Human readable in local timezone
                let humanized_local = datetime_utc
                    .with_timezone(&Local)
                    .format("%A, %B %e, %Y %l:%M %P")
                    .to_string();
                println!("Local human: {}", humanized_local);
            }
            if (include.is_empty() && !exclude.contains(&OutputFormat::Relative))
                || include.contains(&OutputFormat::Relative)
            {
                // Relative time
                let now = Utc::now();
                let duration_since = now.signed_duration_since(datetime_utc);
                let humanized_relative =
                    HumanTime::from(duration_since).to_text_en(Accuracy::Precise, Tense::Past);
                println!("Relative: {}", humanized_relative);
            }
        }
        _ => println!("Invalid timestamp"),
    }
}
