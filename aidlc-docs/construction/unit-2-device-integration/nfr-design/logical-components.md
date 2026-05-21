# Logical Components - Unit 2 HDHomeRun Discovery and Device Integration

## 1. Discovery Adapter
- **Purpose**: Encapsulates IPv4 HDHomeRun tuner-device discovery through `libhdhomerun`.
- **NFR Role**:
  - reusable discover-object lifecycle
  - bounded discovery calls
  - device-string copying and safe adapter boundaries

## 2. Lineup Adapter
- **Purpose**: Retrieves `lineup.json` for the selected device and hands raw results to normalization.
- **NFR Role**:
  - bounded HTTP timeouts
  - selected-device scoped lineup retrieval
  - stale-versus-fresh lineup state handling

## 3. Tuner Diagnostics Adapter
- **Purpose**: Retrieves per-tuner status information from the selected device through vendor APIs.
- **NFR Role**:
  - per-tuner failure isolation
  - summary-friendly signal extraction
  - partial diagnostics degradation without backend-wide failure

## 4. Normalization Layer
- **Purpose**: Converts discovery, lineup, and tuner vendor data into backend-owned normalized entities.
- **NFR Role**:
  - explicit availability classification
  - stable device and channel references
  - sanitization of client-visible metadata

## 5. Device Integration Service
- **Purpose**: Coordinates discovery refresh, selected-device context, lineup access, and diagnostics aggregation for the API layer.
- **NFR Role**:
  - startup plus periodic refresh scheduling
  - remembered-device reconciliation
  - stale data marking and structured failure composition

## 6. Contract Composition Layer
- **Purpose**: Shapes normalized device integration data into stable loopback API responses.
- **NFR Role**:
  - stable response contracts for the client
  - structured partial-failure summaries
  - preservation of Unit 1 error-envelope expectations

## 7. Device Integration Observability Layer
- **Purpose**: Centralizes structured logging and metrics-friendly event emission for Unit 2 behavior.
- **NFR Role**:
  - info-level operational visibility
  - debug-only deep vendor-call tracing
  - diagnosable discovery, lineup, and tuner-status refresh events

## Component Relationships
- Discovery Adapter, Lineup Adapter, and Tuner Diagnostics Adapter feed raw vendor data into the Normalization Layer.
- Device Integration Service orchestrates refresh cadence and selection state using the adapters and Normalization Layer.
- Contract Composition Layer depends on Device Integration Service output and preserves the stable Unit 1 API contract style.
- Device Integration Observability Layer spans adapters, normalization, and service orchestration.