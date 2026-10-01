# LunnaOS Polaris
# Base: Bazzite stable. We intentionally build a new image instead of modifying
# the host system. The desktop layer will be replaced by LunnaOS in later phases.

FROM scratch AS ctx
COPY build_files /
COPY system_files /system_files

FROM ghcr.io/ublue-os/bazzite:stable

ARG IMAGE_NAME=lunnaospollaris
ARG IMAGE_VENDOR=lunnaos
ARG IMAGE_VERSION=0.1.0

LABEL org.opencontainers.image.title="LunnaOS Polaris"
LABEL org.opencontainers.image.vendor="LunnaOS"
LABEL org.opencontainers.image.version="$IMAGE_VERSION"
LABEL org.opencontainers.image.description="LunnaOS Polaris — Bazzite-based immutable Linux OS with the LunnaOS Shell."

RUN --mount=type=bind,from=ctx,source=/,target=/ctx \
    --mount=type=cache,dst=/var/cache \
    --mount=type=cache,dst=/var/log \
    --mount=type=tmpfs,dst=/tmp \
    /ctx/build.sh

RUN bootc container lint
