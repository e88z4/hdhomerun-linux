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