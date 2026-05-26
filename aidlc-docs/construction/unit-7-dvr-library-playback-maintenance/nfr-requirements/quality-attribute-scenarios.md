# Quality Attribute Scenarios - Unit 7 DVR Library Playback and Maintenance

## Scenario 1: Degraded Multi-Source Library Read
- **Attribute**: Availability and usability
- **Source**: One storage source is reachable and another times out.
- **Stimulus**: The client requests the recorded library for the selected device.
- **Environment**: Normal local execution with one failing non-local source.
- **Response**: The backend returns a library assembled from the reachable sources, marks the response degraded, and includes actionable warnings.
- **Response Measure**: Partial results are returned without pretending full freshness, and the request usually completes within 2 seconds under normal local conditions.

## Scenario 2: Missing Recording At Action Time
- **Attribute**: Reliability and safety
- **Source**: A previously listed recording is deleted or moved before the user acts on it.
- **Stimulus**: The client requests playback or deletion for that logical recording.
- **Environment**: The requested item no longer resolves in the current catalog snapshot.
- **Response**: The backend returns a structured missing-recording outcome and requires a library refresh.
- **Response Measure**: No playback or deletion is attempted against stale action targets.

## Scenario 3: Safe Deletion With Post-Action Convergence
- **Attribute**: Safety and consistency
- **Source**: A user deletes a valid recording.
- **Stimulus**: The backend validates the target and invokes the record-engine delete command.
- **Environment**: Normal local execution with a reachable preferred storage source.
- **Response**: The backend executes exactly one delete request, performs sync-aware refresh behavior, and returns confirmed success.
- **Response Measure**: The deleted recording normally disappears from the next library read without requiring a manual backend restart.

## Scenario 4: Explicit Live TV Stop
- **Attribute**: Performance and usability
- **Source**: A user actively streaming Live TV presses Stop.
- **Stimulus**: The client calls the backend stop endpoint.
- **Environment**: A live playback session is active and a device or channel context is remembered.
- **Response**: The backend stops the live stream, releases playback resources promptly, and preserves remembered context for a later restart.
- **Response Measure**: The response clearly reports a stopped session and the UI is not left in a hung or ambiguous playback state.

## Scenario 5: Duplicate Merge Stability
- **Attribute**: Maintainability and usability
- **Source**: The same recording appears from both local and non-local storage sources.
- **Stimulus**: The backend assembles the recording catalog.
- **Environment**: Both sources are reachable and return overlapping metadata.
- **Response**: The backend returns one merged library item, selects the local source as preferred, and retains alternate source metadata internally.
- **Response Measure**: The client sees one stable item instead of duplicated entries, while later action validation still uses the current backend snapshot.