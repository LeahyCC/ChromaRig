//! Transports move bytes to hardware. Drivers only ever talk to a transport,
//! never to hidapi directly, so the same driver runs against real hardware,
//! a dry run, or a recorder in tests.

pub mod dry_run;
pub mod hid;
pub mod recording;

use anyhow::Result;

/// A USB HID device that accepts output reports.
pub trait HidTransport {
    /// Send one output report. The first byte is the report ID (0x00 when the
    /// device does not use numbered reports), matching hidapi's convention.
    fn write(&mut self, report: &[u8]) -> Result<()>;
}

impl HidTransport for Box<dyn HidTransport> {
    fn write(&mut self, report: &[u8]) -> Result<()> {
        (**self).write(report)
    }
}

/// Format bytes as space-separated hex, trimming trailing zero padding so
/// dry-run output stays readable.
pub fn hex(bytes: &[u8]) -> String {
    let end = bytes.iter().rposition(|b| *b != 0).map_or(1, |i| i + 1);
    let mut out: Vec<String> = bytes[..end].iter().map(|b| format!("{b:02x}")).collect();
    if end < bytes.len() {
        out.push(format!("(+{} zero bytes)", bytes.len() - end));
    }
    out.join(" ")
}
