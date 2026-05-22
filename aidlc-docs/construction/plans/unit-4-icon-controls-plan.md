# Code Generation Plan - Unit 4 Icon Control Refresh

## Execution Checklist
- [x] Review the current text-only control buttons in the client shell and fullscreen overlay
- [x] Choose a reusable icon-control approach that works in packaged environments without relying on host icon themes
- [x] Replace text-only playback control buttons with icon buttons while preserving the existing actions
- [x] Preserve discoverability with tooltips for the icon-only controls
- [x] Re-run client and distribution validation after the cosmetic change

## Chosen Default Decisions
- **Icon strategy**: use a local reusable QML icon-button component with canvas-drawn vector icons
- **Scope**: convert shell and overlay control buttons, but keep text-based channel entries and informational labels intact
- **Discoverability**: keep tooltips on icon controls so the UI remains readable after removing button text
- **Packaging safety**: avoid freedesktop theme icon dependencies so AppImage, Debian, and Flatpak render the same control surface