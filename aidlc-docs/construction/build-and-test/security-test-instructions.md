# Security Test Instructions

## Purpose
Validate the security-sensitive aspects of the local backend and packaged desktop application for this DVR increment.

## 1. Dependency Review
```bash
cd /home/felix/src/hdhomerun/hdhomerun-linux/backend
cargo audit
```

If `cargo-audit` is not installed:
```bash
cargo install cargo-audit
cd /home/felix/src/hdhomerun/hdhomerun-linux/backend
cargo audit
```

## 2. Loopback Exposure Check
```bash
cd /home/felix/src/hdhomerun/hdhomerun-linux/backend
cargo run
```

In another terminal:
```bash
curl -i http://127.0.0.1:38080/health
ss -ltnp | grep 38080
```

Expected:
- backend binds only to the expected loopback address unless intentionally overridden
- health endpoint is reachable locally

## 3. DVR Safety Checks
- confirm delete of a missing recording returns a safe missing-recording outcome rather than mutating a stale target
- confirm recording deletion still requires a trusted backend-supplied delete target
- confirm the client never calls vendor DVR APIs directly for delete or rule creation paths

## 4. Manual Validation Targets
- stale recording replay attempt should fail with validation guidance rather than silently switching to another target
- repeated delete should return refresh guidance warning text
- packaged client runs should continue to prefer embedded playback mode instead of requiring a host `mpv` dependency