# Business Logic Model

## Scope
Unit 7 defines the backend behavior for recorded-library assembly, recorded-playback handoff, recording deletion, and explicit Live TV stop. The backend remains the single owner of storage-source ordering, action-target validation, and playback-session transitions.

## Core Decision Summary
- The library merges duplicate recordings into one logical item while preserving per-source metadata for diagnostics and later refresh decisions.
- Recorded playback reuses the existing playback session controller and explicitly transitions the shared session into recorded-playback mode.
- Recording deletion requires validated current action targets and fails clearly if the target can no longer be trusted.
- Live TV stop is a hard stop that releases playback resources promptly while preserving remembered device or channel context.
- If a recording disappears between library load and action time, the backend returns a structured missing-recording result and requires the client to refresh.

## Library Assembly Model

### 1. Source Discovery and Ordering
1. Resolve all storage sources associated with the selected device.
2. Order sources with local storage first and non-local storage after it.
3. Fetch recorded-file listings from each source independently.
4. Continue assembling a partial catalog when one source fails, while surfacing degraded-state warnings.

### 2. Recording Normalization
Each raw recorded-file entry is normalized into a candidate record with:
- stable logical identity inputs such as series or program metadata, start time, channel context, and vendor identifiers when available
- source-qualified playback target metadata
- source-qualified delete target metadata
- source priority and locality markers
- availability and validation state

### 3. Duplicate Merge
1. Group normalized candidates that represent the same logical recording.
2. Select one preferred source using the ordered source list.
3. Produce a single catalog item for the UI-facing library.
4. Retain alternate source metadata inside the backend model for diagnostics, refresh comparison, and later action resolution.

The merge rule is intended to simplify the UI without allowing action targets to become ambiguous.

## Recorded Playback Model

### Action Flow
1. Client requests playback for a logical recording identifier.
2. Backend resolves the current catalog snapshot for the selected device.
3. Backend validates that the requested logical recording still maps to the same actionable preferred source.
4. If validation succeeds, backend converts the recording target into a playback source and sends it through the shared playback session controller.
5. The shared playback session transitions into recorded-playback mode and updates current-session state.

### Failure Path
- If the recording cannot be re-resolved, return a structured missing-recording outcome.
- Do not automatically fall back to a different source after the client has selected a logical recording from an older snapshot.
- Require a library refresh before replay attempts continue.

## Recording Deletion Model

### Action Flow
1. Client requests deletion for a logical recording identifier.
2. Backend re-resolves the current catalog snapshot.
3. Backend validates the delete target from the current preferred source metadata.
4. Backend invokes the validated `CmdURL` using HTTP POST with `cmd=delete`.
5. Backend performs sync-aware refresh behavior against the relevant storage source.
6. Backend returns confirmed deletion success or a structured failure result.

### Safety Boundary
- Raw delete commands are never accepted from the client.
- Previously cached delete targets are not trusted without current re-resolution.
- If the item is missing or the target cannot be confirmed, deletion fails without side effects.

## Live TV Stop Model

### Action Flow
1. Client issues explicit stop for the current playback session.
2. Backend checks whether the active session is Live TV.
3. Backend calls the existing playback stop path to terminate the live stream and release player resources promptly.
4. Backend preserves remembered device or channel context so later restart can use prior selection.
5. Backend returns a stopped session result instead of forcing app exit.

### Behavioral Boundary
- Stop is not a soft pause.
- Stop must not keep the live tuner pinned unnecessarily.
- Stop may leave the last selected device and channel in remembered context, but the session itself becomes inactive.

## State Consistency Model
- Library reads may be degraded, but mutation actions require current validated targets.
- Merged logical items are stable only within the scope of a current catalog snapshot.
- Playback, deletion, and stop all update shared runtime state so the client sees one coherent session view.