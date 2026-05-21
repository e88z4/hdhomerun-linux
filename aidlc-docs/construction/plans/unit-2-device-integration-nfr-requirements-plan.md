# NFR Requirements Plan - Unit 2 HDHomeRun Discovery and Device Integration

## Execution Checklist
- [x] Review Unit 2 functional design and inherited Unit 1 constraints
- [x] Define performance expectations for discovery, lineup retrieval, and tuner diagnostics
- [x] Define reliability expectations for missing devices, stale lineups, and partial tuner failures
- [x] Define observability and security expectations for vendor-device communication
- [x] Define maintainability and testability expectations for the integration layer
- [x] Generate nfr-requirements.md
- [x] Generate tech-stack-decisions.md if new stack or library decisions are needed

## Planning Focus
- Unit 2 uses Rust backend foundations already chosen in Unit 1.
- Unit 2 introduces device-network behavior, vendor library integration, and HTTP lineup retrieval.
- Unit 2 must preserve loopback-only backend exposure while interacting with LAN devices.

## Clarifying Questions

## Question 1
What should be the v1 discovery freshness target for the client experience?

A) Manual refresh only, plus discovery on startup

B) Startup discovery plus lightweight periodic background refresh

C) Aggressive near-real-time background polling

X) Other

[Answer]: B

## Question 2
What latency expectation should Unit 2 target for normal discovery and lineup loading on a healthy local network?

A) Best effort only, no explicit target

B) Discovery and lineup should usually feel responsive, aiming for about 1 to 2 seconds each under normal conditions

C) Very strict sub-second target for both discovery and lineup

X) Other

[Answer]: B

## Question 3
How should Unit 2 handle partial tuner-status failures?

A) Return available tuner diagnostics and mark only failed tuner entries as unavailable

B) Fail the entire diagnostics response if any tuner lookup fails

C) Hide diagnostics unless all tuners succeed

X) Other

[Answer]: A

## Question 4
How much device-communication detail should logs include by default in v1?

A) High-level structured events only, with deeper device call detail only in debug logging

B) Verbose device call detail at info level

C) Minimal logging even in debug mode

X) Other

[Answer]: A

## Question 5
How deep should testing go for Unit 2 in v1?

A) Example-based tests for discovery normalization and lineup mapping, plus property-based tests for pure normalization invariants

B) Example-based tests only

C) Heavy property-based testing even across device-integration boundaries

X) Other

[Answer]: A

## Approved Direction
- Discovery runs on startup and continues with lightweight periodic background refresh.
- Under healthy local-network conditions, discovery and lineup loading should usually complete within about 1 to 2 seconds each.
- Partial tuner-status failures should degrade gracefully by returning available tuner diagnostics and marking only failed entries unavailable.
- Default logging stays high level and structured, with deeper vendor-call detail only in debug mode.
- Testing should combine example-based tests for discovery and lineup mapping with property-based tests for pure normalization invariants.