use clap::Parser;
use rust_cli_template::*;

/// This is a sample program
#[derive(Parser, Debug)]
#[clap(author, version, about)]
struct Args {
    /// Name of the person to greet
    #[clap(short, long)]
    name: String,

    /// Number of times to greet
    #[clap(short, long, default_value_t = 1)]
    count: u32,

    /// Invert string
    #[clap(long)]
    invert: bool,
}

fn main() {
    let args = Args::parse();

    for _ in 0..args.count {
        hello(&args.name, args.invert);
    }
}
