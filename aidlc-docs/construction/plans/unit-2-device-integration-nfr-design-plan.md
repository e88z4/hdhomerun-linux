# NFR Design Plan - Unit 2 HDHomeRun Discovery and Device Integration

## Execution Checklist
- [x] Review Unit 2 functional design and NFR requirements together
- [x] Choose resilience patterns for discovery, lineup retrieval, and tuner diagnostics
- [x] Define logical components that satisfy observability, maintainability, and partial-failure handling needs
- [x] Generate nfr-design-patterns.md
- [x] Generate logical-components.md

## Planning Focus
- Unit 2 needs explicit patterns for lightweight background refresh, bounded LAN-device calls, and partial tuner-status degradation.
- Unit 2 should isolate vendor-facing integration from client-facing contract shaping.
- Unit 2 should avoid turning transient LAN problems into backend-wide instability.

## Clarifying Questions

## Question 1
What refresh pattern should Unit 2 use in v1?

A) Startup discovery plus a fixed lightweight background refresh interval

B) Startup discovery plus only manual refresh after that

C) Adaptive refresh intervals based on device changes and errors

X) Other

[Answer]: A

## Question 2
How should Unit 2 handle lineup data between refreshes?

A) Keep the last successful lineup in memory for the selected device and mark it stale when refresh fails

B) Discard lineup data immediately on refresh failure

C) Persist lineups across app restarts in v1

X) Other

[Answer]: A

## Question 3
What timeout or retry posture should Unit 2 use for LAN device calls in v1?

A) Bounded timeouts with no automatic retry loops except the scheduled refresh cycle

B) Automatic immediate retries for most device calls

C) Long timeouts to avoid false failures

X) Other

[Answer]: A

## Question 4
How should the integration layer be separated logically?

A) Distinct discovery adapter, lineup adapter, normalization layer, and contract service

B) One combined device-integration service for simplicity in v1

C) Push more normalization into the client

X) Other

[Answer]: A

## Question 5
How should partial tuner failures be represented internally?

A) Per-tuner result isolation with a synthesized overall diagnostics summary

B) Single shared diagnostics status for the whole device

C) Best-effort logs only with no structured partial-failure representation

X) Other

[Answer]: A

## Approved Direction
- Use startup discovery plus a fixed lightweight background refresh interval.
- Keep the last successful lineup in memory for the selected device and mark it stale when refresh fails.
- Use bounded timeouts with no automatic retry loops outside the scheduled refresh cycle.
- Separate the integration layer into discovery adapter, lineup adapter, normalization layer, and contract service.
- Represent partial tuner failures with per-tuner isolation and a synthesized overall diagnostics summary.