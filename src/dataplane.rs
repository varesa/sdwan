use crate::analyzer::FlowId;
use std::collections::{HashMap, HashSet};
use std::mem::swap;
use std::sync::mpsc::Receiver;
use std::time::Duration;

const BIG_FLOW_THRESHOLD: usize = 10_000_000;

pub fn run_blocking(rx: Receiver<HashMap<FlowId, usize>>) -> Result<(), anyhow::Error> {
    let mut big_flows = HashSet::new();

    loop {
        let flow_states = rx.recv_timeout(Duration::from_secs(10))?;

        let mut new_big_flows = HashSet::with_capacity(big_flows.len() * 2);
        for (flow, size) in flow_states.into_iter() {
            if size > BIG_FLOW_THRESHOLD {
                new_big_flows.insert(flow);
            }
        }

        let added_flows = &new_big_flows - &big_flows;
        let removed_flows = &big_flows - &new_big_flows;
        if !added_flows.is_empty() {
            println!("Added big flows: {:?}", &added_flows);
        }
        if !removed_flows.is_empty() {
            println!("Removed big flows: {:?}", &removed_flows);
        }

        swap(&mut big_flows, &mut new_big_flows);
    }
}
