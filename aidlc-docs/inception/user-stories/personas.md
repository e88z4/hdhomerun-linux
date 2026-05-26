# Personas

## Persona 1: Primary Linux Viewer
- **Name**: Alex
- **Role**: Everyday Linux desktop user with an HDHomeRun device
- **Goals**:
  - Launch the app and get to live TV quickly
  - See available channels without dealing with tuner internals
  - Switch channels inside one stable player experience
- **Frustrations**:
  - Linux-first TV viewing options are limited
  - Browser playback for live TV is unreliable
  - Existing vendor tooling is too low-level for daily watching
- **Environment**:
  - Uses Linux as a primary desktop environment
  - Has one or more HDHomeRun devices on the local network
  - Expects a polished modern UI, not a debug utility

## Persona 2: Advanced Home TV User
- **Name**: Morgan
- **Role**: Power user who wants more visibility into tuner and signal behavior
- **Goals**:
  - Understand tuner status and signal health while watching or troubleshooting
  - Recover quickly when live playback fails or tuners are busy
  - Use a product that can evolve into a broader Linux TV toolset later
- **Frustrations**:
  - Generic error messages hide the real cause of playback failures
  - Tuner contention is confusing when multiple devices or clients are involved
  - Many consumer apps hide useful device information
- **Environment**:
  - Comfortable with Linux and device-level concepts
  - May operate multiple tuners or test multiple channels frequently

## Persona 3: DVR Household Manager
- **Name**: Taylor
- **Role**: Primary household organizer for recordings, playback, and storage awareness
- **Goals**:
  - Create and manage recording rules without leaving the Linux app
  - Browse recorded content quickly and play it back reliably
  - Understand whether DVR is ready, what will record, and what has already been recorded
  - Prefer local DVR storage when multiple storage sources are visible
- **Frustrations**:
  - Recording systems often hide where shows are stored or why a show did not record
  - Separate apps or web flows for rules and playback feel fragmented
  - DVR readiness problems are hard to diagnose when live TV still appears to work
- **Environment**:
  - Uses the same Linux desktop for live viewing and DVR management
  - May have multiple storage engines or mixed local and non-local DVR sources
  - Expects a clear product surface rather than a pile of low-level endpoints

## Persona Mapping Summary
- **Alex** primarily drives the launch, discovery, channel browsing, and playback stories.
- **Morgan** primarily drives the tuner visibility and playback-failure recovery stories.
- **Taylor** primarily drives DVR readiness, recording-rule management, recorded-library browsing, deletion, and scheduled or recorded state awareness stories.