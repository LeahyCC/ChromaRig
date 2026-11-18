# ChromaRig foundations plan

Goal: one command (later a tray button and hotkey) that turns every light in a PC off, and back on the way it was. ChromaRig talks to the hardware itself, so OpenRGB can be uninstalled.

## What this PC has (pancakes, checked 2026-11-17)

| Light                | Part                                               | How it is reached                                 | Needs admin?                |
| -------------------- | -------------------------------------------------- | ------------------------------------------------- | --------------------------- |
| Mouse                | SteelSeries Rival 650                              | USB HID `1038:1726`                               | No                          |
| Motherboard and fans | MSI PRO B840-VC WIFI, Mystic Light USB `0DB0:0076` | USB HID                                           | No                          |
| GPU                  | ASUS TUF RTX 5090                                  | I2C through NVIDIA's driver (NVAPI), address 0x67 | No                          |
| RAM                  | 2x Kingston FURY DDR5 (KF556C36)                   | SMBus, addresses 0x61, 0x63                       | Yes, plus the PawnIO driver |

No USB fan hub was found, so the fans most likely plug into the motherboard ARGB headers and are reached through the Mystic Light controller. OpenRGB detects the other three but not this one.

## Why going fully native is doable

OpenRGB is huge because it does effects, colours and hundreds of devices. ChromaRig only needs **off and restore** for the devices people actually have. That is a few small writes per device family, and because we are GPL-2.0-only we can port OpenRGB's driver code for Kingston FURY, ASUS GPU and SteelSeries instead of reverse engineering them again.

**The risk is the RAM.** A bad SMBus write can break a module's lighting for good. So RAM goes last, is ported line for line from OpenRGB's known-good driver, only writes to addresses it has positively identified, and has a dry-run mode that prints writes instead of sending them.

## Shape

```
   chromarig off / on / status
              |
          core (detect, snapshot, off, restore)
              |
       Driver trait, one per device family
              |
   +----------+----------+-------------+
 transport: HID      transport: NVAPI   transport: SMBus
   |                    |                 (PawnIO, admin)
 SteelSeries Rival   ASUS Aura GPU        |
 MSI Mystic Light                        Kingston FURY DDR5
```

- **Transports** are how bytes reach hardware (HID, NVAPI I2C, SMBus). Written once, shared by every driver.
- **Drivers** are one folder per device family: a `FAMILY` constant (id, name, USB IDs) plus code against a transport. Adding a device means adding a folder and one line in `drivers/mod.rs`.
- **On colour:** `on` sets the colour from `config.toml` (white by default). Reading a device's state before `off` and restoring it is deferred until a device that can report its state is supported. The Rival 650 cannot.
- **Admin:** only SMBus needs it. Plan is a small elevated helper (or service) that the normal app talks to, so the tray app itself runs as a normal user.

## Lessons taken from existing projects

- **OpenRGB:** the driver code to port, and PawnIO as the signed SMBus driver on Windows (it replaced WinRing0, which is flagged as vulnerable).
- **liquidctl:** one driver per family matched by USB IDs, and tests that replay recorded packets so contributors without the hardware can run them.
- **OpenLinkHub / LumenForge:** any API stays local-only (127.0.0.1).

## Language

Rust. A single small binary with no runtime to install, the `hidapi` crate for USB, and a trait per driver keeps them separate.

## Foundation steps

Ordered safest first. OpenRGB stays installed as a reference until step 7 passes, then it can go.

1. **Done.** Repo setup: one Rust crate (split into a workspace only if needed), rustfmt, prettier for docs, CI on Windows and Linux, GPL-2.0-only, CONTRIBUTING.md.
2. **Done.** `Driver` and `HidTransport` traits, a recording transport for tests, a dry-run transport.
3. **Done.** CLI: `chromarig off`, `on`, `status`, `--dry-run`.
4. **Done.** Mouse: SteelSeries Rival 650 over HID.
5. Fans: MSI Mystic Light `0076` over HID. Start from the USB capture and dev branch in OpenRGB issues. Offer it upstream to OpenRGB too.
6. GPU: ASUS Aura over NVAPI I2C, ported from OpenRGB.
7. RAM: Kingston FURY DDR5 over PawnIO SMBus, ported from OpenRGB, dry run first, elevated helper.
8. Contributor kit: the "add a device" guide is in CONTRIBUTING.md. Still to do: a non-HID transport story (SMBus, NVAPI) in that guide once steps 6 and 7 land.
9. Later: tray icon, hotkey, schedule (off at night, on at login).

## Decisions (2026-11-17)

- **Scope:** every light, peripherals included (the mouse counts).
- **License:** GPL-2.0-only. OpenRGB files are GPL-2.0-or-later or GPL-2.0-only, and both can be ported into a GPL-2.0-only project.
- **No OpenRGB dependency:** ChromaRig drives hardware itself so OpenRGB can be uninstalled.
- **Fans:** the BIOS "EZ LED Control" switch has been unreliable alongside the installed lighting apps, so the native Mystic Light driver stays in.

## Conflicts with other lighting apps

Two apps writing to the same device fight, and the last write wins. `chromarig status` lists other lighting apps that are running (OpenRGB, MSI Center, iCUE, SignalRGB, Armoury Crate, SteelSeries GG) and warns when one may own a device we drive.
