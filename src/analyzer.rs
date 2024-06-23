use crate::capture::PacketMeta;
use std::collections::HashMap;
use std::mem::swap;
use std::sync::mpsc::{Receiver, RecvTimeoutError, Sender};
use std::time::{Duration, Instant};

pub type FlowId = (String, Option<u16>, String, Option<u16>);

const EVALUATION_INTERVAL_MS: u64 = 1000;

pub fn run_blocking(
    rx: Receiver<PacketMeta>,
    tx: Sender<HashMap<FlowId, usize>>,
) -> Result<(), anyhow::Error> {
    let mut current_statistics = HashMap::new();
    let mut next_swap = Instant::now() + Duration::from_millis(EVALUATION_INTERVAL_MS);

    loop {
        let packet = rx.recv_timeout(Duration::from_millis(100));
        match packet {
            Ok(packet) => {
                let tuple = packet.flow_tuple();
                let size = packet.size();
                current_statistics
                    .entry(tuple)
                    .and_modify(|entry| *entry += size as usize)
                    .or_insert(size as usize);
            }
            Err(RecvTimeoutError::Timeout) => {}
            Err(e) => return Err(anyhow::Error::from(e)),
        }

        if Instant::now() > next_swap {
            let mut new_statistics = HashMap::with_capacity(current_statistics.len() * 2);
            swap(&mut current_statistics, &mut new_statistics);
            next_swap = Instant::now() + Duration::from_millis(EVALUATION_INTERVAL_MS);
            tx.send(new_statistics)?;
        }
    }
}
