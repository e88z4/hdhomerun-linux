# Infrastructure Design - Unit 2 HDHomeRun Discovery and Device Integration

## Deployment Context

Unit 2 extends the local Linux backend from Unit 1 with outbound LAN communication to HDHomeRun devices, a bundled native-library dependency, and backend-managed background refresh behavior. The backend remains loopback-only to the desktop client, but it now also needs reliable packaged access to local-network discovery traffic and HTTP lineup retrieval.

## Runtime Decisions
- The backend remains the only process that performs HDHomeRun discovery, lineup retrieval, and tuner diagnostics polling.
- Periodic discovery refresh runs inside the backend process on a fixed lightweight interval.
- The client consumes device, lineup, and diagnostics state over the loopback API rather than owning LAN polling logic.
- `libhdhomerun` is built and shipped from the bundled repo copy and linked into the backend distribution path used in development and packaged modes.
- The last successful lineup snapshot is retained in backend memory only and is marked stale on refresh failure.

## Native Library Integration Model

### Bundled Library Strategy
- Unit 2 uses the repository's bundled `libhdhomerun` sources as the v1 integration baseline.
- Development and packaged builds should compile or link against the same bundled library path rather than depending on distro-specific system packages.
- This keeps discovery and tuner integration behavior under one controlled build story for AppImage, Flatpak, and Debian outputs.

### Build Consistency
- Native library build steps should be incorporated into the backend build flow rather than treated as an external manual prerequisite.
- Development mode should exercise the same library integration path as packaged runtime where practical.
- Packaging artifacts should not assume the end user preinstalls HDHomeRun libraries separately.

## Process Supervision Model

### Primary Mode: App-Managed Backend with Internal Refresh
- Desktop client starts or connects to the backend as established in Unit 1.
- Once ready, the backend performs startup discovery and begins the internal refresh schedule.
- The refresh scheduler lives entirely in the backend process.
- The client reads current device integration results from loopback endpoints and may still request explicit foreground refresh actions when needed.

### Secondary Mode: Optional systemd User Service
- If the backend is run as an optional user service, the same internal refresh scheduler continues to operate inside the backend.
- Service mode must not change device discovery semantics, lineup cache semantics, or loopback API behavior.

## Storage Layout
- **Canonical persisted state**: remains in the XDG state directory as defined in Unit 1.
- **Selected device and remembered context**: persisted through the backend-owned state model.
- **Stale lineup fallback**: backend memory only; it is not persisted across backend restarts in v1.
- **Runtime logs**: structured backend logs with packaging-specific capture handled later by the packaging unit.

## Networking Layout
- Client-to-backend communication remains loopback HTTP only.
- Backend-to-device communication uses outbound LAN discovery and HTTP requests to HDHomeRun devices.
- No remote inbound backend listener is introduced.
- Packaged variants are expected to declare or request the local-network permissions needed for discovery and device HTTP access.

## Packaging and Sandbox Assumptions

### AppImage
- AppImage should preserve the same bundled backend plus bundled native-library layout used in development builds.
- Local-network access is expected to work without introducing a new backend exposure model.

### Flatpak
- Flatpak packaging should explicitly request the network permissions needed for HDHomeRun discovery and lineup access.
- Unit 2 infrastructure assumes these permissions are part of the packaged app contract rather than a manual post-install workaround.
- The backend remains loopback-only to the client even when the package has outbound network permission.

### Debian Package
- Debian packaging should install or bundle the same backend and bundled native-library runtime needed by development mode.
- No separate user-installed `libhdhomerun` prerequisite is assumed in v1.

## Development Layout
- Development mode should stay close to packaged runtime behavior.
- Dev workflows should preserve:
  - separate backend process boundaries
  - the same bundled `libhdhomerun` integration path
  - loopback client-to-backend communication
  - real LAN discovery and lineup retrieval behavior where practical
- This reduces packaging surprises and prevents drift between development and shipped behavior.

## Security Mapping
- Unit 1 localhost-only backend exposure remains unchanged.
- Granting packaged variants outbound network access for local discovery does not permit remote inbound control of the backend.
- Backend-owned normalization and sanitized error shaping remain the boundary between raw device communication and client-visible data.
- Keeping refresh inside the backend avoids exposing direct device credentials, network endpoints, or polling logic to the client process.

## Operational Notes
- Startup discovery seeds initial device availability before steady-state refresh begins.
- Periodic refresh cadence should remain lightweight and backend-controlled.
- Stale lineup memory should be clearly distinguishable from a fresh lineup snapshot in backend metadata.
- Infrastructure choices should remain simple enough that Unit 3 playback orchestration can consume the same selected-device and playback-source outputs without adding a second discovery authority.