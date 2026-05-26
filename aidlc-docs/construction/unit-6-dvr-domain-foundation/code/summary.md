# Unit 6 Code Summary

## Modified Application Code
- `backend/src/device/mod.rs`
- `backend/src/app.rs`
- `backend/src/models.rs`
- `backend/src/http/routes.rs`
- `backend/src/http/types.rs`
- `backend/src/lib.rs`

## Created Application Code
- `backend/src/dvr.rs`

## Key Outcomes
- Added a backend DVR provider abstraction with native and static implementations.
- Promoted `storage_url` from discovery into the normalized device model so DVR readiness and rule-sync flows can use local storage endpoints.
- Added backend contract models for DVR readiness, rule creation, rule listing, upcoming-recordings, and schedule projection.
- Added loopback DVR endpoints:
  - `GET /api/dvr/readiness`
  - `GET /api/dvr/rules`
  - `POST /api/dvr/rules/series`
  - `POST /api/dvr/rules/one-time`
  - `GET /api/dvr/upcoming`
- Implemented native vendor API integration for recording-rules and upcoming-recordings plus storage sync after rule mutations.
- Enforced fail-fast validation for unsupported options and one-time rule channel constraints.
- Implemented conservative schedule projection that prefers explicit upcoming-recordings evidence and avoids guessed scheduled state.

## Tests Executed
- `cargo test` in `backend/`

## Story Coverage
- **US-9**: backend DVR readiness contract implemented.
- **US-14**: series recording-rule creation contract implemented.
- **US-15**: one-time recording-rule creation contract implemented, including invalid-airing handling.
- **US-17**: backend upcoming-state and schedule-projection contract implemented.

## Remaining Scope For Later Units
- Recorded-library browsing, recorded playback, deletion, and local-first storage presentation remain in Unit 7.
- DVR tab UI, hybrid rule-editor UX, and guide-surface integration remain in Unit 8.