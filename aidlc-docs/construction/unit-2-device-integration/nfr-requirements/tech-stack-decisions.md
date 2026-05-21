# Tech Stack Decisions - Unit 2 HDHomeRun Discovery and Device Integration

## Primary Language and Runtime
- **Choice**: Rust on the Unit 1 backend foundation
- **Rationale**:
  - Reuses the approved backend runtime and contract model from Unit 1.
  - Keeps device integration inside the same process and error-modeling approach.
  - Avoids introducing a second runtime or service boundary for LAN device communication.

## Vendor Discovery and Tuner Integration
- **Recommended Direction**: thin Rust adapter layer around `libhdhomerun`
- **Rationale**:
  - `libhdhomerun` is the canonical vendor integration point for discovery and tuner status.
  - An adapter boundary keeps vendor-specific APIs from leaking directly into the public backend contract.
  - The adapter can encapsulate reusable discover-object lifecycle management and string copying rules.

## Lineup Retrieval
- **Recommended Direction**: async HTTP client within the Rust backend for `lineup.json` retrieval
- **Rationale**:
  - The approved Unit 2 design uses `lineup.json` as the canonical channel and playback metadata source.
  - A bounded-timeout async HTTP client fits the existing Tokio-based backend runtime.
  - Keeping lineup retrieval inside the backend preserves one normalization authority for the client contract.

## Normalization and Contract Modeling
- **Recommended Direction**: continue using Serde-backed typed models for normalized devices, channels, tuner diagnostics, and playback-source descriptors
- **Rationale**:
  - Unit 2 introduces more vendor-originated data that must be normalized before client exposure.
  - Typed models help preserve explicit availability flags, selection-required behavior, and sanitized error shaping.

## Refresh Strategy
- **Recommended Direction**: startup discovery plus lightweight periodic background refresh using bounded async tasks
- **Rationale**:
  - Matches the approved freshness target without aggressive polling.
  - Keeps background refresh separate from user-triggered foreground requests.
  - Supports explicit timeout and interval control rather than implicit ad hoc polling.

## Error Modeling and Logging
- **Recommended Direction**: extend typed application errors with Unit 2-specific discovery, lineup, and tuner-diagnostic failure categories while retaining structured info-level logging plus debug-only vendor-call detail
- **Rationale**:
  - Keeps client-visible failures predictable.
  - Makes partial-failure handling explicit.
  - Preserves the approved logging posture for v1.

## Testing Strategy
- **Recommended Direction**: example-based regression tests plus `proptest` for pure normalization and invariant logic
- **Rationale**:
  - Matches the approved Unit 2 testing depth.
  - Avoids overcommitting PBT to live network or hardware integration boundaries.
  - Encourages adapter seams and fixtures that make hardware-dependent behavior testable without always requiring a real device.

## Stack Constraints
- Discovery scope in v1 is IPv4 local-network only.
- `lineup.json` is the canonical lineup source.
- Partial tuner failures must degrade gracefully instead of failing the entire diagnostics response.
- Default logging must stay high level and structured, with deeper device detail only in debug mode.