//! A plain 24-bit colour, parsed from and written as `#rrggbb`.

use std::fmt;
use std::str::FromStr;

use serde::{Deserialize, Deserializer};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Rgb {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Rgb {
    pub const BLACK: Rgb = Rgb::new(0, 0, 0);
    pub const WHITE: Rgb = Rgb::new(255, 255, 255);

    pub const fn new(r: u8, g: u8, b: u8) -> Self {
        Rgb { r, g, b }
    }
}

impl FromStr for Rgb {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let hex = s.strip_prefix('#').unwrap_or(s);
        if hex.len() != 6 || !hex.chars().all(|c| c.is_ascii_hexdigit()) {
            return Err(format!("`{s}` is not a colour like #ff8800"));
        }
        let byte = |i: usize| u8::from_str_radix(&hex[i..i + 2], 16).expect("checked above");
        Ok(Rgb::new(byte(0), byte(2), byte(4)))
    }
}

impl fmt::Display for Rgb {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "#{:02x}{:02x}{:02x}", self.r, self.g, self.b)
    }
}

impl<'de> Deserialize<'de> for Rgb {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let s = String::deserialize(d)?;
        s.parse().map_err(serde::de::Error::custom)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_with_and_without_hash() {
        assert_eq!("#ff8800".parse(), Ok(Rgb::new(255, 136, 0)));
        assert_eq!("FF8800".parse(), Ok(Rgb::new(255, 136, 0)));
    }

    #[test]
    fn rejects_bad_input() {
        assert!("#ff88".parse::<Rgb>().is_err());
        assert!("#gg0000".parse::<Rgb>().is_err());
    }

    #[test]
    fn round_trips_through_display() {
        assert_eq!(Rgb::new(1, 2, 255).to_string(), "#0102ff");
    }
}
