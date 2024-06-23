use mpsc::Sender;
use std::net::{Ipv4Addr, Ipv6Addr};
use std::sync::mpsc;

use etherparse::{LaxNetSlice, LaxSlicedPacket, TransportSlice};
use pcap::{Device, Error};

pub struct Capture {
    device: Device,
}

#[derive(Debug)]
pub struct Ipv4Meta {
    source_address: Ipv4Addr,
    source_port: Option<u16>,
    destination_address: Ipv4Addr,
    destination_port: Option<u16>,
    length: u32,
}

#[derive(Debug)]
pub struct Ipv6Meta {
    source_address: Ipv6Addr,
    source_port: Option<u16>,
    destination_address: Ipv6Addr,
    destination_port: Option<u16>,
    length: u32,
}

#[derive(Debug)]
pub enum PacketMeta {
    Ipv4(Ipv4Meta),
    Ipv6(Ipv6Meta),
}

impl PacketMeta {
    pub fn flow_tuple(&self) -> (String, Option<u16>, String, Option<u16>) {
        match self {
            PacketMeta::Ipv4(ipv4) => (
                ipv4.source_address.to_string(),
                ipv4.source_port,
                ipv4.destination_address.to_string(),
                ipv4.destination_port,
            ),
            PacketMeta::Ipv6(ipv6) => (
                ipv6.source_address.to_string(),
                ipv6.source_port,
                ipv6.destination_address.to_string(),
                ipv6.destination_port,
            ),
        }
    }

    pub fn size(&self) -> u32 {
        match self {
            PacketMeta::Ipv4(ipv4) => ipv4.length,
            PacketMeta::Ipv6(ipv6) => ipv6.length,
        }
    }
}

impl Capture {
    pub fn try_open(interface: &str) -> Result<Self, anyhow::Error> {
        let devices = Device::list()?;
        let device = devices
            .into_iter()
            .find(|device| device.name == interface)
            .ok_or(anyhow::Error::msg(format!(
                "Interface {} not found",
                interface
            )))?;

        let capture = Capture { device };
        Ok(capture)
    }

    pub fn run_blocking(self, tx: Sender<PacketMeta>) -> Result<(), anyhow::Error> {
        let mut cap = pcap::Capture::from_device(self.device)?
            .immediate_mode(true)
            .snaplen(100)
            .open()?;

        cap.filter("inbound", false)?;
        loop {
            let packet = cap.next_packet();
            match packet {
                Ok(packet) => {
                    let parsed = LaxSlicedPacket::from_ethernet(packet.data)?;

                    let length = packet.header.len;

                    let (source_port, destination_port) = match parsed.transport {
                        Some(TransportSlice::Udp(udp)) => {
                            (Some(udp.source_port()), Some(udp.destination_port()))
                        }
                        Some(TransportSlice::Tcp(tcp)) => {
                            (Some(tcp.source_port()), Some(tcp.destination_port()))
                        }
                        _ => (None, None),
                    };

                    let meta = match parsed.net {
                        Some(LaxNetSlice::Ipv4(ipv4)) => Some(PacketMeta::Ipv4(Ipv4Meta {
                            source_address: ipv4.header().source_addr(),
                            source_port,
                            destination_address: ipv4.header().destination_addr(),
                            destination_port,
                            length,
                        })),
                        Some(LaxNetSlice::Ipv6(ipv6)) => Some(PacketMeta::Ipv6(Ipv6Meta {
                            source_address: ipv6.header().source_addr(),
                            source_port,
                            destination_address: ipv6.header().destination_addr(),
                            destination_port,
                            length,
                        })),
                        _ => None,
                    };

                    if let Some(meta) = meta {
                        tx.send(meta)?;
                    }
                }
                Err(Error::TimeoutExpired) => continue,
                Err(e) => return Err(anyhow::Error::from(e)),
            }
        }
    }
}
