# Requirements Verification Questions

This file records the interview answers gathered before requirements generation.

## Question 1
What form should the product target after evaluating browser-extension constraints?

A) Native Linux desktop app

B) Local web app

C) Browser extension

D) VLC plugin or integration

X) Other (please describe after [Answer]: tag below)

[Answer]: A

## Question 2
What architectural shape should v1 use?

A) Single-process desktop app

B) Two-part design with a reusable local backend and a desktop UI client

X) Other (please describe after [Answer]: tag below)

[Answer]: B

## Question 3
Which UI direction is preferred?

A) GTK4 or libadwaita

B) Qt/QML

C) No preference

X) Other (please describe after [Answer]: tag below)

[Answer]: B

## Question 4
How should channel changing behave in v1?

A) Persistent in-app player session preferred

B) Reopen playback on every channel change is acceptable

C) No preference

X) Other (please describe after [Answer]: tag below)

[Answer]: A

## Question 5
Which features are required in the first usable version?

A) Device discovery, channel list, live playback, and tuner status or signal info

B) Add favorites and recents to A

C) Add channel search or filtering to A

D) Minimal live playback only

X) Other (please describe after [Answer]: tag below)

[Answer]: A

## Question 6
What is the packaging target for the first deliverable?

A) AppImage only

B) Flatpak only

C) Debian package only

D) AppImage, Flatpak, and Debian package

X) Other (please describe after [Answer]: tag below)

[Answer]: D

## Question 7
How should the backend be structured from day one?

A) Standalone local service component from day one, bundled with the app

B) Internal app backend first, split later if needed

C) No strong preference

X) Other (please describe after [Answer]: tag below)

[Answer]: A

## Question 8
Should recording support be in v1?

A) No, live TV only

B) Simple record-current-channel button

C) Full DVR thinking later, not now

X) Other (please describe after [Answer]: tag below)

[Answer]: A

## Question 9
Should security extension rules be enforced for this project?

A) Yes, enforce all security rules as blocking constraints

B) No, skip security rules for now

X) Other (please describe after [Answer]: tag below)

[Answer]: A

## Question 10
Should property-based testing rules be enforced for this project?

A) Yes, enforce broadly

B) Partial, enforce for pure functions and serialization-style logic only

C) No, skip property-based testing rules for now

X) Other (please describe after [Answer]: tag below)

[Answer]: B