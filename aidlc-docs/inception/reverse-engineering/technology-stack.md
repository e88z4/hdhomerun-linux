# Technology Stack

## Programming Languages
- **C** - Primary implementation language across `libhdhomerun` and `sdnet`.

## Frameworks and Libraries
- **POSIX sockets** - Network communication on Unix-like systems.
- **pthread** - Threading support used by the vendor build.
- **Linux `librt`** - Runtime support linked by the Linux build.

## Protocols and Data Formats
- **HDHomeRun control protocol** - Device control and tuner operations.
- **HTTP** - Metadata and live-stream access via device endpoints.
- **MPEG-TS** - Live TV transport stream format.
- **JSON, XML, M3U** - Device channel-list metadata formats.

## Build Tools
- **GNU Make** - Used by `libhdhomerun` and selected `sdnet` tools.
- **gcc** - Default compiler in the vendor Makefile.

## Platform Coverage in Source
- **Linux**
- **BSD**
- **macOS**
- **Windows**

## Documentation Sources
- **Local README files** - Minimal entry-point documentation.
- **SiliconDust HTTP API documentation** - External reference for lineup and stream URLs.

## Missing Stack for the Planned Product
- No GUI framework is currently selected.
- No media decoder or playback engine is present.
- No packaging toolchain exists yet for Linux desktop distribution.

## Recommended Additions for the Planned Product
- **Playback Engine**: `mpv` or `libmpv`
- **Application Layer**: A Linux desktop app toolkit rather than a browser-only frontend
- **Project Build**: A first-class app build system in `hdhomerun-linux` once requirements are approved