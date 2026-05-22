import QtQuick
import QtQuick.Controls
import QtQuick.Layouts

Pane {
    id: root

    required property var channels
    required property string currentChannelRef
    signal channelActivated(string channelRef, string guideNumber, string guideName, string availability)

    padding: 16
    background: Rectangle {
        radius: 28
        color: "#0d1a24"
        border.color: "#183447"
    }

    ColumnLayout {
        anchors.fill: parent
        spacing: 14

        Label {
            text: "Channels"
            color: "#f2f7fb"
            font.family: "IBM Plex Sans"
            font.pixelSize: 20
            font.bold: true
        }

        Label {
            text: "Persistent rail for browsing and quick switching"
            color: "#8ea7b9"
            wrapMode: Text.WordWrap
            font.family: "IBM Plex Sans"
        }

        ListView {
            Layout.fillWidth: true
            Layout.fillHeight: true
            clip: true
            spacing: 10
            model: root.channels

            delegate: Button {
                width: ListView.view.width
                height: 72
                padding: 0
                background: Rectangle {
                    radius: 20
                    color: modelData.channelRef === root.currentChannelRef ? "#1d455f" : "#122635"
                    border.color: modelData.channelRef === root.currentChannelRef ? "#69c6ff" : "#23445b"
                }
                contentItem: RowLayout {
                    anchors.fill: parent
                    anchors.leftMargin: 14
                    anchors.rightMargin: 14
                    spacing: 10

                    Label {
                        text: modelData.guideNumber
                        color: "#dff2ff"
                        font.family: "IBM Plex Sans"
                        font.pixelSize: 20
                        font.bold: true
                    }

                    ColumnLayout {
                        Layout.fillWidth: true
                        spacing: 2

                        Label {
                            text: modelData.guideName
                            color: "#f2f7fb"
                            font.family: "IBM Plex Sans"
                            font.pixelSize: 16
                            elide: Text.ElideRight
                        }

                        Label {
                            text: modelData.availability === "playable" ? "Ready to play" : "Restricted"
                            color: modelData.availability === "playable" ? "#8dd6a5" : "#ffb27f"
                            font.family: "IBM Plex Sans"
                        }
                    }
                }
                onClicked: root.channelActivated(modelData.channelRef, modelData.guideNumber, modelData.guideName, modelData.availability)
            }
        }
    }
}