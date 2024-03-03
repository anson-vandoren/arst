use clap::Parser;

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
    println!("{:?}", args);
}
