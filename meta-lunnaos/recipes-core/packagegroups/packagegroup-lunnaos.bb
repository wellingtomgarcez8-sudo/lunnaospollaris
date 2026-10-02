SUMMARY = "LunnaOS desktop package set"
LICENSE = "MIT"
inherit packagegroup
RDEPENDS:${PN} = "systemd dbus udev networkmanager bluez5 pipewire wireplumber mesa-megadriver mesa-vulkan-drivers wayland wayland-protocols gnome-shell gnome-session mutter gdm gsettings-desktop-schemas gnome-control-center gnome-settings-daemon gnome-software nautilus xdg-user-dirs xdg-utils flatpak rpm dnf packagekit packagegroup-lunnaos-shell packagegroup-lunnaos-platform"