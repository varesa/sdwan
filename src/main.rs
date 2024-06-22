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

    let (tx, rx) = mpsc::channel();

    std::thread::spawn(|| {
        let res = capture.start(tx);
        eprintln!("Capture thread exited with: {:?}", res);
        std::process::exit(1);
    });

    while let Ok(meta) = rx.recv() {
        println!("Received: {:?}", meta);
    }

    Ok(())
}
