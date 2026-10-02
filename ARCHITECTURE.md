# LunnaOS Polaris architecture

## Runtime stack

1. UEFI firmware
2. Linux kernel and hardware drivers
3. glibc, systemd, udev, D-Bus and core utilities
4. Mesa/DRM/KMS, Wayland, PipeWire, NetworkManager and BlueZ
5. Mutter + GNOME Shell + GNOME services
6. LunnaOS Shell extension and Polaris visual defaults
7. LunnaOS application entry points and software center

The GNOME/Mutter layer is an implementation foundation, not the LunnaOS product identity. LunnaOS owns the visual layer and application integration.

## Mutable package model

The image uses the Yocto RPM backend with the package-management image feature. This keeps the target package database and provides DNF for runtime package installation and updates.

RPM packages are produced under tmp/deploy/rpm and indexed with the Yocto package-index task. The target is preconfigured for the LunnaOS package feed.

Flatpak is installed separately and Flathub is initialized on first boot. This lets desktop applications be added without changing the base system.

## Desktop

The LunnaOS Shell is currently implemented as a GNOME Shell extension because that path reuses the mature Mutter/Wayland hardware and session stack. It provides the Polaris top panel, floating dock, launcher integration, branding and visual styling.

The Rust/Smithay compositor in src/lunna-compositor is deliberately not part of the production image yet. It remains an experimental future replacement and must not block the stable desktop.

## Hardware target

The image targets generic x86-64 UEFI hardware. The Linux kernel, Mesa, firmware and standard Linux services remain responsible for hardware support; LunnaOS does not reimplement drivers.

## Build

KAS provides reproducible layer configuration. BitBake builds the root filesystem and WIC disk image. CI additionally creates the RPM package index and publishes the generated RPM feed.

## Release safety

A successful BitBake build is necessary but not sufficient for a production release. Every candidate must pass:

- repository validation;
- WIC artifact validation;
- UEFI VM boot;
- login/session startup;
- network, audio and graphics smoke tests;
- DNF install/update test;
- Flatpak install/run test;
- shutdown/reboot test;
- physical laptop smoke test.
