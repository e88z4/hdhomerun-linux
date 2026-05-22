# Business Rules

## Launch and Readiness Rules
- The client must not enter the main browsing shell until backend readiness has been checked.
- The client should auto-start the backend by default when it is not already available.
- Launch overlay behavior must feel like a short-lived progress state, not a configuration wizard.

## Restore and Selection Rules
- The client must treat backend bootstrap as the canonical source of remembered device and playback context.
- If no valid remembered device exists, the client must enter an explicit device-selection state.
- The client must not invent a selected device locally when backend state does not confirm one.

## Browsing and Playback Rules
- Channel selection should trigger playback immediately without a confirmation dialog.
- The channel rail must remain visible or quickly reachable during playback.
- The playback stage must remain stable during channel switches and recoverable failures.
- Client UI must project backend playback session states directly rather than infer hidden player details.

## Device Switching Rules
- Device switching must be explicit and visible in the shell header.
- If playback is active on one device and the user chooses another, the client must require a clean switch action before replacing the current playback context.
- Client-side device switching requires a backend selection contract rather than client-only local state.

## Diagnostics Rules
- Diagnostics should be available as compact summary indicators plus an expandable drawer.
- Diagnostics must emphasize the active playback context first before showing non-active tuners.
- Opening diagnostics must not interrupt current playback or browsing state.

## Failure and Retry Rules
- Recoverable backend, discovery, and playback failures should appear inline in the current workflow.
- Failure copy must stay actionable and user-safe.
- Retry actions must remain close to the failure surface.
- The client should preserve the last focused channel and device context after recoverable playback failures.

## Client State Ownership Rules
- Backend remains the source of truth for canonical device, channel, and playback session state.
- Client may own transient presentation state such as drawer visibility, focused list item, and local layout preferences.
- Client must not duplicate backend orchestration rules inside QML view logic.

## Contract Evolution Rules
- Unit 4 assumes the backend API remains loopback-only.
- Unit 4 requires an explicit device-selection endpoint before full client integration is complete.
- Unit 4 would benefit from a playback retry command path rather than rebuilding retry semantics only in the client.