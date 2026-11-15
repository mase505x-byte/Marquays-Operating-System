# Marquays Operating System

Marquays OS is an experimental Rust-based operating-system project exploring reconfigurable hardware and developer-defined Melted Fibre (`.mfb`) automators.

> **Status:** early prototype. The repository does not yet provide a complete, bootable operating system. Treat the kernel and `.mfb` files as work in progress.

## Repository contents

- `src/` — early Rust kernel and hardware bring-up experiments.
- `cask.mfb` — example Melted Fibre syntax, currently a design/prototype file.
- `fastfetch/` — Fastfetch configuration and the project ASCII logo.

## Fastfetch

Fastfetch is an optional system-information utility; it is not part of the operating system. From the repository root, run:

```sh
fastfetch --config fastfetch/config.jsonc
```

The config loads the logo from `fastfetch/logo.txt`. If your Fastfetch version does not support JSONC configuration, use the same settings in a config format supported by your installed version.

## Development

The kernel build setup is still being developed, and the repository currently lacks a complete build/boot workflow. Contributions that make the project buildable and document the intended bootloader, target hardware, and `.mfb` format are welcome.
