# Code Generation Plan - Unit 8 DVR Client Workspace

## Execution Checklist
- [x] Review Unit 8 functional design, NFR requirements, and NFR design artifacts
- [x] Review existing client shell, QML layout, and AppController seams
- [x] Identify brownfield file targets for DVR workspace UI and controller integration
- [x] Step 1. Extend `client/src/appcontroller.h` and `client/src/appcontroller.cpp` with DVR workspace state, DVR API calls, grouped recordings presentation state, and rule-editor launch state
- [x] Step 2. Extend `client/qml/Main.qml` to add DVR workspace navigation, banner handling, recordings browser, details panel, and upcoming or rule-entry surfaces
- [x] Step 3. Add new QML components under `client/qml/components/` for DVR-specific panels such as recordings groups, recording details, banners, and confirmation dialogs
- [x] Step 4. Reuse and adapt the existing playback stage so recorded playback is presented inside the DVR workspace without introducing a second player architecture
- [x] Step 5. Add delete-confirmation behavior and backend action wiring for delete and delete-and-rerecord choices
- [x] Step 6. Add or update client tests or smoke-oriented coverage for DVR workspace state transitions where practical in the current client setup
- [x] Step 7. Build and run relevant client checks and fix Unit 8 issues directly related to this scope
- [x] Step 8. Create Unit 8 code summary documentation in `aidlc-docs/construction/unit-8-dvr-client-workspace/code/summary.md`
- [x] Step 9. Validate that all Unit 8 changes are in-place brownfield changes with no duplicate file creation outside the planned client surface

## Brownfield File Targets
- `client/src/appcontroller.h`
- `client/src/appcontroller.cpp`
- `client/qml/Main.qml`
- `client/qml/components/`
- `client/tests/` or existing client test surface if applicable
- `aidlc-docs/construction/unit-8-dvr-client-workspace/code/summary.md`

## Unit and Story Mapping
- **Unit**: DVR Client Workspace and Rule-Management UX
- **Folder Name**: `unit-8-dvr-client-workspace`
- **Stories Implemented by This Unit**:
  - [x] US-10 Switch between Live TV and DVR workspaces
  - [x] US-11 Browse recorded content with useful defaults
  - [x] US-12 Play a recorded show inside the app
  - [x] US-13 Delete a recording safely
  - [x] US-16 Manage rule detail with flexible options
- **Supporting Stories**:
  - [x] US-17 Recognize what will record and what already exists
  - [x] US-18 Stop Live TV without quitting the app

## Planned Technical Shape
- Add DVR workspace state to the existing `AppController` rather than introducing a parallel client controller.
- Add DVR-specific QML panels and dialogs under the existing client component structure.
- Reuse the shared playback stage with explicit DVR-mode presentation state.
- Keep destructive actions and rule-creation flows backed by the backend loopback API only.