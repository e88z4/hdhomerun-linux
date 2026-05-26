#pragma once

#include <QJsonArray>
#include <QJsonObject>
#include <QNetworkAccessManager>
#include <QObject>
#include <QProcess>
#include <QSet>
#include <QVariantList>
#include <QVariantMap>

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
    Q_PROPERTY(QString workspaceMode READ workspaceMode WRITE setWorkspaceMode NOTIFY workspaceModeChanged)
    Q_PROPERTY(QString playbackMode READ playbackMode NOTIFY playbackModeChanged)
    Q_PROPERTY(QString currentRecordingId READ currentRecordingId NOTIFY currentRecordingIdChanged)
    Q_PROPERTY(bool embeddedPlaybackEnabled READ embeddedPlaybackEnabled NOTIFY embeddedPlaybackEnabledChanged)
    Q_PROPERTY(bool diagnosticsExpanded READ diagnosticsExpanded WRITE setDiagnosticsExpanded NOTIFY diagnosticsExpandedChanged)
    Q_PROPERTY(QString diagnosticsSummary READ diagnosticsSummary NOTIFY diagnosticsSummaryChanged)
    Q_PROPERTY(QVariantList diagnosticsRows READ diagnosticsRows NOTIFY diagnosticsRowsChanged)
    Q_PROPERTY(bool dvrLoading READ dvrLoading NOTIFY dvrLoadingChanged)
    Q_PROPERTY(bool dvrUpcomingLoading READ dvrUpcomingLoading NOTIFY dvrUpcomingLoadingChanged)
    Q_PROPERTY(QString dvrReadinessState READ dvrReadinessState NOTIFY dvrReadinessStateChanged)
    Q_PROPERTY(QString dvrRecordingsState READ dvrRecordingsState NOTIFY dvrRecordingsStateChanged)
    Q_PROPERTY(QString dvrUpcomingState READ dvrUpcomingState NOTIFY dvrUpcomingStateChanged)
    Q_PROPERTY(bool dvrBannerVisible READ dvrBannerVisible NOTIFY dvrBannerChanged)
    Q_PROPERTY(QString dvrBannerSeverity READ dvrBannerSeverity NOTIFY dvrBannerChanged)
    Q_PROPERTY(QString dvrBannerTitle READ dvrBannerTitle NOTIFY dvrBannerChanged)
    Q_PROPERTY(QString dvrBannerMessage READ dvrBannerMessage NOTIFY dvrBannerChanged)
    Q_PROPERTY(QVariantList dvrRecordingGroups READ dvrRecordingGroups NOTIFY dvrRecordingGroupsChanged)
    Q_PROPERTY(QString selectedRecordingId READ selectedRecordingId NOTIFY selectedRecordingIdChanged)
    Q_PROPERTY(QVariantMap selectedRecordingDetails READ selectedRecordingDetails NOTIFY selectedRecordingDetailsChanged)
    Q_PROPERTY(QVariantList dvrUpcomingEntries READ dvrUpcomingEntries NOTIFY dvrUpcomingEntriesChanged)
    Q_PROPERTY(bool dvrRuleEditorVisible READ dvrRuleEditorVisible NOTIFY dvrRuleEditorVisibleChanged)
    Q_PROPERTY(QString dvrRuleEditorTitle READ dvrRuleEditorTitle NOTIFY dvrRuleEditorChanged)
    Q_PROPERTY(QString dvrRuleEditorMessage READ dvrRuleEditorMessage NOTIFY dvrRuleEditorChanged)
    Q_PROPERTY(bool dvrRuleEditorCanCreateSeries READ dvrRuleEditorCanCreateSeries NOTIFY dvrRuleEditorChanged)
    Q_PROPERTY(bool dvrRuleEditorCanCreateOneTime READ dvrRuleEditorCanCreateOneTime NOTIFY dvrRuleEditorChanged)
    Q_PROPERTY(bool dvrRuleEditorSubmitting READ dvrRuleEditorSubmitting NOTIFY dvrRuleEditorSubmittingChanged)
    Q_PROPERTY(bool dvrDeleteDialogVisible READ dvrDeleteDialogVisible NOTIFY dvrDeleteDialogVisibleChanged)
    Q_PROPERTY(QString dvrDeleteDialogTitle READ dvrDeleteDialogTitle NOTIFY dvrDeleteDialogTitleChanged)
    Q_PROPERTY(bool dvrDeleteDialogBusy READ dvrDeleteDialogBusy NOTIFY dvrDeleteDialogBusyChanged)

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
    QString workspaceMode() const;
    QString playbackMode() const;
    QString currentRecordingId() const;
    bool embeddedPlaybackEnabled() const;
    bool diagnosticsExpanded() const;
    QString diagnosticsSummary() const;
    QVariantList diagnosticsRows() const;
    bool dvrLoading() const;
    bool dvrUpcomingLoading() const;
    QString dvrReadinessState() const;
    QString dvrRecordingsState() const;
    QString dvrUpcomingState() const;
    bool dvrBannerVisible() const;
    QString dvrBannerSeverity() const;
    QString dvrBannerTitle() const;
    QString dvrBannerMessage() const;
    QVariantList dvrRecordingGroups() const;
    QString selectedRecordingId() const;
    QVariantMap selectedRecordingDetails() const;
    QVariantList dvrUpcomingEntries() const;
    bool dvrRuleEditorVisible() const;
    QString dvrRuleEditorTitle() const;
    QString dvrRuleEditorMessage() const;
    bool dvrRuleEditorCanCreateSeries() const;
    bool dvrRuleEditorCanCreateOneTime() const;
    bool dvrRuleEditorSubmitting() const;
    bool dvrDeleteDialogVisible() const;
    QString dvrDeleteDialogTitle() const;
    bool dvrDeleteDialogBusy() const;

    Q_INVOKABLE void initialize();
    Q_INVOKABLE void selectDeviceIndex(int index);
    Q_INVOKABLE void playChannel(const QString &channelRef);
    Q_INVOKABLE void playAdjacentChannel(int direction);
    Q_INVOKABLE void toggleGuide();
    Q_INVOKABLE void shiftGuideWindow(int deltaHours);
    Q_INVOKABLE void jumpGuideToNow();
    Q_INVOKABLE void retryPlayback();
    Q_INVOKABLE void stopPlayback();
    Q_INVOKABLE void toggleDiagnostics();
    Q_INVOKABLE void refreshDvrWorkspace();
    Q_INVOKABLE void toggleRecordingGroup(const QString &groupId);
    Q_INVOKABLE void selectRecording(const QString &recordingId);
    Q_INVOKABLE void playRecording(const QString &recordingId);
    Q_INVOKABLE void requestDeleteRecording(const QString &recordingId);
    Q_INVOKABLE void confirmDeleteRecording(bool rerecord);
    Q_INVOKABLE void cancelDeleteRecording();
    Q_INVOKABLE void deleteRuleForUpcoming(const QString &programId);
    Q_INVOKABLE void openRuleEditorFromRecording(const QString &recordingId);
    Q_INVOKABLE void openRuleEditorFromUpcoming(const QString &programId);
    Q_INVOKABLE void openRuleEditorFromGuide(const QVariantMap &context);
    Q_INVOKABLE void createSeriesRuleFromGuide(const QVariantMap &context);
    Q_INVOKABLE void createOneTimeRuleFromGuide(const QVariantMap &context);
    Q_INVOKABLE void closeRuleEditor();
    Q_INVOKABLE void createSeriesRule();
    Q_INVOKABLE void createOneTimeRule();

    void setDiagnosticsExpanded(bool expanded);
    void setGuideVisible(bool visible);
    void setWorkspaceMode(const QString &mode);

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
    void workspaceModeChanged();
    void playbackModeChanged();
    void currentRecordingIdChanged();
    void embeddedPlaybackEnabledChanged();
    void diagnosticsExpandedChanged();
    void diagnosticsSummaryChanged();
    void diagnosticsRowsChanged();
    void dvrLoadingChanged();
    void dvrUpcomingLoadingChanged();
    void dvrReadinessStateChanged();
    void dvrRecordingsStateChanged();
    void dvrUpcomingStateChanged();
    void dvrBannerChanged();
    void dvrRecordingGroupsChanged();
    void selectedRecordingIdChanged();
    void selectedRecordingDetailsChanged();
    void dvrUpcomingEntriesChanged();
    void dvrRuleEditorVisibleChanged();
    void dvrRuleEditorChanged();
    void dvrRuleEditorSubmittingChanged();
    void dvrDeleteDialogVisibleChanged();
    void dvrDeleteDialogTitleChanged();
    void dvrDeleteDialogBusyChanged();

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
    void loadDvrReadiness();
    void loadDvrRecordings();
    void loadDvrUpcoming();
    void loadDvrRules();
    void loadGuide();
    void loadPlaybackCurrent();
    void loadDiagnostics();
    void refreshSelectedData();
    void rebuildDvrRecordingGroups();
    void rebuildSelectedRecordingDetails();
    void startSeriesRuleCreation(const QVariantMap &context, const QString &successMessage);
    void startOneTimeRuleCreation(const QVariantMap &context, const QString &successMessage);

    void applyDevicesResponse(const QJsonObject &payload);
    void applyGuideResponse(const QJsonObject &payload);
    void applyPlaybackResponse(const QJsonObject &payload);
    void applyDiagnosticsResponse(const QJsonObject &payload);
    void applyDvrReadinessResponse(const QJsonObject &payload);
    void applyDvrRecordingsResponse(const QJsonObject &payload);
    void applyDvrUpcomingResponse(const QJsonObject &payload);
    void applyDvrRulesResponse(const QJsonObject &payload);

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
    void setPlaybackMode(const QString &mode);
    void setCurrentRecordingId(const QString &recordingId);
    void setEmbeddedPlaybackEnabled(bool enabled);
    void setDiagnosticsSummary(const QString &summary);
    void setDiagnosticsRows(const QVariantList &rows);
    void setDvrLoading(bool loading);
    void setDvrUpcomingLoading(bool loading);
    void setDvrReadinessState(const QString &state);
    void setDvrRecordingsState(const QString &state);
    void setDvrUpcomingState(const QString &state);
    void setDvrBanner(bool visible, const QString &severity, const QString &title, const QString &message);
    void setDvrRecordingGroups(const QVariantList &groups);
    void setSelectedRecordingIdInternal(const QString &recordingId);
    void setSelectedRecordingDetails(const QVariantMap &details);
    void setDvrUpcomingEntries(const QVariantList &entries);
    void setDvrRuleEditorVisible(bool visible);
    void setDvrRuleEditorState(const QString &title, const QString &message, bool canCreateSeries, bool canCreateOneTime);
    void setDvrRuleEditorSubmitting(bool submitting);
    void setDvrDeleteDialogTitle(const QString &title);
    void setDvrDeleteDialogVisible(bool visible);
    void setDvrDeleteDialogBusy(bool busy);

    QString selectedDeviceRef() const;
    QVariantMap findRecordingById(const QString &recordingId) const;
    QVariantMap findUpcomingByProgramId(const QString &programId) const;
    QVariantMap findRuleByRecordingRuleId(const QString &recordingRuleId) const;
    void clearDvrWorkspaceState();
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
    QString m_workspaceMode;
    QString m_playbackMode;
    QString m_currentRecordingId;
    bool m_embeddedPlaybackEnabled;
    bool m_diagnosticsExpanded;
    QString m_diagnosticsSummary;
    QVariantList m_diagnosticsRows;
    bool m_dvrLoading;
    bool m_dvrUpcomingLoading;
    QString m_dvrReadinessState;
    QString m_dvrRecordingsState;
    QString m_dvrUpcomingState;
    bool m_dvrBannerVisible;
    QString m_dvrBannerSeverity;
    QString m_dvrBannerTitle;
    QString m_dvrBannerMessage;
    QVariantList m_dvrRecordings;
    QVariantList m_dvrRecordingGroups;
    QSet<QString> m_expandedRecordingGroups;
    QString m_selectedRecordingId;
    QVariantMap m_selectedRecordingDetails;
    QVariantList m_dvrUpcomingEntries;
    QVariantList m_dvrRules;
    bool m_dvrRuleEditorVisible;
    QString m_dvrRuleEditorTitle;
    QString m_dvrRuleEditorMessage;
    bool m_dvrRuleEditorCanCreateSeries;
    bool m_dvrRuleEditorCanCreateOneTime;
    bool m_dvrRuleEditorSubmitting;
    QVariantMap m_dvrRuleEditorContext;
    bool m_dvrDeleteDialogVisible;
    QString m_dvrDeleteDialogTitle;
    bool m_dvrDeleteDialogBusy;
    QString m_pendingDeleteRecordingId;
};