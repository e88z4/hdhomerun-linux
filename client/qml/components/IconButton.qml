import QtQuick
import QtQuick.Controls

ToolButton {
    id: control

    required property string iconKind
    property string toolTipText: ""
    property color iconColor: enabled ? "#eff7fb" : "#627786"

    implicitWidth: 42
    implicitHeight: 42
    padding: 0
    hoverEnabled: true

    ToolTip.visible: hovered && toolTipText !== ""
    ToolTip.text: toolTipText
    ToolTip.delay: 250

    background: Rectangle {
        radius: 14
        color: !control.enabled ? "#10202d" : (control.down ? "#1a4258" : (control.hovered ? "#173247" : "#112838"))
        border.color: !control.enabled ? "#274053" : (control.down ? "#7fd4ff" : "#2d5a74")
    }

    contentItem: Item {
        Canvas {
            id: iconCanvas
            anchors.centerIn: parent
            width: 20
            height: 20

            onPaint: {
                const ctx = getContext("2d")
                ctx.reset()
                ctx.strokeStyle = control.iconColor
                ctx.fillStyle = control.iconColor
                ctx.lineWidth = 1.9
                ctx.lineCap = "round"
                ctx.lineJoin = "round"

                function strokeChevron(x1, y1, x2, y2, x3, y3) {
                    ctx.beginPath()
                    ctx.moveTo(x1, y1)
                    ctx.lineTo(x2, y2)
                    ctx.lineTo(x3, y3)
                    ctx.stroke()
                }

                function drawSpeaker() {
                    ctx.beginPath()
                    ctx.moveTo(3.5, 8)
                    ctx.lineTo(6.8, 8)
                    ctx.lineTo(10.5, 5)
                    ctx.lineTo(10.5, 15)
                    ctx.lineTo(6.8, 12)
                    ctx.lineTo(3.5, 12)
                    ctx.closePath()
                    ctx.stroke()
                }

                switch (control.iconKind) {
                case "diagnostics":
                    ctx.beginPath()
                    ctx.roundRect(3, 3.5, 14, 13, 3)
                    ctx.stroke()
                    ctx.beginPath()
                    ctx.moveTo(7, 7)
                    ctx.lineTo(14, 7)
                    ctx.moveTo(7, 10)
                    ctx.lineTo(14, 10)
                    ctx.moveTo(7, 13)
                    ctx.lineTo(11, 13)
                    ctx.moveTo(5.2, 6.8)
                    ctx.lineTo(5.2, 13.2)
                    ctx.stroke()
                    break
                case "volume-down":
                    drawSpeaker()
                    ctx.beginPath()
                    ctx.moveTo(13.5, 10)
                    ctx.lineTo(17, 10)
                    ctx.stroke()
                    break
                case "volume-up":
                    drawSpeaker()
                    ctx.beginPath()
                    ctx.moveTo(13.3, 10)
                    ctx.lineTo(17, 10)
                    ctx.moveTo(15.15, 8.2)
                    ctx.lineTo(15.15, 11.8)
                    ctx.stroke()
                    break
                case "fullscreen":
                    strokeChevron(7.5, 5.5, 4.5, 5.5, 4.5, 8.5)
                    strokeChevron(12.5, 5.5, 15.5, 5.5, 15.5, 8.5)
                    strokeChevron(7.5, 14.5, 4.5, 14.5, 4.5, 11.5)
                    strokeChevron(12.5, 14.5, 15.5, 14.5, 15.5, 11.5)
                    break
                case "fullscreen-exit":
                    strokeChevron(4.5, 8, 7.5, 8, 7.5, 5)
                    strokeChevron(15.5, 8, 12.5, 8, 12.5, 5)
                    strokeChevron(4.5, 12, 7.5, 12, 7.5, 15)
                    strokeChevron(15.5, 12, 12.5, 12, 12.5, 15)
                    break
                case "retry":
                    ctx.beginPath()
                    ctx.arc(10, 10, 5.8, Math.PI * 0.25, Math.PI * 1.65, false)
                    ctx.stroke()
                    ctx.beginPath()
                    ctx.moveTo(13.8, 4.9)
                    ctx.lineTo(16.8, 5.1)
                    ctx.lineTo(15.5, 7.8)
                    ctx.closePath()
                    ctx.fill()
                    break
                default:
                    ctx.beginPath()
                    ctx.arc(10, 10, 2.5, 0, Math.PI * 2)
                    ctx.fill()
                    break
                }
            }
        }
    }

    onIconKindChanged: iconCanvas.requestPaint()
    onIconColorChanged: iconCanvas.requestPaint()
    onEnabledChanged: iconCanvas.requestPaint()
    onDownChanged: iconCanvas.requestPaint()
    onHoveredChanged: iconCanvas.requestPaint()
}