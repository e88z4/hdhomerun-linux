# Domain Entities

## DvrIntegrationVerificationMatrix
- **Purpose**: Tracks the integrated DVR scenarios that must pass across backend, client, and real-device validation.
- **Fields**:
  - `scenarioId`
  - `storyCoverage[]`
  - `backendContractArea`
  - `clientSurfaceArea`
  - `verificationType` (`unit | local_smoke | real_device`)
  - `status`

## DvrRefreshConvergenceScenario
- **Purpose**: Represents a state refresh path after a playback, delete, or rule mutation.
- **Fields**:
  - `triggerAction`
  - `affectedEndpoints[]`
  - `expectedWorkspaceUpdates[]`
  - `recoveryAction`

## DvrStaleActionOutcomeModel
- **Purpose**: Represents stale or invalid DVR action outcomes that require explicit recovery.
- **Fields**:
  - `outcomeType` (`missing_recording | invalid_airing | degraded_state`)
  - `backendMessage`
  - `clientBannerState`
  - `requiredRefresh`

## DvrPlaybackTransitionScenario
- **Purpose**: Captures the shared playback transitions that must remain coherent after DVR integration.
- **Fields**:
  - `startingMode` (`idle | live | recorded`)
  - `trigger`
  - `endingMode`
  - `expectedSessionState`
  - `expectedWorkspaceContext`

## DvrRuleContextResolutionScenario
- **Purpose**: Describes whether recording or upcoming context is strong enough to support rule creation.
- **Fields**:
  - `entrySource` (`recording_details | upcoming_item`)
  - `contextStrength` (`trusted | inferred | insufficient`)
  - `seriesActionAvailable`
  - `oneTimeActionAvailable`
  - `fallbackMessage`

## RealDeviceTestCheckpoint
- **Purpose**: Represents a real-hardware verification point that may be resumed later.
- **Fields**:
  - `checkpointId`
  - `environmentPreconditions[]`
  - `action`
  - `expectedEvidence`
  - `cleanupRequired`