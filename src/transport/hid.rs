use anyhow::{Context, Result};
use hidapi::HidDevice;

use super::HidTransport;

/// Real hardware, through hidapi.
pub struct HidapiTransport {
    device: HidDevice,
}

impl HidapiTransport {
    pub fn new(device: HidDevice) -> Self {
        HidapiTransport { device }
    }
}

impl HidTransport for HidapiTransport {
    fn write(&mut self, report: &[u8]) -> Result<()> {
        let sent = self.device.write(report).context("HID write failed")?;
        anyhow::ensure!(sent > 0, "HID write sent no bytes");
        Ok(())
    }
}
