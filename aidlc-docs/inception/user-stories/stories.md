# User Stories

## Epic 1: Launch and Restore

### Story US-1: Reopen the last viewing context
**As a** returning Linux viewer  
**I want** the app to restore my last used device and last watched channel when possible  
**So that** I can get back to live TV with minimal friction.

**Primary Persona**: Alex

**Acceptance Criteria**
- When the app starts, it checks whether a previously used device is known.
- If the remembered device is available, the app restores that device selection automatically.
- If the remembered device is not available, the app asks the user to choose a device.
- When a last watched channel is known and available, the app reopens that channel on launch.
- If the last watched channel is unavailable, the app falls back to the channel list without crashing.

### Story US-2: Handle first launch cleanly
**As a** first-time Linux viewer  
**I want** a clear startup flow when no prior device or channel state exists  
**So that** I can begin using the app without configuration confusion.

**Primary Persona**: Alex

**Acceptance Criteria**
- On first launch, the app scans for available HDHomeRun devices.
- If multiple devices are found and none are remembered, the app asks the user to pick one.
- If exactly one device is found and no remembered state exists, the app can continue into the normal browsing flow after selection logic is resolved.
- If no devices are found, the app presents a clear recoverable state instead of an empty failure screen.

## Epic 2: Discover and Browse

### Story US-3: See discovered devices
**As a** Linux viewer  
**I want** to see which HDHomeRun devices are available  
**So that** I can choose the correct device for live viewing.

**Primary Persona**: Alex

**Acceptance Criteria**
- The UI lists discovered HDHomeRun devices returned by the backend.
- Each device entry provides enough identity for the user to distinguish devices.
- The UI clearly indicates the currently selected device.
- Switching the selected device refreshes channel and tuner-related data.

### Story US-4: Browse channel lineup
**As a** Linux viewer  
**I want** a browsable channel list for the selected device  
**So that** I can quickly choose what to watch.

**Primary Persona**: Alex

**Acceptance Criteria**
- The app loads the available channel lineup for the selected device.
- The UI presents channels in a way that supports quick recognition of number and name.
- The channel list remains available as the main navigation surface for playback.
- The UI gracefully handles an empty or partially unavailable lineup.

## Epic 3: Watch Live TV

### Story US-5: Start live playback immediately from channel selection
**As a** Linux viewer  
**I want** channel selection to begin playback immediately  
**So that** watching TV feels direct and responsive.

**Primary Persona**: Alex

**Acceptance Criteria**
- Selecting a channel starts playback without a second confirmation step.
- Playback begins inside the app rather than handing off to a separate viewer.
- The player stays within a persistent viewing session instead of reopening the entire playback surface on every change.
- The UI indicates loading state while the channel is tuning.

### Story US-6: Switch channels inside one persistent player session
**As a** Linux viewer  
**I want** to change channels without leaving the player context  
**So that** the app feels like a real TV product instead of a sequence of disconnected launches.

**Primary Persona**: Alex

**Acceptance Criteria**
- When the user chooses a different channel, the current player session remains active.
- The app retunes playback within the same in-app viewing context.
- The UI updates the visible channel information to reflect the new selection.
- Failed channel changes do not irreversibly break the active session state.

## Epic 4: Understand Tuner and Signal State

### Story US-7: View tuner status and signal info
**As an** advanced home TV user  
**I want** to inspect tuner status and signal information  
**So that** I can understand playback conditions and troubleshoot reception problems.

**Primary Persona**: Morgan

**Acceptance Criteria**
- The UI surfaces tuner status data for the selected playback context.
- The UI surfaces basic signal information relevant to playback health.
- The status presentation is understandable without requiring direct device-variable knowledge.
- The status display updates when channel or tuner context changes.

## Epic 5: Recover from Failures

### Story US-8: Recover from tuner or stream startup failures
**As a** Linux viewer  
**I want** clear inline errors with quick retry actions when playback fails  
**So that** I can recover without guessing what to do next.

**Primary Persona**: Alex

**Supporting Persona**: Morgan

**Acceptance Criteria**
- If no tuner is available, the app shows a clear inline message instead of a silent failure.
- If the stream cannot start, the app shows a clear inline error message in the viewing workflow.
- The error state includes quick retry-oriented actions appropriate to the failure.
- The app preserves enough context that the user does not have to restart the app after a failed attempt.

## INVEST Notes
- **Independent**: Stories are grouped by user outcome and can be implemented in sequence.
- **Negotiable**: Story details leave room for technical choices around backend transport and playback orchestration.
- **Valuable**: Every story maps to a visible user outcome.
- **Estimable**: Stories are bounded around concrete user actions.
- **Small**: Stories are scoped for incremental delivery.
- **Testable**: Every story includes explicit acceptance criteria.

## Epic 6: Enter DVR With Clarity

### Story US-9: Understand whether DVR is available
**As a** DVR household manager  
**I want** the app to tell me whether DVR prerequisites are actually available  
**So that** I can understand why recordings work or do not work before I waste time.

**Primary Persona**: Taylor

**Supporting Persona**: Morgan

**Acceptance Criteria**
- The app distinguishes between Live TV availability and DVR availability.
- If DVR storage or other prerequisites are missing, the app shows a clear recoverable DVR-readiness state.
- The readiness state explains the missing condition without forcing the user to inspect raw API payloads.
- The DVR-readiness presentation remains available from the DVR workflow rather than being buried in diagnostics only.

### Story US-10: Switch between Live TV and DVR workspaces
**As an** everyday Linux viewer  
**I want** separate Live TV and DVR tabs inside the same app  
**So that** I can move between watching television and managing recordings without losing my place.

**Primary Persona**: Alex

**Supporting Persona**: Taylor

**Acceptance Criteria**
- The client presents a dedicated DVR tab alongside the existing Live TV tab.
- Switching tabs preserves overall application continuity instead of feeling like a second product.
- The DVR tab provides access to DVR-specific workflows without forcing them into the Live TV guide alone.
- The app can return from DVR to Live TV without restarting playback or the whole client unnecessarily.

## Epic 7: Browse and Use the Recorded Library

### Story US-11: Browse recorded content with useful defaults
**As a** DVR household manager  
**I want** a browsable recorded-library view that prioritizes local storage first  
**So that** I can find the recordings I care about quickly and trust the app's storage choices.

**Primary Persona**: Taylor

**Acceptance Criteria**
- The DVR tab loads recorded content from available HDHomeRun storage-engine sources.
- The backend prioritizes local storage sources ahead of non-local sources when presenting or selecting recordings.
- The library view provides enough metadata for the user to identify recordings confidently.
- The UI remains usable when multiple storage sources are present.

### Story US-12: Play a recorded show inside the app
**As a** Linux DVR user  
**I want** to start playback of a recorded show from the DVR tab  
**So that** I can use the Linux player as a complete viewing app instead of just a live-TV client.

**Primary Persona**: Taylor

**Supporting Persona**: Alex

**Acceptance Criteria**
- Selecting a recorded item can start playback inside the application.
- Recorded playback does not require handing off to an external player.
- Playback startup failures show a clear recoverable state.
- Recorded playback integrates with the backend-owned playback boundary rather than bypassing it.

### Story US-13: Delete a recording safely
**As a** DVR household manager  
**I want** to delete a recording from the app  
**So that** I can clean up recordings without leaving the Linux workflow.

**Primary Persona**: Taylor

**Acceptance Criteria**
- The user can request deletion for a recorded item from the DVR workflow.
- The app makes the delete action explicit enough to avoid accidental removal.
- After deletion, the library view updates to reflect the new state.
- Delete failures are surfaced clearly without corrupting the visible library state.

## Epic 8: Manage Recording Rules

### Story US-14: Create a series recording rule
**As a** DVR household manager  
**I want** to create a series recording rule inside the app  
**So that** future recordings can be managed without leaving the Linux player.

**Primary Persona**: Taylor

**Acceptance Criteria**
- The app allows the user to create a series recording rule from the appropriate DVR workflow.
- The rule flow supports the HDHomeRun series-rule model instead of a simplified incompatible abstraction.
- Successful rule creation updates the visible scheduling state in the app.
- Rule-creation failures are recoverable and do not leave the UI in an ambiguous state.

### Story US-15: Create a one-time recording rule
**As a** DVR household manager  
**I want** to create a one-time recording rule for a specific airing  
**So that** I can capture an exact program occurrence when that is the better fit than a series rule.

**Primary Persona**: Taylor

**Acceptance Criteria**
- The app allows the user to create a one-time DateTime-and-Channel recording rule for a specific airing.
- The one-time workflow is clearly distinct from the series-rule workflow.
- Successful one-time rule creation updates the app's visible scheduling state.
- The app handles the case where the targeted airing is no longer valid in a recoverable way.

### Story US-16: Manage rule detail with flexible options
**As a** DVR power user  
**I want** the Linux app to expose flexible rule options  
**So that** I am not forced into a shallow DVR experience.

**Primary Persona**: Taylor

**Supporting Persona**: Morgan

**Acceptance Criteria**
- The rule editor can represent richer HDHomeRun rule options that matter for DVR control.
- The app can surface existing rule state clearly enough to support rule review and modification.
- The design leaves room for filters such as channels, teams, padding, and original-airdate behavior where the API supports them.
- The UI avoids hiding advanced options in a way that makes them effectively unusable.

## Epic 9: See Scheduled and Recorded State Clearly

### Story US-17: Recognize what will record and what already exists
**As a** DVR household manager  
**I want** the app to show whether a program is scheduled to record or already recorded  
**So that** I can trust what the DVR will do without manually cross-checking multiple screens.

**Primary Persona**: Taylor

**Acceptance Criteria**
- The app displays clear status cues for scheduled-to-record and already-recorded states.
- The status presentation works in the relevant DVR and guide-related workflows.
- The presentation remains understandable without requiring the user to decode raw backend or vendor terminology.
- The scheduling state reflects rule changes promptly enough to feel trustworthy.

## Epic 10: Control Live TV Sessions Explicitly

### Story US-18: Stop Live TV without quitting the app
**As an** everyday Linux viewer  
**I want** a clear stop control for Live TV  
**So that** I can end streaming and release the tuner without closing the application.

**Primary Persona**: Alex

**Supporting Persona**: Morgan

**Acceptance Criteria**
- The Live TV workflow presents a discoverable stop action.
- Stopping Live TV ends the active live-streaming session cleanly.
- The backend releases the associated tuner or playback resources promptly after stop.
- The app remains open and usable after Live TV is stopped.