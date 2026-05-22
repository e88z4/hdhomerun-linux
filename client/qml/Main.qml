import QtQuick
import QtQuick.Controls
import QtQuick.Layouts

import HDHomeRun.Client
import "components"

ApplicationWindow {
    id: window

    property int overlayPulse: 0
    property bool fullscreenMode: visibility === Window.FullScreen

    function bumpOverlay() {
        overlayPulse += 1
    }

    function toggleFullscreen() {
        if (!fullscreenMode) {
            bumpOverlay()
        }
        visibility = fullscreenMode ? Window.Windowed : Window.FullScreen
    }

    function exitFullscreen() {
        if (fullscreenMode) {
            visibility = Window.Windowed
        }
    }

    minimumWidth: 1180
    minimumHeight: 720
    width: Math.min(Math.max(1440, Math.round(Screen.width * 0.88)), Math.max(960, Screen.width - 80))
    height: Math.min(Math.max(900, Math.round(Screen.height * 0.86)), Math.max(640, Screen.height - 80))
    x: Math.max(0, Math.round((Screen.width - width) / 2))
    y: Math.max(0, Math.round((Screen.height - height) / 2))
    visible: true
    title: "HDHomeRun Linux Player"
    color: "#08131c"

    Shortcut {
        sequence: "F"
        context: Qt.ApplicationShortcut
        onActivated: window.toggleFullscreen()
    }

    Shortcut {
        sequence: "Esc"
        context: Qt.ApplicationShortcut
        enabled: window.fullscreenMode
        onActivated: window.exitFullscreen()
    }

    Shortcut {
        sequence: "Up"
        context: Qt.ApplicationShortcut
        onActivated: {
            window.bumpOverlay()
            playbackStage.adjustVolume(0.05)
        }
    }

    Shortcut {
        sequence: "Down"
        context: Qt.ApplicationShortcut
        onActivated: {
            window.bumpOverlay()
            playbackStage.adjustVolume(-0.05)
        }
    }

    Shortcut {
        sequence: "Right"
        context: Qt.ApplicationShortcut
        onActivated: {
            window.bumpOverlay()
            appController.playAdjacentChannel(1)
        }
    }

    Shortcut {
        sequence: "Left"
        context: Qt.ApplicationShortcut
        onActivated: {
            window.bumpOverlay()
            appController.playAdjacentChannel(-1)
        }
    }

    Rectangle {
        anchors.fill: parent
        gradient: Gradient {
            GradientStop { position: 0.0; color: "#10283a" }
            GradientStop { position: 0.45; color: "#08131c" }
            GradientStop { position: 1.0; color: "#050a10" }
        }
    }

    header: ToolBar {
        visible: !window.fullscreenMode
        padding: 14
        background: Rectangle {
            color: "#0d1b26"
            border.color: "#173245"
        }

        RowLayout {
            anchors.fill: parent
            spacing: 16

            Label {
                text: "HDHomeRun Linux Player"
                color: "#f3f7fa"
                font.family: "IBM Plex Sans"
                font.pixelSize: 24
                font.bold: true
            }

            Rectangle {
                Layout.fillWidth: true
                height: 40
                radius: 20
                color: "#112838"
                border.color: "#20445c"

                RowLayout {
                    anchors.fill: parent
                    anchors.leftMargin: 14
                    anchors.rightMargin: 14

                    Label {
                        text: "Device"
                        color: "#9fb5c5"
                        font.family: "IBM Plex Sans"
                    }

                    ComboBox {
                        id: devicePicker
                        Layout.fillWidth: true
                        model: appController.devices
                        textRole: "name"
                        currentIndex: appController.selectedDeviceIndex
                        onActivated: function(index) { appController.selectDeviceIndex(index) }
                    }
                }
            }

            IconButton {
                iconKind: "diagnostics"
                toolTipText: appController.diagnosticsExpanded ? "Hide diagnostics" : "Show diagnostics"
                onClicked: appController.toggleDiagnostics()
            }
        }
    }

    RowLayout {
        anchors.fill: parent
        anchors.margins: window.fullscreenMode ? 0 : 18
        spacing: window.fullscreenMode ? 0 : 18

        ColumnLayout {
            Layout.fillWidth: true
            Layout.fillHeight: true

            spacing: window.fullscreenMode ? 0 : 14

            PlaybackStage {
                id: playbackStage
                Layout.fillWidth: true
                Layout.fillHeight: true
                immersiveMode: window.fullscreenMode
                fullscreenMode: window.fullscreenMode
                overlayPulse: window.overlayPulse
                shellPhase: appController.shellPhase
                currentTitle: appController.stageTitle
                currentSubtitle: appController.stageSubtitle
                warningText: appController.stageWarning
                failureText: appController.stageFailure
                playbackUrl: appController.playbackUrl
                embeddedPlaybackEnabled: appController.embeddedPlaybackEnabled
                retryEnabled: appController.shellPhase === "playback_failed"
                onExitFullscreenRequested: window.exitFullscreen()
                onToggleFullscreenRequested: window.toggleFullscreen()
                onRetryRequested: appController.retryPlayback()
            }

            ChannelRail {
                visible: !window.fullscreenMode
                Layout.fillWidth: true
                Layout.preferredHeight: 170
                Layout.minimumHeight: 150
                Layout.maximumHeight: 190
                compactMode: true
                channels: appController.channels
                currentChannelRef: appController.currentChannelRef
                onChannelActivated: function(channelRef, guideNumber, guideName, availability) {
                    if (availability === "restricted") {
                        return
                    }
                    appController.playChannel(channelRef)
                }
            }
        }

        DiagnosticsDrawer {
            visible: !window.fullscreenMode
            Layout.preferredWidth: window.fullscreenMode ? 0 : (appController.diagnosticsExpanded ? 320 : 52)
            Layout.maximumWidth: window.fullscreenMode ? 0 : Number.POSITIVE_INFINITY
            Layout.fillHeight: true
            expanded: appController.diagnosticsExpanded
            summaryText: appController.diagnosticsSummary
            diagnosticsRows: appController.diagnosticsRows
        }
    }
}