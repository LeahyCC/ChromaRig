//! User settings, read from `<config dir>/ChromaRig/config.toml`.
//!
//! ```toml
//! [on]
//! color = "#ffffff"            # used by `chromarig on` for every device
//!
//! [on.devices]
//! steelseries-rival-650 = "#ff0000"   # per-device override, keyed by driver id
//! ```

use std::collections::HashMap;
use std::path::PathBuf;

use anyhow::{Context, Result};
use serde::Deserialize;

use crate::rgb::Rgb;

#[derive(Debug, Default, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Config {
    #[serde(default)]
    pub on: OnConfig,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OnConfig {
    #[serde(default = "default_on_color")]
    pub color: Rgb,
    #[serde(default)]
    pub devices: HashMap<String, Rgb>,
}

impl Default for OnConfig {
    fn default() -> Self {
        OnConfig { color: default_on_color(), devices: HashMap::new() }
    }
}

fn default_on_color() -> Rgb {
    Rgb::WHITE
}

impl Config {
    /// Where the config file lives, if the OS has a config folder.
    pub fn path() -> Option<PathBuf> {
        dirs::config_dir().map(|d| d.join("ChromaRig").join("config.toml"))
    }

    /// Load the config file. A missing file is not an error: defaults apply.
    pub fn load() -> Result<Config> {
        let Some(path) = Self::path() else { return Ok(Config::default()) };
        match std::fs::read_to_string(&path) {
            Ok(text) => Self::parse(&text).with_context(|| format!("reading {}", path.display())),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(Config::default()),
            Err(e) => Err(e).with_context(|| format!("reading {}", path.display())),
        }
    }

    pub fn parse(text: &str) -> Result<Config> {
        Ok(toml::from_str(text)?)
    }

    /// The colour `chromarig on` should use for one device.
    pub fn on_color(&self, device_id: &str) -> Rgb {
        self.on.devices.get(device_id).copied().unwrap_or(self.on.color)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_file_uses_white() {
        let c = Config::parse("").unwrap();
        assert_eq!(c.on_color("anything"), Rgb::WHITE);
    }

    #[test]
    fn device_override_wins() {
        let c = Config::parse(
            "[on]\ncolor = \"#000080\"\n[on.devices]\nsteelseries-rival-650 = \"#ff0000\"\n",
        )
        .unwrap();
        assert_eq!(c.on_color("steelseries-rival-650"), Rgb::new(255, 0, 0));
        assert_eq!(c.on_color("other"), Rgb::new(0, 0, 128));
    }

    #[test]
    fn typo_is_an_error() {
        assert!(Config::parse("[on]\ncolour = \"#ffffff\"\n").is_err());
    }
}
