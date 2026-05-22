# NFR Design Plan - Unit 4 Qt/QML Client Shell and Live-TV User Journey

## Execution Checklist
- [x] Review Unit 4 functional and NFR requirements artifacts
- [x] Choose the client-side patterns that satisfy launch, shell, and failure UX constraints
- [x] Define logical client components that separate API integration, shell state, and presentation
- [x] Generate nfr-design-patterns.md
- [x] Generate logical-components.md

## Chosen Default Decisions
- **Launch pattern**: bounded launch overlay with explicit backend wait and recoverable failure state.
- **State pattern**: a dedicated shell-state projection layer between backend contracts and QML presentation.
- **Failure pattern**: inline recovery surfaces instead of modal interruptions.
- **Diagnostics pattern**: expandable side drawer rather than full-screen diagnostics takeover.