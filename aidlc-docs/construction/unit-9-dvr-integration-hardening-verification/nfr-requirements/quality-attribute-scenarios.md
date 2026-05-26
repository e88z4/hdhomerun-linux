# Quality Attribute Scenarios - Unit 9 DVR Integration Hardening and Verification

## Scenario 1 - Missing Recording During Delete
- **Source**: user in DVR workspace
- **Stimulus**: requests delete for a recording that disappeared after the last recordings refresh
- **Environment**: normal client operation with changing DVR library state
- **Artifact**: backend delete endpoint and DVR workspace selection state
- **Response**: backend returns missing-recording outcome, client clears ambiguity, and user gets explicit refresh guidance
- **Response Measure**: no silent success, no stale destructive target execution, and no orphaned selected-recording UI state

## Scenario 2 - Ambiguous Rule Context From Recording Details
- **Source**: user opens rule options from a recording whose series context cannot be trusted uniquely
- **Stimulus**: attempts recovery or scheduling action from recording details
- **Environment**: partial schedule or rule context available
- **Artifact**: rule-entry presentation logic and backend mutation boundary
- **Response**: client disables unsafe rule action paths and explains the missing trustworthy context
- **Response Measure**: no invalid rule mutation request is issued from ambiguous state

## Scenario 3 - Playback Mode Transition After Recorded Playback
- **Source**: user starts a recorded item, then stops playback or returns to live playback
- **Stimulus**: playback transition request
- **Environment**: shared playback controller in active use
- **Artifact**: playback session model and DVR workspace context
- **Response**: session mode, current playback state, and workspace context remain coherent across the transition
- **Response Measure**: no mixed live-versus-recorded presentation state and no incorrect current-recording metadata after stop

## Scenario 4 - Degraded DVR Readiness With Partial Schedule Availability
- **Source**: backend readiness surface reports degraded state while some DVR data remains available
- **Stimulus**: user opens or refreshes the DVR workspace
- **Environment**: record-engine or storage partially degraded
- **Artifact**: client banners, recordings area, upcoming area, and verification guidance
- **Response**: degraded guidance remains visible while still showing any safely available DVR data
- **Response Measure**: user can distinguish degraded operation from total DVR absence without opening diagnostics-only UI