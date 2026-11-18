use std::cell::RefCell;
use std::rc::Rc;

use anyhow::Result;

use super::HidTransport;

/// Shared log of every report written, in order.
pub type ReportLog = Rc<RefCell<Vec<Vec<u8>>>>;

/// Keeps every report in memory. Lets driver tests compare what a driver
/// sends against packets captured from real hardware, with no device plugged in.
pub struct RecordingTransport {
    log: ReportLog,
}

impl RecordingTransport {
    /// Returns the transport and a handle to read its log after the driver
    /// has taken ownership of the transport.
    pub fn new() -> (Self, ReportLog) {
        let log = ReportLog::default();
        (RecordingTransport { log: log.clone() }, log)
    }
}

impl HidTransport for RecordingTransport {
    fn write(&mut self, report: &[u8]) -> Result<()> {
        self.log.borrow_mut().push(report.to_vec());
        Ok(())
    }
}
