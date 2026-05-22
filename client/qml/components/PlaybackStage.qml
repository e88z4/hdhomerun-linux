import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import QtMultimedia

Pane {
    id: root

    required property bool immersiveMode
    required property int overlayPulse
    required property string shellPhase
    required property string currentTitle
    required property string currentSubtitle
    required property string warningText
    required property string failureText
    required property string playbackUrl
    required property bool embeddedPlaybackEnabled
    required property bool retryEnabled
    signal exitFullscreenRequested()
    signal retryRequested()

    property string surfaceErrorText: ""
    property bool immersiveOverlayVisible: false
    property real volumeLevel: 1.0
    readonly property int volumePercent: Math.round(volumeLevel * 100)
    readonly property bool volumeControlEnabled: embeddedPlaybackEnabled

    function revealOverlay() {
        if (!immersiveMode) {
            immersiveOverlayVisible = false
            overlayHideTimer.stop()
            return
        }

        immersiveOverlayVisible = true
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
        if (immersiveMode) {
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
    onImmersiveModeChanged: revealOverlay()
    onOverlayPulseChanged: revealOverlay()
    onCurrentTitleChanged: revealOverlay()
    onCurrentSubtitleChanged: revealOverlay()
    onWarningTextChanged: revealOverlay()
    onFailureTextChanged: revealOverlay()

    Timer {
        id: overlayHideTimer
        interval: 5000
        repeat: false
        onTriggered: root.immersiveOverlayVisible = false
    }

    padding: immersiveMode ? 0 : 22
    background: Rectangle {
        radius: root.immersiveMode ? 0 : 34
        color: root.immersiveMode ? "transparent" : "#09141d"
        border.width: root.immersiveMode ? 0 : 1
        border.color: "#183345"
    }

    ColumnLayout {
        anchors.fill: parent
        spacing: root.immersiveMode ? 0 : 18

        RowLayout {
            visible: !root.immersiveMode
            Layout.fillWidth: true

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
        }

        Rectangle {
            Layout.fillWidth: true
            Layout.fillHeight: true
            radius: root.immersiveMode ? 0 : 28
            color: "#0d1f2d"
            border.width: root.immersiveMode ? 0 : 1
            border.color: root.shellPhase === "playback_failed" ? "#ff955c" : "#26495f"

            ColumnLayout {
                anchors.fill: parent
                anchors.margins: root.immersiveMode ? 0 : 26
                spacing: root.immersiveMode ? 0 : 14

                Label {
                    visible: !root.immersiveMode
                    text: root.shellPhase === "playing" ? "Playback Stage" : (root.shellPhase === "playback_failed" ? "Playback Recovery" : "Playback Loading")
                    color: "#e5f0f7"
                    font.family: "IBM Plex Sans"
                    font.pixelSize: 22
                    font.bold: true
                }

                Label {
                    visible: !root.immersiveMode
                    Layout.fillWidth: true
                    wrapMode: Text.WordWrap
                    text: root.shellPhase === "playing"
                          ? "The client shell is ready for backend-driven playback state binding. The final embedded surface adapter will plug into this stage without changing the surrounding UX."
                          : root.failureText !== ""
                            ? root.failureText
                            : "Loading state is already modeled in the shell so backend bootstrap and playback transitions can stay visually stable."
                    color: root.shellPhase === "playback_failed" ? "#ffd3b8" : "#9cb5c5"
                    font.family: "IBM Plex Sans"
                }

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
                        anchors.fill: parent
                        acceptedButtons: Qt.NoButton
                        hoverEnabled: true
                        enabled: root.immersiveMode
                        onPositionChanged: root.revealOverlay()
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
                        visible: root.immersiveMode && (root.immersiveOverlayVisible || root.retryEnabled)
                        z: 2

                        Rectangle {
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

                            Button {
                                visible: root.retryEnabled
                                text: "Retry"
                                onClicked: root.retryRequested()
                            }

                            Rectangle {
                                radius: 16
                                color: "#8c09141d"
                                border.color: "#324f63"
                                implicitWidth: 96
                                implicitHeight: 40

                                Label {
                                    anchors.centerIn: parent
                                    text: "Vol " + root.volumePercent + "%"
                                    color: "#eff7fb"
                                    font.family: "IBM Plex Sans"
                                    font.bold: true
                                }
                            }

                            Button {
                                enabled: root.volumeControlEnabled
                                text: "Vol -"
                                onClicked: root.adjustVolume(-0.05)
                            }

                            Button {
                                enabled: root.volumeControlEnabled
                                text: "Vol +"
                                onClicked: root.adjustVolume(0.05)
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

                            Button {
                                text: "Exit Fullscreen"
                                onClicked: root.exitFullscreenRequested()
                            }
                        }

                        Rectangle {
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
                            anchors.right: parent.right
                            anchors.bottom: parent.bottom
                            anchors.margins: 24
                            implicitWidth: overlayHint.implicitWidth + 24
                            implicitHeight: overlayHint.implicitHeight + 16
                            radius: 16
                            color: "#8c09141d"
                            border.color: "#324f63"

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

            Button {
                visible: root.retryEnabled
                text: "Retry"
                onClicked: root.retryRequested()
            }
        }
    }

    Component.onCompleted: syncPlayback()
}