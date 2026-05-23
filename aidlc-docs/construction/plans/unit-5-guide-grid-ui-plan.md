# Code Generation Plan - Unit 5 Guide Grid UI

## Execution Checklist
- [x] Verify the live DeviceAuth guide path against a real tuner before designing the schedule UI around it
- [x] Add a backend guide-window contract that reuses the SiliconDust guide provider
- [x] Expose guide state and window navigation controls through the client controller
- [x] Build a QML guide grid panel with a time axis and click-to-play entries
- [x] Update the compact channel rail to render the current program title from lineup metadata
- [x] Re-run backend, client, packaging, and live verification after the UI and contract changes

## Chosen Default Decisions
- **Guide contract owner**: the backend remains the canonical owner of guide fetches so the Qt client does not need a second direct vendor API integration
- **Default guide window**: use a 4-hour window aligned to the current half-hour boundary because it matches the vendor API intent and keeps the guide grid readable in a desktop layout
- **Guide activation**: allow `G` to toggle the grid and let any channel label or guide cell trigger live playback for that channel
- **Rail fallback**: keep the compact rail visible and lightweight, while the fuller schedule detail lives in the dedicated guide panel