# Infrastructure Design Plan - Unit 2 HDHomeRun Discovery and Device Integration

## Execution Checklist
- [x] Review Unit 2 functional design, NFR requirements, and NFR design together
- [x] Define runtime deployment choices for discovery refresh, lineup retrieval, and tuner diagnostics
- [x] Define build and packaging approach for `libhdhomerun` integration in development and distribution formats
- [x] Define Linux permission and network-access assumptions for AppImage, Flatpak, and Debian outputs
- [x] Generate infrastructure-design.md
- [x] Generate deployment-architecture.md

## Planning Focus
- Unit 2 introduces LAN-facing outbound traffic to HDHomeRun devices while keeping the backend loopback-only to the client.
- Unit 2 also introduces a concrete native-library integration story that packaging must preserve.
- Infrastructure design should keep development behavior close to packaged behavior while accounting for sandboxed formats.

## Clarifying Questions

## Question 1
How should `libhdhomerun` be integrated for Unit 2 in v1?

A) Build and ship it as part of this repo and link the backend against the bundled local copy

B) Prefer a system-installed library when available and fall back to a bundled copy

C) Require the user to install the library separately

X) Other

[Answer]: A

## Question 2
What packaging stance should Unit 2 assume for LAN discovery permissions, especially for Flatpak?

A) Design Unit 2 assuming packaged app variants will request the network permissions needed for local LAN discovery and HTTP access

B) Design Unit 2 around manual user configuration for sandbox exceptions

C) Avoid relying on packaged LAN discovery in v1

X) Other

[Answer]: A

## Question 3
Where should the in-memory stale lineup fallback live in v1?

A) Backend process memory only, recreated on each fresh app or backend start

B) Persist stale lineups on disk across restarts

C) Client-side cache instead of backend-owned cache

X) Other

[Answer]: A

## Question 4
How close should development mode stay to packaged runtime for Unit 2?

A) Very close: same backend process boundary, same native library integration path, same LAN behavior where practical

B) Somewhat close: allow dev-only shortcuts for library loading and network behavior

C) Optimize dev convenience first, packaging parity later

X) Other

[Answer]: A

## Question 5
How should periodic discovery refresh run in the deployed app model?

A) Inside the backend process only, with the client just reading results over loopback

B) Client-driven polling that triggers backend refresh on demand

C) Hybrid of backend timer plus client-triggered refreshes as equal primary paths

X) Other

[Answer]: A

## Approved Direction
- Build and ship `libhdhomerun` as part of this repo and link the backend against the bundled local copy.
- Assume packaged variants request the network permissions needed for local LAN discovery and HTTP access.
- Keep stale lineup fallback in backend process memory only.
- Keep development mode very close to packaged runtime, including backend process boundaries, native library integration, and LAN behavior where practical.
- Run periodic discovery refresh inside the backend process, with the client consuming results over loopback.