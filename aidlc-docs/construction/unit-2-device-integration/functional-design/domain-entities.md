# Domain Entities

## 1. DiscoveredDevice
- **Purpose**: Represents a normalized HDHomeRun tuner device discovered on the local network.
- **Fields**:
  - `deviceRef`
  - `deviceId`
  - `modelName`
  - `hardwareModel`
  - `isLegacy`
  - `tunerCount`
  - `baseUrl`
  - `lineupUrl`
  - `discoveryScope`: `ipv4_local`
  - `discoveredAt`

## 2. DeviceSelectionState
- **Purpose**: Represents the backend-owned current and remembered device selection context.
- **Fields**:
  - `selectedDeviceRef`
  - `rememberedDeviceRef`
  - `selectionMode`: `remembered_restore | explicit_user_selection | selection_required`
  - `warnings`
  - `updatedAt`

## 3. DiscoveryResult
- **Purpose**: Represents the outcome of a discovery pass used by bootstrap and device selection flows.
- **Fields**:
  - `devices`
  - `selectedDeviceRef`
  - `selectionRequired`
  - `warnings`
  - `refreshedAt`

## 4. ChannelLineup
- **Purpose**: Represents the normalized channel catalog for one selected HDHomeRun device.
- **Fields**:
  - `deviceRef`
  - `channels`
  - `source`: `lineup_json`
  - `loadedAt`
  - `warnings`

## 5. ChannelEntry
- **Purpose**: Represents one normalized channel entry from `lineup.json`.
- **Fields**:
  - `channelRef`
  - `guideNumber`
  - `guideName`
  - `tags`
  - `playbackUrl`
  - `availability`: `playable | restricted | unavailable`
  - `restrictionReason`

## 6. TunerDiagnostic
- **Purpose**: Represents normalized tuner or signal diagnostics for one tuner on the selected device.
- **Fields**:
  - `deviceRef`
  - `tunerIndex`
  - `isActiveContext`
  - `channel`
  - `virtualChannel`
  - `program`
  - `lockState`
  - `signalPresent`
  - `signalStrength`
  - `signalToNoiseQuality`
  - `symbolErrorQuality`
  - `bitsPerSecond`
  - `packetsPerSecond`
  - `statusSummary`

## 7. PlaybackSourceDescriptor
- **Purpose**: Represents the verified channel-to-stream mapping handed off to later playback orchestration.
- **Fields**:
  - `deviceRef`
  - `channelRef`
  - `playbackUrl`
  - `availability`
  - `restrictionReason`
  - `resolvedAt`

## Entity Relationships
- `DiscoveryResult` contains multiple `DiscoveredDevice` values.
- `DeviceSelectionState.selectedDeviceRef` must correspond to a `DiscoveredDevice` in the latest valid discovery result when selection is active.
- `ChannelLineup` belongs to one selected `DiscoveredDevice`.
- `ChannelLineup.channels` contains multiple `ChannelEntry` values.
- `TunerDiagnostic.deviceRef` links tuner state back to the selected `DiscoveredDevice`.
- `PlaybackSourceDescriptor` resolves one `ChannelEntry` for one selected `DiscoveredDevice`.

## Entity Constraints
- `DiscoveredDevice.lineupUrl` must be present before lineup retrieval can proceed.
- `DeviceSelectionState.selectionMode=selection_required` means `selectedDeviceRef` is absent.
- `ChannelEntry.availability` must always be explicit, even when the source device only signals restrictions through tags.
- At most one `TunerDiagnostic` should have `isActiveContext=true` for a given selected-device snapshot.
- `PlaybackSourceDescriptor.playbackUrl` must originate from normalized lineup data rather than an invented fallback.