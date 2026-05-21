# Functional Design Plan - Unit 2 HDHomeRun Discovery and Device Integration

## Execution Checklist
- [x] Review Unit 2 responsibilities, story mappings, and existing backend foundation together
- [x] Define the business logic model for device discovery, selection, lineup retrieval, and tuner visibility
- [x] Define domain entities for discovered devices, lineups, channels, and tuner diagnostics
- [x] Define business rules for remembered-device reconciliation and channel metadata handling
- [x] Define integration boundaries between libhdhomerun discovery, HTTP lineup retrieval, and backend API contracts
- [x] Generate business-logic-model.md
- [x] Generate business-rules.md
- [x] Generate domain-entities.md
- [x] Validate completeness and consistency

## Unit Context
- **Unit**: HDHomeRun Discovery and Device Integration
- **Purpose**: Integrate `libhdhomerun` for device discovery, lineup retrieval, and tuner or signal visibility.
- **Primary Story Impact**:
  - US-3 See discovered devices
  - US-4 Browse channel lineup
  - US-7 View tuner status and signal info
  - supporting impact on US-1 and US-2 through remembered-device reconciliation

## Observed Technical Inputs
- `libhdhomerun` provides discovery APIs and tuner-status APIs.
- HDHomeRun HTTP API exposes `lineup.json` and stream `URL` fields per channel.
- The existing Unit 1 backend already exposes stable provisional device and playback endpoints.

## Clarifying Questions

## Question 1
What discovery scope should Unit 2 target in v1?

A) IPv4 local-network discovery only

B) IPv4 plus general IPv6 discovery, but no link-local IPv6 complexity in v1

C) Full IPv4 and IPv6 including link-local handling in v1

X) Other (please describe after [Answer]: tag below)

[Answer]: A

## Question 2
How should Unit 2 split responsibility between libhdhomerun and the HDHomeRun HTTP API for channel data?

A) Use libhdhomerun for discovery and tuner status, and use `lineup.json` for channel list and playback URL metadata

B) Use libhdhomerun for everything possible, minimizing direct HTTP usage

C) Use HTTP API for discovery and lineup, with libhdhomerun only for tuner status

X) Other (please describe after [Answer]: tag below)

[Answer]: A

## Question 3
What tuner or signal detail should the Unit 2 backend contract expose in v1?

A) Only the active selected tuner summary

B) All tuners for the selected device, with one active context highlighted

C) Detailed raw tuner fields for every tuner by default

X) Other (please describe after [Answer]: tag below)

[Answer]: B

## Question 4
How should DRM-tagged or otherwise non-playable channels be represented in v1?

A) Include them in the lineup but clearly mark them as unavailable or restricted

B) Hide them from the default lineup entirely

C) Include them with no distinction in v1

X) Other (please describe after [Answer]: tag below)

[Answer]: A

## Question 5
If the remembered device is missing but other HDHomeRun devices are available, what should Unit 2 return to the client?

A) A selection-needed result that includes the newly discovered device list

B) An automatic fallback to the first available device

C) A hard error requiring manual recovery outside the normal flow

X) Other (please describe after [Answer]: tag below)

[Answer]: A

## Approved Direction
- Discovery scope for v1 is IPv4 local-network discovery only.
- `libhdhomerun` owns discovery and tuner-status integration.
- `lineup.json` is the canonical source for channel list and playback URL metadata.
- The backend contract exposes all tuners for the selected device and highlights one active tuner context.
- DRM or otherwise restricted channels remain visible in the lineup and are marked as unavailable or restricted.
- If a remembered device is missing, the backend returns a selection-needed result plus the currently available devices.

## Constraints to Preserve
- Keep backend contracts stable for the future Qt/QML client.
- Preserve loopback-only backend exposure.
- Do not silently guess behavior when device state is ambiguous.
- Maintain compatibility with later Unit 3 playback orchestration.