# LunnaOS Polaris architecture

## Principle

LunnaOS is a new bootable OS image, not a theme layered onto an installed desktop.

### Layers

1. UEFI boot and bootc image
2. Bazzite kernel, firmware, drivers and low-level enablement
3. Linux services: systemd, logind, PipeWire, WirePlumber, NetworkManager, BlueZ, UPower
4. Lunna Wayland compositor
5. LunnaOS Shell
6. LunnaOS applications

## Desktop rule

KDE Plasma and GNOME are not the LunnaOS desktop. We will not depend on either desktop shell for the final session.

During the foundation phase, the Bazzite base is kept intact so that the image can be validated safely. Desktop removal happens only after the Lunna compositor/session is bootable in a VM.

## Hardware rule

Do not replace the Bazzite kernel or driver stack unless a concrete hardware requirement demands it. The Lunna layer should communicate with Linux services rather than reimplementing hardware support.

## Build rule

All development artifacts are built as OCI/bootc images and ISO artifacts in GitHub Actions. Nothing in this repository is allowed to mutate the developer's host installation.

## Milestones

- Phase 0: immutable Bazzite-derived image and CI
- Phase 1: Lunna compositor + minimal Wayland session
- Phase 2: Shell, panel, dock and launcher
- Phase 3: Control Center, notifications and settings
- Phase 4: Files, Store and Task Manager
- Phase 5: login, Plymouth branding and installer polish
- Phase 6: VM smoke tests, ISO release and hardware validation
