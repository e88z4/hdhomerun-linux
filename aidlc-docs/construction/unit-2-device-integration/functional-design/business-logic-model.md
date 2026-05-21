# Business Logic Model

## Overview

Unit 2 defines how the backend discovers HDHomeRun tuner devices on the local network, maintains explicit device selection, retrieves lineup metadata, and surfaces tuner diagnostics in a client-stable contract. Its job is to convert vendor-specific discovery and status APIs into predictable product behavior without yet taking ownership of playback session orchestration.

## Core Workflows

### 1. IPv4 Device Discovery
1. Backend runs discovery through a reusable `libhdhomerun` discover object.
2. Discovery targets tuner-class devices on the local IPv4 network only.
3. Backend copies the first preferred interface data for each discovered device into backend-owned models.
4. Backend returns a normalized discovered-device list with stable identifiers and URLs needed by later steps.
5. If no devices are found, backend returns an empty discovery result rather than inventing device state.

### 2. Remembered Device Reconciliation
1. Backend loads the remembered device reference from canonical state.
2. Backend compares the remembered device reference against the current discovery result.
3. If the remembered device is present, it becomes the selected device context.
4. If the remembered device is absent and other devices are available, backend returns a `selection_required` outcome with the discovered device list.
5. If no devices are available, backend returns a `selection_required` outcome with an empty device list and a warning that no tuner device is currently reachable.

### 3. Explicit Device Selection
1. Client selects a device from the discovered-device list.
2. Backend validates that the requested device still exists in the latest discovery result.
3. Backend persists the selected device reference as part of the canonical remembered context.
4. Backend uses the selected device's copied base and lineup URLs as the canonical integration endpoints for later lineup and playback-source resolution.

### 4. Lineup Retrieval and Channel Normalization
1. Backend reads the selected device's `lineup.json` endpoint.
2. Backend parses the returned channel entries and normalizes channel metadata into backend-owned models.
3. Backend preserves the device-provided playback `URL` as the canonical playback source reference for Unit 3.
4. Backend converts tag metadata, including DRM markers, into explicit availability flags instead of leaving interpretation to the client.
5. Backend returns a stable lineup model ordered for client rendering and selection.

### 5. Tuner Diagnostics Aggregation
1. Backend inspects all tuners on the selected device.
2. Backend retrieves tuner-status data through `libhdhomerun` status APIs for each tuner context.
3. Backend maps raw vendor fields into a consistent diagnostic model with summary-friendly signal fields.
4. Backend marks one tuner context as active when it can be correlated to the selected channel or current playback target.
5. Backend returns all tuners for the selected device so the client can show both overview and focused diagnostics.

### 6. Playback Source Resolution Handoff
1. Backend receives a channel reference for the selected device.
2. Backend resolves the channel to the normalized lineup entry.
3. Backend returns the canonical stream URL and availability metadata needed by Unit 3.
4. Backend does not start playback in Unit 2; it only resolves verified device-side source information.

### 7. Error Shaping
1. Discovery, lineup, and tuner-status failures are converted into stable backend error outcomes.
2. Device communication details remain internal and are not exposed raw to the client.
3. Recoverable conditions such as missing devices, stale remembered selections, or temporary lineup fetch failures are surfaced as structured, user-meaningful results.

## Functional Responsibilities
- Discover HDHomeRun tuner devices on the IPv4 local network.
- Reconcile remembered device state with live discovery results.
- Maintain explicit selected-device context.
- Retrieve and normalize lineup metadata from `lineup.json`.
- Surface all tuner diagnostics for the selected device with one active context highlighted.
- Resolve channel playback-source metadata for Unit 3 without owning playback execution.

## State Transitions

### Device Discovery and Selection
- `not_discovered` -> `discovered_none`
- `not_discovered` -> `devices_available`
- `devices_available + remembered_device_present` -> `device_selected`
- `devices_available + remembered_device_missing` -> `selection_required`
- `selection_required + client_selects_device` -> `device_selected`

### Lineup Availability
- `device_selected` -> `lineup_loading`
- `lineup_loading` -> `lineup_ready`
- `lineup_loading` -> `lineup_unavailable`

### Tuner Visibility
- `device_selected` -> `tuner_status_loading`
- `tuner_status_loading` -> `tuner_status_ready`
- `tuner_status_loading` -> `tuner_status_partial`
- `tuner_status_loading` -> `tuner_status_unavailable`

## Testable Properties
- **Invariant**: Discovery results only include tuner devices accepted by the backend device filter.
- **Invariant**: When the remembered device is absent, the backend returns `selection_required` instead of silently auto-selecting a replacement.
- **Invariant**: Every normalized channel entry includes an explicit availability or restriction state.
- **Invariant**: Tuner diagnostics responses include all tuners for the selected device, even when only one tuner is highlighted as active.
- **Round-trip**: A normalized playback-source resolution refers back to a valid discovered device and lineup entry.