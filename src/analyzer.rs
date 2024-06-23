use crate::capture::PacketMeta;
use std::collections::HashMap;
use std::mem::swap;
use std::sync::mpsc::Receiver;
use std::time::{Duration, Instant};

pub struct Analyzer {
    rx: Receiver<PacketMeta>,
}

impl Analyzer {
    pub fn from_channel(rx: Receiver<PacketMeta>) -> Self {
        Analyzer { rx }
    }

    pub fn run_blocking(self) -> Result<(), anyhow::Error> {
        let mut current_statistics = HashMap::new();
        let mut next_swap = Instant::now() + Duration::from_millis(100);

        while let Ok(packet) = self.rx.recv() {
            let tuple = packet.flow_tuple();
            let size = packet.size();
            current_statistics
                .entry(tuple)
                .and_modify(|entry| *entry += size)
                .or_insert(size);

            if Instant::now() > next_swap {
                let mut new_statistics = HashMap::with_capacity(current_statistics.len() * 2);
                swap(&mut current_statistics, &mut new_statistics);
                next_swap = Instant::now() + Duration::from_millis(100);
                println!("{:?}", new_statistics);
            }
        }
        Err(anyhow::format_err!(
            "Analyzer failed to read channel: {:?}",
            self.rx.recv()
        ))
    }
}
