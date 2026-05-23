# Code Generation Plan - Unit 2 SiliconDust Guide Data Enrichment

## Execution Checklist
- [x] Confirm that the existing HDHomeRun lineup contract does not expose full-lineup now-playing data
- [x] Read the vendor wiki and identify the documented device-authorized guide endpoint for live TV guide lookups
- [x] Promote `DeviceAuth` from discovery into the normalized backend device model
- [x] Replace the XMLTV-first guide provider with a SiliconDust guide API implementation
- [x] Extend the lineup contract with an optional current-program title field
- [x] Update the client channel strip to render current program titles when guide data exists
- [x] Re-run backend and client validation after the provider change

## Chosen Default Decisions
- **Guide source**: use SiliconDust's documented live guide API at `https://api.hdhomerun.com/api/guide` for current-program enrichment because it is the most direct match for per-channel now-playing data
- **Authorization source**: use the `DeviceAuth` value exposed by HDHomeRun discovery so the backend reads a fresh device-authorized token as part of normal tuner discovery
- **Contract shape**: enrich `LineupChannel` with an optional `currentProgramTitle` field instead of creating a separate client-only guide fetch path
- **Client fallback**: when guide data is unavailable or a channel has no current entry, show `Guide unavailable` instead of a misleading playback-ready label