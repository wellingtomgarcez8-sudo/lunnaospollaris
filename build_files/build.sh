#!/usr/bin/env bash
set -euo pipefail

cp -a /ctx/system_files/. /

# Bazzite remains the hardware and kernel foundation. LunnaOS replaces the
# desktop session rather than replacing the kernel/driver stack.
dnf5 install -y   cargo   rust   gcc   gcc-c++   pkg-config   gtk4-devel   gtk4-layer-shell-devel   libdrm   libinput   libseat   libxkbcommon   mesa-dri-drivers   mesa-libEGL   mesa-libGL   mesa-libgbm   pipewire   pipewire-pulseaudio   wireplumber   network-manager-applet   bluez   upower   labwc   labwc-session   foot

install -d -m 0755 /usr/share/lunnaos
cat > /usr/share/lunnaos/build-info <<EOF
NAME=LunnaOS Polaris
CODENAME=Sel(l)enne
VERSION=0.2.0
BASE=Bazzite stable
SHELL=LunnaOS Shell
WINDOW_MANAGER=labwc (bootstrap compositor)
KDE=not used as the LunnaOS session
GNOME=not used as the LunnaOS session
EOF

# Build only the Lunna Shell package. The native Smithay compositor is kept in
# the workspace for the next milestone and is intentionally not a dependency
# of this first bootable shell image.
cargo build --release --manifest-path /ctx/src/Cargo.toml -p lunna-shell
install -m 0755 /ctx/src/target/release/lunna-shell /usr/bin/lunna-shell

chmod 0755 /usr/bin/lunna-settings /usr/bin/lunna-files /usr/bin/lunna-store /usr/bin/lunna-games
chmod 0755 /etc/xdg/labwc/autostart

# Desktop/session metadata.
install -d -m 0755 /usr/share/lunnaos
touch /usr/share/lunnaos/polaris-foundation
