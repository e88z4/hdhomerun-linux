# Functional Design Plan - Unit 7 DVR Library Playback and Maintenance

## Execution Checklist
- [x] Review Unit 7 responsibilities, story mappings, and application design together
- [x] Define the backend business-logic areas that require detailed modeling for recorded library, recorded playback, deletion, and Live TV stop behavior
- [x] Generate context-appropriate functional-design questions in this document
- [x] Collect answers for the functional-design questions
- [x] Resolve any ambiguity in the answers
- [x] Generate business-logic-model.md
- [x] Generate business-rules.md
- [x] Generate domain-entities.md
- [x] Validate design completeness and consistency

## Unit Context
- **Unit**: Recorded Library, Recorded Playback, Deletion, and Live Stop Control
- **Folder Name**: `unit-7-dvr-library-playback-maintenance`
- **Purpose**: Implement recording-catalog resolution and the playback-session behaviors shared by recorded playback, deletion, and explicit Live TV stop.
- **Primary Story Impact**:
  - US-11 Browse recorded content with useful defaults
  - US-12 Play a recorded show inside the app
  - US-13 Delete a recording safely
  - US-18 Stop Live TV without quitting the app
  - Supporting impact on US-17 through recorded-state projection

## Clarifying Questions

## Question 1
How should the backend resolve duplicates when the same recording appears from multiple storage sources?

A) Prefer the first local source and suppress lower-priority duplicates entirely

B) Merge duplicates into one library item while retaining source metadata for diagnostics and later actions

C) Show separate items for each source and let the client decide which one to use

X) Other (please describe after [Answer]: tag below)

[Answer]:
B

## Question 2
How should recorded playback interact with the existing playback session controller?

A) Reuse the same session controller and explicitly switch the session into recorded-playback mode

B) Use a separate recorded-playback session path to avoid affecting Live TV state

C) Reuse the same player adapter only, but not the existing session controller

X) Other (please describe after [Answer]: tag below)

[Answer]:
A

## Question 3
How strict should deletion safety be in this unit?

A) Require explicit backend validation of the resolved delete target and return a clear failure if the target cannot be trusted

B) Trust the record-engine `CmdURL` as long as it came from the recorded-files listing

C) Allow deletion to proceed optimistically and repair the library state afterward if it fails

X) Other (please describe after [Answer]: tag below)

[Answer]:
A

## Question 4
How should Live TV stop affect remembered context and tuner release behavior?

A) Stop the active live session, release playback resources promptly, and keep the remembered device or channel context for later restart

B) Stop the active live session and clear remembered playback context entirely

C) Hide stop as a soft pause that preserves the active tuner until the app exits

X) Other (please describe after [Answer]: tag below)

[Answer]:
A

## Question 5
How should the backend react if a selected recording disappears between library load and playback or deletion?

A) Return a structured missing-recording result and force the client to refresh the library

B) Try the next lower-priority source automatically if one exists

C) Keep stale library entries until a periodic refresh removes them

X) Other (please describe after [Answer]: tag below)

[Answer]:
A

## Constraints to Preserve
- Local storage sources must be prioritized ahead of non-local storage sources.
- Backend owns all recorded-playback target resolution and delete-target validation.
- Live TV stop must end streaming without quitting the application.
- The client must not receive raw unvalidated delete commands.