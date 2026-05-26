# Logical Components - Unit 6 DVR Domain Foundation

## 1. DVR Readiness Evaluator
- **Purpose**: Determines whether DVR behavior is currently usable for the active device context.
- **NFR Role**:
  - explicit degraded versus ready versus not-ready results
  - bounded retries on safe readiness reads
  - structured explanation of missing or failing conditions

## 2. Rule Mutation Coordinator
- **Purpose**: Orchestrates create, update, and delete rule flows with validation and confirmation.
- **NFR Role**:
  - fail-fast validation for unsupported options
  - no automatic mutation retries
  - confirmed post-write state before success

## 3. Schedule Projection Engine
- **Purpose**: Produces trustworthy scheduled-state views from upstream upcoming evidence plus rule or episode context.
- **NFR Role**:
  - projection precedence rules
  - degraded projection behavior under transient upstream failures
  - short-lived freshness reuse on safe reads only

## 4. Vendor DVR Adapter Boundary
- **Purpose**: Isolates upstream vendor DVR reads and writes from domain logic.
- **NFR Role**:
  - bounded retry policy for safe reads
  - validation of vendor-derived metadata before domain use
  - sanitized handling of upstream-sensitive material

## 5. Freshness Cache Layer
- **Purpose**: Holds very short-lived read results for readiness and projection queries.
- **NFR Role**:
  - burst-read smoothing for sub-second normal-path performance
  - immediate invalidation on related rule mutations
  - no long-lived authoritative state

## 6. Error and Outcome Mapper
- **Purpose**: Normalizes validation, degraded-state, stale-airing, and upstream failure results into stable loopback responses.
- **NFR Role**:
  - sanitized client-safe outcomes
  - distinction between degraded reads and confirmed failures
  - retry-oriented guidance where appropriate

## 7. Structured Logging and Testability Support
- **Purpose**: Provides consistent observability and test-friendly domain structure across the unit.
- **NFR Role**:
  - categorized structured logging
  - generator-friendly domain inputs for `proptest`
  - reproducible investigation paths for failing edge cases

## Component Relationships
- DVR Readiness Evaluator depends on Vendor DVR Adapter Boundary and Freshness Cache Layer.
- Rule Mutation Coordinator depends on Vendor DVR Adapter Boundary and Error and Outcome Mapper.
- Schedule Projection Engine depends on Vendor DVR Adapter Boundary, Freshness Cache Layer, and Error and Outcome Mapper.
- Vendor DVR Adapter Boundary feeds validated upstream results into the three domain components.
- Structured Logging and Testability Support spans all unit components.