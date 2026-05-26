# NFR Design Patterns - Unit 6 DVR Domain Foundation

## 1. Degraded Read State Pattern
- **Pattern**: Represent transient upstream read problems as an explicit degraded DVR state with bounded retries and clear retry guidance.
- **Why**: Matches the approved reliability posture without pretending that DVR state is fully trustworthy when upstream reads are unstable.
- **Behavior**:
  - bounded retries for safe read operations only
  - degraded readiness or projection results when retries still fail
  - clear distinction between degraded reads and confirmed successful reads

## 2. Short-Lived Freshness Window Pattern
- **Pattern**: Use a very short-lived in-memory freshness window for safe readiness and schedule reads only.
- **Why**: Helps meet the sub-second normal-path target without broad caching that could make scheduling state misleading.
- **Behavior**:
  - applies only to safe read paths
  - small TTL window sized to reduce duplicate read bursts rather than create long-lived cached state
  - write operations invalidate related freshness entries immediately

## 3. Dedicated Vendor-Derived Validation Pattern
- **Pattern**: Validate vendor-derived DVR metadata before it influences domain decisions or later actions.
- **Why**: Upstream data may be structurally valid yet still unsafe or unsuitable for downstream use.
- **Behavior**:
  - validation occurs at the adapter-to-domain boundary
  - rejected or malformed upstream-derived data produces safe degraded or validation outcomes
  - read-derived metadata does not bypass trust checks just because it came from upstream

## 4. Confirmed Mutation Pattern
- **Pattern**: Treat rule mutations as incomplete until post-write re-fetch confirms current state.
- **Why**: Prevents ambiguous or false success after upstream writes.
- **Behavior**:
  - no automatic retries for writes
  - successful response requires confirmed read-back state
  - uncertain outcomes remain explicit rather than silently upgraded to success

## 5. Separated Domain Responsibility Pattern
- **Pattern**: Keep readiness evaluation, rule mutation coordination, and schedule projection as distinct logical components.
- **Why**: Each concern has different validation, retry, and evolution pressure.
- **Behavior**:
  - separate internal orchestration boundaries
  - shared domain models but isolated primary responsibilities
  - easier targeted testing and later reuse in Units 7 through 9

## 6. Structured Observability Pattern
- **Pattern**: Emit structured logs and traceable outcomes for readiness checks, rule mutations, and projection refreshes.
- **Why**: Supports debugging and compliance without exposing sensitive upstream material.
- **Behavior**:
  - info-level default logging
  - categorized outcomes for degraded reads, validation failures, upstream failures, and confirmation failures
  - correlation-friendly identifiers where safe

## 7. Property-Testable Domain Pattern
- **Pattern**: Keep readiness and projection logic deterministic and model-driven enough for targeted property-based tests.
- **Why**: The selected partial PBT policy is most valuable on invariants and merge behavior in this unit.
- **Behavior**:
  - readiness invariants testable from condition sets
  - rule-option validation testable across combinations
  - projection precedence testable against upstream-evidence permutations