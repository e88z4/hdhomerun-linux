# Business Logic Model

## Overview

Unit 1 defines the backend foundation behavior that every later unit will rely on. Its core business job is not device control yet; it is to make the local service consistently available, expose a stable contract to the client, own canonical remembered state, and shape failures into predictable API results.

## Core Workflows

### 1. Client Startup and Backend Readiness
1. Desktop client starts.
2. Client probes backend health.
3. If the backend is unavailable, the client launches the bundled backend automatically.
4. Client waits for readiness within a bounded startup window.
5. Once ready, the client requests restore state and bootstrap data.

### 2. Restore State Resolution
1. Backend loads remembered state from the canonical state store.
2. Backend checks whether a remembered device reference exists.
3. If no device exists in remembered state, backend returns a `selection_required` bootstrap result.
4. If a remembered device exists but is not currently available, backend clears the device-specific remembered state and returns `selection_required`.
5. If a remembered device exists and is available, backend returns a bootstrap result that includes the remembered device and last watched channel, plus whether playback should auto-resume.

### 3. Early API Contract Availability
1. Backend exposes health and state endpoints immediately.
2. Backend also exposes provisional device and playback endpoints with stable response shapes.
3. Until later units are implemented, those provisional endpoints return placeholder-capable contract responses rather than ad hoc failures.
4. Client can build against the stable response contract before all backend behavior is fully implemented.

### 4. Error Shaping
1. Internal backend failures are mapped to stable structured API errors.
2. Error output includes a machine-readable code, a safe user-facing message, and an optional retry hint.
3. Internal exception details, paths, or raw stack traces are not exposed through the loopback API.

## Functional Responsibilities
- Ensure the service can be started and recognized as ready.
- Establish the canonical restore-state model for the product.
- Provide the initial client contract for later feature units.
- Standardize API success, placeholder, and failure shapes.

## State Transitions

### Backend Availability
- `stopped` -> `starting` -> `ready`
- `starting` -> `failed_to_start`
- `ready` -> `unhealthy`

### Bootstrap Resolution
- `no_state` -> `selection_required`
- `state_loaded + device_missing` -> `selection_required`
- `state_loaded + device_available + auto_resume=false` -> `restored_context`
- `state_loaded + device_available + auto_resume=true` -> `resume_requested`

## Testable Properties
- **Invariant**: Structured API error responses always include `code`, `message`, and retry metadata shape.
- **Round-trip**: Persisted restore state can be serialized and deserialized without losing canonical fields.
- **Invariant**: Loopback bootstrap responses never expose internal exception details.