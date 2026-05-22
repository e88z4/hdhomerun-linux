# Tech Stack Decisions - Unit 4 Qt/QML Client Shell and Live-TV User Journey

## Primary UI Stack
- **Choice**: Qt6 Quick with Qt Quick Controls 2.
- **Rationale**:
  - Matches the approved frontend direction.
  - Strong fit for a modern Linux desktop shell with persistent player-oriented layout.
  - Aligns with Flatpak runtime choices already being prepared.

## Build System
- **Choice**: CMake with Qt6's QML module support.
- **Rationale**:
  - Standard modern Qt workflow.
  - Works cleanly across development and packaging environments.
  - Supports gradual growth from a shell scaffold into a fuller desktop app.

## State Integration Strategy
- **Choice**: Keep backend contracts authoritative and project them into client-facing shell state.
- **Rationale**:
  - Prevents business logic drift between backend and QML.
  - Keeps the client focused on presentation and interaction.

## Playback Presentation Strategy
- **Choice**: Build the client around a persistent central playback stage now, even while the rendering surface remains a scaffolded placeholder.
- **Rationale**:
  - Locks the shell UX shape early.
  - Lets backend-driven playback state wire into a stable layout incrementally.

## Diagnostics Presentation Strategy
- **Choice**: Keep diagnostics in a persistent expandable side drawer.
- **Rationale**:
  - Preserves playback context while making tuner state readily accessible.
  - Matches the approved diagnostics direction from earlier AI-DLC stages.

## Testing Strategy
- **Choice**: Combine backend contract tests with later client smoke tests and focused shell-state tests.
- **Rationale**:
  - Avoids duplicating backend contract verification in the client.
  - Keeps the first client milestone practical even before full Qt test tooling is installed locally.

## Stack Constraints
- The client relies on loopback HTTP/JSON only.
- The client should not own direct playback, device, or tuner logic.
- Full local build validation remains blocked until Qt6 development packages are installed in the environment.