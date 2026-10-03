SUMMARY = "LunnaOS Polaris desktop image"
LICENSE = "MIT"
inherit core-image image-live
IMAGE_FEATURES += "splash package-management"
IMAGE_INSTALL += " packagegroup-lunnaos"
IMAGE_FSTYPES:append = " iso"
