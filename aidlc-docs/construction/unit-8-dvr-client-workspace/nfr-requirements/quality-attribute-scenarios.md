# Quality Attribute Scenarios - Unit 8 DVR Client Workspace

## Scenario 1: Degraded DVR Banner Visibility
- **Attribute**: Usability and reliability
- **Source**: Backend returns a degraded DVR recordings or readiness state.
- **Stimulus**: The user opens or refreshes the DVR workspace.
- **Environment**: Normal runtime with partial backend availability.
- **Response**: The workspace shows a prominent banner with actionable guidance while keeping usable content visible where possible.
- **Response Measure**: The user can understand the degraded condition without opening diagnostics.

## Scenario 2: Recorded Playback Transition
- **Attribute**: Performance and coherence
- **Source**: The user presses Play on a selected recording.
- **Stimulus**: The client receives a successful recorded-playback response from the backend.
- **Environment**: DVR workspace is active.
- **Response**: The shared player panel transitions into recorded mode without moving the user out of the DVR workspace.
- **Response Measure**: The workspace remains recognizably DVR-focused while playback begins promptly.

## Scenario 3: Delete Confirmation Safety
- **Attribute**: Safety and usability
- **Source**: The user presses Delete on a recording.
- **Stimulus**: The confirmation dialog is shown.
- **Environment**: Recording details are visible in the DVR workspace.
- **Response**: The dialog presents Delete, Delete & Re-record, and Cancel as explicit choices.
- **Response Measure**: No destructive action occurs without an intentional confirm choice.

## Scenario 4: Narrow Window Layout
- **Attribute**: Responsiveness and usability
- **Source**: The application window is narrowed.
- **Stimulus**: The user continues browsing recordings or upcoming items.
- **Environment**: Smaller desktop window size.
- **Response**: The DVR workspace preserves critical status, selection, and action affordances through graceful layout adaptation.
- **Response Measure**: No essential action becomes unreachable solely due to reduced width.