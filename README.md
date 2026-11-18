# ChromaRig

Turn every RGB light in your PC off with one command, and back on again.

```
chromarig off
chromarig on
chromarig status
```

ChromaRig talks to the hardware itself, with no vendor apps and no background service.

## Supported devices

| Device                                                             | Status  |
| ------------------------------------------------------------------ | ------- |
| SteelSeries Rival 650 (wired and wireless)                         | Works   |
| MSI Mystic Light motherboards (`0DB0:0076`) and their ARGB headers | Planned |
| ASUS GPUs (Aura over NVIDIA I2C)                                   | Planned |
| Kingston FURY DDR5                                                 | Planned |

Missing yours? See [CONTRIBUTING.md](CONTRIBUTING.md). Adding a device is one folder.

## Usage

- `chromarig off` sets every supported light to black.
- `chromarig on` sets them to the colour in your config (white by default).
- `chromarig status` lists the devices it found and any other lighting apps that are running.
- Add `--dry-run` to any command to print the bytes it would send without sending them.

If another lighting app (OpenRGB, iCUE, MSI Center, SteelSeries GG and so on) is running, it may undo what ChromaRig sets. `status` warns you about this.

## Config

Optional. On Windows it lives at `%APPDATA%\ChromaRig\config.toml`.

```toml
[on]
color = "#ffffff"

[on.devices]
steelseries-rival-650 = "#ff0000"
```

## Building

Needs Rust stable. On Windows you also need the Visual Studio C++ build tools and the Windows SDK. On Linux, install `libudev-dev`.

```
cargo build --release
```

On Windows, run cargo from PowerShell or cmd. Git Bash puts its own `link.exe` ahead of the Visual Studio linker and the build fails.

## Credits

Several drivers are ported from [OpenRGB](https://gitlab.com/CalcProgrammer1/OpenRGB). Each ported file names its source and author.

## License

GPL-2.0-only. See [LICENSE](LICENSE).
