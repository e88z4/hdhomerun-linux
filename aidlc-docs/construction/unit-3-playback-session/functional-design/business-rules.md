# Business Rules

## Session Ownership Rules
- The backend is the canonical owner of playback session orchestration.
- The client must not control `mpv` directly in Unit 3.
- The first real playback adapter in v1 is a backend-owned persistent `mpv` process hidden behind a stable adapter boundary.

## Session Reuse Rules
- Unit 3 must prefer one persistent playback session for consecutive live-TV channel changes.
- Channel switching should replace the active stream inside the existing session whenever the adapter remains healthy.
- The backend must not tear down and recreate the full player process for every normal channel switch.

## Playback Source Rules
- Playback sources used by Unit 3 come from Unit 2 playback-source resolution.
- Unit 3 should hand the direct HDHomeRun playback URL to the player adapter in v1.
- Unit 3 must not introduce a second discovery, lineup, or stream-proxy authority unless a later unit explicitly changes the architecture.

## Playback Command Rules
- The first real playback API surface in Unit 3 must include current session state plus start, stop, and switch-channel commands.
- Playback commands must validate selected device and channel prerequisites before reaching the player adapter.
- The backend must reject playback requests that refer to unavailable, restricted, or unresolved playback sources with structured errors rather than ambiguous adapter failures.

## Retry and Failure Rules
- Unit 3 may perform one bounded automatic retry for clearly retryable startup failures.
- Automatic retry must not loop indefinitely or continue in the background after the bounded retry is exhausted.
- After the bounded automatic retry fails, backend must surface a structured failure result suitable for inline retry UX.
- Explicit user retry remains allowed after automatic retry exhaustion.

## Current State Reporting Rules
- `/api/playback/current` must become a real endpoint in Unit 3 rather than a placeholder.
- Current playback state must be backend-derived, not inferred from client assumptions.
- Playback state responses must include enough normalized information for the client to render current channel, session status, and failure state without parsing adapter-specific details.

## Safety and Error Rules
- Client-visible playback failures must use structured, sanitized error or warning shapes.
- Backend must not leak raw `mpv` IPC details, process internals, or unsafe command output to the client.
- Backend must not report a session as playing unless the adapter has reached a confirmed playing state.
- Backend must not silently substitute a different channel, device, or playback source when the requested one fails.

## Restore Behavior Rules
- Unit 3 may use remembered playback context only when Unit 1 and Unit 2 prerequisites remain valid.
- If remembered playback prerequisites are invalid, backend must return a non-playing but recoverable session state rather than guessing a fallback channel.