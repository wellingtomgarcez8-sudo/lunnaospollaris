# LunnaOS Polaris

LunnaOS Polaris is an independent Linux operating system image built from Bazzite as the low-level hardware/runtime base and a completely original LunnaOS graphical environment.

## Goals

- Bazzite kernel, firmware and hardware enablement.
- UEFI-first installable image.
- No KDE Plasma or GNOME as the LunnaOS desktop.
- Original LunnaOS Shell and compositor.
- Original applications: Files, Settings, App Center, Control Center, Notifications, Task Manager and more.
- Visual language based on the Sel(l)enne 1.0 references: deep navy, violet glow, translucent surfaces, rounded cards, moon/galaxy imagery.
- GitHub Actions builds only images/artifacts. Nothing in this repository modifies the developer's currently installed operating system.

## Architecture

```
UEFI
  -> Bazzite kernel + firmware + drivers
  -> systemd / PipeWire / NetworkManager / BlueZ
  -> Lunna Wayland compositor
  -> LunnaOS Shell
  -> Lunna applications
```

## Current stage

Phase 0 — repository and immutable-image foundation.

The first milestone is a reproducible Bazzite-derived container image. The Lunna compositor and shell will then be implemented incrementally and tested in virtual machines before an installable ISO is promoted.

## Important

Do not run `bootc switch` against a physical machine as part of development. The project is designed to build OCI/bootc artifacts first; installation onto a real PC happens only after the ISO has passed VM and hardware smoke tests.

## Build locally

Requirements: Podman, Git and Just.

```bash
just build
```

A later phase will add the UEFI ISO pipeline and VM boot tests.

## License

The LunnaOS-specific code in this repository is intended to be licensed under Apache-2.0 unless a component states otherwise. Upstream Bazzite, Fedora, Linux and other dependencies retain their respective licenses.
