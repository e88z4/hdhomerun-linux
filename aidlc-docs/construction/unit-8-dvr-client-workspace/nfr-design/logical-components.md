# Logical Components - Unit 8 DVR Client Workspace

## 1. DVR Workspace State Coordinator
- **Purpose**: Owns the top-level client state for DVR readiness, selected recording context, panel visibility, and workflow mode.
- **NFR Role**:
  - keeps DVR mode distinct from Live TV mode
  - preserves coherent workspace state across backend refreshes
  - centralizes banner, selection, and editor visibility logic

## 2. Recordings Grouping And Selection Model
- **Purpose**: Adapts backend recording summaries into series-grouped client presentation state.
- **NFR Role**:
  - deterministic grouping and ordering
  - responsive expand or collapse behavior
  - explicit selected-row and selected-details synchronization

## 3. Shared Player Mode Presenter
- **Purpose**: Reuses the existing player surface while adapting the shell presentation for live versus recorded playback.
- **NFR Role**:
  - no duplicate player architecture
  - explicit recorded-playback mode presentation
  - prompt, coherent UI transition after backend playback responses

## 4. DVR Readiness Banner Coordinator
- **Purpose**: Maps readiness and degraded-state backend results into prominent workspace guidance.
- **NFR Role**:
  - high-visibility issue presentation
  - actionable degraded-state messaging
  - suppression of diagnostics-only hiding for critical DVR issues

## 5. Delete Confirmation Controller
- **Purpose**: Drives safe deletion UX with explicit confirm choices and post-action outcome mapping.
- **NFR Role**:
  - deliberate destructive-action flow
  - clear delete versus delete-and-rerecord branching
  - stable refresh-required failure handling

## 6. Rule Editor Launch Coordinator
- **Purpose**: Opens and seeds the focused rule editor surface from DVR details or upcoming items.
- **NFR Role**:
  - contextual entry points without workspace fragmentation
  - limited and explicit editing state
  - backend-owned payload shaping at the client boundary

## 7. Responsive Layout Coordinator
- **Purpose**: Adapts the DVR workspace layout for narrower desktop window sizes.
- **NFR Role**:
  - preserves key actions and status visibility
  - avoids hiding critical controls under constrained width
  - controls panel stacking or resizing rules explicitly

## 8. Client Logging And Testability Support
- **Purpose**: Provides structured client-side diagnostics and test seams for DVR workspace behavior.
- **NFR Role**:
  - categorized DVR interaction logging
  - focused test seams for grouping, banner logic, and delete confirmation
  - easier Unit 9 hardening and regression coverage

## Component Relationships
- DVR Workspace State Coordinator consumes backend DVR responses and drives the rest of the client components.
- Recordings Grouping And Selection Model feeds both the details panel and the Delete Confirmation Controller.
- Shared Player Mode Presenter reads playback state from the shared shell and adapts it for DVR context.
- Rule Editor Launch Coordinator receives context from both the details panel and upcoming items.
- Responsive Layout Coordinator spans the recordings, details, player, and banner areas.
- Client Logging And Testability Support spans all Unit 8 components.