# Real-Device Backend Smoke Test

## Purpose
This checklist validates the implemented Unit 6 and Unit 7 backend DVR APIs against a real HDHomeRun environment.

It is intended for:
- a machine on the same network as the HDHomeRun device
- a DVR-capable setup where the selected device or record engine exposes `StorageURL`
- a local playback environment where `mpv` is installed or `HDHR_BACKEND_MPV_BIN` is configured

## Preconditions
- The backend builds successfully:

```bash
cargo test --manifest-path /home/felix/src/hdhomerun/hdhomerun-linux/backend/Cargo.toml
```

- `mpv` is installed or configured:

```bash
which mpv
```

If `mpv` is not on `PATH`, set:

```bash
export HDHR_BACKEND_MPV_BIN=/absolute/path/to/mpv
```

- If multiple HDHomeRun devices are present, know which `deviceRef` is DVR-capable.

## Start Backend

```bash
cargo run --manifest-path /home/felix/src/hdhomerun/hdhomerun-linux/backend/Cargo.toml
```

Expected:
- backend binds to `127.0.0.1:38080`

## 1. Health And Bootstrap

```bash
curl -s http://127.0.0.1:38080/api/health
curl -s http://127.0.0.1:38080/api/bootstrap
```

Expected:
- `/api/health` returns `"ready":true`
- `/api/bootstrap` advertises the DVR endpoints:
  - `/api/dvr/readiness`
  - `/api/dvr/rules`
  - `/api/dvr/recordings`
  - `/api/dvr/recordings/{recording_id}/play`
  - `/api/dvr/recordings/{recording_id}/delete`
  - `/api/dvr/upcoming`

## 2. Discover And Select The DVR-Capable Device

```bash
curl -s http://127.0.0.1:38080/api/devices
```

If the selected device is not the DVR-capable one, switch it:

```bash
curl -s \
  -X POST \
  -H 'content-type: application/json' \
  -d '{"deviceRef":"REPLACE_WITH_DEVICE_REF"}' \
  http://127.0.0.1:38080/api/devices/select
```

Expected:
- the DVR-capable device is marked selected
- the selected device should be the one whose environment yields DVR `StorageURL`

## 3. DVR Readiness

```bash
curl -s http://127.0.0.1:38080/api/dvr/readiness
```

Expected for a real DVR-capable environment:
- `"state":"ready"`
- `"usable":true`
- no blocking readiness conditions

If it returns `missing_storage`, the backend is working but the selected device or environment is not exposing DVR storage.

## 4. Recording Rules

```bash
curl -s http://127.0.0.1:38080/api/dvr/rules
```

Expected:
- `"state":"ready"`
- rule list may be empty or populated depending on the account and device state

## 5. Upcoming Recordings

```bash
curl -s http://127.0.0.1:38080/api/dvr/upcoming
```

Expected:
- if the DVR environment is active, this should return either scheduled entries or an empty but valid ready state depending on current rules
- projection entries should be structured and client-safe

## 6. Recordings Library

```bash
curl -s http://127.0.0.1:38080/api/dvr/recordings
```

Expected:
- `"state":"ready"` or `"state":"degraded"`
- `recordings` array present
- recordings are merged logical items, local-first when duplicates exist

If no `StorageURL` is exposed, expect `"state":"unavailable"` with a storage-related warning.

## 7. Recorded Playback

Pick one `recordingId` from `/api/dvr/recordings`, then:

```bash
curl -s \
  -X POST \
  http://127.0.0.1:38080/api/dvr/recordings/REPLACE_WITH_RECORDING_ID/play
```

Then inspect current playback state:

```bash
curl -s http://127.0.0.1:38080/api/playback/current
```

Expected:
- play request succeeds
- `sessionState.status` becomes `playing`
- `sessionState.playbackMode` becomes `recorded`
- `currentRecording` is populated

If playback fails and `/api/playback/current` warns about `mpv`, install it or set `HDHR_BACKEND_MPV_BIN`.

## 8. Live Stop Control

Start a live channel first if needed, then:

```bash
curl -s -X POST http://127.0.0.1:38080/api/playback/stop
curl -s http://127.0.0.1:38080/api/playback/current
```

Expected:
- stop succeeds without exiting the backend
- session state becomes `stopped` or remains stable if already inactive
- remembered device or channel context remains available for later restart

## 9. Optional Rule-Creation Smoke Test

Only run this if you have a known safe `seriesId` and want to validate rule mutation flows.

Series rule example:

```bash
curl -s \
  -X POST \
  -H 'content-type: application/json' \
  -d '{
    "seriesId":"REPLACE_WITH_SERIES_ID",
    "options":{
      "recentOnly":false,
      "channelOnly":[]
    }
  }' \
  http://127.0.0.1:38080/api/dvr/rules/series
```

Expected:
- `outcome` is `confirmed`
- returned rules reflect the new rule
- schedule projection is updated

## 10. Optional Delete Smoke Test

This is destructive. Only run against a recording you are willing to remove.

Standard delete:

```bash
curl -s \
  -X POST \
  http://127.0.0.1:38080/api/dvr/recordings/REPLACE_WITH_RECORDING_ID/delete
```

Expected:
- `outcome` is `confirmed`
- the recording disappears from a follow-up `/api/dvr/recordings` request after refresh convergence

If the backend returns a missing-recording style outcome, refresh the recordings list and retry only after confirming the item still exists.

## Success Criteria
- Backend starts and stays healthy on loopback
- DVR readiness is `ready` on the selected device
- recordings and upcoming endpoints return structured data
- recorded playback enters `recorded` playback mode
- stop works without killing the backend
- optional rule creation and delete flows behave consistently

## Current Known Environment Blockers
The last smoke test on this machine found:
- selected device had no exposed `StorageURL`
- recordings endpoint was therefore unavailable
- `mpv` was not installed or configured for playback smoke testing

Resolve those first before expecting a full real-device pass.