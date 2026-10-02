SUMMARY = "LunnaOS Polaris desktop image"
LICENSE = "MIT"
inherit core-image
IMAGE_FEATURES += "splash package-management"
IMAGE_INSTALL += " packagegroup-lunnaos"