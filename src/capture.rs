use mpsc::Sender;
use pcap::Device;
use std::sync::mpsc;

pub struct Capture {
    device: Device,
}

impl Capture {
    pub fn try_open(interface: &str) -> Result<Self, anyhow::Error> {
        let devices = Device::list()?;
        let device = devices
            .into_iter()
            .find(|device| &device.name == interface)
            .ok_or(anyhow::Error::msg(format!(
                "Interface {} not found",
                interface
            )))?;

        let capture = Capture { device };
        Ok(capture)
    }

    pub fn start(self, _receiver: Sender<String>) -> Result<(), anyhow::Error> {
        dbg!(&self.device);
        let mut cap = pcap::Capture::from_device(self.device)?
            .immediate_mode(true)
            .snaplen(100)
            .open()?;
        println!("cap open");
        while let Ok(_packet) = cap.next_packet() {
            println!("Got packet");
        }
        Ok(())
    }
}
