# Application Design Plan

## Execution Checklist
- [x] Review requirements, stories, and execution plan together
- [x] Confirm component boundaries for backend, client, playback, and state persistence
- [x] Confirm service orchestration and startup model
- [x] Confirm local API style and communication pattern
- [x] Generate components.md
- [x] Generate component-methods.md
- [x] Generate services.md
- [x] Generate component-dependency.md
- [x] Generate application-design.md
- [x] Validate design completeness and consistency

## Mandatory Artifacts
- [x] components.md with component definitions and responsibilities
- [x] component-methods.md with high-level method signatures
- [x] services.md with orchestration and service interactions
- [x] component-dependency.md with dependency relationships and communication patterns
- [x] application-design.md consolidating the full design

## Current Design Direction
- A standalone local backend service is bundled with the desktop app.
- A Qt/QML desktop client provides the user-facing Linux experience.
- Playback is persistent and embedded in the app rather than delegated to an external window.
- `libhdhomerun` anchors device discovery and tuner control.
- Packaging must support AppImage, Flatpak, and Debian.

## Clarifying Questions

## Question 1
What local API style should the Qt/QML client use to communicate with the backend service in v1?

A) HTTP/JSON over loopback only

B) Unix domain socket with a structured RPC or message protocol

C) Hybrid: HTTP/JSON for control plus event streaming for status updates

X) Other (please describe after [Answer]: tag below)

[Answer]: A

## Question 2
How should the bundled backend service be started in v1?

A) Desktop app starts and supervises it automatically

B) System service or user service managed outside the app

C) Support both, but desktop app auto-start is the primary path

X) Other (please describe after [Answer]: tag below)

[Answer]: C

## Question 3
Where should remembered state such as last device and last channel live in v1?

A) Backend-owned local state only

B) Client-owned local state only

C) Backend owns canonical state, client may cache UI preferences

X) Other (please describe after [Answer]: tag below)

[Answer]: C

## Question 4
How much tuner and signal detail should the v1 design assume in the main client surface?

A) Compact summary in the main player view

B) Dedicated diagnostics panel or drawer in addition to summary indicators

C) Diagnostics-first view with lots of device detail visible by default

X) Other (please describe after [Answer]: tag below)

[Answer]: B

## Security and Design Constraints
- Local service exposure should remain limited to the local machine.
- API inputs must be designed with explicit validation boundaries.
- Logging should be structured and avoid sensitive data.
- Design should leave space for layered defenses rather than a single control point.