# LunnaOS Polaris

LunnaOS Polaris is a Linux distribution built with the Linux kernel and Yocto Project/OpenEmbedded. The runtime is designed around a mutable x86_64 UEFI desktop with a LunnaOS visual layer on top of the GNOME/Wayland platform.

## Current release architecture

- Linux kernel + standard x86_64 hardware drivers/firmware.
- systemd, udev, D-Bus, NetworkManager, BlueZ and PipeWire.
- Wayland + Mutter + GNOME Shell as the graphics/session foundation.
- LunnaOS Shell extension providing the Polaris/Sel(l)enne visual identity.
- LunnaOS defaults, wallpaper, dock and branded application entry points.
- RPM runtime package management through DNF.
- Flatpak runtime application support with Flathub initialization.
- GNOME Software used as the package/app backend for both traditional packages and Flatpak.
- UEFI-first WIC image for generic x86-64 hardware.

The system is intentionally **mutable**: the installed OS keeps its RPM package database and DNF is available for runtime installation/update. Flatpak applications live independently from the base image.

## Application model

### RPM
Yocto builds RPM packages and a package index. The image is configured to use the LunnaOS RPM feed when that feed is published by CI. DNF is the runtime package manager.

### Flatpak
Flatpak is installed in the base image and the first-boot integration registers Flathub automatically. Lunna Store opens the graphical software center, which can expose Flatpak applications alongside traditional packages.

This separation is deliberate: base/system software is managed as RPM packages, while desktop applications can be installed as Flatpaks without rebuilding the OS.

## LunnaOS desktop

The LunnaOS visual layer includes:

- dark navy/violet Polaris palette;
- translucent rounded surfaces;
- customized top panel;
- floating dock;
- LunnaOS wallpaper/background;
- custom application launch entries;
- branded Settings, Files, Store and Games entry points;
- GNOME Shell extension enabled by default.

The project does **not** ship the unfinished Rust/Smithay compositor as the production session. That code remains an experimental future component; the release desktop uses the tested Mutter/Wayland stack so that the hardware/session layer is not replaced by an incomplete compositor.

## Build and release flow

1. KAS checks out Yocto/OpenEmbedded Scarthgap.
2. meta-lunnaos configures the distribution and desktop.
3. BitBake builds the mutable RPM image and WIC disk image.
4. CI runs repository validation.
5. CI generates the RPM package index.
6. CI publishes the WIC image and manifest as artifacts.
7. CI publishes the RPM feed to the dedicated rpm-repo branch.
8. Hardware/VM smoke tests are required before calling an image a production release.

## Important

Do not write an image directly to a physical laptop until the CI WIC image has passed a UEFI VM boot test and the hardware smoke-test checklist. Building successfully is not, by itself, proof that every laptop model is compatible.

## Local build

Requirements: Podman, Git and Just.

    just build

For the CI image:

    kas build kas/lunnaos.yml

## Status

The production architecture is now defined around a mutable Yocto image, LunnaOS visual shell integration, RPM/DNF and Flatpak. The remaining release gates are successful CI image generation, VM boot validation and physical hardware validation.

## License

LunnaOS-specific code is intended to be Apache-2.0 unless a component states otherwise. Upstream Yocto, OpenEmbedded, Linux, GNOME and other dependencies retain their respective licenses.
