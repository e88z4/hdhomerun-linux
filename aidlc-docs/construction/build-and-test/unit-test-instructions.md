# Unit Test Execution

## Run Unit Tests

### 1. Execute Backend Tests
```bash
cd /home/felix/src/hdhomerun/hdhomerun-linux/backend
cargo test
```

### 2. Execute Client Tests
```bash
cd /home/felix/src/hdhomerun/hdhomerun-linux
cmake -S client -B build/client -G Ninja
cmake --build build/client
ctest --test-dir build/client --output-on-failure
```

### 3. Review Results
- **Expected Backend Result**: all Rust unit and contract tests pass, including `dvr_contract` and `playback_contract`
- **Expected Client Result**: `client-offscreen-smoke`, `backendlaunchconfig-tests`, `channelnavigation-tests`, and `dvrworkspacehelpers-tests` pass
- **Coverage Note**: this repo currently relies on targeted contract and helper coverage rather than a published numeric coverage threshold

### 4. Fix Failing Tests
1. Review the failing command output.
2. Fix only the code directly responsible for the failure.
3. Rerun the relevant narrow test target first.
4. Rerun the full backend or client test suite before closing the issue.