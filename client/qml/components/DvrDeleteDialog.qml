import QtQuick
import QtQuick.Controls
import QtQuick.Layouts

Dialog {
    id: root

    required property bool dialogVisible
    required property string titleText
    required property bool busy
    signal closeRequested()
    signal deleteRequested()
    signal deleteRerecordRequested()

    modal: true
    focus: true
    closePolicy: Popup.CloseOnEscape
    standardButtons: Dialog.NoButton
    title: ""
    visible: root.dialogVisible
    width: 500
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

    onVisibleChanged: {
        if (!visible && dialogVisible) {
            closeRequested()
        }
    }

    contentItem: ColumnLayout {
        spacing: 12

        Label {
            Layout.fillWidth: true
            text: "Delete Recording"
            color: "#f2f7fb"
            font.family: "IBM Plex Sans"
            font.pixelSize: 18
            font.bold: true
        }

        Label {
            Layout.fillWidth: true
            text: "Choose how to handle " + root.titleText + "."
            wrapMode: Text.Wrap
            color: "#dce6ed"
            font.family: "IBM Plex Sans"
            font.pixelSize: 13
        }

        Label {
            Layout.fillWidth: true
            text: "Delete removes the recording immediately. Delete & Re-record removes it first, then tries to create a replacement series rule when the schedule context is trustworthy."
            wrapMode: Text.Wrap
            color: "#8ea7b9"
            font.family: "IBM Plex Sans"
            font.pixelSize: 12
        }

        BusyIndicator {
            Layout.alignment: Qt.AlignHCenter
            running: root.busy
            visible: root.busy
        }
    }

    footer: RowLayout {
        spacing: 10

        Item { Layout.fillWidth: true }

        ThemeButton {
            text: "Cancel"
            enabled: !root.busy
            onClicked: root.closeRequested()
        }

        ThemeButton {
            text: "Delete"
            enabled: !root.busy
            onClicked: root.deleteRequested()
        }

        ThemeButton {
            text: "Delete & Re-record"
            enabled: !root.busy
            onClicked: root.deleteRerecordRequested()
        }
    }
}