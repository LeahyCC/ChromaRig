//! Spot other lighting apps that may fight ChromaRig for the same device.
//! When two apps write to one device, the last write wins, so lights can
//! seem to ignore a command.

use sysinfo::{ProcessRefreshKind, ProcessesToUpdate, System};

/// Known lighting apps, by process name (lower case, no `.exe`).
/// These names come from each vendor's installs and may drift between
/// versions. Add to this list when you find one that is missing.
const KNOWN_APPS: &[(&str, &str)] = &[
    ("openrgb", "OpenRGB"),
    ("signalrgb", "SignalRGB"),
    ("icue", "Corsair iCUE"),
    ("armourycrate", "ASUS Armoury Crate"),
    ("lightingservice", "ASUS Aura (LightingService)"),
    ("msi.centralserver", "MSI Center"),
    ("lightkeeperservice", "MSI Mystic Light"),
    ("steelseriesgg", "SteelSeries GG"),
    ("steelseriesengine", "SteelSeries Engine"),
    ("fury ctrl", "Kingston FURY CTRL"),
    ("lconnect3", "Lian Li L-Connect 3"),
    ("nzxt cam", "NZXT CAM"),
];

/// Pure matcher, kept separate so it can be tested without real processes.
pub fn match_known(process_names: impl IntoIterator<Item = String>) -> Vec<&'static str> {
    let mut found: Vec<&'static str> = process_names
        .into_iter()
        .filter_map(|name| {
            let name = name.to_lowercase();
            let stem = name.strip_suffix(".exe").unwrap_or(&name);
            KNOWN_APPS.iter().find(|(p, _)| *p == stem).map(|(_, label)| *label)
        })
        .collect();
    found.sort_unstable();
    found.dedup();
    found
}

/// Lighting apps running right now.
/// Empty if the process list cannot be read: a missed warning is better
/// than refusing to change the lights.
pub fn running() -> Vec<&'static str> {
    let Ok(mut sys) = System::new() else { return Vec::new() };
    // Names are always read; skip the costly extras (CPU, memory, disk).
    sys.refresh_processes_specifics(ProcessesToUpdate::All, true, ProcessRefreshKind::nothing());
    match_known(sys.processes().values().map(|p| p.name().to_string_lossy().into_owned()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matches_case_and_exe_insensitively() {
        let names = ["OpenRGB.exe", "explorer.exe", "SteelSeriesGG.EXE", "openrgb"];
        let found = match_known(names.map(String::from));
        assert_eq!(found, vec!["OpenRGB", "SteelSeries GG"]);
    }
}
