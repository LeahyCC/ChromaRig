use anyhow::Result;

use super::{HidTransport, hex};

/// Prints every report instead of sending it. Used by `--dry-run`, and the
/// safe first step for any new driver.
pub struct DryRunTransport {
    label: String,
}

impl DryRunTransport {
    pub fn new(label: impl Into<String>) -> Self {
        DryRunTransport { label: label.into() }
    }
}

impl HidTransport for DryRunTransport {
    fn write(&mut self, report: &[u8]) -> Result<()> {
        println!("  [dry run] {}: {}", self.label, hex(report));
        Ok(())
    }
}
