# LunnaOS Polaris — Linux + GNOME architecture

LunnaOS is built from the upstream Linux kernel and a purpose-built Linux userspace. It does not use Bazzite, Nobara, Ubuntu, Fedora, Arch, or another distribution as its runtime base.

## Runtime
1. UEFI boot
2. Upstream Linux kernel
3. glibc, systemd, udev, dbus, util-linux and coreutils
4. Mesa, DRM/KMS, Wayland, PipeWire, NetworkManager and BlueZ
5. GNOME platform: Mutter, GNOME Shell, GTK and GNOME services
6. LunnaOS GNOME layer: Shell extension, GTK theme, icons, wallpaper, defaults and branded applications

## GNOME rule
GNOME remains the underlying desktop technology, but stock GNOME is not the LunnaOS visual identity.

LunnaOS modifies the Shell, top bar, floating bottom dock, launcher, Action Center, notifications, lock/login visuals, GTK3/GTK4 theme, icons, cursors, wallpaper, animations, blur/glow and rounded surfaces.

The Android-style full application grid is not the primary launcher. Search, pinned applications, categories and recent files are used instead.

## Hardware
Hardware support remains in the Linux kernel, modules, firmware and standard Linux graphics/audio/network stack. LunnaOS does not reimplement hardware drivers.

## Build
OpenEmbedded/Yocto builds the complete runtime image reproducibly. It is a build system, not the runtime base.

## Target
x86_64 UEFI, Wayland-first, GNOME/Mutter, Mesa, PipeWire, NetworkManager, BlueZ, systemd, Flatpak and Steam/Proton compatibility where packaging permits.
