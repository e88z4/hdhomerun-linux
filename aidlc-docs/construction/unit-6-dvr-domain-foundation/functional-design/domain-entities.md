# Domain Entities

## 1. DvrReadinessState
- **Purpose**: Represents whether DVR functions are currently usable for the active device context.
- **Fields**:
  - `status`: `ready | degraded | not_ready | unknown`
  - `usable`: boolean
  - `conditions`: `ReadinessCondition[]`
  - `evaluatedAt`

## 2. ReadinessCondition
- **Purpose**: Represents one explicit readiness finding.
- **Fields**:
  - `code`: `missing_device_auth | missing_storage | missing_record_engine | upstream_error | other`
  - `severity`: `info | warning | blocking`
  - `message`
  - `recoverable`: boolean

## 3. RecordingRuleIntent
- **Purpose**: Represents the normalized client intent for creating or updating a recording rule.
- **Fields**:
  - `kind`: `series | one_time`
  - `targetProgramRef`
  - `targetAiringRef`
  - `channelRef`
  - `timeWindow`
  - `options`: `RuleOptionSet`

## 4. RuleOptionSet
- **Purpose**: Represents the supported DVR rule options the backend can validate and reason about.
- **Fields**:
  - `paddingBefore`
  - `paddingAfter`
  - `channelFilters`
  - `teamFilters`
  - `originalAirdateOnly`
  - `extraOptionsRejected`: boolean

## 5. RecordingRuleState
- **Purpose**: Represents the confirmed current state of a recording rule after read or mutation.
- **Fields**:
  - `ruleId`
  - `kind`: `series | one_time`
  - `status`: `active | invalid | pending_confirmation | deleted`
  - `summary`
  - `options`: `RuleOptionSet`
  - `updatedAt`

## 6. OneTimeAiringRef
- **Purpose**: Represents the identity of an airing targeted by a one-time rule.
- **Fields**:
  - `programId`
  - `channelId`
  - `startTime`
  - `endTime`

## 7. ScheduleProjectionEntry
- **Purpose**: Represents the backend's projected state for whether content is scheduled or already accounted for.
- **Fields**:
  - `programRef`
  - `ruleId`
  - `state`: `scheduled | recorded | not_scheduled | invalidated`
  - `reasonCode`
  - `sourceConfidence`: `explicit_upcoming | rule_context | none`

## 8. RuleMutationResult
- **Purpose**: Represents the outcome of a create, update, or delete rule operation.
- **Fields**:
  - `outcome`: `confirmed | rejected | failed | invalid_airing`
  - `ruleState`
  - `scheduleProjection`: `ScheduleProjectionEntry[]`
  - `warnings`

## 9. DvrDomainError
- **Purpose**: Represents a structured DVR-domain failure safe for the loopback API.
- **Fields**:
  - `code`
  - `message`
  - `retryHint`
  - `category`: `validation | readiness | upstream | stale_airing`

## Entity Relationships
- `DvrReadinessState` contains multiple `ReadinessCondition` values.
- `RecordingRuleIntent` includes a `RuleOptionSet` and may reference a `OneTimeAiringRef`.
- `RecordingRuleState` represents the confirmed persisted state derived from a `RecordingRuleIntent`.
- `RuleMutationResult` may include one `RecordingRuleState` and multiple `ScheduleProjectionEntry` values.
- `DvrDomainError` can be produced by readiness evaluation, rule validation, or upstream rule operations.

## Entity Constraints
- `DvrReadinessState.usable` must be false if any blocking `ReadinessCondition` exists.
- `RecordingRuleIntent.kind=one_time` requires a valid `OneTimeAiringRef`.
- `RuleOptionSet.extraOptionsRejected` must be true when unsupported options are present and the request is rejected.
- `ScheduleProjectionEntry.sourceConfidence=explicit_upcoming` should only be used when backed by current upcoming-recordings data.
- `RuleMutationResult.outcome=confirmed` requires a non-null confirmed `RecordingRuleState`.