# Code Generation Plan - Unit 2 HDHomeRun Discovery and Device Integration

## Execution Checklist
- [x] Review Unit 2 functional, NFR, and infrastructure artifacts against the current backend code
- [x] Decide the first implementation slice for discovery, lineup retrieval, and tuner diagnostics
- [x] Decide the Rust integration strategy for bundled `libhdhomerun`
- [x] Decide how provisional Unit 1 contracts evolve into Unit 2 concrete endpoints
- [x] Decide the minimum test slice for the first Unit 2 implementation pass

## Current Backend Baseline
- `backend/src/http/routes.rs` currently exposes provisional `GET /api/devices` and provisional `GET /api/playback/current`.
- `backend/src/models.rs` currently contains placeholder `DevicesResponse` and playback summary models from Unit 1.
- Unit 2 needs real device discovery, selected-device context, lineup retrieval, and tuner diagnostics while preserving client-stable contracts.

## Clarifying Questions

## Question 1
What should the first concrete Unit 2 implementation slice deliver?

A) Discovery plus selected-device resolution and real `GET /api/devices` response first

B) Discovery plus lineup retrieval in one pass

C) Discovery, lineup, and tuner diagnostics all at once

X) Other

[Answer]: A

## Question 2
How should we integrate bundled `libhdhomerun` into the Rust backend first?

A) Add a small FFI wrapper layer directly in this crate with a build script that compiles or links the bundled library

B) Create a separate internal Rust crate for the FFI boundary first

C) Stub the integration and delay native linkage until later

X) Other

[Answer]: A

## Question 3
How should the API contract evolve first in Unit 2?

A) Replace the provisional `/api/devices` response with a richer real response and add separate lineup and tuner-status endpoints

B) Keep `/api/devices` minimal and overload it with lineup data to reduce endpoint count

C) Focus on internal services first and keep public endpoints provisional until the full unit is complete

X) Other

[Answer]: A

## Question 4
What should the first test slice cover?

A) Real HTTP contract tests for discovery endpoints plus fixture-driven normalization tests

B) Pure unit tests only, no endpoint tests yet

C) End-to-end tests only once hardware is connected

X) Other

[Answer]: A

## Question 5
How should tuner diagnostics fit into the first implementation pass?

A) Defer tuner diagnostics until after discovery and lineup are working

B) Include a minimal all-tuners diagnostics endpoint in the same pass as real discovery

C) Implement tuner diagnostics first and lineup later

X) Other

[Answer]: A

## Approved Direction
- First implementation slice: real discovery, selected-device resolution, and a concrete `GET /api/devices` response.
- Native integration strategy: a small FFI wrapper layer directly in the backend crate, backed by a build script that compiles or links the bundled `libhdhomerun` copy.
- API evolution: replace the provisional `/api/devices` response and add separate lineup and tuner-status endpoints rather than overloading one endpoint.
- Initial test slice: HTTP contract tests for discovery endpoints plus fixture-driven normalization tests.
- Tuner diagnostics: defer until after discovery and lineup are working.