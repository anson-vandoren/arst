mod input_format;
mod output_format;

use chrono::DateTime;
use clap::Parser;

use crate::input_format::InputFormat;

/// Do basic conversions
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// The format of the incoming data
    #[arg(long)]
    from: Option<String>,

    /// The format of the outgoing data
    #[arg(long)]
    to: Option<String>,

    /// The input data
    input: Option<String>,
}

fn main() {
    let args = Args::parse();

    // Check if input data is provided
    if let Some(input) = args.input {
        // Detect format if not specified, or use the specified format
        let format = args
            .from
            .as_ref()
            .and_then(|f| f.parse().ok()) // Try to parse the --from flag
            .or_else(|| input_format::detect_format(&input)); // Try to detect the format

        match format {
            Some(InputFormat::UnixTimestamp) => {
                // Attempt to parse input as f64 for timestamp handling
                if let Ok(timestamp) = input.parse::<f64>() {
                    println!("Detected UNIX timestamp: {}", timestamp);
                    println!("------------------------");
                    // output all formats except for the input format
                    output_format::output_from_unix_timestamp(
                        timestamp,
                        None,
                        Some(vec![output_format::OutputFormat::UnixTimestamp]),
                    );
                } else {
                    println!("Invalid timestamp");
                }
            }
            Some(InputFormat::MillisTimestamp) => {
                if let Ok(timestamp) = input.parse::<f64>() {
                    println!("Detected millisecond timestamp: {}", timestamp);
                    println!("------------------------");
                    output_format::output_from_unix_timestamp(
                        timestamp / 1000.0,
                        None,
                        Some(vec![output_format::OutputFormat::UnixTimestamp]),
                    );
                } else {
                    println!("Invalid timestamp");
                }
            }
            Some(InputFormat::ISO8601) => {
                if let Ok(datetime) = input.parse::<DateTime<chrono::Utc>>() {
                    println!("Detected ISO8601: {}", datetime);
                    println!("------------------------");
                    let timestamp = datetime.timestamp_millis();
                        output_format::output_from_unix_timestamp(
                            timestamp as f64 / 1_000.0,
                            None,
                            Some(vec![output_format::OutputFormat::ISO8601]),
                        );
                } else {
                    println!("Invalid ISO8601 datetime");
                }
            }
            // TODO: handle other formats
            _ => println!("Unsupported format"),
        }
    } else {
        println!("No input data provided");
    }
}
