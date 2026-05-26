# Business Logic Model

## Overview

Unit 6 defines the backend DVR domain behavior that the rest of the increment will rely on. Its job is to decide whether DVR is truly available, translate user intent into safe recording-rule mutations, and project trustworthy scheduled or recorded state without exposing raw vendor semantics directly to the client.

## Core Workflows

### 1. DVR Readiness Evaluation
1. Backend loads the active device context and associated DeviceAuth state.
2. Backend evaluates whether DVR-specific prerequisites are present.
3. Backend records each missing or degraded condition explicitly, such as missing DeviceAuth, missing storage source, or unavailable record engine.
4. Backend derives a structured readiness result with a boolean usable state plus condition details.
5. Client receives a DVR-readable explanation rather than raw vendor payloads.

### 2. Rule Creation or Update Confirmation
1. Client submits a rule mutation request.
2. Backend validates the request against supported rule shapes and required fields.
3. Backend executes the upstream write operation.
4. Backend re-fetches the relevant rule and schedule state before returning success.
5. Backend returns a confirmed result that includes the current rule representation and refreshed projected scheduling state.

### 3. One-Time Airing Validation
1. Client identifies a specific airing for a one-time rule.
2. Backend validates that the airing still has a valid channel and scheduled start context.
3. If the airing is stale or no longer valid, backend returns a structured invalid-airing outcome.
4. Backend does not guess a nearby replacement airing automatically.

### 4. Scheduled-State Projection
1. Backend loads vendor upcoming-recordings data as the primary execution signal.
2. Backend loads rule state and related episode or program context.
3. Backend merges these inputs into a projection model that can answer whether a program is scheduled, why it is scheduled, and which rule is responsible.
4. Where no trustworthy execution signal exists, backend returns a non-scheduled result instead of inventing certainty.

### 5. Readiness and Rule Error Shaping
1. Validation failures, unsupported options, stale-airing outcomes, and upstream failures are normalized into structured API errors or domain results.
2. Responses remain safe for direct client rendering.
3. Internal API details, tokens, or unchecked upstream command data never cross the loopback contract.

## Functional Responsibilities
- Produce a trustworthy DVR readiness model.
- Translate series and one-time rule intents into validated backend operations.
- Confirm post-write state before reporting rule success.
- Project scheduled state using upstream execution evidence plus rule context.
- Standardize DVR-domain errors and validation outcomes.

## State Transitions

### DVR Readiness
- `unknown` -> `ready`
- `unknown` -> `not_ready`
- `ready` -> `degraded`
- `degraded` -> `ready`

### Rule Mutation
- `draft` -> `validated`
- `validated` -> `submitted`
- `submitted` -> `confirmed`
- `validated` -> `rejected`
- `submitted` -> `failed`

### Schedule Projection
- `unresolved` -> `scheduled`
- `unresolved` -> `not_scheduled`
- `scheduled` -> `recorded`
- `scheduled` -> `invalidated`

## Testable Properties
- **Invariant**: A readiness result always explains blocking conditions when DVR is not usable.
- **Invariant**: Unsupported rule options never produce silent downgrades.
- **Invariant**: One-time rules are never created from stale airings by guesswork.
- **Round-trip**: A successful rule mutation returns a rule view that can be matched against a follow-up list or detail fetch.
- **Invariant**: Scheduled-state projection prefers explicit upcoming-recordings evidence over inferred guesses.