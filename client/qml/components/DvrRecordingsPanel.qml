import QtQuick
import QtQuick.Controls
import QtQuick.Layouts

Pane {
    id: root

    component ActionButton: Button {
        id: control

        implicitHeight: 34
        leftPadding: 14
        rightPadding: 14
        topPadding: 0
        bottomPadding: 0
        font.family: "IBM Plex Sans"
        font.pixelSize: 12
        font.bold: true

        contentItem: Text {
            text: control.text
            font: control.font
            color: control.enabled ? "#eff7fb" : "#627786"
            horizontalAlignment: Text.AlignHCenter
            verticalAlignment: Text.AlignVCenter
            elide: Text.ElideRight
        }

        background: Rectangle {
            radius: 12
            color: !control.enabled ? "#10202d" : (control.down ? "#1a4258" : (control.hovered ? "#173247" : "#112838"))
            border.color: !control.enabled ? "#274053" : (control.down ? "#7fd4ff" : "#2d5a74")
        }
    }

    required property var recordingGroups
    required property string selectedRecordingId
    required property bool loading
    required property string recordingsState
    signal refreshRequested()
    signal groupToggled(string groupId)
    signal recordingSelected(string recordingId)
    signal playRequested(string recordingId)
    signal deleteRequested(string recordingId)
    signal ruleEditorRequested(string recordingId)

    padding: 14
    background: Rectangle {
        radius: 26
        color: "#0d1a24"
        border.color: "#183447"
    }

    ColumnLayout {
        anchors.fill: parent
        spacing: 12

        RowLayout {
            Layout.fillWidth: true

            ColumnLayout {
                spacing: 2

                Label {
                    text: "Recordings"
                    color: "#f2f7fb"
                    font.family: "IBM Plex Sans"
                    font.pixelSize: 20
                    font.bold: true
                }

                Label {
                    text: root.recordingGroups.length > 0
                        ? "Click a series title to expand its recordings"
                        : (root.recordingsState === "ready" ? "No recordings found" : "Waiting for DVR content")
                    color: "#8ea7b9"
                    font.family: "IBM Plex Sans"
                    font.pixelSize: 12
                }
            }

            Item { Layout.fillWidth: true }

            ActionButton {
                text: "Refresh"
                onClicked: root.refreshRequested()
            }
        }

        BusyIndicator {
            Layout.alignment: Qt.AlignHCenter
            running: root.loading
            visible: root.loading
        }

        Label {
            visible: !root.loading && root.recordingGroups.length === 0
            Layout.fillWidth: true
            wrapMode: Text.Wrap
            text: root.recordingsState === "selection_required"
                ? "Select a device to load recordings."
                : "No recordings are currently available in the DVR workspace."
            color: "#8ea7b9"
            font.family: "IBM Plex Sans"
        }

        ScrollView {
            Layout.fillWidth: true
            Layout.fillHeight: true
            visible: !root.loading && root.recordingGroups.length > 0
            clip: true

            Column {
                width: parent.width
                spacing: 10

                Repeater {
                    model: root.recordingGroups

                    Pane {
                        required property var modelData
                        property var groupData: modelData

                        width: parent.width
                        padding: 10
                        background: Rectangle {
                            radius: 18
                            color: "#0a131b"
                            border.color: "#193447"
                        }

                        ColumnLayout {
                            anchors.fill: parent
                            spacing: 8

                            Rectangle {
                                Layout.fillWidth: true
                                implicitHeight: 54
                                radius: 14
                                color: groupData.expanded ? "#16384c" : "#143a50"
                                border.color: groupData.expanded ? "#69c6ff" : "#3d6f8d"

                                MouseArea {
                                    anchors.fill: parent
                                    onClicked: root.groupToggled(groupData.groupId)
                                }

                                RowLayout {
                                    anchors.fill: parent
                                    anchors.leftMargin: 14
                                    anchors.rightMargin: 14

                                    Text {
                                        text: groupData.expanded ? "-" : "+"
                                        color: "#dcecf7"
                                        font.family: "IBM Plex Sans"
                                        font.pixelSize: 22
                                        font.bold: true
                                        verticalAlignment: Text.AlignVCenter
                                    }

                                    ColumnLayout {
                                        Layout.fillWidth: true
                                        spacing: 2

                                        Text {
                                            Layout.fillWidth: true
                                            text: groupData.seriesTitle
                                            color: "#eff7fb"
                                            font.family: "IBM Plex Sans"
                                            font.pixelSize: 15
                                            font.bold: true
                                            elide: Text.ElideRight
                                        }

                                        Text {
                                            Layout.fillWidth: true
                                            text: groupData.episodeCount + " episode" + (groupData.episodeCount === 1 ? "" : "s")
                                            color: "#8ea7b9"
                                            font.family: "IBM Plex Sans"
                                            font.pixelSize: 11
                                            elide: Text.ElideRight
                                        }
                                    }
                                }
                            }

                            Column {
                                visible: groupData.expanded
                                spacing: 6
                                width: parent.width

                                Repeater {
                                    model: groupData.episodes

                                    Rectangle {
                                        required property var modelData
                                        property var recordingData: modelData

                                        width: parent.width
                                        height: 76
                                        radius: 16
                                        color: recordingData.recordingId === root.selectedRecordingId ? "#16384c" : "#101e29"
                                        border.color: recordingData.recordingId === root.selectedRecordingId ? "#69c6ff" : "#1d4259"

                                        MouseArea {
                                            anchors.fill: parent
                                            onClicked: root.recordingSelected(recordingData.recordingId)
                                        }

                                        RowLayout {
                                            anchors.fill: parent
                                            anchors.leftMargin: 12
                                            anchors.rightMargin: 12
                                            spacing: 8

                                            ColumnLayout {
                                                Layout.fillWidth: true
                                                spacing: 2

                                                Text {
                                                    Layout.fillWidth: true
                                                    text: recordingData.episodeTitle ? recordingData.episodeTitle : recordingData.title
                                                    color: "#eff7fb"
                                                    font.family: "IBM Plex Sans"
                                                    font.pixelSize: 13
                                                    font.bold: true
                                                    elide: Text.ElideRight
                                                }

                                                Text {
                                                    Layout.fillWidth: true
                                                    text: (recordingData.channelNumber ? recordingData.channelNumber + " • " : "")
                                                        + (recordingData.channelName ? recordingData.channelName : "")
                                                    color: "#8ea7b9"
                                                    font.family: "IBM Plex Sans"
                                                    font.pixelSize: 11
                                                    elide: Text.ElideRight
                                                }

                                                Text {
                                                    Layout.fillWidth: true
                                                    text: Qt.formatDateTime(new Date(recordingData.recordStartTime * 1000), "MMM d h:mm AP")
                                                    color: "#8ea7b9"
                                                    font.family: "IBM Plex Sans"
                                                    font.pixelSize: 11
                                                }
                                            }

                                            Text {
                                                text: recordingData.recordingId === root.selectedRecordingId ? "Selected" : "Open"
                                                color: recordingData.recordingId === root.selectedRecordingId ? "#7fd4ff" : "#8ea7b9"
                                                font.family: "IBM Plex Sans"
                                                font.pixelSize: 11
                                                font.bold: true
                                                verticalAlignment: Text.AlignVCenter
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}