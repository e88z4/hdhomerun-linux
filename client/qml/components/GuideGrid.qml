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
    signal recordSeriesRequested(var guideContext)
    signal recordOnceRequested(var guideContext)

    readonly property int labelWidth: 188
    readonly property int rowHeight: 72
    readonly property int headerHeight: 42
    readonly property int slotWidth: 172
    readonly property int slotCount: Math.max(1, durationHours * 2)
    readonly property int scheduleWidth: slotCount * slotWidth
    readonly property double windowEnd: windowStart + (Math.max(1, durationHours) * 3600)
    property var selectedGuideContext: ({})
    property string selectedGuideChannelRef: ""
    property string selectedGuideTitle: ""
    property string selectedGuideSubtitle: ""

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

    function guideRuleContext(channelData, entryData) {
        return {
            title: entryData.title,
            episodeTitle: entryData.episodeTitle,
            seriesId: entryData.seriesId,
            channelNumber: channelData.guideNumber,
            channelName: channelData.guideName,
            startTime: entryData.startTime
        }
    }

    function openGuideActions(channelData, entryData) {
        selectedGuideContext = guideRuleContext(channelData, entryData)
        selectedGuideChannelRef = channelData.channelRef
        selectedGuideTitle = entryData.title
        selectedGuideSubtitle = channelData.guideNumber + " " + channelData.guideName + " • "
            + formatStamp(entryData.startTime, "MMM d h:mm AP")
        guideActionDialog.open()
    }

    function currentChannelIndex() {
        for (let index = 0; index < guideChannels.length; index += 1) {
            if (guideChannels[index].channelRef === currentChannelRef) {
                return index
            }
        }

        return -1
    }

    function ensureCurrentChannelVisible() {
        const index = currentChannelIndex()
        if (index < 0) {
            return
        }

        const rowTop = headerHeight + (index * rowHeight)
        const rowBottom = rowTop + rowHeight
        const viewportTop = guideFlickable.contentY
        const viewportBottom = viewportTop + guideFlickable.height

        if (rowTop >= viewportTop && rowBottom <= viewportBottom) {
            return
        }

        const centeredTop = rowTop - Math.max(0, (guideFlickable.height - rowHeight) / 2)
        const maxTop = Math.max(0, guideFlickable.contentHeight - guideFlickable.height)
        guideFlickable.contentY = Math.max(0, Math.min(maxTop, centeredTop))
    }

    onCurrentChannelRefChanged: ensureCurrentChannelVisible()
    onGuideChannelsChanged: ensureCurrentChannelVisible()

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

            ThemeButton {
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
                id: guideFlickable
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

                                    RowLayout {
                                        width: parent.width
                                        spacing: 8

                                        Rectangle {
                                            implicitWidth: 28
                                            implicitHeight: 28
                                            radius: 8
                                            color: "#10232f"
                                            border.color: "#23445b"
                                            visible: !!rowItem.channelData.imageUrl

                                            Image {
                                                anchors.fill: parent
                                                anchors.margins: 3
                                                source: rowItem.channelData.imageUrl ? rowItem.channelData.imageUrl : ""
                                                asynchronous: true
                                                fillMode: Image.PreserveAspectFit
                                                cache: true
                                            }
                                        }

                                        Label {
                                            Layout.fillWidth: true
                                            text: rowItem.channelData.guideNumber + "  " + rowItem.channelData.guideName
                                            color: "#eff7fb"
                                            font.family: "IBM Plex Sans"
                                            font.pixelSize: 15
                                            font.bold: true
                                            elide: Text.ElideRight
                                        }
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
                                    readonly property bool showArtwork: !!entryData.imageUrl && width >= 220

                                    x: root.entryX(entryData.startTime)
                                    y: 8
                                    width: root.entryWidth(entryData.startTime, entryData.endTime)
                                    height: root.rowHeight - 16
                                    radius: 14
                                    clip: true
                                    color: entryData.isCurrent ? "#2a6b52" : "#1a3243"
                                    border.color: entryData.isCurrent ? "#8dd6a5" : "#35617b"

                                    MouseArea {
                                        anchors.fill: parent
                                        onClicked: root.openGuideActions(rowItem.channelData, entryRect.entryData)
                                    }

                                    RowLayout {
                                        anchors.fill: parent
                                        anchors.leftMargin: 12
                                        anchors.rightMargin: 12
                                        anchors.topMargin: 8
                                        anchors.bottomMargin: 8
                                        spacing: 8

                                        Image {
                                            Layout.alignment: Qt.AlignVCenter
                                            Layout.preferredWidth: 44
                                            Layout.preferredHeight: 30
                                            visible: entryRect.showArtwork
                                            source: entryRect.entryData.imageUrl ? entryRect.entryData.imageUrl : ""
                                            asynchronous: true
                                            fillMode: Image.PreserveAspectFit
                                            cache: true
                                        }

                                        ColumnLayout {
                                            Layout.fillWidth: true
                                            Layout.alignment: Qt.AlignVCenter
                                            spacing: 2

                                            Label {
                                                Layout.fillWidth: true
                                                text: entryRect.entryData.title
                                                color: "#eff7fb"
                                                font.family: "IBM Plex Sans"
                                                font.pixelSize: 13
                                                font.bold: true
                                                leftPadding: 1
                                                rightPadding: 1
                                                elide: Text.ElideRight
                                            }

                                            Label {
                                                Layout.fillWidth: true
                                                text: root.formatStamp(entryRect.entryData.startTime, "h:mm AP")
                                                      + " - "
                                                      + root.formatStamp(entryRect.entryData.endTime, "h:mm AP")
                                                color: "#c5d8e5"
                                                font.family: "IBM Plex Sans"
                                                font.pixelSize: 11
                                                leftPadding: 1
                                                rightPadding: 1
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
        }
    }

    Dialog {
        id: guideActionDialog

        modal: true
        focus: true
        closePolicy: Popup.CloseOnEscape
        standardButtons: Dialog.NoButton
        title: ""
        width: 520
        padding: 18

        background: Rectangle {
            radius: 24
            color: "#0d1a24"
            border.color: "#23445b"
        }

        Overlay.modal: Rectangle {
            color: "#07111a"
            opacity: 0.76
        }

        contentItem: ColumnLayout {
            spacing: 12

            Label {
                Layout.fillWidth: true
                text: root.selectedGuideTitle
                color: "#f2f7fb"
                font.family: "IBM Plex Sans"
                font.pixelSize: 18
                font.bold: true
                wrapMode: Text.Wrap
            }

            Label {
                Layout.fillWidth: true
                text: root.selectedGuideSubtitle
                color: "#dce6ed"
                font.family: "IBM Plex Sans"
                font.pixelSize: 13
                wrapMode: Text.Wrap
            }

            Label {
                Layout.fillWidth: true
                text: "Choose what to do with this guide airing."
                color: "#8ea7b9"
                font.family: "IBM Plex Sans"
                font.pixelSize: 12
                wrapMode: Text.Wrap
            }
        }

        footer: RowLayout {
            spacing: 10

            Item { Layout.fillWidth: true }

            ThemeButton {
                text: "Cancel"
                onClicked: guideActionDialog.close()
            }

            ThemeButton {
                text: "Watch"
                onClicked: {
                    guideActionDialog.close()
                    root.channelActivated(root.selectedGuideChannelRef)
                }
            }

            ThemeButton {
                text: "Record Series"
                enabled: !!root.selectedGuideContext.seriesId
                onClicked: {
                    guideActionDialog.close()
                    root.recordSeriesRequested(root.selectedGuideContext)
                }
            }

            ThemeButton {
                text: "Record Once"
                enabled: !!root.selectedGuideContext.seriesId
                    && !!root.selectedGuideContext.channelNumber
                    && root.selectedGuideContext.startTime > 0
                onClicked: {
                    guideActionDialog.close()
                    root.recordOnceRequested(root.selectedGuideContext)
                }
            }
        }
    }
}