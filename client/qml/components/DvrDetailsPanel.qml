import QtQuick
import QtQuick.Controls
import QtQuick.Layouts

Pane {
    id: root

    required property var recording
    required property string playbackMode
    required property string currentRecordingId
    signal playRequested(string recordingId)
    signal stopPlaybackRequested()
    signal deleteRequested(string recordingId)
    signal ruleEditorRequested(string recordingId)

    readonly property bool hasRecording: recording && recording.recordingId !== undefined && recording.recordingId !== ""
    readonly property bool currentPlayback: hasRecording && playbackMode === "recorded" && currentRecordingId === recording.recordingId

    padding: 14
    background: Rectangle {
        radius: 24
        color: "#0d1a24"
        border.color: "#183447"
    }

    ColumnLayout {
        anchors.fill: parent
        spacing: 10

        Label {
            text: root.currentPlayback ? "Now Playing" : "Recording Details"
            color: "#f2f7fb"
            font.family: "IBM Plex Sans"
            font.pixelSize: 20
            font.bold: true
        }

        Label {
            visible: !root.hasRecording
            Layout.fillWidth: true
            wrapMode: Text.Wrap
            text: "Select a recorded episode to inspect playback and DVR actions."
            color: "#8ea7b9"
            font.family: "IBM Plex Sans"
        }

        ColumnLayout {
            visible: root.hasRecording
            Layout.fillWidth: true
            spacing: 8

            Label {
                Layout.fillWidth: true
                text: root.recording.title
                color: "#eff7fb"
                font.family: "IBM Plex Sans"
                font.pixelSize: 18
                font.bold: true
                elide: Text.ElideRight
            }

            Label {
                Layout.fillWidth: true
                visible: !!root.recording.episodeTitle
                text: root.recording.episodeTitle
                color: "#bcd0dc"
                font.family: "IBM Plex Sans"
                font.pixelSize: 14
                elide: Text.ElideRight
            }

            Label {
                Layout.fillWidth: true
                text: Qt.formatDateTime(new Date(root.recording.recordStartTime * 1000), "MMM d h:mm AP")
                    + (root.recording.channelName ? " • " + root.recording.channelName : "")
                color: "#8ea7b9"
                font.family: "IBM Plex Sans"
                font.pixelSize: 12
                wrapMode: Text.Wrap
            }

            Label {
                Layout.fillWidth: true
                visible: !!root.recording.synopsis
                text: root.recording.synopsis
                wrapMode: Text.Wrap
                color: "#dce6ed"
                font.family: "IBM Plex Sans"
                font.pixelSize: 13
            }

            Label {
                Layout.fillWidth: true
                visible: !!root.recording.ruleEditorMessage
                text: root.recording.ruleEditorMessage
                wrapMode: Text.Wrap
                color: "#8ea7b9"
                font.family: "IBM Plex Sans"
                font.pixelSize: 12
            }

            RowLayout {
                Layout.fillWidth: true
                spacing: 10

                ThemeButton {
                    text: root.currentPlayback ? "Restart" : "Play"
                    onClicked: root.playRequested(root.recording.recordingId)
                }

                ThemeButton {
                    visible: root.playbackMode !== "idle"
                    text: "Stop"
                    onClicked: root.stopPlaybackRequested()
                }

                ThemeButton {
                    text: "Delete"
                    onClicked: root.deleteRequested(root.recording.recordingId)
                }

                ThemeButton {
                    text: "Rule Options"
                    onClicked: root.ruleEditorRequested(root.recording.recordingId)
                }
            }
        }
    }
}