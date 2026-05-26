# Business Logic Model

## Scope
Unit 8 defines the first client-facing DVR workspace using the approved backend DVR contracts. It covers the DVR tab layout, recordings browsing interactions, recorded playback presentation, delete confirmation UX, and rule-creation entry points.

## Core Decision Summary
- The recordings area is grouped by series title with expandable rows for episodes.
- Starting recorded playback keeps the user inside the DVR workspace and transitions the shared player panel into recorded-playback state.
- Delete uses a confirmation dialog with `Delete`, `Delete & Re-record`, and `Cancel` actions.
- DVR readiness and degraded-state warnings appear as prominent workspace banners until resolved or dismissed by the underlying state clearing.
- Rule creation is entered from both upcoming items and recording details, with editing performed in a focused panel or sheet.

## Workspace Layout Model

### Primary Regions
1. DVR status banner area
2. Recordings browser area
3. Details or player panel area
4. Upcoming or rules context area

The layout keeps DVR-specific state visible without forcing the user back into the Live TV workspace.

## Recordings Browser Model

### Data Organization
1. Client receives merged recording summaries from the backend.
2. Client groups items by series title.
3. Each group supports expand or collapse for the contained episodes.
4. The default ordering is most recent content first within each series group.

### Interaction Flow
1. Selecting a series reveals grouped episodes.
2. Selecting an episode updates the details panel.
3. The details panel exposes play and delete actions plus any relevant rule-entry points.

## Recorded Playback Model

### Action Flow
1. User selects a recording from the DVR workspace.
2. Client requests recorded playback from the backend.
3. Client keeps the DVR workspace active.
4. Shared player panel transitions into recorded-playback mode.
5. Details and action affordances update to match recorded playback rather than Live TV state.

### State Boundary
- The player surface is shared with Live TV, but the surrounding workspace context remains DVR-specific.
- Recorded playback must not visually imply that the user has navigated back to Live TV.

## Delete Interaction Model

### Action Flow
1. User chooses delete from recording details.
2. Client shows a confirm dialog with `Delete`, `Delete & Re-record`, and `Cancel`.
3. For the first release, standard delete is always available and delete-and-rerecord is presented as an explicit recovery path.
4. After backend confirmation, the client refreshes the affected series or recording group.

### Failure Path
- Missing-recording or validation failures lead to an actionable refresh prompt rather than silent disappearance.
- The dialog must close into a clear success or failure state.

## Readiness and Warning Model

### Banner Behavior
1. Client reads readiness and degraded-state responses from the backend.
2. If the DVR workspace is not ready or degraded, show a prominent banner at the top of the workspace.
3. The banner explains the issue and points the user toward the affected action or retry path.

### Purpose
- The banner makes DVR readiness impossible to miss during first-use flows.
- It avoids burying essential DVR guidance inside diagnostics-only UI.

## Rule Entry Model

### Entry Points
- Upcoming items may open the rule editor for scheduling decisions.
- Recording details may open the rule editor for related series behavior.

### Editing Surface
- The first release uses a focused panel or sheet instead of a fully separate workspace.
- This keeps rule editing contextual to the recording or upcoming item that triggered it.