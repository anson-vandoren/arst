use chrono::{Local, LocalResult, TimeZone, Utc};
use chrono_humanize::{HumanTime, Accuracy, Tense};

pub enum OutputFormat {
    UnixTimestamp,
    MillisTimestamp,
    ISO8601,
    RelativeTime,
    HumanTime,
}

pub(crate) fn output_from_unix_timestamp(timestamp: f64) {
    let seconds = timestamp.trunc() as i64;
    let nanos = (timestamp.fract() * 1_000_000_000.0) as u32;

    let datetime_utc_result = Utc.timestamp_opt(seconds, nanos);

    match datetime_utc_result {
        LocalResult::Single(datetime_utc) => {
            // Conversion is valid and unambiguous

            // DateTime in local timezone
            let datetime_local = datetime_utc.with_timezone(&Local);

            // ISO8601 format
            let iso8601 = datetime_utc.to_rfc3339_opts(chrono::SecondsFormat::Millis, true);

            // Relative time
            let now = Utc::now();
            let duration_since = now.signed_duration_since(datetime_utc);
            let humanized_relative = HumanTime::from(duration_since).to_text_en(Accuracy::Precise, Tense::Past);

            // Human readable in local timezone
            let humanized_local = datetime_local.format("%A, %B %e, %Y %l:%M %P").to_string();

            println!("Unix Timestamp: {}", timestamp);
            println!("Local: {}", datetime_local);
            println!("ISO 8601: {}", iso8601);
            println!("Local human: {}", humanized_local);
            println!("Relative: {}", humanized_relative);
        },
        _ => println!("Invalid timestamp")
    }
}