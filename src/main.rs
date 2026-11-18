use anyhow::Result;
use clap::{Parser, Subcommand};
use hidapi::HidApi;

use chromarig::config::Config;
use chromarig::rgb::Rgb;
use chromarig::{conflicts, devices};

#[derive(Parser)]
#[command(version, about = "Turn every RGB light in a PC off and on again")]
struct Cli {
    /// Print what would be sent instead of sending it.
    #[arg(long, global = true)]
    dry_run: bool,

    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Turn every supported light off.
    Off,
    /// Turn every supported light on, using the colours in config.toml.
    On,
    /// List supported devices and other lighting apps that are running.
    Status,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let api = HidApi::new()?;
    let found = devices::find(&api);

    match cli.command {
        Command::Status => status(&found),
        Command::Off => apply(&api, &found, cli.dry_run, |_| Rgb::BLACK),
        Command::On => {
            let config = Config::load()?;
            apply(&api, &found, cli.dry_run, |id| config.on_color(id))
        }
    }
}

/// Set every device, keep going past failures, and fail at the end if any did.
fn apply(
    api: &HidApi,
    found: &[devices::Found],
    dry_run: bool,
    color_for: impl Fn(&str) -> Rgb,
) -> Result<()> {
    warn_conflicts();
    if found.is_empty() {
        println!("No supported devices found.");
        return Ok(());
    }
    let mut failed = 0;
    for device in found {
        let color = color_for(device.family.id);
        let result = devices::open(api, device, dry_run).and_then(|mut d| d.set_all(color));
        match result {
            Ok(()) => println!("ok    {} -> {color}", device.family.name),
            Err(e) => {
                failed += 1;
                println!("FAIL  {}: {e:#}", device.family.name);
            }
        }
    }
    anyhow::ensure!(failed == 0, "{failed} device(s) failed");
    Ok(())
}

fn status(found: &[devices::Found]) -> Result<()> {
    println!("Devices:");
    if found.is_empty() {
        println!("  none supported found");
    }
    for d in found {
        println!("  {}  ({})", d.family.name, d.family.id);
    }
    match Config::path() {
        Some(p) if p.exists() => println!("Config: {}", p.display()),
        Some(p) => println!("Config: {} (not created, using defaults)", p.display()),
        None => println!("Config: no config folder on this OS, using defaults"),
    }
    warn_conflicts();
    Ok(())
}

fn warn_conflicts() {
    let apps = conflicts::running();
    if !apps.is_empty() {
        println!(
            "Warning: other lighting apps are running and may undo changes: {}",
            apps.join(", ")
        );
    }
}
