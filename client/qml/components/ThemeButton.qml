import QtQuick
import QtQuick.Controls

Button {
    id: control

    property bool compact: false

    implicitHeight: compact ? 32 : 36
    implicitWidth: Math.max(compact ? 72 : 88, contentItem.implicitWidth + leftPadding + rightPadding)
    leftPadding: compact ? 12 : 14
    rightPadding: compact ? 12 : 14
    topPadding: 0
    bottomPadding: 0
    hoverEnabled: true

    font.family: "IBM Plex Sans"
    font.pixelSize: compact ? 12 : 13
    font.bold: true

    background: Rectangle {
        radius: control.compact ? 10 : 12
        color: !control.enabled
            ? "#10202d"
            : ((control.highlighted || control.checked)
                ? (control.down ? "#255977" : (control.hovered ? "#214f6a" : "#1d455f"))
                : (control.down ? "#1a4258" : (control.hovered ? "#173247" : "#112838")))
        border.color: !control.enabled ? "#274053" : ((control.highlighted || control.checked) ? "#7fd4ff" : "#2d5a74")
    }

    contentItem: Text {
        text: control.text
        font: control.font
        color: control.enabled ? "#eff7fb" : "#627786"
        horizontalAlignment: Text.AlignHCenter
        verticalAlignment: Text.AlignVCenter
        elide: Text.ElideRight
    }
}