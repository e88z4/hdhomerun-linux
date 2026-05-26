# Functional Design Plan - Unit 6 DVR Domain Foundation

## Execution Checklist
- [x] Review Unit 6 responsibilities, story mappings, and application design together
- [x] Define the backend business-logic areas that require detailed modeling for DVR readiness, rules, and schedule projection
- [x] Generate context-appropriate functional-design questions in this document
- [x] Collect answers for the functional-design questions
- [x] Resolve any ambiguity in the answers
- [x] Generate business-logic-model.md
- [x] Generate business-rules.md
- [x] Generate domain-entities.md
- [x] Validate design completeness and consistency

## Unit Context
- **Unit**: DVR Readiness, Rule Lifecycle, and Schedule Projection
- **Folder Name**: `unit-6-dvr-domain-foundation`
- **Purpose**: Establish the backend-owned DVR domain foundation around readiness detection, recording-rule management, and scheduled-state projection.
- **Primary Story Impact**:
  - US-9 Understand whether DVR is available
  - US-14 Create a series recording rule
  - US-15 Create a one-time recording rule
  - US-17 Recognize what will record and what already exists
  - Supporting impact on US-10, US-16, and later recorded-library workflows through shared DVR state

## Clarifying Questions

## Question 1
How detailed should the DVR readiness model be in v1?

A) One simple ready or not-ready result with a single message

B) Structured readiness result with specific missing conditions such as no DeviceAuth, no storage, or no record engine

C) Multi-level readiness with informational, warning, and blocking states

X) Other (please describe after [Answer]: tag below)

[Answer]: B

## Question 2
After creating or editing a recording rule, how should the backend confirm the resulting state?

A) Return success immediately after the write call and let later refreshes catch up

B) Re-fetch rule and schedule state before returning success so the client gets confirmed current state

C) Return provisional success plus an explicit pending-refresh state

X) Other (please describe after [Answer]: tag below)

[Answer]: B

## Question 3
What should drive scheduled-state projection in Unit 6?

A) Upcoming-recordings data only

B) Rule data only, with scheduling inferred locally

C) Hybrid model using vendor upcoming state as primary execution signal plus rule and episode context for explanation

X) Other (please describe after [Answer]: tag below)

[Answer]: C

## Question 4
How should unsupported or partially supported rule options be handled in v1?

A) Reject unsupported options explicitly with structured validation errors

B) Accept and pass through anything the upstream API might tolerate

C) Persist unknown options loosely for future use even if the backend cannot reason about them yet

X) Other (please describe after [Answer]: tag below)

[Answer]: A

## Question 5
How should the backend handle stale or no-longer-valid airings for one-time rules?

A) Return a clear invalid-airing result and require the client to select again

B) Try a best-effort match to a nearby airing automatically

C) Create the rule anyway and let downstream scheduling determine whether it works

X) Other (please describe after [Answer]: tag below)

[Answer]: A

## Selected Functional Design Decisions
- Readiness model: structured readiness result with explicit missing conditions.
- Rule confirmation: re-fetch rule and schedule state before reporting success.
- Schedule projection: hybrid model using vendor upcoming state as the primary execution signal plus rule and episode context for explanation.
- Unsupported options: reject explicitly with structured validation errors.
- Stale airings: return a clear invalid-airing outcome and require reselection.

## Constraints to Preserve
- Backend owns all DVR API integration.
- Readiness and rule state must be understandable without exposing raw vendor payloads.
- Rule support must include both series and one-time models.
- Local-only security assumptions still require strict input validation and safe error shaping.