mod input_format;
mod output_format;
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
        let format = args.from
            .as_ref()
            .and_then(|f| f.parse().ok()) // Try to parse the --from flag
            .or_else(|| input_format::detect_format(&input)); // Try to detect the format

        match format {
            Some(InputFormat::UnixTimestamp) => {
                // Attempt to parse input as f64 for timestamp handling
                if let Ok(timestamp) = input.parse::<f64>() {
                    println!("Detected UNIX timestamp: {}", timestamp);
                    println!("------------------------");
                    output_format::output_from_unix_timestamp(timestamp);
                } else {
                    println!("Invalid timestamp");
                }
            },
            Some(InputFormat::MillisTimestamp) => {
                if let Ok(timestamp) = input.parse::<f64>() {
                    println!("Detected millisecond timestamp: {}", timestamp);
                    println!("------------------------");
                    output_format::output_from_unix_timestamp(timestamp / 1000.0);
                } else {
                    println!("Invalid timestamp");
                }
            },
            // TODO: handle other formats
            _ => println!("Unsupported format"),
        }
    } else {
        println!("No input data provided");
    }
}
