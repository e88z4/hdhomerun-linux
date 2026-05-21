# Business Rules

## Discovery Rules
- Unit 2 discovery in v1 is limited to IPv4 local-network discovery.
- Discovery must use `libhdhomerun` as the system of record for tuner-device discovery.
- Discovery results must be filtered to tuner-capable HDHomeRun devices only.
- Discovery code should reuse a discover object where practical rather than creating a fresh one for every poll.
- Strings returned by `libhdhomerun` discovery APIs must be copied into backend-owned memory before later processing.

## Device Selection Rules
- Device selection is explicit backend-owned state.
- A remembered device may be restored only if it is present in the latest discovery result.
- If the remembered device is missing and other devices are available, the backend must return a selection-needed outcome with the current device list.
- The backend must not automatically choose the first available replacement device when the remembered device disappears.
- Selected-device state must use a stable device reference derived from the discovered device identity, not a transient request-local pointer.

## Lineup Retrieval Rules
- Lineup data in v1 comes from the selected device's `lineup.json` endpoint.
- `libhdhomerun` remains responsible for discovery and tuner-status access, but not for replacing `lineup.json` as the canonical lineup source.
- The backend must not treat channel ordering, tags, or availability as implicit client concerns; these must be normalized in the backend contract.
- The channel playback `URL` returned by the device must be preserved as the canonical source reference for later playback orchestration.
- If lineup retrieval fails, the backend must return a stable structured failure or unavailable result rather than a malformed partial contract.

## Restricted-Channel Rules
- DRM-tagged or otherwise restricted channels remain visible in the lineup in v1.
- Restricted channels must carry explicit availability metadata so the client can render them as unavailable or restricted.
- The backend must not present restricted channels as immediately playable when device metadata marks them otherwise.
- The backend should preserve restriction-relevant tags needed for later UX messaging.

## Tuner Diagnostics Rules
- The backend contract in v1 must expose all tuners for the selected device.
- One tuner context may be highlighted as active when the backend can determine the active playback or selection relationship.
- Raw vendor tuner fields may exist internally, but the default client contract should emphasize normalized summary fields such as signal presence, lock state, signal strength, signal quality, and error quality.
- Partial tuner-status failure for one tuner must not invalidate otherwise available diagnostics for other tuners.

## Playback-Source Resolution Rules
- Unit 2 may resolve a channel to a playback source reference, but it must not own stream startup or persistent player session logic.
- Playback-source resolution must require both a selected device and a valid lineup entry.
- The backend must not invent fallback stream URLs when the lineup metadata is missing or inconsistent.

## Error and Safety Rules
- All discovery, lineup, and tuner-status failures exposed to the client must use stable structured error shapes.
- Client-visible messages must remain safe and concise and must not leak internal socket details, raw library pointers, or other implementation details.
- The backend must not guess ambiguous device, tuner, or channel state.
- Loopback-only backend exposure rules from Unit 1 remain in force for all Unit 2 behavior.