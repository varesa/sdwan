use clap::Parser;
use std::sync::mpsc;
use std::time::Duration;

mod analyzer;
mod capture;
mod dataplane;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    interface: String,
}

fn main() -> Result<(), anyhow::Error> {
    let args = Args::parse();
    let (capture_tx, capture_rx) = mpsc::channel();
    let (analysis_tx, analysis_rx) = mpsc::channel();

    let capture = capture::Capture::try_open(&args.interface)?;
    std::thread::spawn(|| {
        let res = capture.run_blocking(capture_tx);
        eprintln!("Capture thread exited with: {:?}", res);
        std::process::exit(1);
    });

    std::thread::spawn(|| {
        let res = analyzer::run_blocking(capture_rx, analysis_tx);
        eprintln!("Analyzer thread exited with: {:?}", res);
        std::process::exit(1);
    });

    std::thread::spawn(|| {
        let res = dataplane::run_blocking(analysis_rx);
        eprintln!("Dataplane thread exited with: {:?}", res);
        std::process::exit(1);
    });

    loop {
        std::thread::sleep(Duration::from_secs(1));
    }
}
