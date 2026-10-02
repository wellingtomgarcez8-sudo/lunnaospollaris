# LunnaOS Polaris build architecture

LunnaOS Polaris is built with Yocto Project/OpenEmbedded through KAS.

1. KAS checks out Yocto/OpenEmbedded.
2. OE-Core builds the target toolchain and base system.
3. meta-gnome provides GNOME Shell, Mutter, GDM and GNOME applications.
4. meta-lunnaos installs branding, defaults and the LunnaOS shell extension.
5. Yocto builds a generic x86-64 UEFI WIC image.
6. GitHub Actions validates and publishes the image.

The numbered scripts are orchestration/validation entry points; compilation remains inside Yocto for deterministic dependency handling.
