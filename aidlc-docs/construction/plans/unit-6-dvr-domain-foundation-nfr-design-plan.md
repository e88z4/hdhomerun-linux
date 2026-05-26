# NFR Design Plan - Unit 6 DVR Domain Foundation

## Execution Checklist
- [x] Review Unit 6 NFR requirements and tech stack decisions together
- [x] Identify resilience, performance, security, and logical-component patterns for the DVR domain unit
- [x] Generate context-appropriate NFR design questions in this document
- [x] Collect answers for the NFR design questions
- [x] Resolve any ambiguity in the answers
- [x] Generate nfr-design-patterns.md
- [x] Generate logical-components.md
- [x] Validate completeness and consistency

## Clarifying Questions

## Question 1
How should Unit 6 represent transient upstream failures during readiness and schedule reads?

A) Surface a degraded state with bounded read retries and explicit retry guidance

B) Fail every read immediately with no degraded intermediate state

C) Hide transient failures from the client unless all retries are exhausted

X) Other (please describe after [Answer]: tag below)

[Answer]: A

## Question 2
What performance pattern should Unit 6 use for short-lived DVR status reads?

A) No caching; every read goes directly upstream

B) Very short-lived in-memory freshness window for safe readiness and projection reads only

C) Broader caching of rule and schedule state across the unit

X) Other (please describe after [Answer]: tag below)

[Answer]: B

## Question 3
How should Unit 6 enforce trust boundaries around vendor-derived DVR metadata?

A) Dedicated validation layer before vendor-derived data influences domain decisions or later actions

B) Rely mostly on upstream correctness and validate only externally supplied client input

C) Validate only write paths, not read-derived metadata

X) Other (please describe after [Answer]: tag below)

[Answer]: A

## Question 4
How should the core logical responsibilities be organized inside Unit 6?

A) Separate logical components for readiness evaluation, rule mutation coordination, and schedule projection

B) One consolidated DVR domain service with minimal internal separation

C) Separate readiness only, with rules and projection combined

X) Other (please describe after [Answer]: tag below)

[Answer]: A

## Selected NFR Design Decisions
- Resilience pattern: surface degraded state with bounded read retries and explicit retry guidance.
- Performance pattern: use a very short-lived in-memory freshness window for safe readiness and projection reads only.
- Security pattern: validate vendor-derived metadata at the adapter-to-domain boundary before it affects decisions or later actions.
- Logical-component pattern: keep readiness evaluation, rule mutation coordination, and schedule projection as separate logical components.

## Constraints to Preserve
- Loopback-only exposure remains fixed.
- Rule mutations still require confirmed post-write state.
- Security Baseline and partial PBT remain active constraints.