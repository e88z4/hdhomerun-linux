# Logical Components - Unit 7 DVR Library Playback and Maintenance

## 1. Recording Source Resolver
- **Purpose**: Discovers recording sources for the selected device and orders them with local-first priority.
- **NFR Role**:
  - deterministic source ordering
  - degraded-source reporting without total catalog failure
  - central place for locality and priority decisions

## 2. Recording Catalog Merge Engine
- **Purpose**: Normalizes recorded-file entries and merges duplicates into one logical library item.
- **NFR Role**:
  - stable duplicate-merge behavior
  - alternate source retention without UI duplication
  - deterministic library assembly suitable for tests

## 3. Recording Action Validation Boundary
- **Purpose**: Re-resolves current playback and delete targets before any action is executed.
- **NFR Role**:
  - strict current-snapshot validation for mutations and playback actions
  - refresh-required outcomes instead of silent fallback
  - client-safe rejection when targets go stale

## 4. Recorded Playback Coordinator
- **Purpose**: Reuses the shared playback session controller for recorded playback flows.
- **NFR Role**:
  - explicit playback-mode transition
  - no duplicate playback orchestration path
  - prompt transition from validated recording target to player load

## 5. Recording Delete Coordinator
- **Purpose**: Executes safe recording deletion using validated current targets and sync-aware refresh behavior.
- **NFR Role**:
  - no automatic destructive retries
  - confirmed post-delete convergence behavior
  - strict shielding of raw delete URLs from client access

## 6. Live Stop Coordinator
- **Purpose**: Extends the existing playback service with an explicit stop behavior for Live TV.
- **NFR Role**:
  - prompt resource release
  - idempotent client-facing stop result
  - remembered-context preservation after stop

## 7. Error and Outcome Mapper
- **Purpose**: Produces stable loopback responses for degraded reads, missing recordings, validation failures, delete failures, and stop outcomes.
- **NFR Role**:
  - safe client-renderable errors
  - consistent refresh-required signaling
  - distinction between degraded reads and strict action failures

## 8. Structured Logging and Testability Support
- **Purpose**: Provides traceable observability and deterministic seams for merge, validation, and stop logic.
- **NFR Role**:
  - categorized structured logging
  - targeted example-based and property-based testing support
  - reproducible diagnostics for destructive-action edge cases

## Component Relationships
- Recording Source Resolver feeds ordered sources into the Recording Catalog Merge Engine.
- Recording Catalog Merge Engine produces current snapshots consumed by the Recording Action Validation Boundary.
- Recording Action Validation Boundary feeds validated playback targets to the Recorded Playback Coordinator.
- Recording Action Validation Boundary feeds validated delete targets to the Recording Delete Coordinator.
- Live Stop Coordinator builds on the existing shared playback service and returns outcomes through the Error and Outcome Mapper.
- Structured Logging and Testability Support spans all Unit 7 components.