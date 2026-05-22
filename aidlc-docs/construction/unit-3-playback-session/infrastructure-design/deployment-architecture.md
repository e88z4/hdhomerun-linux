# Deployment Architecture - Unit 3 Playback Session Orchestration and Player Adapter

## Runtime Topology
- Qt/QML client talks to the Rust backend over loopback HTTP.
- Rust backend remains the only public control plane for playback.
- Backend starts and supervises one local `mpv` child process when playback is requested.
- Backend and `mpv` communicate through a local IPC socket.

## Execution Flow
1. Client issues a loopback playback command.
2. Backend validates the device and channel through Unit 2 state.
3. Backend ensures the `mpv` adapter is ready or starts it on demand.
4. Backend loads or replaces the stream inside the existing player session.
5. Backend updates the normalized playback state exposed through `/api/playback/current`.

## Operational Boundaries
- Only the backend may issue playback commands to `mpv`.
- `mpv` is an internal runtime dependency, not a client-facing subsystem.
- Retry behavior is bounded and backend-owned.

## Packaging Consequences
- AppImage, Flatpak, and Debian packaging work must preserve:
  - loopback backend communication
  - local process-spawn capability for the player adapter
  - availability of the `mpv` executable or an equivalent packaged runtime path