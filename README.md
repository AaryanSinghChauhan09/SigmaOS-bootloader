# SigmaOS Bootloader

This crate exports UEFI and boot-configuration APIs, including GPT, command
line, initramfs, and device-tree models. The public module surface and parsing
models do not establish that this code can boot SigmaOS on physical hardware.
In particular, boot claims require reproducible firmware or emulator evidence,
validated image inputs, and recovery behavior for malformed or unsupported
media.

The crate depends on the sibling `SigmaOS-kernel` checkout through a local path.

## License

GPL-3.0-or-later. See [`LICENSE`](LICENSE) and [`Cargo.toml`](Cargo.toml).
