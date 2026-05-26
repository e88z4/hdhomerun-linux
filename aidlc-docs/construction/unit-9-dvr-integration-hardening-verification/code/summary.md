# Unit 9 Code Summary - DVR Integration Hardening and Verification

## Implemented Scope
- Hardened DVR recording catalog normalization so real-device series aggregates now expand through `EpisodesURL` to concrete episode entries with trusted playback and delete targets.
- Hardened recorded-playback stop behavior so stopping a recorded session clears stale recorded context instead of leaving the client with lingering recorded metadata.
- Hardened DVR delete error reporting so record-engine `400 Bad Request` responses now surface as explicit validation failures instead of generic internal backend errors.
- Corrected the backend delete command shape for current firmware: use HTTP `POST`, but place `cmd=delete&rerecord=0` on the preserved `CmdURL` query string rather than in the request body.
- Hardened client playback-state interpretation so DVR and live playback presentation now follows explicit session state and playback mode rather than inferring mode only from leftover payload fields.
- Added backend contract coverage for stale delete behavior and recorded-playback stop cleanup.
- Added client helper coverage for rule-context fallback from existing rules and preserved explicit group expansion behavior.
- Refined operator-facing recorded playback and delete smoke-test guidance to match the hardened stale-delete and recorded-stop expectations.

## Brownfield Files Updated
- `backend/src/playback.rs`
- `backend/src/dvr.rs`
- `backend/tests/dvr_contract.rs`
- `client/src/appcontroller.full.cpp.inc`
- `client/tests/dvrworkspacehelpers_tests.cpp`
- `aidlc-docs/construction/unit-7-dvr-library-playback-maintenance/code/recorded-playback-delete-smoke-test.md`

## Validation
- `cargo test` in `/home/felix/src/hdhomerun/hdhomerun-linux/backend`
- `cmake --build /home/felix/src/hdhomerun/hdhomerun-linux/build/client`
- `ctest --output-on-failure` in `/home/felix/src/hdhomerun/hdhomerun-linux/build/client`
- Result: backend and client test suites passed after the Unit 9 changes.

## Notes
- Real-device follow-up confirmed that recorded playback and recorded-stop cleanup now work against the concrete episode metadata returned from `EpisodesURL`.
- Real-device follow-up confirmed that the delete contract required a `POST` request with delete parameters on the query string, matching a community-reported working example rather than the older body-form interpretation.
- After aligning the backend to that query-string `POST` shape, the tested recording was removed from device storage and the loopback API converged to an empty recordings list plus the expected stale-delete `missing_recording` outcome.
- The backend now treats recorded-stop cleanup as a first-class state transition instead of reusing live-stop semantics unchanged.
- The static DVR fixture path now mirrors the real missing-recording warning behavior so stale-delete tests exercise the real contract shape.