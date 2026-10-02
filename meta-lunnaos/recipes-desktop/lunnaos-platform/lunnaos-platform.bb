SUMMARY = "LunnaOS runtime integration and application launchers"
LICENSE = "MIT"
inherit allarch systemd

RDEPENDS:${PN} = "bash gnome-control-center nautilus gnome-software flatpak dnf packagekit"

SYSTEMD_SERVICE:${PN} = "lunnaos-firstboot.service"
SYSTEMD_AUTO_ENABLE = "enable"

do_install() {
    install -d ${D}/usr/bin
    install -d ${D}/usr/share/applications
    install -d ${D}/usr/lib/systemd/system
    install -d ${D}/var/lib/lunnaos

    cat > ${D}/usr/bin/lunna-settings <<'EOF'
#!/bin/sh
exec gnome-control-center "$@"
EOF
    cat > ${D}/usr/bin/lunna-files <<'EOF'
#!/bin/sh
exec nautilus --new-window "$@"
EOF
    cat > ${D}/usr/bin/lunna-store <<'EOF'
#!/bin/sh
exec gnome-software "$@"
EOF
    cat > ${D}/usr/bin/lunna-games <<'EOF'
#!/bin/sh
exec gnome-software --search=games "$@"
EOF
    chmod 0755 ${D}/usr/bin/lunna-*

    cat > ${D}/usr/share/applications/lunna-settings.desktop <<'EOF'
[Desktop Entry]
Name=LunnaOS Settings
Comment=Configure LunnaOS
Exec=lunna-settings
Icon=preferences-system
Terminal=false
Type=Application
Categories=Settings;System;
EOF
    cat > ${D}/usr/share/applications/lunna-files.desktop <<'EOF'
[Desktop Entry]
Name=Lunna Files
Comment=Browse your files
Exec=lunna-files
Icon=system-file-manager
Terminal=false
Type=Application
Categories=Utility;FileManager;
EOF
    cat > ${D}/usr/share/applications/lunna-store.desktop <<'EOF'
[Desktop Entry]
Name=Lunna Store
Comment=Install RPM and Flatpak applications
Exec=lunna-store
Icon=gnome-software
Terminal=false
Type=Application
Categories=System;PackageManager;
EOF
    cat > ${D}/usr/share/applications/lunna-games.desktop <<'EOF'
[Desktop Entry]
Name=Lunna Games
Comment=Discover and install games
Exec=lunna-games
Icon=applications-games
Terminal=false
Type=Application
Categories=Game;
EOF

    cat > ${D}/usr/lib/systemd/system/lunnaos-firstboot.service <<'EOF'
[Unit]
Description=LunnaOS package and Flatpak repository initialization
After=network-online.target
Wants=network-online.target
ConditionPathExists=!/var/lib/lunnaos/repositories-initialized

[Service]
Type=oneshot
ExecStart=/bin/sh -c 'flatpak remote-add --if-not-exists flathub https://dl.flathub.org/repo/flathub.flatpakrepo && dnf makecache || true'
ExecStart=/bin/sh -c 'touch /var/lib/lunnaos/repositories-initialized'
RemainAfterExit=yes

[Install]
WantedBy=multi-user.target
EOF
}

FILES:${PN} += "/usr/bin/lunna-* /usr/share/applications /usr/lib/systemd/system/lunnaos-firstboot.service /var/lib/lunnaos"
