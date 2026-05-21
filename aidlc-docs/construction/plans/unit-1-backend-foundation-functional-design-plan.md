# Functional Design Plan - Unit 1 Backend Foundation and Local API

## Execution Checklist
- [x] Review Unit 1 responsibilities, story mappings, and application design together
- [x] Define the backend business logic model for startup, readiness, state ownership, and API contracts
- [x] Define the domain entities and relationships for backend foundation concerns
- [x] Define business rules for restore behavior, state persistence, and error shaping
- [x] Define input validation and failure-handling rules at the loopback API boundary
- [x] Generate business-logic-model.md
- [x] Generate business-rules.md
- [x] Generate domain-entities.md
- [x] Validate design completeness and consistency

## Unit Context
- **Unit**: Backend Foundation and Local API
- **Purpose**: Establish the reusable service runtime, loopback HTTP/JSON boundary, structured logging, validation boundary, and canonical state ownership.
- **Primary Story Impact**:
  - US-1 Reopen the last viewing context
  - US-2 Handle first launch cleanly
  - Supporting impact on US-5, US-6, and US-8 through shared contracts and error handling

## Clarifying Questions

## Question 1
What should happen if the desktop client starts and the backend process is not running?

A) The client should automatically start the backend and wait for readiness

B) The client should prompt the user before starting the backend

C) The client should only connect to an already-running backend

X) Other (please describe after [Answer]: tag below)

[Answer]: A

## Question 2
What should the backend persist in v1 as canonical remembered state?

A) Only last used device and last watched channel

B) A plus last known playback state such as whether playback should auto-resume

C) A plus richer session metadata for future expansion

X) Other (please describe after [Answer]: tag below)

[Answer]: B

## Question 3
How should the backend treat remembered state if the stored device is no longer available at launch?

A) Clear the device-specific remembered state and return a fallback selection-needed result

B) Keep the remembered state untouched and let the client decide everything

C) Try a best-effort fallback to another discovered device automatically

X) Other (please describe after [Answer]: tag below)

[Answer]: A

## Question 4
How much should the Unit 1 API expose before device integration and playback units are implemented?

A) Health, state, and contract placeholder endpoints only

B) Health and state endpoints plus provisional device and playback endpoints with stable response shapes

C) Minimal health endpoint only until later units fill everything in

X) Other (please describe after [Answer]: tag below)

[Answer]: B

## Question 5
What error style should the loopback API use in v1?

A) Stable structured error objects with code, message, and retry hint

B) Simple status code plus plain message

C) Detailed diagnostic payloads by default

X) Other (please describe after [Answer]: tag below)

[Answer]: A

## Constraints to Preserve
- Loopback-only exposure
- Structured logging without sensitive data
- Validation at every API boundary
- Backend-owned canonical restore state
- Compatibility with both auto-started and managed-service runtime modes