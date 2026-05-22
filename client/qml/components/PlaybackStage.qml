import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import QtMultimedia

Pane {
    id: root

    required property string shellPhase
    required property string currentTitle
    required property string currentSubtitle
    required property string warningText
    required property string failureText
    required property string playbackUrl
    required property bool embeddedPlaybackEnabled
    required property bool retryEnabled
    signal retryRequested()

    property string surfaceErrorText: ""

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

    onShellPhaseChanged: syncPlayback()
    onPlaybackUrlChanged: {
        surfaceErrorText = ""
        syncPlayback()
    }
    onEmbeddedPlaybackEnabledChanged: syncPlayback()

    padding: 22
    background: Rectangle {
        radius: 34
        color: "#09141d"
        border.color: "#183345"
    }

    ColumnLayout {
        anchors.fill: parent
        spacing: 18

        RowLayout {
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
            radius: 28
            color: "#0d1f2d"
            border.color: root.shellPhase === "playback_failed" ? "#ff955c" : "#26495f"

            ColumnLayout {
                anchors.fill: parent
                anchors.margins: 26
                spacing: 14

                Label {
                    text: root.shellPhase === "playing" ? "Playback Stage" : (root.shellPhase === "playback_failed" ? "Playback Recovery" : "Playback Loading")
                    color: "#e5f0f7"
                    font.family: "IBM Plex Sans"
                    font.pixelSize: 22
                    font.bold: true
                }

                Label {
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
                    radius: 22
                    color: "#050b11"
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
                        volume: 1.0
                    }

                    VideoOutput {
                        id: videoOutput
                        anchors.fill: parent
                        fillMode: VideoOutput.PreserveAspectFit
                        visible: root.embeddedPlaybackEnabled && root.playbackUrl !== "" && root.surfaceErrorText === ""
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
                }
            }
        }

        RowLayout {
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