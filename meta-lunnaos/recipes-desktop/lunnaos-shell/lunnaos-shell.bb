SUMMARY = "LunnaOS Sel(l)enne GNOME Shell customization"
LICENSE = "MIT"
SRC_URI = "file://metadata.json file://extension.js file://stylesheet.css"
S = "${WORKDIR}"
inherit allarch
do_install() {
    install -d ${D}/usr/share/gnome-shell/extensions/lunnaos@lunnaos.org
    install -m 0644 ${WORKDIR}/metadata.json ${D}/usr/share/gnome-shell/extensions/lunnaos@lunnaos.org/
    install -m 0644 ${WORKDIR}/extension.js ${D}/usr/share/gnome-shell/extensions/lunnaos@lunnaos.org/
    install -m 0644 ${WORKDIR}/stylesheet.css ${D}/usr/share/gnome-shell/extensions/lunnaos@lunnaos.org/
}
FILES:${PN} += "/usr/share/gnome-shell/extensions"