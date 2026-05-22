# AppImage Notes

## `mpv` Strategy

AppImage should prefer a bundled `mpv` executable inside the AppDir so the playback runtime does not depend on the host distro layout.

Recommended placement:

- `AppDir/usr/bin/mpv`

The shared launcher logic also falls back to a host `mpv` during development or partial staging, which keeps local iteration practical before the full AppImage bundle exists.

## Expected Runtime Layout

- `AppDir/usr/bin/hdhomerun-linux-player`
- `AppDir/usr/bin/hdhomerun-backend`
- `AppDir/usr/bin/mpv`
- `AppDir/usr/lib/hdhomerun-linux-player/export-mpv-env.sh`

## Launcher Behavior

`AppRun` sources the shared environment helper, exports `HDHR_BACKEND_MPV_BIN`, and then launches the packaged app entry point.

If the final staged app binary is missing, `AppRun` should fail loudly rather than hiding a broken package layout.
