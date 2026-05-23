import QtQuick
import QtQuick.Controls
import QtQuick.Layouts

Pane {
    id: root

    required property var guideChannels
    required property string currentChannelRef
    required property double windowStart
    required property int durationHours
    required property bool loading

    signal channelActivated(string channelRef)
    signal jumpToNowRequested()

    readonly property int labelWidth: 188
    readonly property int rowHeight: 72
    readonly property int headerHeight: 42
    readonly property int slotWidth: 172
    readonly property int slotCount: Math.max(1, durationHours * 2)
    readonly property int scheduleWidth: slotCount * slotWidth
    readonly property double windowEnd: windowStart + (Math.max(1, durationHours) * 3600)

    function formatStamp(unixSeconds, pattern) {
        return Qt.formatDateTime(new Date(unixSeconds * 1000), pattern)
    }

    function entryX(startTime) {
        const clampedStart = Math.max(startTime, windowStart)
        return labelWidth + ((clampedStart - windowStart) / (windowEnd - windowStart)) * scheduleWidth
    }

    function entryWidth(startTime, endTime) {
        const clampedStart = Math.max(startTime, windowStart)
        const clampedEnd = Math.min(endTime, windowEnd)
        const proportionalWidth = ((clampedEnd - clampedStart) / (windowEnd - windowStart)) * scheduleWidth
        return Math.max(78, proportionalWidth)
    }

    function nowLineX() {
        const now = Date.now() / 1000
        if (now < windowStart || now > windowEnd) {
            return -1
        }

        return labelWidth + ((now - windowStart) / (windowEnd - windowStart)) * scheduleWidth
    }

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
            spacing: 12

            ColumnLayout {
                spacing: 2

                Label {
                    text: "Guide"
                    color: "#f2f7fb"
                    font.family: "IBM Plex Sans"
                    font.pixelSize: 18
                    font.bold: true
                }

                Label {
                    text: "30-minute slots • scroll horizontally for more guide data"
                    color: "#8ea7b9"
                    font.family: "IBM Plex Sans"
                    font.pixelSize: 13
                }
            }

            Item {
                Layout.fillWidth: true
            }

            Button {
                text: "Now"
                onClicked: root.jumpToNowRequested()
            }
        }

        Rectangle {
            Layout.fillWidth: true
            Layout.fillHeight: true
            radius: 20
            color: "#08131c"
            border.color: "#173245"

            BusyIndicator {
                anchors.centerIn: parent
                running: root.loading
                visible: root.loading
            }

            Label {
                anchors.centerIn: parent
                visible: !root.loading && root.guideChannels.length === 0
                text: "Guide data is unavailable for this window."
                color: "#8ea7b9"
                font.family: "IBM Plex Sans"
            }

            Flickable {
                anchors.fill: parent
                anchors.margins: 10
                visible: !root.loading && root.guideChannels.length > 0
                clip: true
                contentWidth: root.labelWidth + root.scheduleWidth
                contentHeight: root.headerHeight + (root.guideChannels.length * root.rowHeight)
                boundsBehavior: Flickable.StopAtBounds

                ScrollBar.horizontal: ScrollBar {}
                ScrollBar.vertical: ScrollBar {}

                Item {
                    width: parent.contentWidth
                    height: parent.contentHeight

                    Repeater {
                        model: root.slotCount + 1

                        Rectangle {
                            x: root.labelWidth + (index * root.slotWidth)
                            y: 0
                            width: 1
                            height: parent.height
                            color: index === root.slotCount ? "#204358" : "#173245"
                        }
                    }

                    Rectangle {
                        x: 0
                        y: 0
                        width: root.labelWidth
                        height: root.headerHeight
                        color: "#112838"
                        border.color: "#20445c"

                        Label {
                            anchors.verticalCenter: parent.verticalCenter
                            anchors.left: parent.left
                            anchors.leftMargin: 14
                            text: "Channels"
                            color: "#eff7fb"
                            font.family: "IBM Plex Sans"
                            font.bold: true
                        }
                    }

                    Repeater {
                        model: root.slotCount

                        Rectangle {
                            x: root.labelWidth + (index * root.slotWidth)
                            y: 0
                            width: root.slotWidth
                            height: root.headerHeight
                            color: index % 2 === 0 ? "#10232f" : "#0d1d28"
                            border.color: "#173245"

                            Label {
                                anchors.centerIn: parent
                                text: root.formatStamp(root.windowStart + (index * 1800), "h:mm AP")
                                color: "#dcecf7"
                                font.family: "IBM Plex Sans"
                                font.bold: index % 2 === 0
                            }
                        }
                    }

                    Rectangle {
                        visible: root.nowLineX() >= 0
                        x: root.nowLineX()
                        y: 0
                        width: 2
                        height: parent.height
                        color: "#ff9d5c"
                        opacity: 0.9
                    }

                    Repeater {
                        model: root.guideChannels

                        Item {
                            id: rowItem

                            required property int index
                            required property var modelData
                            property var channelData: modelData

                            x: 0
                            y: root.headerHeight + (index * root.rowHeight)
                            width: root.labelWidth + root.scheduleWidth
                            height: root.rowHeight

                            Rectangle {
                                anchors.fill: parent
                                color: rowItem.channelData.channelRef === root.currentChannelRef
                                       ? "#12384d"
                                       : (rowItem.index % 2 === 0 ? "#0b1721" : "#09141d")
                                border.color: "#173245"
                            }

                            Rectangle {
                                x: 0
                                y: 0
                                width: root.labelWidth
                                height: root.rowHeight
                                color: rowItem.channelData.channelRef === root.currentChannelRef ? "#1c455f" : "#10232f"
                                border.color: rowItem.channelData.channelRef === root.currentChannelRef ? "#69c6ff" : "#1d4259"

                                MouseArea {
                                    anchors.fill: parent
                                    onClicked: root.channelActivated(rowItem.channelData.channelRef)
                                }

                                Column {
                                    anchors.fill: parent
                                    anchors.leftMargin: 12
                                    anchors.rightMargin: 12
                                    anchors.topMargin: 10
                                    spacing: 3

                                    Label {
                                        text: rowItem.channelData.guideNumber + "  " + rowItem.channelData.guideName
                                        color: "#eff7fb"
                                        font.family: "IBM Plex Sans"
                                        font.pixelSize: 15
                                        font.bold: true
                                        elide: Text.ElideRight
                                        width: parent.width
                                    }

                                    Label {
                                        text: rowItem.channelData.currentProgramTitle ? rowItem.channelData.currentProgramTitle : "Guide unavailable"
                                        color: rowItem.channelData.currentProgramTitle ? "#8dd6a5" : "#8ea7b9"
                                        font.family: "IBM Plex Sans"
                                        font.pixelSize: 12
                                        elide: Text.ElideRight
                                        width: parent.width
                                    }
                                }
                            }

                            Repeater {
                                model: rowItem.channelData.entries

                                Rectangle {
                                    id: entryRect

                                    required property var modelData
                                    property var entryData: modelData

                                    x: root.entryX(entryData.startTime)
                                    y: 8
                                    width: root.entryWidth(entryData.startTime, entryData.endTime)
                                    height: root.rowHeight - 16
                                    radius: 14
                                    color: entryData.isCurrent ? "#2a6b52" : "#1a3243"
                                    border.color: entryData.isCurrent ? "#8dd6a5" : "#35617b"

                                    MouseArea {
                                        anchors.fill: parent
                                        onClicked: root.channelActivated(rowItem.channelData.channelRef)
                                    }

                                    Column {
                                        anchors.fill: parent
                                        anchors.margins: 10
                                        spacing: 2

                                        Label {
                                            text: entryRect.entryData.title
                                            color: "#eff7fb"
                                            font.family: "IBM Plex Sans"
                                            font.pixelSize: 13
                                            font.bold: true
                                            elide: Text.ElideRight
                                            width: parent.width
                                        }

                                        Label {
                                            text: root.formatStamp(entryRect.entryData.startTime, "h:mm AP")
                                                  + " - "
                                                  + root.formatStamp(entryRect.entryData.endTime, "h:mm AP")
                                            color: "#c5d8e5"
                                            font.family: "IBM Plex Sans"
                                            font.pixelSize: 11
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