# Business Rules

## BR-U7-1 Local-First Source Priority
The backend must order recording sources so that local storage is preferred ahead of non-local storage for catalog construction and preferred action-target selection.

## BR-U7-2 Duplicate Merge Policy
If multiple storage sources expose the same logical recording, the backend must emit one merged library item rather than multiple user-visible duplicates.

## BR-U7-3 Alternate Source Retention
When duplicates are merged, the backend must preserve alternate source metadata internally for diagnostics and refresh comparison, even though only one preferred item is exposed to the client.

## BR-U7-4 Preferred Source Determination
The preferred source for a merged recording must be selected by source priority rather than arbitrary listing order.

## BR-U7-5 Recorded Playback Session Reuse
Recorded playback must use the shared playback session controller so the backend exposes one consistent session model for Live TV and recorded media.

## BR-U7-6 Explicit Mode Transition
When recorded playback begins, the backend must explicitly transition the active playback session into recorded-playback mode rather than leaving the session type implicit.

## BR-U7-7 Missing Recording Handling
If a requested recording cannot be re-resolved at action time, the backend must return a structured missing-recording result and require a library refresh before retry.

## BR-U7-8 No Silent Action Fallback
After the client has selected a logical recording, the backend must not silently redirect playback or deletion to a different source when the current preferred action target no longer matches.

## BR-U7-9 Delete Target Validation
The backend must validate deletion targets from the current catalog snapshot and must reject deletion when the target cannot be confidently confirmed.

## BR-U7-10 Delete Command Protection
The client must never supply raw delete command URLs or mutation parameters directly to the backend.

## BR-U7-11 Delete Execution Contract
Recording deletion must invoke the record-engine `CmdURL` using HTTP POST with `cmd=delete`.

## BR-U7-12 Sync-Aware Delete Refresh
After a successful delete, the backend must trigger sync-aware refresh behavior so subsequent library reads converge promptly.

## BR-U7-13 Live Stop Resource Release
Explicit Live TV stop must terminate the active live stream and release playback resources promptly without quitting the application.

## BR-U7-14 Context Preservation After Stop
Explicit Live TV stop must preserve remembered device or channel context for later restart unless a broader session-reset action is requested outside this unit.

## BR-U7-15 Degraded Read Versus Strict Mutation
The backend may tolerate degraded reads when assembling the library, but playback and deletion actions must require current validated targets.