#!/usr/bin/env bash
set -euo pipefail

# Copy static LunnaOS files into the image.
cp -a /ctx/system_files/. /

# Keep the first foundation build conservative.
# These are build/runtime primitives needed by the future Lunna compositor,
# session and desktop applications. We do NOT enable a new graphical session yet.
dnf5 install -y \
  dbus-tools \
  libdrm \
  libinput \
  libseat \
  libxkbcommon \
  mesa-dri-drivers \
  mesa-libEGL \
  mesa-libGL \
  mesa-libgbm \
  pipewire \
  pipewire-pulseaudio \
  wireplumber \
  network-manager-applet \
  bluez \
  upower

# Create a machine-readable build identity.
install -d -m 0755 /usr/share/lunnaos
cat > /usr/share/lunnaos/build-info <<EOF
NAME=LunnaOS Polaris
CODENAME=Sel(l)enne
VERSION=0.1.0
BASE=Bazzite stable
DESKTOP=LunnaOS Shell (under construction)
KDE=not selected as LunnaOS desktop
GNOME=not selected as LunnaOS desktop
EOF

# This marker lets future CI smoke tests verify that the image really is ours.
touch /usr/share/lunnaos/polaris-foundation
