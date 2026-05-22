# Infrastructure Design - Unit 4 Qt/QML Client Shell and Live-TV User Journey

## Deployment Context

Unit 4 introduces a real Linux desktop client process that sits beside the backend rather than inside it. Infrastructure therefore focuses on desktop runtime startup, local display environment support, loopback networking, client-side preference storage, and consistency with the packaging direction already defined for AppImage, Flatpak, and Debian outputs.

## Runtime Decisions
- The desktop client is a separate Qt6 process.
- The client talks to the backend over loopback HTTP only.
- The backend remains responsible for HDHomeRun LAN traffic and backend-owned playback orchestration.
- The client owns launch overlay behavior, shell navigation, and presentation-only preferences.

## Process Supervision Model

### Primary Mode: Desktop-Managed Backend Coordination
- The desktop client launches first from the user-facing app entry point.
- The client probes backend health and starts or waits for the backend when required.
- The client enters the main shell only after backend readiness and bootstrap state are resolved.

### Secondary Mode: Optional Prestarted Backend
- The client may connect to an already-running backend started by a user service or packaging-specific launcher.
- This does not change the loopback contract or state ownership.

## Storage Layout
- **Client preferences**: XDG config-style location through normal Qt client settings mechanisms.
- **Backend canonical state**: remains in the backend XDG state directory.
- **Packaged binaries**: client and backend remain distinct runtime components even when launched through one desktop entry.

## Display and Platform Notes
- The client should support normal Linux desktop display environments with Qt's Wayland support and X11 fallback.
- The client must not assume a full-screen-only environment.
- The shell layout should remain viable at ordinary desktop window sizes.

## Development Layout
- Development mode should preserve the same client-to-backend relationship as packaged runtime.
- When practical, development should use:
  - separate client and backend processes
  - loopback HTTP communication
  - the same packaging-aware runtime assumptions for `mpv`
- This reduces integration surprises once real packaging is exercised.

## Security Mapping
- The client does not widen backend exposure beyond loopback use.
- Client-side preferences remain separate from backend-owned canonical state.
- Infrastructure must preserve sanitized error rendering and avoid encouraging direct backend or player manipulation outside the approved loopback path.