# Functional Design Plan - Unit 8 DVR Client Workspace and Rule-Management UX

## Execution Checklist
- [x] Review Unit 8 responsibilities, story mappings, and approved backend contracts together
- [x] Define the client-side behavior areas that require detailed design for DVR workspace layout, recordings interactions, and rule-management flows
- [x] Generate context-appropriate functional-design questions in this document
- [x] Collect answers for the functional-design questions
- [x] Resolve any ambiguity in the answers
- [x] Generate business-logic-model.md
- [x] Generate business-rules.md
- [x] Generate domain-entities.md
- [x] Validate design completeness and consistency

## Unit Context
- **Unit**: DVR Client Workspace and Rule-Management UX
- **Folder Name**: `unit-8-dvr-client-workspace`
- **Purpose**: Deliver the client-side DVR tab, recorded-library UX, recorded playback actions, delete interactions, and rule-management entry points on top of the approved backend DVR contracts.
- **Primary Story Impact**:
  - US-10 Switch between Live TV and DVR workspaces
  - US-11 Browse recorded content with useful defaults
  - US-12 Play a recorded show inside the app
  - US-13 Delete a recording safely
  - US-16 Manage rule detail with flexible options
  - Supporting impact on US-17 and US-18

## Clarifying Questions

## Question 1
How should the DVR recordings list be organized in the first client iteration?

A) Single flat list sorted by most recent recording first

B) Group by series title with expandable rows for episodes

C) Split into separate sections for recent recordings and all recordings

X) Other (please describe after [Answer]: tag below)

[Answer]:
B

## Question 2
When the user starts recorded playback, how should the DVR workspace react?

A) Stay on the DVR workspace and swap the player panel into recorded playback state

B) Switch back to the Live TV workspace because the same player surface is reused

C) Open recorded playback in a separate modal or overlay player

X) Other (please describe after [Answer]: tag below)

[Answer]:
A

## Question 3
What confirmation behavior should the first delete flow use?

A) Require a simple confirm dialog with Delete and Cancel

B) Require a confirm dialog with Delete, Delete & Re-record, and Cancel

C) Allow immediate delete with inline undo

X) Other (please describe after [Answer]: tag below)

[Answer]:
B

## Question 4
How visible should DVR readiness and degraded-state warnings be in the workspace?

A) Show a prominent top-of-workspace status banner until the issue is cleared

B) Show compact inline status near the affected panel only

C) Hide them behind a diagnostics drawer unless the user opens it

X) Other (please describe after [Answer]: tag below)

[Answer]:
A

## Question 5
How should rule creation be entered from the first DVR workspace release?

A) Offer clear entry points from both upcoming items and recording details, but keep editing in a focused panel or sheet

B) Only allow rule creation from guide or upcoming items in this release

C) Only allow rule creation from a dedicated rules tab or panel in this release

X) Other (please describe after [Answer]: tag below)

[Answer]:
A

## Constraints to Preserve
- The backend remains the owner of all DVR integration, validation, and deletion behavior.
- The client should not expose raw action URLs or upstream mutation details.
- Live TV stop and recorded playback must coexist with one shared player model.
- The first client release should favor clear, low-risk interactions over dense power-user flows.