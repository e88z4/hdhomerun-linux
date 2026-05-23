#pragma once

#include <QJsonArray>
#include <QJsonObject>
#include <QNetworkAccessManager>
#include <QObject>
#include <QProcess>
#include <QVariantList>

#include <functional>

class AppController : public QObject
{
    Q_OBJECT
    Q_PROPERTY(QVariantList devices READ devices NOTIFY devicesChanged)
    Q_PROPERTY(int selectedDeviceIndex READ selectedDeviceIndex NOTIFY selectedDeviceIndexChanged)
    Q_PROPERTY(QVariantList channels READ channels NOTIFY channelsChanged)
    Q_PROPERTY(QVariantList guideChannels READ guideChannels NOTIFY guideChannelsChanged)
    Q_PROPERTY(bool guideVisible READ guideVisible WRITE setGuideVisible NOTIFY guideVisibleChanged)
    Q_PROPERTY(bool guideLoading READ guideLoading NOTIFY guideLoadingChanged)
    Q_PROPERTY(qint64 guideWindowStart READ guideWindowStart NOTIFY guideWindowStartChanged)
    Q_PROPERTY(int guideDurationHours READ guideDurationHours NOTIFY guideDurationHoursChanged)
    Q_PROPERTY(QString shellPhase READ shellPhase NOTIFY shellPhaseChanged)
    Q_PROPERTY(QString currentChannelRef READ currentChannelRef NOTIFY currentChannelRefChanged)
    Q_PROPERTY(QString stageTitle READ stageTitle NOTIFY stageTitleChanged)
    Q_PROPERTY(QString stageSubtitle READ stageSubtitle NOTIFY stageSubtitleChanged)
    Q_PROPERTY(QString stageWarning READ stageWarning NOTIFY stageWarningChanged)
    Q_PROPERTY(QString stageFailure READ stageFailure NOTIFY stageFailureChanged)
    Q_PROPERTY(QString playbackUrl READ playbackUrl NOTIFY playbackUrlChanged)
    Q_PROPERTY(bool embeddedPlaybackEnabled READ embeddedPlaybackEnabled NOTIFY embeddedPlaybackEnabledChanged)
    Q_PROPERTY(bool diagnosticsExpanded READ diagnosticsExpanded WRITE setDiagnosticsExpanded NOTIFY diagnosticsExpandedChanged)
    Q_PROPERTY(QString diagnosticsSummary READ diagnosticsSummary NOTIFY diagnosticsSummaryChanged)
    Q_PROPERTY(QVariantList diagnosticsRows READ diagnosticsRows NOTIFY diagnosticsRowsChanged)

public:
    explicit AppController(QObject *parent = nullptr);
    ~AppController() override;

    QVariantList devices() const;
    int selectedDeviceIndex() const;
    QVariantList channels() const;
    QVariantList guideChannels() const;
    bool guideVisible() const;
    bool guideLoading() const;
    qint64 guideWindowStart() const;
    int guideDurationHours() const;
    QString shellPhase() const;
    QString currentChannelRef() const;
    QString stageTitle() const;
    QString stageSubtitle() const;
    QString stageWarning() const;
    QString stageFailure() const;
    QString playbackUrl() const;
    bool embeddedPlaybackEnabled() const;
    bool diagnosticsExpanded() const;
    QString diagnosticsSummary() const;
    QVariantList diagnosticsRows() const;

    Q_INVOKABLE void initialize();
    Q_INVOKABLE void selectDeviceIndex(int index);
    Q_INVOKABLE void playChannel(const QString &channelRef);
    Q_INVOKABLE void playAdjacentChannel(int direction);
    Q_INVOKABLE void toggleGuide();
    Q_INVOKABLE void shiftGuideWindow(int deltaHours);
    Q_INVOKABLE void jumpGuideToNow();
    Q_INVOKABLE void retryPlayback();
    Q_INVOKABLE void toggleDiagnostics();

    void setDiagnosticsExpanded(bool expanded);
    void setGuideVisible(bool visible);

signals:
    void devicesChanged();
    void selectedDeviceIndexChanged();
    void channelsChanged();
    void guideChannelsChanged();
    void guideVisibleChanged();
    void guideLoadingChanged();
    void guideWindowStartChanged();
    void guideDurationHoursChanged();
    void shellPhaseChanged();
    void currentChannelRefChanged();
    void stageTitleChanged();
    void stageSubtitleChanged();
    void stageWarningChanged();
    void stageFailureChanged();
    void playbackUrlChanged();
    void embeddedPlaybackEnabledChanged();
    void diagnosticsExpandedChanged();
    void diagnosticsSummaryChanged();
    void diagnosticsRowsChanged();

private:
    using SuccessHandler = std::function<void(const QJsonObject &)>;
    using ErrorHandler = std::function<void(const QString &)>;

    void getJson(const QString &path, const SuccessHandler &onSuccess, const ErrorHandler &onError);
    void postJson(const QString &path, const QJsonObject &payload, const SuccessHandler &onSuccess, const ErrorHandler &onError);
    void handleJsonReply(QObject *replyObject, const SuccessHandler &onSuccess, const ErrorHandler &onError);

    void probeBackend(bool allowStart);
    void waitForBackend(int remainingAttempts);
    void startBundledBackend();
    void shutdownBackendProcess();
    void loadBootstrap();
    void loadDevices();
    void loadLineup();
    void loadGuide();
    void loadPlaybackCurrent();
    void loadDiagnostics();
    void refreshSelectedData();

    void applyDevicesResponse(const QJsonObject &payload);
    void applyGuideResponse(const QJsonObject &payload);
    void applyPlaybackResponse(const QJsonObject &payload);
    void applyDiagnosticsResponse(const QJsonObject &payload);

    void setDevices(const QVariantList &devices);
    void setSelectedDeviceIndex(int index);
    void setChannels(const QVariantList &channels);
    void setGuideChannels(const QVariantList &channels);
    void setGuideLoading(bool loading);
    void setGuideWindowStart(qint64 start);
    void setGuideDurationHours(int hours);
    void setShellPhase(const QString &phase);
    void setCurrentChannelRef(const QString &channelRef);
    void setStageTitle(const QString &title);
    void setStageSubtitle(const QString &subtitle);
    void setStageWarning(const QString &warning);
    void setStageFailure(const QString &failure);
    void setPlaybackUrl(const QString &playbackUrl);
    void setEmbeddedPlaybackEnabled(bool enabled);
    void setDiagnosticsSummary(const QString &summary);
    void setDiagnosticsRows(const QVariantList &rows);

    QString selectedDeviceRef() const;
    void setLaunchFailure(const QString &message);

    static QVariantList jsonArrayToVariantList(const QJsonArray &array);

    QNetworkAccessManager m_network;
    QProcess *m_backendProcess;
    QString m_backendBaseUrl;
    QVariantList m_devices;
    int m_selectedDeviceIndex;
    QVariantList m_channels;
    QVariantList m_guideChannels;
    bool m_guideVisible;
    bool m_guideLoading;
    qint64 m_guideWindowStart;
    int m_guideDurationHours;
    bool m_guideEndpointAvailable;
    QString m_shellPhase;
    QString m_currentChannelRef;
    QString m_stageTitle;
    QString m_stageSubtitle;
    QString m_stageWarning;
    QString m_stageFailure;
    QString m_playbackUrl;
    bool m_embeddedPlaybackEnabled;
    bool m_diagnosticsExpanded;
    QString m_diagnosticsSummary;
    QVariantList m_diagnosticsRows;
};