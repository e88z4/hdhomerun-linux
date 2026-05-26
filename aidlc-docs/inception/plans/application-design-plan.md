# DVR Application Design Plan

## Execution Checklist
- [x] Review approved DVR requirements and user stories
- [x] Review existing Linux player components and services
- [x] Collect answers for DVR application-design questions in this document
- [x] Resolve any ambiguity in the answers
- [x] Generate components.md updates for DVR-specific components
- [x] Generate component-methods.md updates for DVR-specific interfaces
- [x] Generate services.md updates for DVR-specific services and orchestration
- [x] Generate component-dependency.md updates for DVR communication patterns
- [x] Generate application-design.md summary updates for the DVR increment
- [x] Validate design completeness and consistency

## Design Focus
- Introduce DVR-specific backend services without breaking the current Live TV architecture.
- Define how recorded library, rule management, DVR readiness, and Live TV stop control fit into the existing client and backend boundaries.
- Keep the backend as the sole owner of DVR API integration.

## Selected Design Decisions
- DVR tab structure: split layout with recordings on one side and details or actions on the other.
- Rule editor placement: hybrid approach with contextual entry and fuller dedicated editing when needed.
- Storage-source presentation: merged default view plus optional source-level filtering or diagnostics.
- Backend API shape: hybrid approach with a clear DVR endpoint group plus selective reuse of existing playback and guide boundaries.
- Live TV stop control placement: playback overlay or primary player controls.

## Clarifying Questions

## Question 1
How should the DVR tab be structured at a high level?

A) Library-first layout with recordings as the default primary view

B) Dashboard layout with readiness, upcoming state, rules, and recordings visible together

C) Split layout with recordings on one side and details or actions on the other

X) Other (please describe after [Answer]: tag below)

[Answer]: C

## Question 2
Where should recording-rule creation and editing live in the UX?

A) Dedicated rule editor view inside the DVR tab

B) Contextual drawer or modal opened from recordings, guide, or episode actions

C) Hybrid approach with quick actions in context and a fuller dedicated editor when needed

X) Other (please describe after [Answer]: tag below)

[Answer]: C

## Question 3
How should storage sources be presented to the user?

A) Mostly merged into one DVR view, with source details shown only when useful

B) Explicit source selector as a primary control

C) Merged default view plus optional source-level filtering or diagnostics

X) Other (please describe after [Answer]: tag below)

[Answer]: C

## Question 4
How should the backend expose DVR APIs to the client?

A) Separate DVR endpoint group with recordings, rules, readiness, and status endpoints

B) Extend existing guide and playback endpoints heavily rather than adding a clear DVR surface

C) Hybrid approach with a clear DVR group plus selective reuse of existing playback or guide endpoints

X) Other (please describe after [Answer]: tag below)

[Answer]: C

## Question 5
Where should the explicit Live TV stop control live in the interface?

A) In the playback overlay or primary player controls

B) In a header or tab-level action area

C) In both a visible playback control and a secondary menu path

X) Other (please describe after [Answer]: tag below)

[Answer]: A