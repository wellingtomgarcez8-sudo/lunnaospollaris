# LunnaOS Polaris roadmap

## Phase 0 — Foundation and mutable runtime
- [x] Yocto/KAS build foundation
- [x] x86_64 UEFI WIC output
- [x] RPM package backend
- [x] Runtime package-management support
- [x] Flatpak runtime
- [x] Flathub first-boot initialization
- [x] LunnaOS Shell visual layer
- [x] Polaris top panel and dock
- [x] LunnaOS wallpaper/defaults
- [x] LunnaOS Settings/Files/Store/Games entry points
- [x] RPM package-index pipeline
- [x] RPM feed publication pipeline
- [ ] First successful post-integration CI image
- [ ] UEFI VM smoke test
- [ ] Physical laptop smoke test

## Phase 1 — Desktop completeness
- [ ] Custom notification center
- [ ] Real media controls
- [ ] Real network/audio/Bluetooth controls
- [ ] Battery/brightness controls
- [ ] Workspace switcher
- [ ] Lock/login branding
- [ ] Plymouth branding
- [ ] More Polaris-native GTK styling and iconography
- [ ] Accessibility review

## Phase 2 — Lunna applications
- [ ] Native Lunna Settings frontend
- [ ] Native Lunna Files frontend
- [ ] Native Lunna Store frontend
- [ ] Native Task Manager
- [ ] Native Terminal
- [ ] Native Camera
- [ ] Native Games hub
- [ ] Recovery and system updater UI

## Phase 3 — Optional native compositor
- [ ] Smithay DRM/KMS backend
- [ ] libseat/logind integration
- [ ] libinput
- [ ] GPU accelerated rendering
- [ ] XWayland compatibility
- [ ] VM and hardware validation
- [ ] Only after validation: consider replacing Mutter

## Phase 4 — Production release
- [ ] Automated VM boot test
- [ ] Automated package-manager smoke tests
- [ ] Hardware validation matrix
- [ ] Checksums and release artifacts
- [ ] Signed production images
- [ ] Stable release channel and rollback procedure
