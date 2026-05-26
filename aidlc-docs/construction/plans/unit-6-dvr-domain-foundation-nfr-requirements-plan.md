# NFR Requirements Plan - Unit 6 DVR Domain Foundation

## Execution Checklist
- [x] Review Unit 6 functional design, security constraints, and DVR integration risks together
- [x] Identify the NFR areas that matter most for readiness, rule lifecycle, and schedule projection
- [x] Generate context-appropriate NFR questions in this document
- [x] Collect answers for the NFR questions
- [x] Resolve any ambiguity in the answers
- [x] Generate nfr-requirements.md
- [x] Generate tech-stack-decisions.md
- [x] Validate completeness and consistency

## Clarifying Questions

## Question 1
What response-time target should Unit 6 aim for under normal local conditions for DVR readiness checks and rule-state refreshes?

A) Usually under 250 ms

B) Usually under 1 second

C) Usually under 3 seconds

X) Other (please describe after [Answer]: tag below)

[Answer]: B

## Question 2
How should the backend treat upstream retries in this unit?

A) No automatic retries; fail fast with structured errors

B) Allow bounded retries for safe read operations, but not for rule mutations

C) Allow bounded retries for both reads and rule mutations

X) Other (please describe after [Answer]: tag below)

[Answer]: B

## Question 3
How strict should handling of vendor tokens, DeviceAuth material, and upstream-returned DVR metadata be in this unit?

A) Keep sensitive upstream material transient, sanitize logs aggressively, and validate any derived metadata before use

B) Keep the same protections as A, but allow broader debug logging in normal runs

C) Persist more raw upstream material locally to simplify later debugging

X) Other (please describe after [Answer]: tag below)

[Answer]: A

## Question 4
What maintainability and testability expectation should guide Unit 6?

A) Example-based tests only for the initial DVR backend unit

B) Example-based tests plus targeted property-based tests for readiness invariants, rule validation, and schedule projection behavior

C) Minimal tests now, more coverage later in integration hardening

X) Other (please describe after [Answer]: tag below)

[Answer]: B

## Question 5
What tech stack direction should Unit 6 follow?

A) Stay on the established Rust backend stack and reuse the same HTTP, serialization, logging, and PBT direction as earlier backend units

B) Keep Rust, but allow this unit to introduce a different API or async stack if convenient

C) Reconsider the backend language or stack for DVR-specific concerns

X) Other (please describe after [Answer]: tag below)

[Answer]: A

## Selected NFR Decisions
- Performance target: readiness checks and schedule-refresh reads should usually complete within 1 second.
- Retry policy: bounded retries for safe reads only, with no automatic retries for rule mutations.
- Sensitive upstream handling: keep sensitive material transient, sanitize logs aggressively, and validate derived metadata before use.
- Testability: use example-based tests plus targeted property-based tests for readiness, validation, and projection behavior.
- Tech stack: stay on the established Rust backend stack and reuse the same HTTP, serialization, logging, and PBT direction.

## Constraints to Preserve
- Security Baseline remains enabled and blocking.
- Property-based testing remains partially enabled and should be applied where it adds value.
- Backend-owned DVR integration must preserve loopback-only exposure and safe error shaping.