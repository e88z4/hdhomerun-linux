# Component Inventory

## Application Packages
- `libhdhomerun/hdhomerun_config.c` - Existing command-line application for device operations.

## Infrastructure Packages
- `libhdhomerun/` - Vendor control and stream transport library.
- `sdnet/` - Cross-platform support library for networking, crypto, web, files, threads, and OS abstractions.

## Shared Packages
- `documentation/` - Documentation pointer and references.
- `hdhomerun-linux/` - Host repository for AI-DLC artifacts and future Linux player application code.

## Test Packages
- No dedicated automated test package is currently present.
- `sdnet/src/crypto/crypto_test.c` suggests isolated test-style code, not a complete test harness for the product.

## Total Count
- **Top-Level Packages**: 4
- **Application**: 1
- **Infrastructure**: 2
- **Documentation/Host**: 2 roles within 2 packages
- **Dedicated Test Packages**: 0

## Priority Reuse Ranking
1. `libhdhomerun` for discovery, tuner control, and stream handling concepts.
2. HDHomeRun HTTP API for lineup enumeration and direct stream URLs.
3. `sdnet` only if later iterations need an in-tree web or networking support layer.