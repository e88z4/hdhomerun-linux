# Functional Design Plan - Unit 4 Qt/QML Client Shell and Live-TV User Journey

## Execution Checklist
- [x] Review Unit 1 through Unit 3 contracts, user stories, and application design expectations
- [x] Define startup and restore behavior for the desktop shell
- [x] Define channel-browsing, playback, and diagnostics interaction flows
- [x] Define user-facing failure and retry behavior for backend, device, and playback errors
- [x] Define the client-side view-state model and identify required backend contract extensions
- [x] Generate business-logic-model.md
- [x] Generate business-rules.md
- [x] Generate domain-entities.md

## Unit Context
- **Unit**: Qt/QML Client Shell and Live-TV User Journey
- **Purpose**: Deliver the desktop-facing launch, browsing, playback, and diagnostics experience against the loopback backend.
- **Primary Story Impact**:
  - US-1 Reopen the last viewing context
  - US-2 Handle first launch cleanly
  - US-3 See discovered devices
  - US-4 Browse channel lineup
  - US-5 Start live playback immediately from channel selection
  - US-6 Switch channels inside one persistent player session
  - US-7 View tuner status and signal info
  - US-8 Recover from tuner or stream startup failures

## Current Technical Baseline
- The Rust backend already exposes `health`, `state`, `bootstrap`, `devices`, `lineup`, `tuners`, and playback endpoints.
- There is no `client/` project yet.
- Playback session state is real, but a future client still needs a clear shell model for launch, device selection, and in-app playback presentation.

## Chosen Default Decisions

## Decision 1
**Startup flow**: Use a branded launch overlay that auto-starts or probes the backend, performs bootstrap automatically, and lands in either restored playback context, device selection, or the primary browsing shell without a separate setup wizard.

## Decision 2
**Main layout**: Use a player-first shell with three persistent zones:
- a left channel rail for browsing and quick switching
- a central playback stage that always owns the main visual focus
- a right diagnostics drawer that can expand without replacing playback context

## Decision 3
**No-device state**: Use a blocking but recoverable empty state with clear retry scanning, explanatory copy, and immediate device selection once devices appear.

## Decision 4
**Playback failure UX**: Show failures inline in the playback stage with direct retry action, quick jump to diagnostics, and retained channel context so the user does not lose their place.

## Decision 5
**Device switching behavior**: Keep device switching in a visible header selector. If playback is active and the user chooses a different device, require an explicit switch action that stops the current session cleanly before the new device lineup becomes primary.

## Required Backend Follow-Up
- Unit 4 needs an explicit device-selection contract rather than relying only on remembered backend state.
- Unit 4 benefits from a retry endpoint or equivalent command path for replaying the last failed playback attempt.