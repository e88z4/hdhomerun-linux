# NFR Design Patterns - Unit 9 DVR Integration Hardening and Verification

## 1. Explicit Outcome Normalization Pattern
- **Pattern**: Normalize hard DVR edge cases into stable named outcomes before they reach user-facing workflows.
- **Why**: Integration hardening fails if each layer interprets stale or invalid state differently.
- **Behavior**:
  - missing recording remains a distinct recovery outcome
  - invalid airing remains a distinct rule-mutation rejection outcome
  - degraded readiness remains distinct from total unavailability

## 2. Ordered Refresh Convergence Pattern
- **Pattern**: Apply explicit refresh ordering after DVR mutations instead of relying on incidental UI refresh timing.
- **Why**: Recordings, schedule projection, and rules can otherwise drift after delete or rule creation.
- **Behavior**:
  - delete refreshes recordings and related DVR context deliberately
  - rule mutations refresh rules and upcoming state deliberately
  - stale selection is cleared or reselected explicitly

## 3. Shared Playback Integrity Pattern
- **Pattern**: Preserve one playback session model while verifying that mode transitions remain presentation-safe.
- **Why**: The shared player is an architectural advantage only if mode changes stay coherent.
- **Behavior**:
  - recorded playback enters recorded mode explicitly
  - stop clears recorded state explicitly
  - switching back to live playback does not leak recorded metadata

## 4. Trust-Gated Recovery Pattern
- **Pattern**: Permit recovery mutations only when the rule context is trusted enough for safe action.
- **Why**: Delete-and-rerecord should never create a misleading or low-confidence mutation request.
- **Behavior**:
  - trusted context enables the action
  - inferred-but-unique context remains explicit and explainable
  - insufficient context disables the mutation and surfaces the reason

## 5. Banner-Backed Recovery Pattern
- **Pattern**: Pair normalized edge outcomes with persistent banner guidance until the user refreshes or the state clears.
- **Why**: Hardening is ineffective if critical recovery instructions disappear too quickly.
- **Behavior**:
  - stale-state outcomes map to recovery banners
  - degraded environments keep warning context visible while safe data remains available
  - success banners remain short and action-oriented

## 6. Verification Layering Pattern
- **Pattern**: Split Unit 9 verification into helper tests, client or backend smoke coverage, and resumable real-device checkpoints.
- **Why**: DVR hardening spans deterministic code paths and environment-dependent behavior.
- **Behavior**:
  - deterministic helpers stay unit-testable
  - local integration surfaces stay smoke-testable in build automation
  - real-device checkpoints remain documented for later resumption