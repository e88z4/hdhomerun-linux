# Business Rules

## BR-U8-1 DVR-First Workspace
The client must keep DVR browsing and recorded playback interactions inside a dedicated DVR workspace rather than redirecting the user back to the Live TV workspace.

## BR-U8-2 Series-Grouped Recordings List
The recordings browser must group items by series title and allow expansion into episode rows.

## BR-U8-3 Recent-First Episode Ordering
Episodes within a series group should be ordered with the most recent recording first.

## BR-U8-4 Shared Player, Preserved DVR Context
Recorded playback must use the shared player surface while preserving DVR workspace context, labels, and actions.

## BR-U8-5 Recorded-State Presentation
When the player is in recorded mode, the client must present recorded-playback actions and metadata rather than Live TV controls alone.

## BR-U8-6 Explicit Delete Confirmation
Deleting a recording must require an explicit confirmation dialog.

## BR-U8-7 Delete-and-Rerecord Recovery Path
The delete confirmation dialog must expose a `Delete & Re-record` option in addition to standard delete and cancel.

## BR-U8-8 Visible Readiness Guidance
DVR readiness and degraded-state issues must be shown prominently in the workspace banner area while they remain relevant.

## BR-U8-9 Contextual Rule Entry
Rule creation entry points must be available from both upcoming items and recording details.

## BR-U8-10 Focused Rule Editing Surface
The initial rule-editing experience must appear in a focused panel or sheet rather than navigating into an unrelated workflow.

## BR-U8-11 Backend-Owned Actions
The client must trigger playback, deletion, and rule mutations only through the backend-owned loopback API and must not expose raw upstream targets.

## BR-U8-12 Clear Refresh Recovery
If the backend reports a missing recording or degraded state, the client must present a clear refresh path instead of leaving the DVR workspace ambiguous.