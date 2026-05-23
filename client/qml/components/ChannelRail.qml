import QtQuick
import QtQuick.Controls
import QtQuick.Layouts

Pane {
    id: root

    property bool compactMode: false
    required property var channels
    required property string currentChannelRef
    signal channelActivated(string channelRef, string guideNumber, string guideName, string availability)

    function currentChannelIndex() {
        for (let index = 0; index < channels.length; index += 1) {
            const channel = channels[index]
            if (channel.channelRef === currentChannelRef) {
                return index
            }
        }

        return -1
    }

    function ensureCurrentVisible() {
        const index = currentChannelIndex()
        if (index < 0) {
            return
        }

        channelList.positionViewAtIndex(index, ListView.Contain)
    }

    onCurrentChannelRefChanged: ensureCurrentVisible()
    onChannelsChanged: ensureCurrentVisible()

    padding: root.compactMode ? 12 : 16
    background: Rectangle {
        radius: root.compactMode ? 22 : 28
        color: "#0d1a24"
        border.color: "#183447"
    }

    ColumnLayout {
        anchors.fill: parent
        spacing: root.compactMode ? 10 : 14

        RowLayout {
            Layout.fillWidth: true

            Label {
                text: "Channels"
                color: "#f2f7fb"
                font.family: "IBM Plex Sans"
                font.pixelSize: root.compactMode ? 17 : 20
                font.bold: true
            }

            Label {
                visible: root.compactMode
                Layout.fillWidth: true
                horizontalAlignment: Text.AlignRight
                text: "Scroll or use Left/Right"
                color: "#8ea7b9"
                font.family: "IBM Plex Sans"
                font.pixelSize: 13
            }
        }

        Label {
            visible: !root.compactMode
            text: "Persistent rail for browsing and quick switching"
            color: "#8ea7b9"
            font.family: "IBM Plex Sans"
        }

        ListView {
            id: channelList
            Layout.fillWidth: true
            Layout.fillHeight: true
            clip: true
            spacing: 10
            orientation: root.compactMode ? ListView.Horizontal : ListView.Vertical
            boundsBehavior: Flickable.StopAtBounds
            model: root.channels

            ScrollBar.horizontal: ScrollBar {
                visible: root.compactMode
            }

            ScrollBar.vertical: ScrollBar {
                visible: !root.compactMode
            }

            delegate: Button {
                width: root.compactMode ? 220 : ListView.view.width
                height: root.compactMode ? Math.max(84, ListView.view.height - 12) : 72
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
                            Layout.fillWidth: true
                            text: modelData.availability === "playable"
                                  ? (modelData.currentProgramTitle ? modelData.currentProgramTitle : "Guide unavailable")
                                  : "Restricted"
                            color: modelData.availability === "playable"
                                   ? (modelData.currentProgramTitle ? "#8dd6a5" : "#8ea7b9")
                                   : "#ffb27f"
                            font.family: "IBM Plex Sans"
                            elide: Text.ElideRight
                        }
                    }
                }
                onClicked: root.channelActivated(modelData.channelRef, modelData.guideNumber, modelData.guideName, modelData.availability)
            }
        }
    }

    Component.onCompleted: ensureCurrentVisible()
}