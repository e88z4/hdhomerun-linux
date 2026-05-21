# Infrastructure Design - Unit 1 Backend Foundation and Local API

## Deployment Context

Unit 1 is a local Linux service component bundled with a desktop application. It is not a cloud service and does not require remote network exposure. Infrastructure design therefore focuses on local process management, filesystem layout, loopback networking, and development-to-packaging consistency.

## Runtime Decisions
- Backend HTTP service binds to localhost only.
- App-managed startup is the default runtime path.
- Optional systemd user-service support is part of the design, but not the primary interaction model.
- Development mode should mimic packaged behavior as closely as practical.

## Process Supervision Model

### Primary Mode: App-Managed Startup
- Desktop client probes backend availability.
- If unavailable, the client starts the backend process directly.
- Client waits for readiness using the bounded startup pattern defined in Unit 1.
- If readiness fails, the client receives a structured failure state and may offer explicit retry.

### Secondary Mode: Optional systemd User Service
- A user-service definition may be provided for Linux environments that prefer managed background execution.
- The client may connect to an already-running backend rather than spawning it directly.
- The same loopback API contract and state locations are used in both modes.

## Storage Layout
- **Canonical state location**: XDG state directory strategy.
- **Runtime logs**: structured logs suitable for local capture and debugging; packaged location details may vary by distribution format.
- **Packaged binaries**: backend and client remain distinct bundled components.

## Networking Layout
- Loopback HTTP only.
- No LAN binding.
- No reverse proxy, API gateway, or external service mesh layer.

## Development Layout
- Development commands should preserve the same process relationships as packaged runtime where practical.
- Dev mode should still use:
  - separate backend process
  - loopback API communication
  - XDG-style state location conventions when feasible
- This reduces late surprises during packaging and integration.

## Security Mapping
- Localhost-only binding satisfies the v1 local exposure requirement.
- No remote listener or network-facing intermediary is introduced.
- Sanitized structured errors and structured logs remain application-layer enforcement concerns, but infrastructure should not weaken them.

## Operational Notes
- Startup probes must distinguish process launched from service ready.
- Optional user-service mode must not change API semantics.
- Infrastructure should remain simple enough that packaging formats can wrap the same core backend behavior.