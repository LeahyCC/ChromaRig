# Contributing

The most useful contribution is support for a new device. This guide walks through one.

## How the code fits together

```
main.rs        the commands (off, on, status)
devices.rs     finds plugged-in hardware that a driver supports
drivers/       one folder per device family
transport/     moves bytes: real HID, dry run, or a recorder for tests
```

A driver never talks to USB directly. It writes reports to a transport. That is what lets the same driver run against real hardware, print a dry run, or be tested with no device plugged in.

## Adding a device

1. **Find its IDs.** Note the USB vendor ID, product ID and interface number. On Windows, Device Manager shows them as `VID_xxxx&PID_xxxx&MI_xx`.
2. **Find its protocol.** If OpenRGB supports the device, port its controller. Our license (GPL-2.0-only) allows it. Keep the original author and source in the file header. Otherwise capture what the vendor app sends with Wireshark and USBPcap.
3. **Make the folder.** Copy `src/drivers/steelseries_rival_650/` to `src/drivers/<vendor>_<family>/`. Give it:
   - a `FAMILY` constant: a stable `id` (never rename one that has shipped, it is a config key), a display name, and its `HidMatch` entries
   - a type that implements `Driver::set_all`
4. **Register it.** Add `<your_module>::FAMILY` to `HID_FAMILIES` in `src/drivers/mod.rs`.
5. **Test the bytes.** Use `RecordingTransport` to check that `set_all` sends exactly the reports from your source or capture. See the Rival 650 tests.
6. **Dry run first.** Run `chromarig off --dry-run` on real hardware and compare the bytes against your capture before sending anything for real.
7. **Update the device table** in README.md.

## Safety

Some devices can be damaged by bad writes, RAM above all. For those:

- only write to addresses or devices you have positively identified
- port known-good code byte for byte, and do not "tidy" magic numbers
- never send a "save to device" command unless the user asked for it

## Checks

CI runs these, so run them before opening a PR:

```
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test
npx prettier --check .
```
