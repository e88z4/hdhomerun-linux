import QtQuick
import QtQuick.Controls
import QtQuick.Layouts

Dialog {
    id: root

    required property bool dialogVisible
    required property string titleText
    required property string messageText
    required property bool canCreateSeries
    required property bool canCreateOneTime
    required property bool submitting
    signal closeRequested()
    signal createSeriesRequested()
    signal createOneTimeRequested()

    modal: true
    focus: true
    closePolicy: Popup.CloseOnEscape
    standardButtons: Dialog.NoButton
    title: ""
    visible: root.dialogVisible
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

    onVisibleChanged: {
        if (!visible && dialogVisible) {
            closeRequested()
        }
    }

    contentItem: ColumnLayout {
        spacing: 12

        Label {
            Layout.fillWidth: true
            text: root.titleText
            wrapMode: Text.Wrap
            color: "#f2f7fb"
            font.family: "IBM Plex Sans"
            font.pixelSize: 18
            font.bold: true
        }

        Label {
            Layout.fillWidth: true
            text: root.messageText
            wrapMode: Text.Wrap
            color: "#dce6ed"
            font.family: "IBM Plex Sans"
            font.pixelSize: 13
        }

        Label {
            visible: !root.canCreateSeries && !root.canCreateOneTime
            Layout.fillWidth: true
            text: "No DVR rule action is currently available for this context."
            wrapMode: Text.Wrap
            color: "#8ea7b9"
            font.family: "IBM Plex Sans"
            font.pixelSize: 12
        }

        BusyIndicator {
            Layout.alignment: Qt.AlignHCenter
            running: root.submitting
            visible: root.submitting
        }
    }

    footer: RowLayout {
        spacing: 10

        Item { Layout.fillWidth: true }

        ThemeButton {
            text: "Cancel"
            enabled: !root.submitting
            onClicked: root.closeRequested()
        }

        ThemeButton {
            text: "Create Series Rule"
            enabled: root.canCreateSeries && !root.submitting
            onClicked: root.createSeriesRequested()
        }

        ThemeButton {
            text: "Create One-Time Rule"
            enabled: root.canCreateOneTime && !root.submitting
            onClicked: root.createOneTimeRequested()
        }
    }
}