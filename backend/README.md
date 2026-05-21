# Backend Foundation

This directory contains Unit 1 of the HDHomeRun Linux player project: the Rust backend foundation and local loopback API.

## Current Scope
- loopback HTTP service skeleton
- health endpoint
- bootstrap and runtime state contract endpoints
- canonical remembered-state persistence in the XDG state directory
- structured API errors
- example-based and property-based test foundations

## Run

```bash
cargo run
```

The backend binds to `127.0.0.1:38080` by default.

## Test

```bash
cargo test
```