//! Device drivers, one folder per device family.
//!
//! To add a device family:
//! 1. Create `src/drivers/<family>/mod.rs` with a `FAMILY` constant and a
//!    type that implements [`Driver`].
//! 2. Add it to [`HID_FAMILIES`] below.
//! 3. Add a test that checks the bytes it sends against a known-good capture.
//!
//! See CONTRIBUTING.md for the full guide.

pub mod steelseries_rival_650;

use anyhow::Result;

use crate::rgb::Rgb;
use crate::transport::HidTransport;

/// Something with lights that ChromaRig can set.
pub trait Driver {
    /// Set every LED on the device to one colour. Black means off.
    fn set_all(&mut self, color: Rgb) -> Result<()>;
}

/// Which USB HID interface a family answers on.
#[derive(Clone, Copy, Debug)]
pub struct HidMatch {
    pub vendor_id: u16,
    pub product_id: u16,
    /// USB interface number, or `None` to accept any.
    pub interface: Option<i32>,
}

impl HidMatch {
    pub fn matches(&self, vendor_id: u16, product_id: u16, interface: i32) -> bool {
        self.vendor_id == vendor_id
            && self.product_id == product_id
            && self.interface.is_none_or(|i| i == interface)
    }
}

/// The manifest for one device family: how to find it and how to drive it.
pub struct HidFamily {
    /// Stable id, used as the key in config.toml. Never rename one that shipped.
    pub id: &'static str,
    /// Human name shown in `chromarig status`.
    pub name: &'static str,
    pub matches: &'static [HidMatch],
    pub open: fn(Box<dyn HidTransport>) -> Box<dyn Driver>,
}

/// Every HID family ChromaRig knows about.
pub const HID_FAMILIES: &[HidFamily] = &[steelseries_rival_650::FAMILY];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn family_ids_are_unique() {
        let mut ids: Vec<_> = HID_FAMILIES.iter().map(|f| f.id).collect();
        ids.sort();
        ids.dedup();
        assert_eq!(ids.len(), HID_FAMILIES.len());
    }

    #[test]
    fn match_respects_interface() {
        let m = HidMatch { vendor_id: 1, product_id: 2, interface: Some(0) };
        assert!(m.matches(1, 2, 0));
        assert!(!m.matches(1, 2, 1));
        assert!(!m.matches(1, 3, 0));
        let any = HidMatch { interface: None, ..m };
        assert!(any.matches(1, 2, 5));
    }
}
