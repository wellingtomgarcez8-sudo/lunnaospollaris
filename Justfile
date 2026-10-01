set dotenv-filename := "image-template.env"
set dotenv-load

image_name := env_var("IMAGE_NAME")
default_tag := env_var("DEFAULT_TAG")

default:
    @just --list

build target_image=image_name tag=default_tag:
    #!/usr/bin/env bash
    set -euo pipefail
    podman build --pull=newer --tag "{{ target_image }}:{{ tag }}" --file Containerfile .

build-iso target_image=("localhost/" + image_name) tag=default_tag:
    #!/usr/bin/env bash
    set -euo pipefail
    podman run --rm --privileged \
      --pull=newer \
      --network=host \
      --security-opt label=type:unconfined_t \
      -v "$PWD/iso.toml:/config.toml:ro" \
      -v "$PWD/output:/output" \
      -v "/var/lib/containers/storage:/var/lib/containers/storage" \
      quay.io/centos-bootc/bootc-image-builder:latest \
      --type iso \
      --rootfs btrfs \
      --use-librepo \
      --config /config.toml \
      "{{ target_image }}:{{ tag }}"

lint:
    #!/usr/bin/env bash
    set -euo pipefail
    command -v shellcheck >/dev/null
    find . -type f -name "*.sh" -print0 | xargs -0 -r shellcheck
