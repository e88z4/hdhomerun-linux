# Domain Entities

## RecordingSourceDescriptor
- **Purpose**: Describes one storage source that can contribute recorded files.
- **Fields**:
  - `source_ref`
  - `base_url`
  - `is_local`
  - `priority_rank`
  - `availability_state`
  - `warning_messages[]`

## RecordedCandidate
- **Purpose**: Normalized representation of one recorded-file entry before duplicate merge.
- **Fields**:
  - `logical_identity`
  - `source_ref`
  - `title`
  - `subtitle`
  - `episode_number`
  - `recorded_start_at`
  - `recorded_end_at`
  - `channel_name`
  - `image_url`
  - `playback_target`
  - `delete_target`
  - `validation_state`

## LogicalRecordingIdentity
- **Purpose**: Stable grouping key used to merge candidate recordings that represent the same logical show or airing.
- **Fields**:
  - `series_id?`
  - `program_id?`
  - `title`
  - `subtitle?`
  - `recorded_start_at`
  - `channel_name?`

## RecordingPlaybackTarget
- **Purpose**: Validated backend-owned playback target for one preferred recording source.
- **Fields**:
  - `source_ref`
  - `stream_url`
  - `content_type?`
  - `recording_locator`
  - `validated_at`

## RecordingDeleteTarget
- **Purpose**: Validated backend-owned delete command derived from the current catalog snapshot.
- **Fields**:
  - `source_ref`
  - `cmd_url`
  - `default_rerecord`
  - `recording_locator`
  - `validated_at`

## RecordingLibraryItem
- **Purpose**: User-facing merged recording item returned by the library endpoint.
- **Fields**:
  - `recording_id`
  - `title`
  - `subtitle`
  - `description`
  - `recorded_start_at`
  - `duration_seconds`
  - `channel_name`
  - `preferred_source_ref`
  - `source_count`
  - `availability_state`
  - `warnings[]`

## RecordingSourceVariant
- **Purpose**: Backend-retained alternate source metadata for a merged recording.
- **Fields**:
  - `source_ref`
  - `is_preferred`
  - `is_local`
  - `playback_target`
  - `delete_target`
  - `validation_state`

## RecordingCatalogSnapshot
- **Purpose**: Immutable resolution result used to keep action validation tied to a specific current view of storage state.
- **Fields**:
  - `selected_device_ref`
  - `generated_at`
  - `sources[]`
  - `items[]`
  - `degraded`
  - `warnings[]`

## RecordingActionResult
- **Purpose**: Common action result envelope for playback or deletion requests.
- **Fields**:
  - `outcome`
  - `recording_id`
  - `requires_refresh`
  - `message`
  - `warnings[]`

## LiveStopResult
- **Purpose**: Represents the outcome of an explicit Live TV stop request.
- **Fields**:
  - `session_status`
  - `released_resources`
  - `remembered_device_ref?`
  - `remembered_channel_ref?`
  - `stopped_at`

## Outcome Enumerations
- `RecordingAvailabilityState`: `ready | degraded | missing`
- `RecordingValidationState`: `validated | stale | invalid`
- `RecordingActionOutcome`: `succeeded | missing_recording | validation_failed | upstream_failed`
- `PlaybackMode`: `live | recorded | idle`