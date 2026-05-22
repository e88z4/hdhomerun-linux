# Infrastructure Design - Unit 3 Playback Session Orchestration and Player Adapter

## Deployment Context

Unit 3 remains a local Linux backend component, but it now supervises a child playback process in addition to its loopback HTTP server. Infrastructure design therefore expands from loopback service management into local process control, runtime socket placement, and packaging-time handling of the `mpv` dependency.

## Runtime Decisions
- Playback commands continue to enter through the loopback-only backend API.
- The backend spawns and supervises a local `mpv` child process on demand.
- Backend and player communicate through a local IPC socket located under backend-managed state or runtime directories.
- Normal stop keeps the player process reusable when practical instead of forcing immediate teardown.

## Process Supervision Model

### Primary Mode: Backend-Owned Player Process
- Backend starts `mpv` lazily on the first playback command that requires it.
- Backend waits within a bounded startup window for the IPC socket and command readiness.
- If startup or first load fails in a retryable way, backend may consume one bounded automatic retry.
- If the adapter still fails, backend exposes a failed session state and waits for an explicit next playback action.

### Operational Failure Model
- Unexpected player exit is treated as an adapter failure, not as an invisible background event.
- Backend keeps the failed state visible through `/api/playback/current`.
- Adapter rebuild happens only on the bounded retry path or a later explicit playback command.

## Storage and Runtime Layout
- **Canonical remembered playback context**: continues to use the backend XDG-style state location.
- **Player IPC socket**: lives under a backend-owned runtime subdirectory within the local state area.
- **Structured logs**: remain backend-owned and may be captured by the same development or packaged runtime tooling as Unit 1 and Unit 2.

## Packaging and Dependency Notes
- The current Unit 3 runtime expects an `mpv` executable to be available in the target environment.
- The backend now checks for the executable explicitly before normal playback startup and can use `HDHR_BACKEND_MPV_BIN` when packaging needs to point at a non-default path.
- Packaging work must later decide whether `mpv` is bundled directly, declared as an install-time dependency, or resolved per distribution format.
- This dependency decision must not change the public loopback API contract or backend-owned orchestration model.

## Development Parity
- Development mode should use the same backend-to-player process relationship as packaged runtime.
- Local development should exercise the same IPC socket lifecycle, retry boundary, and remembered-context persistence behavior used in packaged builds.

## Security Mapping
- Loopback-only backend exposure remains unchanged.
- The player IPC socket is a local implementation detail and must not be re-exposed as a client control surface.
- Infrastructure must not widen playback control beyond the backend or leak raw player-process details through packaging defaults.