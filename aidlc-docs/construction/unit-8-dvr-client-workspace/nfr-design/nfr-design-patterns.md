# NFR Design Patterns - Unit 8 DVR Client Workspace

## 1. Workspace-Scoped Mode Pattern
- **Pattern**: Keep DVR interactions inside a dedicated DVR workspace while reusing shared application primitives.
- **Why**: The user must be able to tell whether they are in DVR or Live TV mode even when the player surface is shared.
- **Behavior**:
  - DVR state is owned at the workspace level
  - player mode is adapted inside the workspace rather than causing navigation away from it
  - recorded playback does not implicitly collapse back into the live shell

## 2. Deterministic Grouped Collection Pattern
- **Pattern**: Transform flat backend recording summaries into stable series-grouped presentation state.
- **Why**: The first release needs scanning clarity without overcomplicating the backend contract.
- **Behavior**:
  - deterministic grouping by series title
  - recent-first ordering within groups
  - expand or collapse state preserved explicitly in client state

## 3. Banner-First Degraded State Pattern
- **Pattern**: Surface readiness and degraded DVR conditions as prominent workspace banners before secondary diagnostics.
- **Why**: Critical DVR issues should be obvious during first-use and troubleshooting workflows.
- **Behavior**:
  - prominent top-of-workspace banners for blocking or degraded states
  - actionable language tied to backend conditions
  - diagnostics remain secondary rather than primary for DVR readiness guidance

## 4. Shared Player Surface Adaptation Pattern
- **Pattern**: Reuse the existing playback surface and controller while adapting the shell presentation for recorded playback.
- **Why**: Avoids a split player architecture and keeps playback-state handling coherent.
- **Behavior**:
  - explicit live versus recorded mode presentation
  - shared stop and playback-state semantics
  - no separate DVR-only player runtime

## 5. Confirm-Before-Mutate Pattern
- **Pattern**: Route deletion through an explicit confirmation controller with concrete user choices.
- **Why**: Destructive flows should remain deliberate and understandable.
- **Behavior**:
  - dialog with `Delete`, `Delete & Re-record`, and `Cancel`
  - no immediate destructive action on first click
  - outcome-specific refresh or error handling after backend response

## 6. Contextual Editor Launch Pattern
- **Pattern**: Launch rule editing from the current DVR context rather than navigating to a disconnected management screen.
- **Why**: The first release should minimize cognitive jumps while keeping rule editing discoverable.
- **Behavior**:
  - upcoming items and recording details both provide launch context
  - focused panel or sheet hosts editor state
  - editor closes back into the current DVR workflow context

## 7. Responsive Split-To-Stack Pattern
- **Pattern**: Degrade the DVR workspace from a wide split layout to a narrower stacked or compressed layout when width is constrained.
- **Why**: Critical status, selection, and actions must remain reachable in smaller windows.
- **Behavior**:
  - recordings list stays navigable at narrow widths
  - details and actions remain visible without requiring hidden overflow-only access
  - banners stay visible regardless of layout mode

## 8. Explicit Action Outcome Mapping Pattern
- **Pattern**: Map backend responses into clear client states instead of letting raw payload differences leak into the UI.
- **Why**: Missing recordings, degraded reads, and delete failures should produce consistent user-facing behavior.
- **Behavior**:
  - backend outcomes are normalized into explicit UI states
  - refresh-required flows are represented clearly
  - logging categorizes playback launch failures, delete failures, and rule-launch failures distinctly

## 9. Testable Presentation Adapter Pattern
- **Pattern**: Keep grouping, banner, and destructive-action logic deterministic enough for targeted presentation-level tests.
- **Why**: Unit 8 risk is concentrated in UI state transitions rather than algorithmic backend work.
- **Behavior**:
  - grouping and selection logic testable without full UI rendering
  - banner visibility rules testable from backend-state permutations
  - delete dialog branching testable from explicit action inputs