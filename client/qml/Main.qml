import QtQuick
import QtQuick.Controls
import QtQuick.Layouts

import HDHomeRun.Client
import "components"

ApplicationWindow {
    id: window

    property int overlayPulse: 0
    property bool fullscreenMode: visibility === Window.FullScreen
    property real guidePanelHeight: 390
    property real guidePanelMinHeight: 280
    property real guidePanelMaxHeight: Math.max(360, height * 0.52)
    property bool dvrWorkspaceActive: appController.workspaceMode === "dvr"

    function bumpOverlay() {
        overlayPulse += 1
    }

    function toggleFullscreen() {
        if (!fullscreenMode) {
            bumpOverlay()
        }
        visibility = fullscreenMode ? Window.Windowed : Window.FullScreen
    }

    function exitFullscreen() {
        if (fullscreenMode) {
            visibility = Window.Windowed
        }
    }

    function adjustActivePlaybackVolume(delta) {
        if (dvrWorkspaceActive) {
            if (dvrWorkspaceLoader.item && dvrWorkspaceLoader.item.adjustVolume) {
                dvrWorkspaceLoader.item.adjustVolume(delta)
            }
            return
        }

        livePlaybackStage.adjustVolume(delta)
    }

    function toggleDvrPlayback() {
        if (dvrWorkspaceLoader.item && dvrWorkspaceLoader.item.togglePlayPause) {
            bumpOverlay()
            dvrWorkspaceLoader.item.togglePlayPause()
        }
    }

    function seekDvrPlayback(deltaMilliseconds) {
        if (dvrWorkspaceLoader.item && dvrWorkspaceLoader.item.seekPlayback) {
            bumpOverlay()
            dvrWorkspaceLoader.item.seekPlayback(deltaMilliseconds)
        }
    }

    function stopDvrPlayback() {
        if (dvrWorkspaceLoader.item && dvrWorkspaceLoader.item.stopPlayback) {
            bumpOverlay()
            dvrWorkspaceLoader.item.stopPlayback()
        }
    }

    function restartDvrPlayback() {
        if (dvrWorkspaceLoader.item && dvrWorkspaceLoader.item.restartPlayback) {
            bumpOverlay()
            dvrWorkspaceLoader.item.restartPlayback()
        }
    }

    function playAdjacentDvrEpisode(direction) {
        if (dvrWorkspaceLoader.item && dvrWorkspaceLoader.item.playAdjacentEpisode) {
            bumpOverlay()
            dvrWorkspaceLoader.item.playAdjacentEpisode(direction)
        }
    }

    minimumWidth: 1180
    minimumHeight: 720
    width: Math.min(Math.max(1440, Math.round(Screen.width * 0.88)), Math.max(960, Screen.width - 80))
    height: Math.min(Math.max(900, Math.round(Screen.height * 0.86)), Math.max(640, Screen.height - 80))
    x: Math.max(0, Math.round((Screen.width - width) / 2))
    y: Math.max(0, Math.round((Screen.height - height) / 2))
    visible: true
    title: "HDHomeRun Linux Player"
    color: "#08131c"

    onHeightChanged: {
        guidePanelHeight = Math.max(guidePanelMinHeight, Math.min(guidePanelHeight, guidePanelMaxHeight))
    }

    Shortcut {
        sequence: "F"
        context: Qt.ApplicationShortcut
        onActivated: window.toggleFullscreen()
    }

    Shortcut {
        sequence: "Esc"
        context: Qt.ApplicationShortcut
        enabled: window.fullscreenMode
        onActivated: window.exitFullscreen()
    }

    Shortcut {
        sequence: "Up"
        context: Qt.ApplicationShortcut
        onActivated: {
            window.bumpOverlay()
            if (window.dvrWorkspaceActive) {
                window.adjustActivePlaybackVolume(0.05)
                return
            }
            appController.playAdjacentChannel(-1)
        }
    }

    Shortcut {
        sequence: "Down"
        context: Qt.ApplicationShortcut
        onActivated: {
            window.bumpOverlay()
            if (window.dvrWorkspaceActive) {
                window.adjustActivePlaybackVolume(-0.05)
                return
            }
            appController.playAdjacentChannel(1)
        }
    }

    Shortcut {
        sequence: "Page Up"
        context: Qt.ApplicationShortcut
        onActivated: {
            window.bumpOverlay()
            window.adjustActivePlaybackVolume(0.05)
        }
    }

    Shortcut {
        sequence: "Page Down"
        context: Qt.ApplicationShortcut
        onActivated: {
            window.bumpOverlay()
            window.adjustActivePlaybackVolume(-0.05)
        }
    }

    Shortcut {
        sequence: "Right"
        context: Qt.ApplicationShortcut
        onActivated: {
            if (window.dvrWorkspaceActive) {
                window.seekDvrPlayback(10000)
                return
            }
            window.bumpOverlay()
            appController.playAdjacentChannel(1)
        }
    }

    Shortcut {
        sequence: "Left"
        context: Qt.ApplicationShortcut
        onActivated: {
            if (window.dvrWorkspaceActive) {
                window.seekDvrPlayback(-10000)
                return
            }
            window.bumpOverlay()
            appController.playAdjacentChannel(-1)
        }
    }

    Shortcut {
        sequence: "Space"
        context: Qt.ApplicationShortcut
        enabled: window.dvrWorkspaceActive
        onActivated: window.toggleDvrPlayback()
    }

    Shortcut {
        sequence: "K"
        context: Qt.ApplicationShortcut
        enabled: window.dvrWorkspaceActive
        onActivated: window.toggleDvrPlayback()
    }

    Shortcut {
        sequence: "J"
        context: Qt.ApplicationShortcut
        enabled: window.dvrWorkspaceActive
        onActivated: window.seekDvrPlayback(-10000)
    }

    Shortcut {
        sequence: "L"
        context: Qt.ApplicationShortcut
        enabled: window.dvrWorkspaceActive
        onActivated: window.seekDvrPlayback(10000)
    }

    Shortcut {
        sequence: "Home"
        context: Qt.ApplicationShortcut
        enabled: window.dvrWorkspaceActive
        onActivated: window.restartDvrPlayback()
    }

    Shortcut {
        sequence: "S"
        context: Qt.ApplicationShortcut
        enabled: window.dvrWorkspaceActive
        onActivated: window.stopDvrPlayback()
    }

    Shortcut {
        sequence: "["
        context: Qt.ApplicationShortcut
        enabled: window.dvrWorkspaceActive
        onActivated: window.playAdjacentDvrEpisode(-1)
    }

    Shortcut {
        sequence: "]"
        context: Qt.ApplicationShortcut
        enabled: window.dvrWorkspaceActive
        onActivated: window.playAdjacentDvrEpisode(1)
    }

    Rectangle {
        anchors.fill: parent
        gradient: Gradient {
            GradientStop { position: 0.0; color: "#10283a" }
            GradientStop { position: 0.45; color: "#08131c" }
            GradientStop { position: 1.0; color: "#050a10" }
        }
    }

    header: ToolBar {
        visible: !window.fullscreenMode
        padding: 14
        background: Rectangle {
            color: "#0d1b26"
            border.color: "#173245"
        }

        RowLayout {
            anchors.fill: parent
            spacing: 16

            Label {
                text: "HDHomeRun Linux Player"
                color: "#f3f7fa"
                font.family: "IBM Plex Sans"
                font.pixelSize: 24
                font.bold: true
            }

            Rectangle {
                radius: 18
                color: "#112838"
                border.color: "#20445c"
                implicitWidth: 220
                implicitHeight: 42

                RowLayout {
                    anchors.fill: parent
                    anchors.margins: 4
                    spacing: 4

                    ThemeButton {
                        Layout.fillWidth: true
                        compact: true
                        text: "Live TV"
                        highlighted: !window.dvrWorkspaceActive
                        onClicked: appController.workspaceMode = "live"
                    }

                    ThemeButton {
                        Layout.fillWidth: true
                        compact: true
                        text: "DVR"
                        highlighted: window.dvrWorkspaceActive
                        onClicked: appController.workspaceMode = "dvr"
                    }
                }
            }

            Rectangle {
                Layout.fillWidth: true
                height: 40
                radius: 20
                color: "#112838"
                border.color: "#20445c"

                RowLayout {
                    anchors.fill: parent
                    anchors.leftMargin: 14
                    anchors.rightMargin: 14

                    Label {
                        text: "Device"
                        color: "#9fb5c5"
                        font.family: "IBM Plex Sans"
                    }

                    ComboBox {
                        id: devicePicker
                        Layout.fillWidth: true
                        model: appController.devices
                        textRole: "name"
                        currentIndex: appController.selectedDeviceIndex
                        leftPadding: 12
                        rightPadding: 34
                        topPadding: 0
                        bottomPadding: 0

                        background: Rectangle {
                            radius: 12
                            color: "#102838"
                            border.color: devicePicker.visualFocus ? "#69c6ff" : "#2d5a74"
                        }

                        contentItem: Text {
                            text: devicePicker.displayText
                            color: "#eff7fb"
                            font.family: "IBM Plex Sans"
                            font.pixelSize: 13
                            verticalAlignment: Text.AlignVCenter
                            elide: Text.ElideRight
                        }

                        indicator: Text {
                            text: "▾"
                            color: "#b9cfde"
                            font.family: "IBM Plex Sans"
                            font.pixelSize: 15
                            anchors.verticalCenter: parent.verticalCenter
                            anchors.right: parent.right
                            anchors.rightMargin: 12
                        }

                        delegate: ItemDelegate {
                            required property var modelData

                            width: devicePicker.width - 12
                            text: modelData && modelData.name ? modelData.name : ""
                            font.family: "IBM Plex Sans"
                            font.pixelSize: 13
                            highlighted: devicePicker.highlightedIndex === index
                            hoverEnabled: true

                            background: Rectangle {
                                radius: 10
                                color: parent.highlighted ? "#1d455f" : (parent.hovered ? "#173247" : "transparent")
                                border.color: parent.highlighted ? "#69c6ff" : "transparent"
                            }

                            contentItem: Text {
                                text: parent.text
                                font: parent.font
                                color: "#eff7fb"
                                verticalAlignment: Text.AlignVCenter
                                elide: Text.ElideRight
                            }
                        }

                        popup: Popup {
                            y: devicePicker.height + 6
                            width: devicePicker.width
                            padding: 6

                            background: Rectangle {
                                radius: 16
                                color: "#0d1a24"
                                border.color: "#23445b"
                            }

                            contentItem: ListView {
                                clip: true
                                implicitHeight: Math.min(contentHeight, 240)
                                model: devicePicker.delegateModel
                                currentIndex: devicePicker.highlightedIndex
                                boundsBehavior: Flickable.StopAtBounds
                                ScrollBar.vertical: ScrollBar {}
                            }
                        }

                        onActivated: function(index) { appController.selectDeviceIndex(index) }
                    }
                }
            }
        }
    }

    Item {
        anchors.fill: parent
        anchors.margins: window.fullscreenMode ? 0 : 18

        StackLayout {
            anchors.fill: parent
            currentIndex: window.dvrWorkspaceActive ? 1 : 0

            Item {
                id: liveWorkspace

                PlaybackStage {
                    id: livePlaybackStage
                    anchors.top: parent.top
                    anchors.left: parent.left
                    anchors.right: parent.right
                    anchors.bottom: window.fullscreenMode ? parent.bottom : guideContainer.top
                    anchors.bottomMargin: window.fullscreenMode ? 0 : 14
                    immersiveMode: window.fullscreenMode
                    fullscreenMode: window.fullscreenMode
                    overlayPulse: window.overlayPulse
                    shellPhase: appController.shellPhase
                    currentTitle: appController.stageTitle
                    currentSubtitle: appController.stageSubtitle
                    warningText: appController.stageWarning
                    failureText: appController.stageFailure
                    playbackUrl: appController.playbackUrl
                    playbackMode: appController.playbackMode
                    embeddedPlaybackEnabled: appController.embeddedPlaybackEnabled
                    dvrControlsAllowed: false
                    diagnosticsSummary: appController.diagnosticsSummary
                    diagnosticsRows: appController.diagnosticsRows
                    currentRecordingId: appController.currentRecordingId
                    selectedRecordingId: appController.selectedRecordingId
                    recordingGroups: appController.dvrRecordingGroups
                    retryEnabled: appController.shellPhase === "playback_failed"
                    onExitFullscreenRequested: window.exitFullscreen()
                    onToggleFullscreenRequested: window.toggleFullscreen()
                    onRetryRequested: appController.retryPlayback()
                    onStopPlaybackRequested: appController.stopPlayback()
                    onPlayRecordingRequested: function(recordingId) { appController.playRecording(recordingId) }
                }

                Item {
                    id: guideContainer
                    visible: !window.fullscreenMode
                    anchors.left: parent.left
                    anchors.right: parent.right
                    anchors.bottom: parent.bottom
                    height: window.guidePanelHeight + 12

                    Rectangle {
                        id: guideResizeHandle
                        anchors.top: parent.top
                        anchors.horizontalCenter: parent.horizontalCenter
                        width: 96
                        height: 12
                        radius: 6
                        color: "#23445b"

                        MouseArea {
                            anchors.fill: parent
                            cursorShape: Qt.SizeVerCursor
                            property real dragStartHeight: 0
                            property real dragStartY: 0

                            onPressed: function(mouse) {
                                dragStartHeight = window.guidePanelHeight
                                dragStartY = mouse.y
                            }

                            onPositionChanged: function(mouse) {
                                if (!pressed) {
                                    return
                                }

                                const delta = mouse.y - dragStartY
                                window.guidePanelHeight = Math.max(
                                    window.guidePanelMinHeight,
                                    Math.min(window.guidePanelMaxHeight, dragStartHeight - delta)
                                )
                            }
                        }
                    }

                    GuideGrid {
                        anchors.top: guideResizeHandle.bottom
                        anchors.left: parent.left
                        anchors.right: parent.right
                        anchors.bottom: parent.bottom
                        guideChannels: appController.guideChannels
                        currentChannelRef: appController.currentChannelRef
                        windowStart: appController.guideWindowStart
                        durationHours: appController.guideDurationHours
                        loading: appController.guideLoading
                        onChannelActivated: appController.playChannel(channelRef)
                        onJumpToNowRequested: appController.jumpGuideToNow()
                        onRecordSeriesRequested: appController.createSeriesRuleFromGuide(guideContext)
                        onRecordOnceRequested: appController.createOneTimeRuleFromGuide(guideContext)
                    }
                }
            }

            Item {
                id: dvrWorkspace

                Loader {
                    id: dvrWorkspaceLoader
                    anchors.fill: parent
                    sourceComponent: window.fullscreenMode
                        ? dvrFullscreenComponent
                        : (width < 1480 ? dvrNarrowComponent : dvrWideComponent)
                }
            }
        }
    }

    Component {
        id: dvrFullscreenComponent

        Item {
            function adjustVolume(delta) {
                playbackStage.adjustVolume(delta)
            }

            function togglePlayPause() {
                playbackStage.togglePlayPause()
            }

            function seekPlayback(deltaMilliseconds) {
                playbackStage.seekBy(deltaMilliseconds)
            }

            function stopPlayback() {
                playbackStage.stopDvrPlayback()
            }

            function restartPlayback() {
                playbackStage.restartPlayback()
            }

            function playAdjacentEpisode(direction) {
                playbackStage.playAdjacentRecording(direction)
            }

            PlaybackStage {
                id: playbackStage
                anchors.fill: parent
                immersiveMode: true
                fullscreenMode: window.fullscreenMode
                overlayPulse: window.overlayPulse
                shellPhase: appController.shellPhase
                currentTitle: appController.stageTitle
                currentSubtitle: appController.stageSubtitle
                warningText: appController.stageWarning
                failureText: appController.stageFailure
                playbackUrl: appController.playbackUrl
                playbackMode: appController.playbackMode
                embeddedPlaybackEnabled: appController.embeddedPlaybackEnabled
                dvrControlsAllowed: true
                diagnosticsSummary: appController.diagnosticsSummary
                diagnosticsRows: appController.diagnosticsRows
                currentRecordingId: appController.currentRecordingId
                selectedRecordingId: appController.selectedRecordingId
                recordingGroups: appController.dvrRecordingGroups
                retryEnabled: appController.shellPhase === "playback_failed"
                onExitFullscreenRequested: window.exitFullscreen()
                onToggleFullscreenRequested: window.toggleFullscreen()
                onRetryRequested: appController.retryPlayback()
                onStopPlaybackRequested: appController.stopPlayback()
                onPlayRecordingRequested: function(recordingId) { appController.playRecording(recordingId) }
            }
        }
    }

    Component {
        id: dvrWideComponent

        Item {
            function adjustVolume(delta) {
                playbackStage.adjustVolume(delta)
            }

            function togglePlayPause() {
                playbackStage.togglePlayPause()
            }

            function seekPlayback(deltaMilliseconds) {
                playbackStage.seekBy(deltaMilliseconds)
            }

            function stopPlayback() {
                playbackStage.stopDvrPlayback()
            }

            function restartPlayback() {
                playbackStage.restartPlayback()
            }

            function playAdjacentEpisode(direction) {
                playbackStage.playAdjacentRecording(direction)
            }

            RowLayout {
                anchors.fill: parent
                spacing: 12

                DvrRecordingsPanel {
                    Layout.preferredWidth: 430
                    Layout.fillHeight: true
                    recordingGroups: appController.dvrRecordingGroups
                    selectedRecordingId: appController.selectedRecordingId
                    loading: appController.dvrLoading
                    recordingsState: appController.dvrRecordingsState
                    onRefreshRequested: appController.refreshDvrWorkspace()
                    onGroupToggled: function(groupId) { appController.toggleRecordingGroup(groupId) }
                    onRecordingSelected: function(recordingId) { appController.selectRecording(recordingId) }
                    onPlayRequested: function(recordingId) { appController.playRecording(recordingId) }
                    onDeleteRequested: function(recordingId) { appController.requestDeleteRecording(recordingId) }
                    onRuleEditorRequested: function(recordingId) { appController.openRuleEditorFromRecording(recordingId) }
                }

                Item {
                    Layout.fillWidth: true
                    Layout.fillHeight: true

                    DvrStatusBanner {
                        id: dvrWideStatusBanner
                        visible: appController.dvrBannerVisible
                        anchors.top: parent.top
                        anchors.left: parent.left
                        anchors.right: parent.right
                        severity: appController.dvrBannerSeverity
                        titleText: appController.dvrBannerTitle
                        messageText: appController.dvrBannerMessage
                        onRefreshRequested: appController.refreshDvrWorkspace()
                    }

                    PlaybackStage {
                        id: playbackStage
                        anchors.top: dvrWideStatusBanner.visible ? dvrWideStatusBanner.bottom : parent.top
                        anchors.topMargin: dvrWideStatusBanner.visible ? 12 : 0
                        anchors.left: parent.left
                        anchors.right: parent.right
                        anchors.bottom: dvrWideDetailsPanel.top
                        anchors.bottomMargin: 12
                        immersiveMode: false
                        fullscreenMode: window.fullscreenMode
                        overlayPulse: window.overlayPulse
                        shellPhase: appController.shellPhase
                        currentTitle: appController.stageTitle
                        currentSubtitle: appController.stageSubtitle
                        warningText: appController.stageWarning
                        failureText: appController.stageFailure
                        playbackUrl: appController.playbackUrl
                        playbackMode: appController.playbackMode
                        embeddedPlaybackEnabled: appController.embeddedPlaybackEnabled
                        dvrControlsAllowed: true
                        diagnosticsSummary: appController.diagnosticsSummary
                        diagnosticsRows: appController.diagnosticsRows
                        currentRecordingId: appController.currentRecordingId
                        selectedRecordingId: appController.selectedRecordingId
                        recordingGroups: appController.dvrRecordingGroups
                        retryEnabled: appController.shellPhase === "playback_failed"
                        onExitFullscreenRequested: window.exitFullscreen()
                        onToggleFullscreenRequested: window.toggleFullscreen()
                        onRetryRequested: appController.retryPlayback()
                        onStopPlaybackRequested: appController.stopPlayback()
                        onPlayRecordingRequested: function(recordingId) { appController.playRecording(recordingId) }
                    }

                    DvrDetailsPanel {
                        id: dvrWideDetailsPanel
                        anchors.left: parent.left
                        anchors.right: parent.right
                        anchors.bottom: parent.bottom
                        height: 250
                        recording: appController.selectedRecordingDetails
                        playbackMode: appController.playbackMode
                        currentRecordingId: appController.currentRecordingId
                        onPlayRequested: function(recordingId) { appController.playRecording(recordingId) }
                        onStopPlaybackRequested: appController.stopPlayback()
                        onDeleteRequested: function(recordingId) { appController.requestDeleteRecording(recordingId) }
                        onRuleEditorRequested: function(recordingId) { appController.openRuleEditorFromRecording(recordingId) }
                    }
                }

                DvrUpcomingPanel {
                    Layout.preferredWidth: 360
                    Layout.fillHeight: true
                    upcomingEntries: appController.dvrUpcomingEntries
                    loading: appController.dvrUpcomingLoading
                    upcomingState: appController.dvrUpcomingState
                    onRefreshRequested: appController.refreshDvrWorkspace()
                    onRuleEditorRequested: function(programId) { appController.openRuleEditorFromUpcoming(programId) }
                    onDeleteRuleRequested: function(programId) { appController.deleteRuleForUpcoming(programId) }
                }
            }
        }
    }

    Component {
        id: dvrNarrowComponent

        Item {
            function adjustVolume(delta) {
                playbackStage.adjustVolume(delta)
            }

            function togglePlayPause() {
                playbackStage.togglePlayPause()
            }

            function seekPlayback(deltaMilliseconds) {
                playbackStage.seekBy(deltaMilliseconds)
            }

            function stopPlayback() {
                playbackStage.stopDvrPlayback()
            }

            function restartPlayback() {
                playbackStage.restartPlayback()
            }

            function playAdjacentEpisode(direction) {
                playbackStage.playAdjacentRecording(direction)
            }

            ColumnLayout {
                anchors.fill: parent
                spacing: 12

                DvrStatusBanner {
                    visible: appController.dvrBannerVisible
                    Layout.fillWidth: true
                    severity: appController.dvrBannerSeverity
                    titleText: appController.dvrBannerTitle
                    messageText: appController.dvrBannerMessage
                    onRefreshRequested: appController.refreshDvrWorkspace()
                }

                DvrRecordingsPanel {
                    Layout.fillWidth: true
                    Layout.preferredHeight: 320
                    recordingGroups: appController.dvrRecordingGroups
                    selectedRecordingId: appController.selectedRecordingId
                    loading: appController.dvrLoading
                    recordingsState: appController.dvrRecordingsState
                    onRefreshRequested: appController.refreshDvrWorkspace()
                    onGroupToggled: function(groupId) { appController.toggleRecordingGroup(groupId) }
                    onRecordingSelected: function(recordingId) { appController.selectRecording(recordingId) }
                    onPlayRequested: function(recordingId) { appController.playRecording(recordingId) }
                    onDeleteRequested: function(recordingId) { appController.requestDeleteRecording(recordingId) }
                    onRuleEditorRequested: function(recordingId) { appController.openRuleEditorFromRecording(recordingId) }
                }

                PlaybackStage {
                    id: playbackStage
                    Layout.fillWidth: true
                    Layout.preferredHeight: 320
                    immersiveMode: false
                    fullscreenMode: window.fullscreenMode
                    overlayPulse: window.overlayPulse
                    shellPhase: appController.shellPhase
                    currentTitle: appController.stageTitle
                    currentSubtitle: appController.stageSubtitle
                    warningText: appController.stageWarning
                    failureText: appController.stageFailure
                    playbackUrl: appController.playbackUrl
                    playbackMode: appController.playbackMode
                    embeddedPlaybackEnabled: appController.embeddedPlaybackEnabled
                    dvrControlsAllowed: true
                    diagnosticsSummary: appController.diagnosticsSummary
                    diagnosticsRows: appController.diagnosticsRows
                    currentRecordingId: appController.currentRecordingId
                    selectedRecordingId: appController.selectedRecordingId
                    recordingGroups: appController.dvrRecordingGroups
                    retryEnabled: appController.shellPhase === "playback_failed"
                    onExitFullscreenRequested: window.exitFullscreen()
                    onToggleFullscreenRequested: window.toggleFullscreen()
                    onRetryRequested: appController.retryPlayback()
                    onStopPlaybackRequested: appController.stopPlayback()
                    onPlayRecordingRequested: function(recordingId) { appController.playRecording(recordingId) }
                }

                DvrDetailsPanel {
                    Layout.fillWidth: true
                    Layout.preferredHeight: 240
                    recording: appController.selectedRecordingDetails
                    playbackMode: appController.playbackMode
                    currentRecordingId: appController.currentRecordingId
                    onPlayRequested: function(recordingId) { appController.playRecording(recordingId) }
                    onStopPlaybackRequested: appController.stopPlayback()
                    onDeleteRequested: function(recordingId) { appController.requestDeleteRecording(recordingId) }
                    onRuleEditorRequested: function(recordingId) { appController.openRuleEditorFromRecording(recordingId) }
                }

                DvrUpcomingPanel {
                    Layout.fillWidth: true
                    Layout.fillHeight: true
                    upcomingEntries: appController.dvrUpcomingEntries
                    loading: appController.dvrUpcomingLoading
                    upcomingState: appController.dvrUpcomingState
                    onRefreshRequested: appController.refreshDvrWorkspace()
                    onRuleEditorRequested: function(programId) { appController.openRuleEditorFromUpcoming(programId) }
                    onDeleteRuleRequested: function(programId) { appController.deleteRuleForUpcoming(programId) }
                }
            }
        }
    }

    DvrRuleEditorDialog {
        dialogVisible: appController.dvrRuleEditorVisible
        titleText: appController.dvrRuleEditorTitle
        messageText: appController.dvrRuleEditorMessage
        canCreateSeries: appController.dvrRuleEditorCanCreateSeries
        canCreateOneTime: appController.dvrRuleEditorCanCreateOneTime
        submitting: appController.dvrRuleEditorSubmitting
        onCloseRequested: appController.closeRuleEditor()
        onCreateSeriesRequested: appController.createSeriesRule()
        onCreateOneTimeRequested: appController.createOneTimeRule()
    }

    DvrDeleteDialog {
        dialogVisible: appController.dvrDeleteDialogVisible
        titleText: appController.dvrDeleteDialogTitle
        busy: appController.dvrDeleteDialogBusy
        onCloseRequested: appController.cancelDeleteRecording()
        onDeleteRequested: appController.confirmDeleteRecording(false)
        onDeleteRerecordRequested: appController.confirmDeleteRecording(true)
    }
}
