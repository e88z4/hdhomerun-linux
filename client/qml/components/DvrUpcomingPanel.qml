import QtQuick
import QtQuick.Controls
import QtQuick.Layouts

Pane {
    id: root

    required property var upcomingEntries
    required property bool loading
    required property string upcomingState
    signal refreshRequested()
    signal ruleEditorRequested(string programId)
    signal deleteRuleRequested(string programId)

    property string pendingDeleteProgramId: ""
    property string pendingDeleteTitle: ""
    property string pendingDeleteEpisodeTitle: ""

    function openDeleteRuleDialog(entryData) {
        pendingDeleteProgramId = entryData.programId
        pendingDeleteTitle = entryData.title
        pendingDeleteEpisodeTitle = entryData.episodeTitle || ""
        deleteRuleDialog.open()
    }

    padding: 14
    background: Rectangle {
        radius: 24
        color: "#0d1a24"
        border.color: "#183447"
    }

    ColumnLayout {
        anchors.fill: parent
        spacing: 12

        ColumnLayout {
            Layout.fillWidth: true
            spacing: 8

            ColumnLayout {
                Layout.fillWidth: true
                spacing: 2

                Label {
                    Layout.fillWidth: true
                    text: "Upcoming"
                    color: "#f2f7fb"
                    font.family: "IBM Plex Sans"
                    font.pixelSize: 20
                    font.bold: true
                }

                Label {
                    Layout.fillWidth: true
                    text: "Create series or one-time rules from upcoming airings"
                    color: "#8ea7b9"
                    font.family: "IBM Plex Sans"
                    font.pixelSize: 12
                    wrapMode: Text.Wrap
                }
            }

            RowLayout {
                Layout.fillWidth: true

                Item { Layout.fillWidth: true }

                ThemeButton {
                    text: "Refresh"
                    onClicked: root.refreshRequested()
                }
            }
        }

        BusyIndicator {
            Layout.alignment: Qt.AlignHCenter
            running: root.loading
            visible: root.loading
        }

        Label {
            visible: !root.loading && root.upcomingEntries.length === 0
            Layout.fillWidth: true
            text: root.upcomingState === "selection_required"
                ? "Select a device to view upcoming DVR schedule state."
                : "No upcoming DVR items are currently available."
            wrapMode: Text.Wrap
            color: "#8ea7b9"
            font.family: "IBM Plex Sans"
        }

        ScrollView {
            Layout.fillWidth: true
            Layout.fillHeight: true
            visible: !root.loading && root.upcomingEntries.length > 0
            clip: true

            Column {
                width: parent.width
                spacing: 8

                Repeater {
                    model: root.upcomingEntries

                    Rectangle {
                        required property var modelData

                        width: parent.width
                        height: 124
                        radius: 18
                        color: "#101e29"
                        border.color: "#1d4259"

                        ColumnLayout {
                            anchors.fill: parent
                            anchors.margins: 12
                            spacing: 4

                            Label {
                                Layout.fillWidth: true
                                text: parent.parent.modelData.title
                                color: "#eff7fb"
                                font.family: "IBM Plex Sans"
                                font.pixelSize: 14
                                font.bold: true
                                elide: Text.ElideRight
                            }

                            Label {
                                Layout.fillWidth: true
                                visible: !!parent.parent.modelData.episodeTitle
                                text: parent.parent.modelData.episodeTitle
                                color: "#bcd0dc"
                                font.family: "IBM Plex Sans"
                                font.pixelSize: 12
                                elide: Text.ElideRight
                            }

                            Label {
                                Layout.fillWidth: true
                                text: parent.parent.modelData.channelNumber + " • " + parent.parent.modelData.channelName + " • "
                                    + Qt.formatDateTime(new Date(parent.parent.modelData.startTime * 1000), "MMM d h:mm AP")
                                color: "#8ea7b9"
                                font.family: "IBM Plex Sans"
                                font.pixelSize: 11
                                wrapMode: Text.Wrap
                            }

                            Item { Layout.fillHeight: true }

                            RowLayout {
                                Layout.fillWidth: true
                                spacing: 10

                                ThemeButton {
                                    text: "Rule Options"
                                    onClicked: root.ruleEditorRequested(parent.parent.modelData.programId)
                                }

                                ThemeButton {
                                    text: "Delete Rule"
                                    onClicked: root.openDeleteRuleDialog(parent.parent.modelData)
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    Dialog {
        id: deleteRuleDialog

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
            spacing: 10

            Label {
                Layout.fillWidth: true
                text: "Delete Recording Rule"
                color: "#f2f7fb"
                font.family: "IBM Plex Sans"
                font.pixelSize: 18
                font.bold: true
            }

            Label {
                Layout.fillWidth: true
                text: root.pendingDeleteEpisodeTitle === ""
                    ? root.pendingDeleteTitle
                    : root.pendingDeleteTitle + " • " + root.pendingDeleteEpisodeTitle
                color: "#eff7fb"
                font.family: "IBM Plex Sans"
                font.pixelSize: 14
                font.bold: true
                wrapMode: Text.Wrap
            }

            Label {
                Layout.fillWidth: true
                text: "This deletes the recording rule backing this upcoming item. If it is a series rule, future scheduled airings tied to that rule will also be removed."
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
                onClicked: deleteRuleDialog.close()
            }

            ThemeButton {
                text: "Delete Rule"
                onClicked: {
                    const programId = root.pendingDeleteProgramId
                    deleteRuleDialog.close()
                    root.deleteRuleRequested(programId)
                }
            }
        }
    }
}