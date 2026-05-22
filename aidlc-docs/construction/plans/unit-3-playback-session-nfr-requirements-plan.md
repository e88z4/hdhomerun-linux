# NFR Requirements Plan - Unit 3 Playback Session Orchestration and Player Adapter

## Execution Checklist
- [x] Review Unit 3 functional design and inherited Unit 1 and Unit 2 constraints
- [x] Define performance expectations for playback start, channel switch, and stop behavior
- [x] Define reliability expectations for persistent session reuse, retry boundaries, and adapter failure recovery
- [x] Define observability and security expectations for backend-owned `mpv` control
- [x] Define maintainability and testability expectations for playback orchestration
- [x] Generate nfr-requirements.md
- [x] Generate tech-stack-decisions.md if new stack or process-control decisions are needed

## Planning Focus
- Unit 3 introduces a backend-owned persistent `mpv` process and direct live-stream handoff.
- Unit 3 must keep playback failures structured and retry-safe without destabilizing the backend.
- Unit 3 should stay compatible with the later Qt/QML client while owning orchestration server-side.

## Clarifying Questions

## Question 1
What responsiveness target should Unit 3 aim for in v1 under normal local conditions?

A) Best effort only, no explicit target

B) Initial playback start should usually feel responsive, around 2 to 4 seconds, with channel switches faster when the persistent session is healthy

C) Very strict near-instant playback and switch targets from the start

X) Other

[Answer]: B

## Question 2
How should Unit 3 treat the persistent `mpv` process after a normal stop?

A) Keep the process available for reuse for a short-lived session window when practical

B) Always tear down the process immediately on stop

C) Keep it running indefinitely until backend shutdown

X) Other

[Answer]: A

## Question 3
How much process-level detail should logs expose by default for playback orchestration?

A) High-level structured playback lifecycle events only, with `mpv` command or IPC detail reserved for debug mode

B) Verbose `mpv` command detail at info level

C) Minimal logging even in debug mode

X) Other

[Answer]: A

## Question 4
What testing depth should Unit 3 aim for in v1?

A) HTTP contract tests plus orchestration unit tests with a fake player adapter

B) Pure unit tests only, no playback API contract tests yet

C) Depend mainly on manual end-to-end testing with a real player process

X) Other

[Answer]: A

## Question 5
How should Unit 3 handle adapter crashes or unrecoverable player errors?

A) Mark the session failed, rebuild the adapter only on the next explicit or bounded retry path, and keep failures structured

B) Automatically respawn the adapter continuously until it recovers

C) Leave the backend unaware and let the client infer failure from missing playback

X) Other

[Answer]: A

## Approved Direction
- Initial playback start should usually land in roughly 2 to 4 seconds under normal local conditions, with faster switches when the persistent session is healthy.
- The backend should keep the `mpv` process available for short-lived reuse after a normal stop whenever practical.
- Default logs should stay at high-level structured lifecycle events, with raw command or IPC detail reserved for debug mode.
- Unit 3 should ship with HTTP contract tests plus orchestration unit tests that run against a fake player adapter.
- Adapter crashes or unrecoverable player failures should mark the session failed and rebuild the adapter only on the next explicit or bounded retry path.