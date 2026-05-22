# Code Generation Plan - Unit 4 Qt/QML Client Shell and Live-TV User Journey

## Execution Checklist
- [x] Review Unit 4 functional, NFR, and infrastructure artifacts against the current repo state
- [x] Choose the first real client slice for launch, device browsing, playback projection, and diagnostics
- [x] Decide the first client-side integration seam between loopback API contracts and QML presentation
- [x] Implement a real backend-aware client controller while preserving the shell scaffold layout

## Chosen Implementation Slice
- Add a Qt-side application controller that talks to the loopback backend.
- Keep the player-first shell layout from the earlier scaffold.
- Replace mock device, lineup, playback, and diagnostics data with real backend calls.
- Support backend probing and configurable backend startup command resolution.
- Defer the final embedded video-surface integration while keeping the stage layout stable.