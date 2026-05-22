# Code Generation Plan - Unit 4 Bottom Channel Strip Refresh

## Execution Checklist
- [x] Review the current left-side channel rail and its effect on shell density
- [x] Replace the left rail layout with a bottom-positioned channel selector under the playback stage
- [x] Convert channel browsing into a horizontal scrollable strip aligned with left/right keyboard channel switching
- [x] Keep the current-channel item visible when selection changes
- [x] Re-run client and distribution validation after the layout change

## Chosen Default Decisions
- **Placement**: move the available-channel selector below the playback stage instead of reserving a full left column
- **Interaction model**: horizontal scrollable strip with flick support and current-channel auto-scroll
- **Fullscreen behavior**: keep the strip hidden in immersive fullscreen so playback still owns the screen
- **Keyboard alignment**: explicitly mirror the left/right channel-switch shortcut model in the strip hint text