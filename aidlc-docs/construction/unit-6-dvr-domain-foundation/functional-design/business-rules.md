# Business Rules

## DVR Readiness Rules
- DVR readiness is distinct from general device availability.
- A device may be usable for Live TV while still being not ready for DVR.
- Readiness results must identify specific blocking conditions rather than a generic failure.
- At minimum, readiness evaluation must consider DeviceAuth availability, accessible DVR storage context, and record-engine availability.
- The backend must not claim DVR readiness when required conditions cannot be confirmed.

## Recording Rule Rules
- The backend owns all translation between client rule requests and HDHomeRun DVR API operations.
- The backend must support both series rules and one-time DateTime-plus-channel rules.
- Unsupported or partially supported rule options must fail fast with structured validation errors.
- The backend must not silently drop unsupported rule fields and report success.
- Successful rule mutations must be confirmed by re-fetching current state before success is returned to the client.

## Scheduled-State Projection Rules
- Upcoming-recordings data is the primary execution signal for scheduled-state projection.
- Rule state and episode context may explain or enrich schedule state, but they must not override explicit upcoming-recordings evidence without justification.
- If execution evidence is absent or contradictory, the backend must prefer a conservative result over a misleading scheduled state.
- Projection results must remain understandable without raw vendor terminology.

## One-Time Airing Validation Rules
- A one-time rule requires a still-valid airing identity with channel and start-time context.
- If the targeted airing is stale, removed, or no longer valid, the backend must return an invalid-airing result.
- The backend must not automatically choose a nearby airing as a substitute.
- Invalid-airing responses should preserve enough context for the client to tell the user to reselect.

## API and Validation Rules
- Loopback DVR API requests must validate required fields, allowed value ranges, and rule-shape consistency before upstream calls occur.
- Validation failures must return structured domain-safe errors.
- Client-facing responses must avoid exposing raw tokens, internal stack traces, or unchecked upstream payload fragments.

## Logging Rules
- Rule mutations, readiness failures, and projection refreshes must be logged in structured form.
- Logs must avoid sensitive tokens or raw vendor payload dumps.
- Failed rule mutations should be traceable without exposing unsafe detail to the client.

## Non-Guessing Rules
- The backend must not guess DVR readiness from incomplete evidence.
- The backend must not guess rule success without confirmed post-write state.
- The backend must not guess alternative airings for one-time rules.
- The backend must not invent scheduled state when explicit execution evidence is missing.