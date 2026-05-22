#!/usr/bin/env sh
set -eu

check_command() {
    label="$1"
    command_name="$2"

    if command -v "$command_name" >/dev/null 2>&1; then
        printf '[ok] %s: %s\n' "$label" "$(command -v "$command_name")"
        return 0
    fi

    printf '[missing] %s: %s\n' "$label" "$command_name"
    return 1
}

status=0

check_command 'Client build tooling' 'cmake' || status=1
check_command 'Client build tooling' 'ninja' || status=1
check_command 'AppImage tooling' 'appimagetool' || status=1
check_command 'Flatpak tooling' 'flatpak-builder' || status=1
check_command 'Debian tooling' 'dpkg-deb' || status=1

exit "$status"