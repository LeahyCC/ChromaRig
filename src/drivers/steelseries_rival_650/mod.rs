//! SteelSeries Rival 650 (wired and wireless).
//!
//! Ported from OpenRGB's SteelSeriesRivalController (B Horn, 2020,
//! GPL-2.0-or-later): https://gitlab.com/CalcProgrammer1/OpenRGB
//!
//! The mouse has 8 LEDs, addressed 0x10 to 0x17 (wheel, logo, then three
//! down each side). Setting one LED takes four 60-byte reports. The byte
//! values are copied from OpenRGB; their meaning is not documented, so do
//! not "tidy" them.
//!
//! We never send the save command (0x09), so the mouse's own stored
//! lighting comes back after it is power cycled.

use anyhow::Result;

use super::{Driver, HidFamily, HidMatch};
use crate::rgb::Rgb;
use crate::transport::HidTransport;

const STEELSERIES_VID: u16 = 0x1038;

pub const FAMILY: HidFamily = HidFamily {
    id: "steelseries-rival-650",
    name: "SteelSeries Rival 650",
    matches: &[
        // Wired
        HidMatch { vendor_id: STEELSERIES_VID, product_id: 0x172b, interface: Some(0) },
        // Wireless
        HidMatch { vendor_id: STEELSERIES_VID, product_id: 0x1726, interface: Some(0) },
    ],
    open: |transport| Box::new(Rival650::new(transport)),
};

const LEDS: std::ops::Range<u8> = 0x10..0x18;
const PAYLOAD_LEN: usize = 60;

pub struct Rival650<T: HidTransport> {
    transport: T,
}

impl<T: HidTransport> Rival650<T> {
    pub fn new(transport: T) -> Self {
        Rival650 { transport }
    }

    fn send(&mut self, payload: [u8; PAYLOAD_LEN]) -> Result<()> {
        // Report ID 0, then the payload.
        let mut report = [0u8; PAYLOAD_LEN + 1];
        report[1..].copy_from_slice(&payload);
        self.transport.write(&report)
    }

    fn set_led(&mut self, led: u8, c: Rgb) -> Result<()> {
        for payload in led_payloads(led, c) {
            self.send(payload)?;
        }
        Ok(())
    }
}

/// The four reports that set one LED, as OpenRGB's SetRival650Color sends them.
fn led_payloads(led: u8, c: Rgb) -> [[u8; PAYLOAD_LEN]; 4] {
    let mut colour = [0u8; PAYLOAD_LEN];
    for (i, v) in [
        (0x00, 0x03),
        (0x04, 0x30),
        (0x06, 0x10),
        (0x07, 0x27),
        (0x16, 0x01),
        (0x1e, 0x04),
        (0x1f, c.r),
        (0x20, c.g),
        (0x21, c.b),
        (0x22, 0xff),
        (0x27, 0xff),
        (0x29, 0x54),
        (0x2c, 0xff),
        (0x2d, 0x54),
        (0x2e, c.r),
        (0x2f, c.g),
        (0x30, c.b),
        (0x31, 0x56),
    ] {
        colour[i] = v;
    }

    let mut second = [0u8; PAYLOAD_LEN];
    second[0x00] = 0x03;
    second[0x02] = 0x30;
    second[0x04] = 0x2c;

    let mut select = [0u8; PAYLOAD_LEN];
    select[0x00] = 0x05;
    select[0x02] = led;
    select[0x03] = 0xff;
    select[0x08] = 0x5c;

    let mut commit = [0u8; PAYLOAD_LEN];
    commit[0x00] = 0x1c;
    commit[0x02] = 0x55;
    commit[0x04] = 0x46;

    [colour, second, select, commit]
}

impl<T: HidTransport> Driver for Rival650<T> {
    fn set_all(&mut self, color: Rgb) -> Result<()> {
        for led in LEDS {
            self.set_led(led, color)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::transport::recording::RecordingTransport;

    #[test]
    fn off_sends_four_reports_per_led() {
        let (t, log) = RecordingTransport::new();
        Rival650::new(t).set_all(Rgb::BLACK).unwrap();
        let log = log.borrow();
        assert_eq!(log.len(), 8 * 4);
        assert!(log.iter().all(|r| r.len() == 61 && r[0] == 0x00));
        // The select report names each LED in turn.
        let leds: Vec<u8> = log.iter().skip(2).step_by(4).map(|r| r[1 + 0x02]).collect();
        assert_eq!(leds, (0x10..0x18).collect::<Vec<u8>>());
    }

    #[test]
    fn first_led_matches_openrgb_bytes() {
        let (t, log) = RecordingTransport::new();
        Rival650::new(t).set_all(Rgb::new(0x11, 0x22, 0x33)).unwrap();
        let log = log.borrow();

        // Non-zero bytes of each report (offset into the 60-byte payload, value),
        // taken from OpenRGB's SetRival650Color.
        let expected: [&[(usize, u8)]; 4] = [
            &[
                (0x00, 0x03),
                (0x04, 0x30),
                (0x06, 0x10),
                (0x07, 0x27),
                (0x16, 0x01),
                (0x1e, 0x04),
                (0x1f, 0x11),
                (0x20, 0x22),
                (0x21, 0x33),
                (0x22, 0xff),
                (0x27, 0xff),
                (0x29, 0x54),
                (0x2c, 0xff),
                (0x2d, 0x54),
                (0x2e, 0x11),
                (0x2f, 0x22),
                (0x30, 0x33),
                (0x31, 0x56),
            ],
            &[(0x00, 0x03), (0x02, 0x30), (0x04, 0x2c)],
            &[(0x00, 0x05), (0x02, 0x10), (0x03, 0xff), (0x08, 0x5c)],
            &[(0x00, 0x1c), (0x02, 0x55), (0x04, 0x46)],
        ];
        for (report, want) in log.iter().zip(expected) {
            let got: Vec<(usize, u8)> = report[1..]
                .iter()
                .enumerate()
                .filter(|(_, b)| **b != 0)
                .map(|(i, b)| (i, *b))
                .collect();
            assert_eq!(got, want);
        }
    }
}
