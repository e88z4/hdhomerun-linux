# Code Generation Plan - Unit 4 Fullscreen and Keyboard Channel Switching

## Execution Checklist
- [x] Review the current client shell, playback stage, and channel rail integration points
- [x] Confirm the desired user interaction defaults for fullscreen and keyboard channel switching
- [ ] Add shell-level fullscreen controls that maximize playback space without breaking the existing layout
- [ ] Add keyboard-driven previous and next playable channel navigation that skips unavailable entries
- [ ] Extend automated client coverage for the new channel navigation behavior
- [ ] Re-run client and backend validation after implementation

## Chosen Default Decisions
- **Fullscreen toggle key**: `F`
- **Exit fullscreen key**: `Esc`
- **Channel navigation keys**: `Up` for previous playable channel and `Down` for next playable channel
- **Fullscreen layout behavior**: maximize playback by hiding chrome-heavy side regions while fullscreen is active
- **Channel navigation rule**: skip non-playable channels automatically and wrap across the playable channel list