SUMMARY = "LunnaOS desktop defaults and branding"
LICENSE = "MIT"
SRC_URI = "file://lunnaos.gschema.override file://lunnaos.desktop file://lunnaos-background.xml file://lunnaos.css"
S = "${WORKDIR}"
inherit allarch

do_install() {
    install -d ${D}/usr/share/glib-2.0/schemas
    install -m 0644 ${WORKDIR}/lunnaos.gschema.override ${D}/usr/share/glib-2.0/schemas/
    install -d ${D}/usr/share/backgrounds/lunnaos
    install -m 0644 ${WORKDIR}/lunnaos-background.xml ${D}/usr/share/backgrounds/lunnaos/
    install -d ${D}/usr/share/gnome-shell/theme
    install -m 0644 ${WORKDIR}/lunnaos.css ${D}/usr/share/gnome-shell/theme/
    install -d ${D}/usr/share/wayland-sessions
    install -m 0644 ${WORKDIR}/lunnaos.desktop ${D}/usr/share/wayland-sessions/
}

FILES:${PN} += "/usr/share/glib-2.0/schemas /usr/share/backgrounds /usr/share/gnome-shell/theme /usr/share/wayland-sessions"