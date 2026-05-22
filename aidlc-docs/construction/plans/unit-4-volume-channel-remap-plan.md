# Code Generation Plan - Unit 4 Volume Controls and Channel Remap

## Execution Checklist
- [x] Review the current fullscreen overlay, playback-stage audio path, and keyboard shortcut bindings
- [x] Confirm the desired interaction model for volume and channel remapping
- [x] Add client-side volume up and volume down controls to the playback shell
- [x] Remap keyboard navigation so `Up` and `Down` control volume and `Left` and `Right` switch channels
- [x] Keep fullscreen overlay behavior aligned with the new control surface
- [x] Re-run client validation after implementation

## Chosen Default Decisions
- **Volume keys**: `Up` raises volume and `Down` lowers volume
- **Channel keys**: `Right` moves to the next playable channel and `Left` moves to the previous playable channel
- **Volume control owner**: the Qt client owns volume because embedded playback is rendered through local `AudioOutput`
- **Windowed controls**: expose visible `Vol -` and `Vol +` buttons in the header
- **Fullscreen controls**: expose `Vol -` and `Vol +` buttons plus current volume percentage in the transient overlay