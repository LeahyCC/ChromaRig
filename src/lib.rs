//! ChromaRig: turn every light in a PC off, and back on.
//!
//! Layers, top to bottom:
//! - `devices`: find supported hardware and open it
//! - `drivers`: one folder per device family, speaks that device's protocol
//! - `transport`: moves bytes (real HID, dry run, or a recorder for tests)

pub mod config;
pub mod conflicts;
pub mod devices;
pub mod drivers;
pub mod rgb;
pub mod transport;
