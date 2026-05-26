# DVR Feature Requirements Verification Questions

Please answer the following questions to define the DVR feature increment for the existing HDHomeRun Linux Player project.

## Question 1
What is the primary DVR outcome you want this feature to deliver first?

A) Schedule recordings from guide or discover surfaces

B) Browse and play recordings that already exist on the DVR storage engine

C) Manage upcoming recordings and recording rules

D) Deliver an end-to-end DVR slice that includes scheduling, upcoming items, and recorded playback

X) Other (please describe after [Answer]: tag below)

[Answer]: X - set recording rules within the application and support playback capability for recorded content

## Question 2
What should the first implementation slice prioritize?

A) A read-only DVR dashboard so I can inspect the DVR state before changing anything

B) Recording rule creation and editing first

C) Recorded library playback first

D) A minimal end-to-end flow from selecting a program to seeing it appear in upcoming recordings

X) Other (please describe after [Answer]: tag below)

[Answer]: C

## Question 3
Where should DVR controls and entry points appear in the product?

A) Inside the existing live guide only

B) Inside discover, search, and series or episode flows only

C) In a dedicated DVR area for recordings, upcoming items, and rules

D) In all of the above so DVR is integrated across the app

X) Other (please describe after [Answer]: tag below)

[Answer]: C

## Question 4
Which recording-rule types should the feature support in its first usable version?

A) Series rules only

B) One-time rules only

C) Both series and one-time rules

D) Start with one-time rules, but keep the design ready for series rules next

X) Other (please describe after [Answer]: tag below)

[Answer]: C

## Question 5
How much of the recording-rule option surface should be exposed initially?

A) Minimal on or off behavior only

B) Basic options such as recent-only and start or end padding

C) Full HDHomeRun option coverage including channel filters, team filters, padding, and original-airdate filtering

D) A custom subset chosen for the first release

X) Other (please describe after [Answer]: tag below)

[Answer]: C

## Question 6
What should happen with upcoming recordings in the first release?

A) Show the upcoming list only

B) Show the upcoming list and allow cancel or edit actions

C) Show the upcoming list, rule priorities, and enough context to understand scheduling outcomes

D) Defer upcoming-recordings UI until after rule creation works

X) Other (please describe after [Answer]: tag below)

[Answer]: X - use scheduling-guide color coding so the UI shows whether a show will be recorded or has already been recorded

## Question 7
What recorded-library behavior do you want in scope?

A) No recorded-library UI yet

B) List recordings and play them

C) List recordings, play them, and delete them

D) List recordings, play them, delete them, and track resume or watched state

X) Other (please describe after [Answer]: tag below)

[Answer]: C

## Question 8
How should the app treat multiple local HDHomeRun storage engines or StorageURL endpoints?

A) Merge all discovered storage engines into one logical DVR view

B) Let the user choose one storage engine at a time

C) Support a merged user view plus per-engine diagnostics and status

X) Other (please describe after [Answer]: tag below)

[Answer]: X - prioritize local storage first, then non-local storage next

## Question 9
What environment should this DVR feature assume for the first release?

A) The user already has HDHomeRun DVR storage and guide service working, so the app can focus on control and playback

B) The app should help reveal missing DVR prerequisites such as no storage engine or no guide-backed capability

C) The app should eventually help set up DVR, but the first release can assume a working DVR environment

X) Other (please describe after [Answer]: tag below)

[Answer]: B

## Question 10
How should the Linux Player architecture handle the DVR APIs?

A) The backend should own DVR API access and expose stable loopback endpoints to the client

B) The client may call some SiliconDust APIs directly for read-only DVR data while the backend handles local storage interactions

C) No strong preference as long as the final UX is good

X) Other (please describe after [Answer]: tag below)

[Answer]: A

## Question 11
What outcome would make you say the first DVR increment is successful?

A) I can create or modify recording rules from the Linux app

B) I can see upcoming scheduled recordings clearly and trust what will record

C) I can browse and watch existing recordings from the Linux app

D) I can do all of A, B, and C in one coherent workflow

X) Other (please describe after [Answer]: tag below)

[Answer]: X - I am able to set up a recording rule and play back the recorded show

## Question 12
Should security extension rules continue to be enforced for this DVR feature work?

A) Yes — enforce all SECURITY rules as blocking constraints

B) No — skip all SECURITY rules for this feature

X) Other (please describe after [Answer]: tag below)

[Answer]: A

## Question 13
Should property-based testing rules continue to be enforced for this DVR feature work?

A) Yes — enforce all PBT rules as blocking constraints

B) Partial — enforce PBT rules only for pure functions and serialization round-trips

C) No — skip all PBT rules for this feature

X) Other (please describe after [Answer]: tag below)

[Answer]: B