import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import QtMultimedia

Pane {
    id: root

    required property bool immersiveMode
    required property bool fullscreenMode
    required property int overlayPulse
    required property string shellPhase
    required property string currentChannelRef
    required property string currentTitle
    required property string currentSubtitle
    required property string warningText
    required property string failureText
    required property string playbackUrl
    required property string playbackMode
    required property bool embeddedPlaybackEnabled
    required property bool dvrControlsAllowed
    required property string diagnosticsSummary
    required property var diagnosticsRows
    required property bool retryEnabled
    property string currentRecordingId: ""
    property string selectedRecordingId: ""
    property var recordingGroups: []
    signal exitFullscreenRequested()
    signal toggleFullscreenRequested()
    signal retryRequested()
    signal stopPlaybackRequested()
    signal playChannelRequested(string channelRef)
    signal playRecordingRequested(string recordingId)

    property string surfaceErrorText: ""
    property bool controlOverlayVisible: false
    property bool immersiveOverlayVisible: false
    property real volumeLevel: 1.0
    property real pendingSeekPosition: 0
    readonly property int volumePercent: Math.round(volumeLevel * 100)
    readonly property bool volumeControlEnabled: embeddedPlaybackEnabled
    readonly property bool dvrControlMode: dvrControlsAllowed && (playbackMode === "recorded" || selectedRecordingId !== "" || currentRecordingId !== "")
    readonly property bool liveControlMode: !dvrControlsAllowed && (playbackMode === "live" || currentChannelRef !== "")
    readonly property bool surfaceOverlayEnabled: immersiveMode || dvrControlMode || liveControlMode
    readonly property string activeRecordingId: currentRecordingId !== "" ? currentRecordingId : selectedRecordingId
    readonly property var recordingNavigation: findRecordingNavigation(activeRecordingId)
    readonly property bool seekControlEnabled: dvrControlMode && player.seekable && player.duration > 0

    function formatPlaybackTime(milliseconds) {
        if (milliseconds <= 0) {
            return "00:00"
        }

        const totalSeconds = Math.floor(milliseconds / 1000)
        const hours = Math.floor(totalSeconds / 3600)
        const minutes = Math.floor((totalSeconds % 3600) / 60)
        const seconds = totalSeconds % 60

        if (hours > 0) {
            return hours + ":" + String(minutes).padStart(2, "0") + ":" + String(seconds).padStart(2, "0")
        }

        return String(minutes).padStart(2, "0") + ":" + String(seconds).padStart(2, "0")
    }

    function findRecordingNavigation(recordingId) {
        if (!recordingId) {
            return { previousId: "", nextId: "" }
        }

        for (const group of recordingGroups) {
            const recordings = group.recordings || []
            for (let index = 0; index < recordings.length; index += 1) {
                const recording = recordings[index]
                if (recording.recordingId === recordingId) {
                    return {
                        previousId: index > 0 ? recordings[index - 1].recordingId : "",
                        nextId: index + 1 < recordings.length ? recordings[index + 1].recordingId : ""
                    }
                }
            }
        }

        return { previousId: "", nextId: "" }
    }

    function togglePlayPause() {
        if (!dvrControlMode) {
            return
        }

        if (player.playbackState === MediaPlayer.PlayingState) {
            player.pause()
            return
        }

        if (player.playbackState === MediaPlayer.PausedState) {
            player.play()
            return
        }

        if (activeRecordingId !== "") {
            playRecordingRequested(activeRecordingId)
        }
    }

    function restartPlayback() {
        if (!dvrControlMode) {
            return
        }

        if (player.playbackState === MediaPlayer.StoppedState) {
            if (activeRecordingId !== "") {
                playRecordingRequested(activeRecordingId)
            }
            return
        }

        player.position = 0
        player.play()
    }

    function playAdjacentRecording(direction) {
        const targetRecordingId = direction < 0 ? recordingNavigation.previousId : recordingNavigation.nextId
        if (targetRecordingId !== "") {
            playRecordingRequested(targetRecordingId)
        }
    }

    function pointInItem(mouse, item) {
        if (!item || !item.visible) {
            return false
        }

        return mouse.x >= item.x
            && mouse.x <= item.x + item.width
            && mouse.y >= item.y
            && mouse.y <= item.y + item.height
    }

    function seekBy(deltaMilliseconds) {
        if (!seekControlEnabled) {
            return
        }

        player.position = Math.max(0, Math.min(player.duration, player.position + deltaMilliseconds))
        pendingSeekPosition = player.position
        revealOverlay()
    }

    function stopDvrPlayback() {
        if (!dvrControlMode) {
            return
        }

        stopPlaybackRequested()
        revealOverlay()
    }

    function toggleLivePlayback() {
        if (!liveControlMode) {
            return
        }

        if (player.playbackState === MediaPlayer.PlayingState) {
            player.pause()
            return
        }

        if (player.playbackState === MediaPlayer.PausedState) {
            player.play()
            return
        }

        if (currentChannelRef !== "") {
            playChannelRequested(currentChannelRef)
        }
    }

    function stopLivePlayback() {
        if (!liveControlMode) {
            return
        }

        stopPlaybackRequested()
        revealOverlay()
    }

    function revealOverlay() {
        controlOverlayVisible = true
        immersiveOverlayVisible = immersiveMode
        overlayHideTimer.restart()
    }

    function adjustVolume(delta) {
        if (!volumeControlEnabled) {
            return
        }

        volumeLevel = Math.max(0.0, Math.min(1.0, volumeLevel + delta))
        revealOverlay()
    }

    function syncPlayback() {
        if (!embeddedPlaybackEnabled || playbackUrl === "" || shellPhase === "playback_failed" || shellPhase === "device_selection") {
            player.stop()
            return
        }

        if (shellPhase === "playing" || shellPhase === "playback_loading") {
            player.play()
            return
        }

        player.stop()
    }

    onShellPhaseChanged: {
        if (surfaceOverlayEnabled) {
            revealOverlay()
        }
        if (shellPhase === "playback_loading") {
            surfaceErrorText = ""
        }
        syncPlayback()
    }
    onPlaybackUrlChanged: {
        surfaceErrorText = ""
        syncPlayback()
    }
    onEmbeddedPlaybackEnabledChanged: syncPlayback()
    onImmersiveModeChanged: if (surfaceOverlayEnabled) revealOverlay()
    onOverlayPulseChanged: if (surfaceOverlayEnabled) revealOverlay()
    onCurrentTitleChanged: if (surfaceOverlayEnabled) revealOverlay()
    onCurrentSubtitleChanged: if (surfaceOverlayEnabled) revealOverlay()
    onCurrentChannelRefChanged: if (surfaceOverlayEnabled) revealOverlay()
    onWarningTextChanged: if (surfaceOverlayEnabled) revealOverlay()
    onFailureTextChanged: if (surfaceOverlayEnabled) revealOverlay()
    onVolumeLevelChanged: if (surfaceOverlayEnabled) revealOverlay()
    onDvrControlModeChanged: pendingSeekPosition = player.position

    Timer {
        id: overlayHideTimer
        interval: 5000
        repeat: false
        onTriggered: {
            root.controlOverlayVisible = false
            root.immersiveOverlayVisible = false
        }
    }

    padding: immersiveMode ? 0 : 14
    background: Rectangle {
        radius: root.immersiveMode ? 0 : 34
        color: root.immersiveMode ? "transparent" : "#09141d"
        border.width: root.immersiveMode ? 0 : 1
        border.color: "#183345"
    }

    Item {
        anchors.fill: parent

        RowLayout {
            id: stageHeader
            visible: !root.immersiveMode
            anchors.top: parent.top
            anchors.left: parent.left
            anchors.right: parent.right

            ColumnLayout {
                Layout.fillWidth: true

                Label {
                    text: root.currentTitle
                    color: "#f6fbff"
                    font.family: "IBM Plex Sans"
                    font.pixelSize: 30
                    font.bold: true
                }

                Label {
                    text: root.currentSubtitle
                    color: "#91a8b7"
                    font.family: "IBM Plex Sans"
                    font.pixelSize: 15
                }
            }

            Rectangle {
                radius: 16
                color: root.shellPhase === "playing" ? "#163c29" : (root.shellPhase === "playback_failed" ? "#472016" : "#173247")
                border.color: root.shellPhase === "playing" ? "#4ab97d" : (root.shellPhase === "playback_failed" ? "#ff955c" : "#5cb9ea")
                implicitWidth: 150
                implicitHeight: 40

                Label {
                    anchors.centerIn: parent
                    text: root.shellPhase === "playing" ? "Playing" : (root.shellPhase === "playback_failed" ? "Needs Retry" : "Preparing")
                    color: "#eff7fb"
                    font.family: "IBM Plex Sans"
                    font.bold: true
                }
            }

            Rectangle {
                visible: !root.immersiveMode
                radius: 16
                color: "#112434"
                border.color: "#23475f"
                implicitWidth: Math.min(720, Math.max(420, (root.diagnosticsRows.length * 152) + 32))
                implicitHeight: 92

                ColumnLayout {
                    anchors.fill: parent
                    anchors.leftMargin: 14
                    anchors.rightMargin: 14
                    anchors.topMargin: 8
                    anchors.bottomMargin: 8
                    spacing: 4

                    Label {
                        text: "Tuners"
                        color: "#eff7fb"
                        font.family: "IBM Plex Sans"
                        font.pixelSize: 12
                        font.bold: true
                        elide: Text.ElideRight
                    }

                    Label {
                        text: root.diagnosticsSummary
                        color: "#8ea7b9"
                        font.family: "IBM Plex Sans"
                        font.pixelSize: 11
                        elide: Text.ElideRight
                    }

                    Flickable {
                        Layout.fillWidth: true
                        Layout.preferredHeight: 46
                        contentWidth: diagnosticsRowStrip.implicitWidth
                        contentHeight: diagnosticsRowStrip.implicitHeight
                        clip: true
                        boundsBehavior: Flickable.StopAtBounds

                        ScrollBar.horizontal: ScrollBar {
                            policy: ScrollBar.AsNeeded
                        }

                        Row {
                            id: diagnosticsRowStrip
                            spacing: 8

                            Repeater {
                                model: root.diagnosticsRows

                                Rectangle {
                                    required property var modelData
                                    width: 144
                                    height: 42
                                    radius: 12
                                    color: "#163042"
                                    border.color: "#28516a"

                                    Column {
                                        anchors.fill: parent
                                        anchors.leftMargin: 10
                                        anchors.rightMargin: 10
                                        anchors.topMargin: 6
                                        anchors.bottomMargin: 6
                                        spacing: 1

                                        Label {
                                            width: parent.width
                                            text: parent.parent.modelData.title
                                            color: "#eff7fb"
                                            font.family: "IBM Plex Sans"
                                            font.pixelSize: 11
                                            font.bold: true
                                            elide: Text.ElideRight
                                        }

                                        Label {
                                            width: parent.width
                                            text: parent.parent.modelData.detail
                                            color: "#8ea7b9"
                                            font.family: "IBM Plex Sans"
                                            font.pixelSize: 10
                                            elide: Text.ElideRight
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        Rectangle {
            anchors.top: root.immersiveMode ? parent.top : stageHeader.bottom
            anchors.topMargin: root.immersiveMode ? 0 : 12
            anchors.left: parent.left
            anchors.right: parent.right
            anchors.bottom: parent.bottom
            radius: root.immersiveMode ? 0 : 28
            color: "#0d1f2d"
            border.width: root.immersiveMode ? 0 : 1
            border.color: root.shellPhase === "playback_failed" ? "#ff955c" : "#26495f"

            ColumnLayout {
                anchors.fill: parent
                anchors.margins: root.immersiveMode ? 0 : 8
                spacing: 0

                Rectangle {
                    Layout.fillWidth: true
                    Layout.fillHeight: true
                    radius: root.immersiveMode ? 0 : 22
                    color: "#050b11"
                    border.width: root.immersiveMode ? 0 : 1
                    border.color: "#1b3343"

                    MediaPlayer {
                        id: player
                        source: root.embeddedPlaybackEnabled && root.playbackUrl !== "" ? root.playbackUrl : ""
                        audioOutput: audioOutput
                        videoOutput: videoOutput
                        onErrorOccurred: function(error, errorString) {
                            root.surfaceErrorText = errorString
                        }
                        onPositionChanged: if (!progressSlider.pressed) root.pendingSeekPosition = position
                    }

                    AudioOutput {
                        id: audioOutput
                        volume: root.volumeLevel
                    }

                    VideoOutput {
                        id: videoOutput
                        anchors.fill: parent
                        fillMode: VideoOutput.PreserveAspectFit
                        visible: root.embeddedPlaybackEnabled && root.playbackUrl !== "" && root.surfaceErrorText === ""
                    }

                    MouseArea {
                        anchors.top: parent.top
                        anchors.left: parent.left
                        anchors.right: parent.right
                        anchors.bottom: overlayControlsBar.visible ? overlayControlsBar.top : parent.bottom
                        acceptedButtons: root.liveControlMode ? Qt.LeftButton : Qt.NoButton
                        hoverEnabled: true
                        enabled: true
                        onClicked: {
                            root.revealOverlay()
                            if (root.liveControlMode) {
                                root.toggleLivePlayback()
                            }
                        }
                        onPositionChanged: root.revealOverlay()
                        onEntered: root.revealOverlay()
                    }

                    Canvas {
                        anchors.fill: parent
                        visible: !videoOutput.visible
                        onPaint: {
                            const ctx = getContext("2d")
                            ctx.reset()
                            ctx.fillStyle = "#050b11"
                            ctx.fillRect(0, 0, width, height)
                            ctx.strokeStyle = "#2e617f"
                            ctx.lineWidth = 1.5
                            for (let i = -height; i < width; i += 28) {
                                ctx.beginPath()
                                ctx.moveTo(i, 0)
                                ctx.lineTo(i + height, height)
                                ctx.stroke()
                            }
                        }
                    }

                    Label {
                        anchors.centerIn: parent
                                                z: 3
                        visible: !videoOutput.visible
                        text: root.surfaceErrorText !== ""
                              ? root.surfaceErrorText
                              : root.embeddedPlaybackEnabled && root.playbackUrl !== ""
                                ? "Preparing embedded playback surface"
                                : root.shellPhase === "playing"
                                  ? "Backend is using external player mode"
                                  : "Select a playable channel to start the embedded surface"
                        color: "#9ab5c7"
                        font.family: "IBM Plex Sans"
                        font.pixelSize: 20
                        horizontalAlignment: Text.AlignHCenter
                        wrapMode: Text.WordWrap
                        width: parent.width * 0.7
                    }

                    Item {
                        anchors.fill: parent
                        visible: root.surfaceOverlayEnabled
                            && ((root.immersiveMode ? root.immersiveOverlayVisible : root.controlOverlayVisible)
                                || root.retryEnabled
                                || volumeSlider.pressed
                                || progressSlider.pressed)
                        z: 2

                        Rectangle {
                            id: overlayTitleCard
                            anchors.left: parent.left
                            anchors.top: parent.top
                            anchors.margins: 24
                            width: Math.min(parent.width * 0.42, 520)
                            implicitHeight: overlayTitleColumn.implicitHeight + 24
                            radius: 18
                            color: "#8c09141d"
                            border.color: "#324f63"
                            visible: root.currentTitle !== "" || root.currentSubtitle !== ""

                            Column {
                                id: overlayTitleColumn
                                anchors.fill: parent
                                anchors.margins: 12
                                spacing: 4

                                Label {
                                    width: parent.width
                                    text: root.currentTitle
                                    color: "#f6fbff"
                                    font.family: "IBM Plex Sans"
                                    font.pixelSize: 22
                                    font.bold: true
                                    elide: Text.ElideRight
                                }

                                Label {
                                    width: parent.width
                                    text: root.currentSubtitle
                                    color: "#b8c9d6"
                                    font.family: "IBM Plex Sans"
                                    font.pixelSize: 14
                                    elide: Text.ElideRight
                                    visible: text !== ""
                                }
                            }
                        }

                        Row {
                            anchors.top: parent.top
                            anchors.right: parent.right
                            anchors.margins: 24
                            spacing: 10

                            IconButton {
                                visible: root.retryEnabled
                                iconKind: "retry"
                                toolTipText: "Retry playback"
                                onClicked: root.retryRequested()
                            }

                            Rectangle {
                                radius: 16
                                color: root.shellPhase === "playing" ? "#163c29" : (root.shellPhase === "playback_failed" ? "#472016" : "#173247")
                                border.color: root.shellPhase === "playing" ? "#4ab97d" : (root.shellPhase === "playback_failed" ? "#ff955c" : "#5cb9ea")
                                implicitWidth: 138
                                implicitHeight: 40

                                Label {
                                    anchors.centerIn: parent
                                    text: root.shellPhase === "playing" ? "Playing" : (root.shellPhase === "playback_failed" ? "Needs Retry" : "Preparing")
                                    color: "#eff7fb"
                                    font.family: "IBM Plex Sans"
                                    font.bold: true
                                }
                            }

                        }

                        Rectangle {
                            id: overlayControlsBar
                            anchors.left: parent.left
                            anchors.right: parent.right
                            anchors.bottom: parent.bottom
                            anchors.margins: 24
                            implicitHeight: root.dvrControlMode ? 98 : 68
                            radius: 20
                            color: "#be09131d"
                            border.color: "#304d61"
                            visible: root.controlOverlayVisible || volumeSlider.pressed || progressSlider.pressed

                            Rectangle {
                                anchors.left: parent.left
                                anchors.right: parent.right
                                anchors.bottom: parent.bottom
                                height: 34
                                radius: 20
                                color: "#00000000"
                                gradient: Gradient {
                                    GradientStop { position: 0.0; color: "#00000000" }
                                    GradientStop { position: 1.0; color: "#35000000" }
                                }
                            }

                            ColumnLayout {
                                anchors.fill: parent
                                anchors.leftMargin: 14
                                anchors.rightMargin: 14
                                spacing: root.dvrControlMode ? 6 : 0

                                RowLayout {
                                    visible: root.dvrControlMode
                                    Layout.fillWidth: true
                                    spacing: 8

                                    Label {
                                        text: root.formatPlaybackTime(player.position)
                                        color: "#eff7fb"
                                        font.family: "IBM Plex Sans"
                                        font.pixelSize: 11
                                        font.bold: true
                                    }

                                    Item {
                                        id: progressSlider
                                        Layout.fillWidth: true
                                        implicitHeight: 20
                                        enabled: root.seekControlEnabled
                                        property bool pressed: progressSeekArea.pressed
                                        readonly property real visualPosition: root.seekControlEnabled && player.duration > 0
                                            ? Math.max(0, Math.min(1, root.pendingSeekPosition / player.duration))
                                            : 0

                                        Rectangle {
                                            anchors.left: parent.left
                                            anchors.right: parent.right
                                            anchors.verticalCenter: parent.verticalCenter
                                            height: 6
                                            radius: 3
                                            color: "#274356"

                                            Rectangle {
                                                width: progressSlider.visualPosition * parent.width
                                                height: parent.height
                                                radius: parent.radius
                                                color: "#5cb9ea"
                                            }
                                        }

                                        Rectangle {
                                            x: progressSlider.visualPosition * (progressSlider.width - width)
                                            y: progressSlider.height / 2 - height / 2
                                            width: 14
                                            height: 14
                                            radius: 7
                                            color: progressSlider.pressed ? "#ffffff" : "#d9f3ff"
                                            border.color: "#5cb9ea"
                                        }

                                        MouseArea {
                                            id: progressSeekArea
                                            anchors.fill: parent
                                            enabled: progressSlider.enabled
                                            hoverEnabled: true
                                            cursorShape: enabled ? Qt.PointingHandCursor : Qt.ArrowCursor

                                            function updateSeek(mouseX) {
                                                if (!enabled || player.duration <= 0) {
                                                    return
                                                }

                                                const ratio = Math.max(0, Math.min(1, mouseX / width))
                                                root.pendingSeekPosition = ratio * player.duration
                                            }

                                            onPressed: {
                                                updateSeek(mouse.x)
                                                root.revealOverlay()
                                            }
                                            onPositionChanged: if (pressed) updateSeek(mouse.x)
                                            onReleased: {
                                                if (progressSlider.enabled) {
                                                    player.position = root.pendingSeekPosition
                                                    root.revealOverlay()
                                                }
                                            }
                                        }
                                    }

                                    Rectangle {
                                        visible: progressSlider.pressed
                                        radius: 10
                                        color: "#102838"
                                        border.color: "#5cb9ea"
                                        implicitWidth: seekPreviewLabel.implicitWidth + 14
                                        implicitHeight: seekPreviewLabel.implicitHeight + 8

                                        Label {
                                            id: seekPreviewLabel
                                            anchors.centerIn: parent
                                            text: root.formatPlaybackTime(root.pendingSeekPosition)
                                            color: "#eff7fb"
                                            font.family: "IBM Plex Sans"
                                            font.pixelSize: 11
                                            font.bold: true
                                        }
                                    }

                                    Label {
                                        text: root.formatPlaybackTime(player.duration)
                                        color: "#b8c9d6"
                                        font.family: "IBM Plex Sans"
                                        font.pixelSize: 11
                                        font.bold: true
                                    }
                                }

                                RowLayout {
                                    Layout.fillWidth: true
                                    spacing: 10

                                    IconButton {
                                        visible: root.dvrControlMode
                                        compact: true
                                        iconKind: "previous"
                                        toolTipText: "Previous episode in this series"
                                        enabled: root.recordingNavigation.previousId !== ""
                                        onClicked: root.playAdjacentRecording(-1)
                                    }

                                    IconButton {
                                        visible: root.dvrControlMode
                                        compact: true
                                        iconKind: player.playbackState === MediaPlayer.PlayingState ? "pause" : "play"
                                        toolTipText: player.playbackState === MediaPlayer.PlayingState ? "Pause" : "Play"
                                        enabled: root.activeRecordingId !== "" || player.playbackState !== MediaPlayer.StoppedState
                                        onClicked: root.togglePlayPause()
                                    }

                                    IconButton {
                                        visible: root.dvrControlMode
                                        compact: true
                                        iconKind: "stop"
                                        toolTipText: "Stop playback"
                                        enabled: root.playbackMode === "recorded" || root.playbackUrl !== ""
                                        onClicked: root.stopDvrPlayback()
                                    }

                                    IconButton {
                                        visible: root.dvrControlMode
                                        compact: true
                                        iconKind: "restart"
                                        toolTipText: "Restart from the beginning"
                                        enabled: root.activeRecordingId !== ""
                                        onClicked: root.restartPlayback()
                                    }

                                    IconButton {
                                        visible: root.dvrControlMode
                                        compact: true
                                        iconKind: "next"
                                        toolTipText: "Next episode in this series"
                                        enabled: root.recordingNavigation.nextId !== ""
                                        onClicked: root.playAdjacentRecording(1)
                                    }

                                    Item {
                                        Layout.fillWidth: true
                                        visible: root.dvrControlMode
                                    }

                                    IconButton {
                                        visible: root.liveControlMode
                                        compact: true
                                        iconKind: player.playbackState === MediaPlayer.PlayingState ? "pause" : "play"
                                        toolTipText: player.playbackState === MediaPlayer.PlayingState ? "Pause live TV" : "Play current live channel"
                                        enabled: root.currentChannelRef !== "" || player.playbackState !== MediaPlayer.StoppedState
                                        onClicked: root.toggleLivePlayback()
                                    }

                                    IconButton {
                                        visible: root.liveControlMode
                                        compact: true
                                        iconKind: "stop"
                                        toolTipText: "Stop live playback"
                                        enabled: root.playbackMode === "live" || root.playbackUrl !== ""
                                        onClicked: root.stopLivePlayback()
                                    }

                                    Item {
                                        Layout.fillWidth: true
                                        visible: root.liveControlMode
                                    }

                                    IconButton {
                                        compact: true
                                        iconKind: "volume-down"
                                        toolTipText: "Volume down (Down key)"
                                        enabled: root.volumeControlEnabled
                                        onClicked: root.adjustVolume(-0.05)
                                    }

                                    Item {
                                        id: volumeSlider
                                        Layout.fillWidth: !(root.dvrControlMode || root.liveControlMode)
                                        Layout.preferredWidth: (root.dvrControlMode || root.liveControlMode) ? 140 : -1
                                        implicitHeight: 20
                                        enabled: root.volumeControlEnabled
                                        property bool pressed: volumeSeekArea.pressed
                                        readonly property real visualPosition: root.volumeLevel

                                        Rectangle {
                                            anchors.left: parent.left
                                            anchors.right: parent.right
                                            anchors.verticalCenter: parent.verticalCenter
                                            height: 6
                                            radius: 3
                                            color: "#274356"

                                            Rectangle {
                                                width: volumeSlider.visualPosition * parent.width
                                                height: parent.height
                                                radius: parent.radius
                                                color: "#ff4e45"
                                            }
                                        }

                                        Rectangle {
                                            x: volumeSlider.visualPosition * (volumeSlider.width - width)
                                            y: volumeSlider.height / 2 - height / 2
                                            width: 14
                                            height: 14
                                            radius: 7
                                            color: volumeSlider.pressed ? "#ffffff" : "#ffe6e4"
                                            border.color: "#ff4e45"
                                        }

                                        MouseArea {
                                            id: volumeSeekArea
                                            anchors.fill: parent
                                            enabled: volumeSlider.enabled
                                            hoverEnabled: true
                                            cursorShape: enabled ? Qt.PointingHandCursor : Qt.ArrowCursor

                                            function updateVolume(mouseX) {
                                                if (!enabled) {
                                                    return
                                                }

                                                root.volumeLevel = Math.max(0, Math.min(1, mouseX / width))
                                            }

                                            onPressed: {
                                                updateVolume(mouse.x)
                                                root.revealOverlay()
                                            }
                                            onPositionChanged: if (pressed) updateVolume(mouse.x)
                                        }
                                    }

                                    Label {
                                        text: root.volumePercent + "%"
                                        color: "#eff7fb"
                                        font.family: "IBM Plex Sans"
                                        font.pixelSize: 12
                                        font.bold: true
                                    }

                                    IconButton {
                                        compact: true
                                        iconKind: "volume-up"
                                        toolTipText: "Volume up (Up key)"
                                        enabled: root.volumeControlEnabled
                                        onClicked: root.adjustVolume(0.05)
                                    }

                                    IconButton {
                                        compact: true
                                        iconKind: root.fullscreenMode ? "fullscreen-exit" : "fullscreen"
                                        toolTipText: root.fullscreenMode ? "Return to windowed mode (F or Esc)" : "Enter fullscreen (F)"
                                        onClicked: root.toggleFullscreenRequested()
                                    }
                                }
                            }
                        }

                        Rectangle {
                            id: overlayMessageCard
                            anchors.left: parent.left
                            anchors.bottom: parent.bottom
                            anchors.margins: 24
                            width: Math.min(parent.width * 0.5, 560)
                            implicitHeight: overlayMessage.implicitHeight + 24
                            radius: 18
                            color: "#8c09141d"
                            border.color: root.failureText !== "" ? "#ff955c" : "#6c8aa0"
                            visible: root.failureText !== "" || root.warningText !== ""

                            Label {
                                id: overlayMessage
                                anchors.fill: parent
                                anchors.margins: 12
                                text: root.failureText !== "" ? root.failureText : root.warningText
                                color: root.failureText !== "" ? "#ffd3b8" : "#f2c27c"
                                wrapMode: Text.WordWrap
                                font.family: "IBM Plex Sans"
                            }
                        }

                        Rectangle {
                            id: overlayHintCard
                            anchors.right: parent.right
                            anchors.bottom: parent.bottom
                            anchors.margins: 24
                            implicitWidth: overlayHint.implicitWidth + 24
                            implicitHeight: overlayHint.implicitHeight + 16
                            radius: 16
                            color: "#8c09141d"
                            border.color: "#324f63"
                            visible: false

                            Label {
                                id: overlayHint
                                anchors.centerIn: parent
                                text: "Up/Down volume  Left/Right switch channels  F toggle fullscreen  Esc exit"
                                color: "#d7e5ef"
                                font.family: "IBM Plex Sans"
                                font.pixelSize: 13
                            }
                        }
                    }
                }
            }
        }

        RowLayout {
            visible: !root.immersiveMode
            Layout.fillWidth: true
            spacing: 12

            Label {
                Layout.fillWidth: true
                visible: root.warningText !== ""
                text: root.warningText
                color: "#f2c27c"
                wrapMode: Text.WordWrap
                font.family: "IBM Plex Sans"
            }

            IconButton {
                visible: root.retryEnabled
                iconKind: "retry"
                toolTipText: "Retry playback"
                onClicked: root.retryRequested()
            }
        }
    }

    Component.onCompleted: {
        syncPlayback()
        revealOverlay()
    }
}