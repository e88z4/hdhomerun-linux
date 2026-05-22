import QtQuick
import QtQuick.Controls
import QtQuick.Layouts

Pane {
    id: root

    required property bool expanded
    required property string summaryText
    required property var diagnosticsRows

    padding: root.expanded ? 16 : 8
    background: Rectangle {
        radius: 28
        color: root.expanded ? "#0f1d28" : "#10202d"
        border.color: "#214358"
    }

    ColumnLayout {
        anchors.fill: parent
        spacing: 12

        Label {
            text: root.expanded ? "Diagnostics" : "Diag"
            color: "#eef8ff"
            rotation: root.expanded ? 0 : -90
            horizontalAlignment: Text.AlignHCenter
            verticalAlignment: Text.AlignVCenter
            font.family: "IBM Plex Sans"
            font.bold: true
            Layout.alignment: Qt.AlignHCenter
        }

        Item {
            visible: root.expanded
            Layout.fillWidth: true
            Layout.fillHeight: true

            ColumnLayout {
                anchors.fill: parent
                spacing: 12

                Rectangle {
                    Layout.fillWidth: true
                    radius: 18
                    color: "#142a39"
                    border.color: "#29536a"
                    implicitHeight: summaryColumn.implicitHeight + 28

                    ColumnLayout {
                        id: summaryColumn
                        anchors.fill: parent
                        anchors.margins: 14

                        Label {
                            text: "Playback Summary"
                            color: "#f1f7fb"
                            font.family: "IBM Plex Sans"
                            font.bold: true
                        }

                        Label {
                            Layout.fillWidth: true
                            text: root.summaryText
                            color: "#a8c0d2"
                            wrapMode: Text.WordWrap
                            font.family: "IBM Plex Sans"
                        }
                    }
                }

                Repeater {
                    model: root.diagnosticsRows

                    delegate: Rectangle {
                        Layout.fillWidth: true
                        radius: 16
                        color: index === 0 ? "#183648" : "#132735"
                        border.color: index === 0 ? "#66c7ff" : "#22465e"
                        implicitHeight: detailColumn.implicitHeight + 24

                        ColumnLayout {
                            id: detailColumn
                            anchors.fill: parent
                            anchors.margins: 12

                            Label {
                                Layout.fillWidth: true
                                text: modelData.title
                                color: "#f2f7fb"
                                font.family: "IBM Plex Sans"
                                font.bold: true
                                wrapMode: Text.WordWrap
                            }

                            Label {
                                Layout.fillWidth: true
                                text: modelData.detail
                                color: "#9eb6c6"
                                font.family: "IBM Plex Sans"
                                wrapMode: Text.WordWrap
                            }
                        }
                    }
                }
            }
        }
    }
}