# Domain Entities

## DvrWorkspaceViewModel
- **Purpose**: Top-level client state for the DVR workspace.
- **Fields**:
  - `readinessBanner`
  - `recordingGroups[]`
  - `selectedSeriesId?`
  - `selectedRecordingId?`
  - `detailsPanelState`
  - `playerPanelState`
  - `upcomingPanelState`

## DvrReadinessBannerModel
- **Purpose**: Prominent warning or readiness guidance shown at the top of the DVR workspace.
- **Fields**:
  - `severity`
  - `title`
  - `message`
  - `actions[]`
  - `visible`

## RecordingSeriesGroupViewModel
- **Purpose**: Client grouping model for recordings sharing one series title.
- **Fields**:
  - `seriesTitle`
  - `groupId`
  - `expanded`
  - `latestRecordTime`
  - `episodes[]`

## RecordingEpisodeRowViewModel
- **Purpose**: One episode row inside an expanded recordings group.
- **Fields**:
  - `recordingId`
  - `title`
  - `episodeTitle?`
  - `channelName?`
  - `recordStartTime`
  - `watched`
  - `selected`

## RecordingDetailsViewModel
- **Purpose**: Details-panel state for the currently selected recording.
- **Fields**:
  - `recordingId`
  - `title`
  - `episodeTitle?`
  - `synopsis?`
  - `imageUrl?`
  - `channelName?`
  - `recordStartTime`
  - `resumePosition`
  - `actions[]`

## PlayerPanelState
- **Purpose**: Shared player presentation state within the DVR workspace.
- **Fields**:
  - `mode` (`live | recorded | idle`)
  - `status`
  - `currentRecordingId?`
  - `currentChannelRef?`
  - `warning?`

## DeleteConfirmationDialogModel
- **Purpose**: Confirmation model for recording deletion.
- **Fields**:
  - `recordingId`
  - `title`
  - `supportsRerecord`
  - `confirmDeleteAction`
  - `confirmDeleteRerecordAction`
  - `cancelAction`

## RuleEditorLauncherContext
- **Purpose**: Context object that opens the rule editor from a recording or upcoming item.
- **Fields**:
  - `entrySource` (`recording_details | upcoming_item`)
  - `seriesId?`
  - `programId?`
  - `title`
  - `channelNumber?`
  - `startTime?`

## RuleEditorPanelState
- **Purpose**: Focused panel or sheet state for rule creation or editing.
- **Fields**:
  - `visible`
  - `mode` (`series | one_time | edit`)
  - `context`
  - `formState`
  - `submitState`

## UpcomingItemViewModel
- **Purpose**: DVR upcoming-state row rendered in the workspace.
- **Fields**:
  - `programId`
  - `title`
  - `episodeTitle?`
  - `channelName`
  - `startTime`
  - `scheduleState`
  - `ruleEntryAction`