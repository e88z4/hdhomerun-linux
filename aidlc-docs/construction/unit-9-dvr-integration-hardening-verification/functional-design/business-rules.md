# Business Rules

## BR-U9-1 Hardening Targets Current DVR Flows
Unit 9 must harden the integrated DVR increment as it exists after Units 6 through 8 rather than redesigning already approved feature boundaries.

## BR-U9-2 Stale Action Outcomes Stay Explicit
Missing-recording, invalid-airing, and degraded-state outcomes must map to explicit refresh or recovery guidance in both backend and client behavior.

## BR-U9-3 Mutation Convergence Must Be Verified
After playback, delete, or rule-creation actions, recordings, upcoming schedule state, and rule state must converge through explicit refresh behavior rather than implicit UI assumptions.

## BR-U9-4 Shared Playback Semantics Must Hold
Recorded playback, live playback, and stop behavior must continue to use the shared playback session model without mode confusion or orphaned state.

## BR-U9-5 Destructive Paths Remain Safety-First
Delete and delete-and-rerecord verification must prefer reversible or low-risk test paths and require explicit operator awareness for real-device runs.

## BR-U9-6 Upstream Targets Stay Backend-Owned
Hardening must preserve the rule that raw upstream playback or delete targets remain transient backend concerns and never become client-controlled inputs.

## BR-U9-7 Verification Artifacts Must Be Actionable
Unit 9 verification output must produce concrete build, smoke-test, and real-device validation steps that can be executed later without reverse engineering the implementation.

## BR-U9-8 Partial DVR Environments Must Degrade Clearly
When storage, schedule, or record-engine availability is partial, the user must see an understandable degraded-state flow instead of silent absence.