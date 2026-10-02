# LunnaOS Polaris build architecture

LunnaOS Polaris is built with Yocto Project/OpenEmbedded through KAS.

1. KAS checks out Yocto/OpenEmbedded Scarthgap.
2. OE-Core and the OpenEmbedded layers build the x86_64 desktop runtime.
3. meta-gnome provides the mature GNOME/Mutter/Wayland session foundation.
4. meta-lunnaos installs Polaris branding, defaults and the LunnaOS Shell extension.
5. The image uses the RPM backend plus the package-management feature so the installed system remains mutable.
6. Flatpak is installed and Flathub is registered during first boot.
7. bitbake package-index creates the RPM repository metadata after the image build.
8. GitHub Actions publishes the WIC image, manifest and RPM feed.

The Rust/Smithay compositor sources remain in the repository as an experimental future component and are intentionally not required by the current production image. This avoids making the release depend on an unvalidated compositor.

## Runtime package management

Yocto documents package_rpm together with package-management as the basis for runtime RPM/DNF management. The RPM package feed must expose repository metadata in addition to the RPM files.

The CI pipeline therefore runs the package-index step separately after the image build and publishes the resulting tmp/deploy/rpm tree.

## Build outputs

The primary installable candidate is:

build/tmp/deploy/images/genericx86-64/*.wic

The mutable-package feed is:

build/tmp/deploy/rpm/**

A WIC image is only promoted to a physical-laptop release after UEFI VM and hardware smoke tests pass.
