//! Finds plugged-in devices that a driver family supports and opens them.

use std::ffi::CString;

use anyhow::{Context, Result};
use hidapi::HidApi;

use crate::drivers::{Driver, HID_FAMILIES, HidFamily};
use crate::transport::HidTransport;
use crate::transport::dry_run::DryRunTransport;
use crate::transport::hid::HidapiTransport;

/// A supported device that is plugged in.
pub struct Found {
    pub family: &'static HidFamily,
    pub path: CString,
}

/// List supported devices. One entry per family and product, because a
/// single USB interface can show up as several HID collections on Windows.
pub fn find(api: &HidApi) -> Vec<Found> {
    let mut found: Vec<Found> = Vec::new();
    let mut seen: Vec<(&str, u16)> = Vec::new();
    for info in api.device_list() {
        for family in HID_FAMILIES {
            let hit = family
                .matches
                .iter()
                .any(|m| m.matches(info.vendor_id(), info.product_id(), info.interface_number()));
            let key = (family.id, info.product_id());
            if hit && !seen.contains(&key) {
                seen.push(key);
                found.push(Found { family, path: info.path().to_owned() });
            }
        }
    }
    found
}

/// Open a found device, either for real or as a dry run.
pub fn open(api: &HidApi, device: &Found, dry_run: bool) -> Result<Box<dyn Driver>> {
    let transport: Box<dyn HidTransport> = if dry_run {
        Box::new(DryRunTransport::new(device.family.name))
    } else {
        let hid = api
            .open_path(&device.path)
            .with_context(|| format!("opening {}", device.family.name))?;
        Box::new(HidapiTransport::new(hid))
    };
    Ok((device.family.open)(transport))
}
