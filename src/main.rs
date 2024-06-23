use clap::Parser;
use std::sync::mpsc;
use std::time::Duration;

mod analyzer;
mod capture;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    interface: String,
}

fn main() -> Result<(), anyhow::Error> {
    let args = Args::parse();
    let (tx, rx) = mpsc::channel();

    let capture = capture::Capture::try_open(&args.interface)?;
    std::thread::spawn(|| {
        let res = capture.run_blocking(tx);
        eprintln!("Capture thread exited with: {:?}", res);
        std::process::exit(1);
    });

    let analyzer = analyzer::Analyzer::from_channel(rx);
    std::thread::spawn(|| {
        let res = analyzer.run_blocking();
        eprintln!("Analyzer thread exited with: {:?}", res);
        std::process::exit(1);
    });

    loop {
        std::thread::sleep(Duration::from_secs(1));
    }
}
