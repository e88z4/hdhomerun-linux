# API Documentation

## HDHomeRun HTTP APIs

### Channel Lineup
- **Method**: GET
- **Paths**:
  - `http://hdhomerun.local/lineup.json`
  - `http://hdhomerun.local/lineup.xml`
  - `http://hdhomerun.local/lineup.m3u`
- **Purpose**: Enumerate channel/program entries exposed by the device.
- **Response Fields of Interest**:
  - `GuideNumber`
  - `GuideName`
  - `Tags`
  - `URL`

### Live Stream URL
- **Method**: GET
- **Path Pattern**: `http://<device>:5004/auto/v<channel>`
- **Examples**:
  - `http://192.168.0.100:5004/auto/v5.1`
  - `http://192.168.0.100:5004/tuner1/v5.1`
- **Purpose**: Allocate a tuner and start a real-time MPEG-TS stream.
- **Optional Query Parameters**:
  - `duration=<seconds>`
  - `transcode=<profile>` on supported models only
- **Relevant Error Conditions**:
  - `404` unknown channel
  - `503` tuner busy, tune failed, or no video data

## Internal Library APIs

### Device Lifecycle
- **Interface**: `hdhomerun_device_*`
- **Representative Methods**:
  - `hdhomerun_device_create`
  - `hdhomerun_device_destroy`
  - `hdhomerun_device_set_tuner`
  - `hdhomerun_device_get_name`
- **Purpose**: Create and manage device handles per tuner.

### Discovery
- **Interface**: `hdhomerun_discover_*`
- **Representative Usage**:
  - `hdhomerun_discover_create`
  - `hdhomerun_discover2_find_devices_broadcast`
  - iterator functions over discovered devices
- **Purpose**: Locate HDHomeRun devices on the local network.

### Tuner Control and Status
- **Interface**: `hdhomerun_device_*`
- **Representative Methods**:
  - `hdhomerun_device_get_tuner_status`
  - `hdhomerun_device_get_tuner_streaminfo`
  - `hdhomerun_device_set_tuner_channel`
  - `hdhomerun_device_set_tuner_vchannel`
  - `hdhomerun_device_tuner_lockkey_request`
- **Purpose**: Read tuner state and issue tuning commands.

### Stream Transport
- **Interface**: `hdhomerun_device_stream_*`
- **Representative Methods**:
  - `hdhomerun_device_stream_start`
  - `hdhomerun_device_stream_recv`
  - `hdhomerun_device_stream_stop`
- **Purpose**: Receive raw MPEG-TS data through the vendor library.

### Channel Scan
- **Interface**: `hdhomerun_device_channelscan_*`
- **Representative Methods**:
  - `hdhomerun_device_channelscan_init`
  - `hdhomerun_device_channelscan_advance`
  - `hdhomerun_device_channelscan_detect`
- **Purpose**: Scan channels and inspect detected programs.

## Existing CLI API

### Discovery Command
- **Command**: `hdhomerun_config discover [-4] [-6] [--dedupe] [<ip>]`
- **Purpose**: Enumerate devices on the local network.

### Variable Query Command
- **Command**: `hdhomerun_config <id> get <item>`
- **Purpose**: Read device or tuner variables.

### Variable Set Command
- **Command**: `hdhomerun_config <id> set <item> <value>`
- **Purpose**: Update device or tuner variables.

### Scan Command
- **Command**: `hdhomerun_config <id> scan <tuner> [<filename>]`
- **Purpose**: Run a channel scan and optionally persist output.

### Save Command
- **Command**: `hdhomerun_config <id> save <tuner> <filename>`
- **Purpose**: Save the raw transport stream to disk.

## API Gaps for the Linux Player
- No existing high-level API returns a Linux-ready embedded playback surface.
- No existing API manages on-screen controls, favorites, guide data, or playback state UX.
- No decoding pipeline API exists in the workspace; that must be added by the new player.