# Performance Test Instructions

## Purpose
Provide a lightweight performance validation path for the local loopback backend and embedded client playback shell.

## Current Applicability
- Formal load testing is not yet required for this increment.
- Use these checks when validating regressions in DVR browse, playback startup, or packaging startup time.

## Recommended Checks

### 1. Backend Endpoint Responsiveness
```bash
cd /home/felix/src/hdhomerun/hdhomerun-linux/backend
HDHR_BACKEND_PLAYER_MODE=client cargo run
```

In another terminal:
```bash
time curl -s http://127.0.0.1:38080/api/dvr/readiness >/dev/null
time curl -s http://127.0.0.1:38080/api/dvr/recordings >/dev/null
time curl -s http://127.0.0.1:38080/api/playback/current >/dev/null
```

### 2. Client Startup Smoke Timing
```bash
cd /home/felix/src/hdhomerun/hdhomerun-linux/build/client
time ./hdhomerun-linux-player
```

## Success Guidance
- Local loopback endpoints should respond quickly enough that UI refresh actions feel immediate during manual use.
- Client startup should remain within the existing development baseline for the machine under test.
- Any regression should be investigated before packaging a release candidate.

## If Performance Regresses
1. Capture the slow command output and environment used.
2. Compare against the same scenario on the previous known-good build.
3. Inspect new DVR refresh or playback-state changes first.