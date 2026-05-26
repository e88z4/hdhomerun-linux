# Build Instructions

## Prerequisites
- **Build Tools**: Rust toolchain with `cargo`, CMake, Ninja
- **Qt Dependencies**: Qt 6 Quick, Quick Controls 2, Network, and Multimedia development packages
- **Optional Packaging Tooling**: Flatpak tooling, Debian packaging tools, AppImage dependencies used by the packaging scripts
- **Environment Variables**:
  - `HDHR_BACKEND_PLAYER_MODE=client` for embedded playback-oriented backend runs during validation
  - `HDHR_BACKEND_BIND` only when validating a non-default loopback port
  - `HDHR_BACKEND_URL` and `HDHR_BACKEND_CMD` only when intentionally overriding client backend launch behavior
- **System Requirements**: Linux development environment with network access to the target HDHomeRun device for real-device validation

## Build Steps

### 1. Verify Core Tooling
```bash
cargo --version
cmake --version
ninja --version
```

### 2. Build Backend Debug Artifacts
```bash
cd /home/felix/src/hdhomerun/hdhomerun-linux/backend
cargo build
```

### 3. Build Client Debug Artifacts
```bash
cd /home/felix/src/hdhomerun/hdhomerun-linux
cmake -S client -B build/client -G Ninja
cmake --build build/client
```

### 4. Build Release Artifacts
```bash
cd /home/felix/src/hdhomerun/hdhomerun-linux/backend
cargo build --release

cd /home/felix/src/hdhomerun/hdhomerun-linux
cmake -S client -B build/client-release -G Ninja -DCMAKE_BUILD_TYPE=Release
cmake --build build/client-release
```

### 5. Build Packaged Outputs
```bash
cd /home/felix/src/hdhomerun/hdhomerun-linux
./packaging/build-and-verify-dist.sh
```

## Verify Build Success
- **Backend Artifacts**: `backend/target/debug/hdhomerun-backend`, `backend/target/release/hdhomerun-backend`
- **Client Artifacts**: `build/client/hdhomerun-linux-player`, `build/client-release/hdhomerun-linux-player`
- **Packaged Artifacts**: `dist/*.deb`, `dist/*.AppImage`, `dist/*.flatpak`
- **Expected Output**: backend and client builds complete without compile errors; packaging script finishes with distribution artifacts under `dist/`

## Troubleshooting

### Build Fails With Missing Qt Modules
- **Cause**: Qt 6 development packages are incomplete
- **Solution**: install the Quick, Quick Controls 2, Network, and Multimedia development packages for your distribution, then rerun the client configure step

### Build Fails With Cargo Dependency Or Toolchain Errors
- **Cause**: stale Rust toolchain or missing target configuration
- **Solution**: update Rust, confirm `cargo` is on `PATH`, and rerun `cargo build` from `backend/`

### Packaging Pipeline Fails Late
- **Cause**: one of the package-specific prerequisites is missing or a package verification step failed
- **Solution**: rerun the lower-level commands from `packaging/README.md` to isolate the failing format before retrying the full pipeline