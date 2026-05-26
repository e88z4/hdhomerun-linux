import QtQuick
import QtQuick.Controls
import QtQuick.Layouts

Pane {
    id: root

    required property string severity
    required property string titleText
    required property string messageText
    signal refreshRequested()

    padding: 14
    background: Rectangle {
        radius: 22
        color: root.severity === "blocking" ? "#412118" : (root.severity === "warning" ? "#3e3111" : "#113043")
        border.color: root.severity === "blocking" ? "#ff9b7a" : (root.severity === "warning" ? "#f0cf72" : "#69c6ff")
    }

    RowLayout {
        anchors.fill: parent
        spacing: 16

        ColumnLayout {
            Layout.fillWidth: true
            spacing: 4

            Label {
                text: root.titleText
                color: "#f4f8fb"
                font.family: "IBM Plex Sans"
                font.pixelSize: 17
                font.bold: true
            }

            Label {
                Layout.fillWidth: true
                text: root.messageText
                wrapMode: Text.Wrap
                color: "#d7e4ee"
                font.family: "IBM Plex Sans"
                font.pixelSize: 13
            }
        }

        ThemeButton {
            text: "Refresh"
            onClicked: root.refreshRequested()
        }
    }
}