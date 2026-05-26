# DVR Feature Requirements

## Intent Analysis Summary
- **User Request**: Extend the existing HDHomeRun Linux Player with DVR functionality using the full AI-DLC process and then implement it.
- **Request Type**: New user-facing feature in an existing brownfield project.
- **Scope Estimate**: Multiple components, including backend DVR integration, storage-engine integration, client navigation changes, DVR metadata projection, recorded-playback UX, and a Live TV stop-control enhancement.
- **Complexity Estimate**: Complex.

## Product Summary

Add DVR support to the existing HDHomeRun Linux Player as a first-class product capability. The DVR feature must be integrated into the current two-part architecture with the backend owning all DVR API interaction. The client should expose a dedicated DVR tab alongside Live TV, support recorded-library browsing first, and still deliver a first meaningful increment where users can create recording rules inside the app and play back recorded shows.

The DVR feature must integrate both SiliconDust vendor-side rule and schedule APIs and local storage-engine APIs. The product must also detect and explain missing DVR prerequisites rather than silently failing when live TV is available but DVR is not. As an additional feature requirement, the app must provide an explicit way to stop active Live TV streaming cleanly.

## Functional Requirements

### FR-1 DVR Readiness Detection
- The backend must detect whether DVR prerequisites are available for the selected HDHomeRun environment.
- The system must detect the presence or absence of local storage-engine endpoints that publish `StorageURL`.
- The UI must surface a clear DVR-readiness state when live TV works but DVR prerequisites are incomplete.
- The system must distinguish between tuner availability and DVR availability.

### FR-2 DVR Information Architecture
- The client must provide a dedicated DVR tab alongside the existing Live TV tab.
- The user must be able to switch between Live TV and DVR without losing the overall application context.
- DVR workflows should be grouped in the DVR tab rather than scattered only across the Live TV guide surface.

### FR-3 Recorded Library Browsing
- The backend must retrieve recorded-library entries from available HDHomeRun storage-engine endpoints.
- The backend must prioritize local storage engines ahead of non-local storage engines when presenting or selecting DVR storage sources.
- The UI must present a browsable recorded-library experience in the DVR tab.
- The recorded-library view must support enough metadata for the user to identify recorded content reliably.

### FR-4 Recorded Playback
- The user must be able to start playback of recorded content from inside the application.
- Recorded playback must remain inside the app rather than requiring an external player.
- The backend must provide the client with the necessary recorded-playback integration boundary.

### FR-5 Recorded Deletion
- The user must be able to delete recorded items from inside the application.
- Deletion must be driven through the local storage-engine control surface associated with the recording.
- The product should be designed so delete-related actions can remain safe and explicit.

### FR-6 Recording Rule Management
- The user must be able to create and manage recording rules inside the application.
- The first usable DVR version must support both Series rules and one-time DateTime plus Channel rules.
- The backend must own rule listing, creation, update, deletion, and any required synchronization logic.

### FR-7 Full Rule Flexibility
- The first usable DVR version should expose the most flexible HDHomeRun rule surface practical for the supported APIs.
- The design must account for advanced rule options such as channel restrictions, team restrictions, padding, and original-airdate-based filtering.
- The app should avoid artificially constraining the rule model to a shallow subset if the API supports richer behavior.

### FR-8 Scheduling and Status Projection
- The UI must show whether a program is going to be recorded or has already been recorded using clear schedule-aware visual status cues.
- Schedule and library state should be projected into the UI in a way that helps the user understand recording intent and recorded outcomes.
- The DVR feature must account for guide-aware status coloring rather than relying only on a separate textual upcoming-recordings list.

### FR-9 Backend-Owned DVR Integration
- The backend must own all DVR API integration.
- The client must consume DVR capabilities only through stable loopback application APIs.
- The architecture must avoid direct client calls to vendor DVR APIs or local storage-engine APIs.

### FR-10 Explicit Live TV Stop Control
- The product must provide an explicit way for the user to stop active Live TV streaming without quitting the application.
- Stopping Live TV must terminate the active live-streaming session cleanly.
- The backend must release the associated tuner or playback resources promptly when Live TV is stopped.
- The UI should make the stop action discoverable as a normal playback control rather than requiring the user to infer it from navigation alone.

## Sequencing Requirements

### SR-1 Library-First Delivery Order
- Implementation should begin with recorded-library capabilities first.
- Early slices should establish recorded-list retrieval, playback readiness, deletion wiring, and storage prioritization.

### SR-2 First Increment Success Bar
- Even with a library-first implementation order, the first meaningful DVR increment is not considered successful unless the user can set a recording rule inside the app and play back the resulting recorded show.
- This means rule management and recorded playback must both land in the first approved DVR feature increment.

## Non-Functional Requirements

### NFR-1 Security Baseline
- Security baseline rules remain enabled and must be treated as blocking constraints.
- The backend must keep DVR integration loopback-only unless explicitly re-approved otherwise.
- Delete and control actions must validate inputs carefully and avoid unsafe exposure.

### NFR-2 Testability
- Property-based testing remains partially enabled.
- Pure DVR-related normalization, merge, ordering, prioritization, and serialization logic should be designed so it can be property-tested.
- Example-based tests and integration-style contract tests are also required where property tests are not the best fit.

### NFR-3 UX Clarity
- DVR readiness failures must be clear and recoverable.
- The split between Live TV and DVR workflows must feel intentional rather than bolted on.
- Status indicators for recording and recorded state must be understandable without exposing raw API jargon.

### NFR-4 Architectural Maintainability
- Vendor rule and guide APIs, local storage-engine APIs, and client presentation concerns must remain clearly separated.
- The backend should project a coherent DVR domain model instead of leaking raw vendor payloads straight into the client.

### NFR-5 Compatibility with Existing Live TV Product
- DVR work must preserve the existing live-TV architecture and not destabilize the current Live TV user journey.
- The design should extend the current client-backend shape rather than creating a second disconnected product path.
- The added DVR scope should also improve Live TV session control where needed, including an explicit stop behavior.

## Acceptance Criteria
- The application exposes a dedicated DVR tab alongside Live TV.
- The backend can detect whether DVR prerequisites are available and explain missing prerequisites when they are not.
- The app can browse recorded content from HDHomeRun storage engines with local-storage priority.
- The user can play a recorded show inside the application.
- The user can delete a recorded show from inside the application.
- The user can create at least one supported recording rule type inside the application.
- The first usable DVR release supports both Series rules and one-time rules.
- The UI indicates when content is scheduled to record or already recorded.
- The backend remains the sole owner of DVR API integration.
- The user can explicitly stop Live TV streaming and the active live session ends cleanly.

## Key Decisions Captured
- DVR is now in scope as a new feature increment for the Linux player.
- The client should gain a dedicated DVR tab rather than mixing all DVR behavior only into Live TV.
- Delivery should start with recorded-library work, but the first meaningful increment must still include rule creation plus recorded playback.
- Both Series and one-time recording rules are required.
- Recorded playback and deletion are in scope.
- Backend-owned DVR integration is mandatory.
- Storage ordering should prefer local storage first, then non-local storage.
- The app must detect and explain missing DVR prerequisites.
- Explicit stopping of Live TV streaming is now in scope as an additional feature requirement.
- Security remains enforced and property-based testing remains partially enabled.