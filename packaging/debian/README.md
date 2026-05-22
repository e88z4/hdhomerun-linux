# Debian Notes

## `mpv` Strategy

Debian packaging should declare `mpv` as a package dependency instead of embedding another copy of the player.

This matches Debian's normal package-management model, keeps security updates in the distro channel, and avoids shipping a second unmanaged player runtime.

## Expected Packaging Shape

- package name: `hdhomerun-linux-player`
- runtime dependency: `mpv`
- installed launcher should either rely on `mpv` being on `PATH` or export `HDHR_BACKEND_MPV_BIN=/usr/bin/mpv`

The final `.deb` wiring should add the future client binary and backend launcher once Unit 4 produces the desktop entry point.
