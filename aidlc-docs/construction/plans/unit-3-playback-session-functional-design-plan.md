# Functional Design Plan - Unit 3 Playback Session Orchestration and Player Adapter

## Execution Checklist
- [x] Review Unit 3 responsibilities, story mappings, and the current Unit 2 backend surface
- [x] Define the business logic model for playback session lifecycle and channel switching
- [x] Define domain entities for playback session state, player adapter state, and retryable failures
- [x] Define business rules for persistent session reuse, source handoff, and recoverable playback errors
- [x] Define integration boundaries between backend orchestration and mpv or libmpv control
- [x] Generate business-logic-model.md
- [x] Generate business-rules.md
- [x] Generate domain-entities.md
- [x] Validate completeness and consistency

## Unit Context
- **Unit**: Playback Session Orchestration and Player Adapter
- **Purpose**: Implement persistent playback sessions and connect backend orchestration to mpv or libmpv.
- **Primary Story Impact**:
  - US-5 Start live playback immediately from channel selection
  - US-6 Switch channels inside one persistent player session
  - US-8 Recover from tuner or stream startup failures
  - supporting impact on US-1 through remembered playback context

## Current Technical Baseline
- Unit 2 now exposes concrete `/api/devices`, `/api/lineup`, and `/api/tuners` endpoints.
- `/api/playback/current` is still provisional and is the natural contract entry point for Unit 3.
- The approved architecture calls for a persistent in-app player session backed by mpv or libmpv.

## Clarifying Questions

## Question 1
What should own the first real playback adapter in Unit 3 v1?

A) Backend-owned persistent mpv process controlled through a stable adapter boundary

B) Backend-owned direct libmpv in-process adapter from the start

C) Delay real player integration and keep Unit 3 mostly contract-only for now

X) Other

[Answer]: A

## Question 2
How should channel switching behave in the first Unit 3 implementation?

A) Reuse one persistent player session and replace the current stream inside that session

B) Tear down and recreate the player for every channel switch

C) Use a hybrid approach depending on the failure type

X) Other

[Answer]: A

## Question 3
How should the backend hand playback sources to the player in v1?

A) Pass the direct HDHomeRun playback URL from Unit 2 into the player adapter

B) Proxy the stream through the backend before handing it to the player

C) Decide dynamically per channel or device type

X) Other

[Answer]: A

## Question 4
What retry posture should Unit 3 use for startup or tune failures?

A) One bounded automatic retry for clearly retryable startup failures, then surface a structured error

B) No automatic retry, only explicit user-triggered retry

C) Aggressive background retry until playback succeeds or the user cancels

X) Other

[Answer]: A

## Question 5
What should the first real playback API surface include in Unit 3?

A) Current session state plus start, stop, and switch-channel endpoints

B) Only current session state and start, leaving switch-channel for later

C) Current session state only in the first pass

X) Other

[Answer]: A

## Approved Direction
- Unit 3 starts with a backend-owned persistent `mpv` process behind a stable adapter boundary.
- Channel switching reuses one persistent player session and replaces the current stream inside that session.
- The backend hands the direct HDHomeRun playback URL from Unit 2 to the player adapter.
- Playback startup gets one bounded automatic retry for clearly retryable startup failures before surfacing a structured error.
- The first real playback API surface includes current session state plus start, stop, and switch-channel endpoints.

## Constraints to Preserve
- Keep the backend as the canonical orchestration authority.
- Preserve compatibility with the later Qt/QML client integration.
- Avoid introducing a second discovery or lineup authority in the playback path.
- Keep failure results structured and suitable for inline retry UX.