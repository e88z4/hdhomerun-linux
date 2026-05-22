# Code Generation Plan - Unit 4 Playback Overlay Controls Refresh

## Execution Checklist
- [x] Review the current icon-button placement in the header and fullscreen overlay
- [x] Move playback-centric controls into a YouTube-style overlay on top of the playback stage
- [x] Replace discrete volume buttons with an overlay slider while preserving keyboard volume bindings
- [x] Use the same overlay control model in both windowed and fullscreen modes
- [x] Re-run client and distribution validation after the overlay refresh

## Chosen Default Decisions
- **Control placement**: place volume and fullscreen controls inside the playback stage instead of the header
- **Volume interaction**: use a slider-backed overlay with icon buttons on both sides and keep `Up`/`Down` keyboard control
- **Fullscreen interaction**: use one overlay icon that switches between fullscreen-enter and fullscreen-exit states based on the current mode
- **Behavior model**: reveal the control bar on interaction and pointer movement in both windowed and fullscreen playback