# Recorded Playback And Delete Smoke Test

## Purpose
This checklist validates the last two unverified Unit 7 behaviors once at least one real recording exists:
- recorded playback
- recording deletion

Use this after the broader real-device backend smoke test has already passed.

## Preconditions
- A real recording exists in DVR storage.
- Backend starts successfully.
- DVR readiness is already `ready`.
- Use client-managed playback mode unless you explicitly want to validate `mpv`:

```bash
HDHR_BACKEND_PLAYER_MODE=client cargo run --manifest-path /home/felix/src/hdhomerun/hdhomerun-linux/backend/Cargo.toml
```

## 1. Confirm A Recording Exists

```bash
curl -s http://127.0.0.1:38080/api/dvr/recordings
```

Expected:
- `state` is `ready` or `degraded`
- `recordings` contains at least one item

Capture one `recordingId` from the response for the following steps.

## 2. Recorded Playback Smoke Test

Start playback for the chosen recording:

```bash
curl -s \
  -X POST \
  http://127.0.0.1:38080/api/dvr/recordings/REPLACE_WITH_RECORDING_ID/play
```

Then inspect playback state:

```bash
curl -s http://127.0.0.1:38080/api/playback/current
```

Expected:
- play request succeeds
- `sessionState.status` becomes `playing`
- `sessionState.playbackMode` becomes `recorded`
- `currentRecording` is populated
- `currentChannel` remains `null`

## 3. Stop After Recorded Playback

```bash
curl -s -X POST http://127.0.0.1:38080/api/playback/stop
curl -s http://127.0.0.1:38080/api/playback/current
```

Expected:
- stop request succeeds
- playback session becomes `stopped`
- `sessionState.playbackMode` returns to `idle`
- `currentRecording` is cleared rather than lingering after stop
- session remains coherent after recorded playback, not just after live playback

## 4. Deletion Smoke Test

Choose a recording you are willing to remove.

Delete it:

```bash
curl -s \
  -X POST \
  http://127.0.0.1:38080/api/dvr/recordings/REPLACE_WITH_RECORDING_ID/delete
```

Expected:
- response `outcome` is `confirmed`
- warnings are empty or only informational

Then confirm removal:

```bash
curl -s http://127.0.0.1:38080/api/dvr/recordings
```

Expected:
- the deleted recording no longer appears in the recordings list after refresh convergence

## 5. Missing-Recording Safety Check

Immediately retry the same delete request for the same `recordingId`:

```bash
curl -s \
  -X POST \
  http://127.0.0.1:38080/api/dvr/recordings/REPLACE_WITH_RECORDING_ID/delete
```

Expected:
- response should indicate a missing-recording style outcome rather than silently succeeding against a stale target
- response should include refresh guidance warning text

## Success Criteria
- recorded playback enters `recorded` mode correctly
- stop works after recorded playback
- delete confirms and removes the recording
- repeated delete attempts do not mutate stale targets silently

## Notes
- The current backend implementation validates current targets before playback and delete.
- When probing the device `CmdURL` directly during diagnostics, current firmware expects HTTP `POST` with `cmd=delete&rerecord=0` on the preserved query string rather than in the form body.
- If the backend reports that the recording is missing, refresh the recordings list before retrying any action.