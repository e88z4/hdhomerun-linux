import QtQuick
import QtQuick.Controls
import QtQuick.Layouts

import HDHomeRun.Client
import "components"

ApplicationWindow {
    id: window

    width: 1440
    height: 900
    visible: true
    title: "HDHomeRun Linux Player"
    color: "#08131c"

    Rectangle {
        anchors.fill: parent
        gradient: Gradient {
            GradientStop { position: 0.0; color: "#10283a" }
            GradientStop { position: 0.45; color: "#08131c" }
            GradientStop { position: 1.0; color: "#050a10" }
        }
    }

    header: ToolBar {
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

            Button {
                text: appController.diagnosticsExpanded ? "Hide Diagnostics" : "Show Diagnostics"
                onClicked: appController.toggleDiagnostics()
            }
        }
    }

    RowLayout {
        anchors.fill: parent
        anchors.margins: 18
        spacing: 18

        ChannelRail {
            Layout.preferredWidth: 300
            Layout.fillHeight: true
            channels: appController.channels
            currentChannelRef: appController.currentChannelRef
            onChannelActivated: function(channelRef, guideNumber, guideName, availability) {
                if (availability === "restricted") {
                    return
                }
                appController.playChannel(channelRef)
            }
        }

        PlaybackStage {
            Layout.fillWidth: true
            Layout.fillHeight: true
            shellPhase: appController.shellPhase
            currentTitle: appController.stageTitle
            currentSubtitle: appController.stageSubtitle
            warningText: appController.stageWarning
            failureText: appController.stageFailure
            playbackUrl: appController.playbackUrl
            embeddedPlaybackEnabled: appController.embeddedPlaybackEnabled
            retryEnabled: appController.shellPhase === "playback_failed"
            onRetryRequested: appController.retryPlayback()
        }

        DiagnosticsDrawer {
            Layout.preferredWidth: appController.diagnosticsExpanded ? 320 : 52
            Layout.fillHeight: true
            expanded: appController.diagnosticsExpanded
            summaryText: appController.diagnosticsSummary
            diagnosticsRows: appController.diagnosticsRows
        }
    }
}