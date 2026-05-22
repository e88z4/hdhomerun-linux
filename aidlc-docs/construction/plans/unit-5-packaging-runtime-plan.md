# Packaging Runtime Plan - Unit 5 Embedded Playback Packaging

## Execution Checklist
- [x] Re-evaluate packaging assumptions against the embedded Qt Multimedia playback architecture
- [x] Replace `mpv`-first launcher helpers with backend-path and client-managed playback runtime helpers
- [x] Update AppImage, Flatpak, and Debian packaging skeletons to package the client and backend together
- [x] Re-run packaging shell validation and runtime smoke checks against repo-local build outputs

## Chosen Default Decisions
- **Packaged playback mode**: default to `HDHR_BACKEND_PLAYER_MODE=client` so the backend remains headless and the Qt client renders the playback surface.
- **Launcher shape**: package the Qt client as the primary entrypoint and pass the packaged backend path through `HDHR_BACKEND_CMD`.
- **Debian dependency strategy**: depend on Qt QML multimedia/runtime modules instead of `mpv`.
- **Local validation path**: packaging smoke tests should resolve repo-local backend and client binaries when running in development.