SUMMARY = "Raspirus"
LICENSE = "GPLv3"
LIC_FILES_CHKSUM = "file://LICENSE;md5=0123456789abcdef0123456789abcdef"

SRC_URI = "git://github.com/Raspirus/Raspirus.git"
SRCREV = "HEAD"

DEPENDS = "nodejs rust tauri-cli"

inherit cargo tauri

do_install() {
    install -d ${D}${bindir}
    install -m 0755 ${B}/target/release/raspirus ${D}${bindir}
}
