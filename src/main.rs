use clap::Parser;
use std::sync::mpsc;

mod capture;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    interface: String,
}

fn main() -> Result<(), anyhow::Error> {
    let args = Args::parse();
    let capture = capture::Capture::try_open(&args.interface)?;

    let (tx, _rx) = mpsc::channel();
    capture.start(tx)?;

    Ok(())
}
