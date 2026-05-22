# Logical Components - Unit 4 Qt/QML Client Shell and Live-TV User Journey

## 1. Launch Coordinator
- **Purpose**: Owns startup overlay state, backend probing, and bootstrap flow.
- **NFR Role**:
  - fast perceived startup
  - explicit readiness handling
  - recoverable backend failure UX

## 2. Backend Gateway
- **Purpose**: Encapsulates loopback API calls from the client.
- **NFR Role**:
  - contract isolation
  - sanitized error mapping into shell state
  - maintainable request handling

## 3. Shell State Projection Layer
- **Purpose**: Translates backend responses into stable client-facing view state.
- **NFR Role**:
  - testable state transformations
  - QML simplification
  - resilience to contract evolution

## 4. Channel Rail Presenter
- **Purpose**: Maintains browsable channel-list state and selection behavior.
- **NFR Role**:
  - immediate navigation feedback
  - persistent access during playback
  - device-aware lineup refresh behavior

## 5. Playback Stage Presenter
- **Purpose**: Projects playback current, loading, and failure states into the central viewing area.
- **NFR Role**:
  - stable player-first layout
  - inline recovery behavior
  - clear session-state visibility

## 6. Diagnostics Drawer Presenter
- **Purpose**: Projects tuner summaries and expanded diagnostics into a side drawer.
- **NFR Role**:
  - expandable detail without context loss
  - active-context emphasis
  - low-disruption troubleshooting

## 7. Local Preference Store
- **Purpose**: Persists non-canonical client preferences such as drawer or layout choices.
- **NFR Role**:
  - separation from backend-owned state
  - simple client-side persistence
  - maintainable UX customization

## Component Relationships
- Launch Coordinator uses Backend Gateway and Shell State Projection Layer.
- Channel Rail Presenter, Playback Stage Presenter, and Diagnostics Drawer Presenter all consume Shell State Projection outputs.
- Local Preference Store feeds presentation-only shell choices but never overrides backend-owned canonical state.