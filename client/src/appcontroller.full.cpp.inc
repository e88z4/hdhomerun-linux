#include "appcontroller.h"

#include "backendlaunchconfig.h"
#include "channelnavigation.h"
#include "dvrworkspacehelpers.h"

#include <QCoreApplication>
#include <QDateTime>
#include <QDebug>
#include <QJsonDocument>
#include <QNetworkReply>
#include <QNetworkRequest>
#include <QProcessEnvironment>
#include <QStandardPaths>
#include <QTimer>
#include <QUrl>

namespace {

constexpr auto kDefaultBackendUrl = "http://127.0.0.1:38080";
constexpr auto kDefaultBackendExecutable = "hdhomerun-backend";

qint64 defaultGuideWindowStart()
{
    const auto now = QDateTime::currentSecsSinceEpoch();
    return now - (now % 1800);
}

QString formatRecordingSubtitle(const QJsonObject &recording)
{
    QStringList segments;

    const auto episodeTitle = recording.value(QStringLiteral("episodeTitle")).toString();
    if (!episodeTitle.isEmpty()) {
        segments << episodeTitle;
    }

    const auto channelName = recording.value(QStringLiteral("channelName")).toString();
    if (!channelName.isEmpty()) {
        segments << channelName;
    }

    const auto recordStartTime = recording.value(QStringLiteral("recordStartTime")).toVariant().toLongLong();
    if (recordStartTime > 0) {
        segments << QDateTime::fromSecsSinceEpoch(recordStartTime).toString(QStringLiteral("MMM d h:mm AP"));
    }

    if (segments.isEmpty()) {
        return QStringLiteral("Recorded playback");
    }

    return segments.join(QStringLiteral(" • "));
}

QString firstWarning(const QJsonArray &warnings)
{
    return warnings.isEmpty() ? QString() : warnings.first().toString();
}

}

AppController::AppController(QObject *parent)
    : QObject(parent)
    , m_backendProcess(nullptr)
    , m_backendBaseUrl(qEnvironmentVariable("HDHR_BACKEND_URL", kDefaultBackendUrl))
    , m_selectedDeviceIndex(-1)
    , m_guideVisible(true)
    , m_guideLoading(false)
    , m_guideWindowStart(defaultGuideWindowStart())
    , m_guideDurationHours(24)
    , m_guideEndpointAvailable(true)
    , m_shellPhase(QStringLiteral("launching"))
    , m_workspaceMode(QStringLiteral("live"))
    , m_playbackMode(QStringLiteral("idle"))
    , m_embeddedPlaybackEnabled(false)
    , m_diagnosticsExpanded(true)
    , m_diagnosticsSummary(QStringLiteral("Diagnostics unavailable"))
    , m_dvrLoading(false)
    , m_dvrUpcomingLoading(false)
    , m_dvrReadinessState(QStringLiteral("selection_required"))
    , m_dvrRecordingsState(QStringLiteral("selection_required"))
    , m_dvrUpcomingState(QStringLiteral("selection_required"))
    , m_dvrBannerVisible(false)
    , m_dvrRuleEditorVisible(false)
    , m_dvrRuleEditorCanCreateSeries(false)
    , m_dvrRuleEditorCanCreateOneTime(false)
    , m_dvrRuleEditorSubmitting(false)
    , m_dvrDeleteDialogVisible(false)
    , m_dvrDeleteDialogBusy(false)
{
    setStageTitle(QStringLiteral("HDHomeRun Linux Player"));
    setStageSubtitle(QStringLiteral("Checking backend availability"));

    if (auto *application = QCoreApplication::instance()) {
        connect(application, &QCoreApplication::aboutToQuit, this, &AppController::shutdownBackendProcess, Qt::DirectConnection);
    }
}

AppController::~AppController()
{
    shutdownBackendProcess();
}

QVariantList AppController::devices() const { return m_devices; }
int AppController::selectedDeviceIndex() const { return m_selectedDeviceIndex; }
QVariantList AppController::channels() const { return m_channels; }
QVariantList AppController::guideChannels() const { return m_guideChannels; }
bool AppController::guideVisible() const { return m_guideVisible; }
bool AppController::guideLoading() const { return m_guideLoading; }
qint64 AppController::guideWindowStart() const { return m_guideWindowStart; }
int AppController::guideDurationHours() const { return m_guideDurationHours; }
QString AppController::shellPhase() const { return m_shellPhase; }
QString AppController::currentChannelRef() const { return m_currentChannelRef; }
QString AppController::stageTitle() const { return m_stageTitle; }
QString AppController::stageSubtitle() const { return m_stageSubtitle; }
QString AppController::stageWarning() const { return m_stageWarning; }
QString AppController::stageFailure() const { return m_stageFailure; }
QString AppController::playbackUrl() const { return m_playbackUrl; }
QString AppController::workspaceMode() const { return m_workspaceMode; }
QString AppController::playbackMode() const { return m_playbackMode; }
QString AppController::currentRecordingId() const { return m_currentRecordingId; }
bool AppController::embeddedPlaybackEnabled() const { return m_embeddedPlaybackEnabled; }
bool AppController::diagnosticsExpanded() const { return m_diagnosticsExpanded; }
QString AppController::diagnosticsSummary() const { return m_diagnosticsSummary; }
QVariantList AppController::diagnosticsRows() const { return m_diagnosticsRows; }
bool AppController::dvrLoading() const { return m_dvrLoading; }
bool AppController::dvrUpcomingLoading() const { return m_dvrUpcomingLoading; }
QString AppController::dvrReadinessState() const { return m_dvrReadinessState; }
QString AppController::dvrRecordingsState() const { return m_dvrRecordingsState; }
QString AppController::dvrUpcomingState() const { return m_dvrUpcomingState; }
bool AppController::dvrBannerVisible() const { return m_dvrBannerVisible; }
QString AppController::dvrBannerSeverity() const { return m_dvrBannerSeverity; }
QString AppController::dvrBannerTitle() const { return m_dvrBannerTitle; }
QString AppController::dvrBannerMessage() const { return m_dvrBannerMessage; }
QVariantList AppController::dvrRecordingGroups() const { return m_dvrRecordingGroups; }
QString AppController::selectedRecordingId() const { return m_selectedRecordingId; }
QVariantMap AppController::selectedRecordingDetails() const { return m_selectedRecordingDetails; }
QVariantList AppController::dvrUpcomingEntries() const { return m_dvrUpcomingEntries; }
bool AppController::dvrRuleEditorVisible() const { return m_dvrRuleEditorVisible; }
QString AppController::dvrRuleEditorTitle() const { return m_dvrRuleEditorTitle; }
QString AppController::dvrRuleEditorMessage() const { return m_dvrRuleEditorMessage; }
bool AppController::dvrRuleEditorCanCreateSeries() const { return m_dvrRuleEditorCanCreateSeries; }
bool AppController::dvrRuleEditorCanCreateOneTime() const { return m_dvrRuleEditorCanCreateOneTime; }
bool AppController::dvrRuleEditorSubmitting() const { return m_dvrRuleEditorSubmitting; }
bool AppController::dvrDeleteDialogVisible() const { return m_dvrDeleteDialogVisible; }
QString AppController::dvrDeleteDialogTitle() const { return m_dvrDeleteDialogTitle; }
bool AppController::dvrDeleteDialogBusy() const { return m_dvrDeleteDialogBusy; }

void AppController::initialize()
{
    setShellPhase(QStringLiteral("backend_wait"));
    setStageTitle(QStringLiteral("Starting TV shell"));
    setStageSubtitle(QStringLiteral("Waiting for backend readiness"));
    setStageWarning(QString());
    setStageFailure(QString());
    probeBackend(true);
}

void AppController::selectDeviceIndex(int index)
{
    if (index < 0 || index >= m_devices.size()) {
        return;
    }

    const auto device = m_devices.at(index).toMap();
    const auto deviceRef = device.value(QStringLiteral("deviceRef")).toString();
    if (deviceRef.isEmpty()) {
        return;
    }

    postJson(
        QStringLiteral("/api/devices/select"),
        QJsonObject{{QStringLiteral("deviceRef"), deviceRef}},
        [this](const QJsonObject &payload) {
            applyDevicesResponse(payload);
            setCurrentChannelRef(QString());
            setCurrentRecordingId(QString());
            setPlaybackMode(QStringLiteral("idle"));
            setStageWarning(QString());
            setStageFailure(QString());
            setStageTitle(QStringLiteral("Select a channel"));
            setStageSubtitle(QStringLiteral("The lineup has been refreshed for the selected device"));
            setShellPhase(QStringLiteral("ready"));
            refreshSelectedData();
        },
        [this](const QString &message) {
            setShellPhase(QStringLiteral("playback_failed"));
            setStageFailure(message);
        });
}

void AppController::playChannel(const QString &channelRef)
{
    const auto deviceRef = selectedDeviceRef();
    if (deviceRef.isEmpty() || channelRef.trimmed().isEmpty()) {
        return;
    }

    if (channelRef == m_currentChannelRef && m_shellPhase == QStringLiteral("playing")) {
        return;
    }

    setShellPhase(QStringLiteral("playback_loading"));
    setStageFailure(QString());
    setStageWarning(QString());

    const auto path = (!m_currentChannelRef.isEmpty() && m_currentChannelRef != channelRef)
        ? QStringLiteral("/api/playback/switch")
        : QStringLiteral("/api/playback/start");

    postJson(
        path,
        QJsonObject{{QStringLiteral("deviceRef"), deviceRef}, {QStringLiteral("channelRef"), channelRef}},
        [this](const QJsonObject &payload) {
            applyPlaybackResponse(payload);
            loadDiagnostics();
        },
        [this](const QString &message) {
            setShellPhase(QStringLiteral("playback_failed"));
            setStageFailure(message);
        });
}

void AppController::playRecording(const QString &recordingId)
{
    if (recordingId.trimmed().isEmpty()) {
        return;
    }

    setWorkspaceMode(QStringLiteral("dvr"));
    selectRecording(recordingId);
    setShellPhase(QStringLiteral("playback_loading"));
    setStageFailure(QString());
    setStageWarning(QString());

    postJson(
        QStringLiteral("/api/dvr/recordings/%1/play").arg(recordingId),
        QJsonObject(),
        [this](const QJsonObject &payload) {
            applyPlaybackResponse(payload);
            loadDiagnostics();
        },
        [this](const QString &message) {
            qWarning().noquote() << "dvr playback launch failed:" << message;
            setShellPhase(QStringLiteral("playback_failed"));
            setStageFailure(message);
            setDvrBanner(true, QStringLiteral("warning"), QStringLiteral("Playback failed"), message);
        });
}

void AppController::retryPlayback()
{
    setShellPhase(QStringLiteral("playback_loading"));
    setStageFailure(QString());

    postJson(
        QStringLiteral("/api/playback/retry"),
        QJsonObject(),
        [this](const QJsonObject &payload) {
            applyPlaybackResponse(payload);
            loadDiagnostics();
        },
        [this](const QString &message) {
            setShellPhase(QStringLiteral("playback_failed"));
            setStageFailure(message);
        });
}

void AppController::stopPlayback()
{
    postJson(
        QStringLiteral("/api/playback/stop"),
        QJsonObject(),
        [this](const QJsonObject &payload) {
            applyPlaybackResponse(payload);
            loadDiagnostics();
        },
        [this](const QString &message) {
            setStageFailure(message);
            setDvrBanner(true, QStringLiteral("warning"), QStringLiteral("Stop playback failed"), message);
        });
}

void AppController::playAdjacentChannel(int direction)
{
    const auto channelRef = findAdjacentPlayableChannelRef(m_channels, m_currentChannelRef, direction);
    if (channelRef.isEmpty()) {
        return;
    }

    playChannel(channelRef);
}

void AppController::toggleGuide()
{
    if (!m_guideEndpointAvailable) {
        setStageWarning(QStringLiteral("The connected backend does not expose guide support. Stop any older backend already running on 127.0.0.1:38080 and relaunch the packaged app."));
        return;
    }

    if (!m_guideVisible && selectedDeviceRef().isEmpty()) {
        setStageWarning(QStringLiteral("Select a device before opening the guide"));
        return;
    }

    setGuideVisible(true);
    loadGuide();
}

void AppController::shiftGuideWindow(int deltaHours)
{
    if (deltaHours == 0) {
        return;
    }

    setGuideWindowStart(m_guideWindowStart + (static_cast<qint64>(deltaHours) * 3600));
    if (m_guideVisible) {
        loadGuide();
    }
}

void AppController::jumpGuideToNow()
{
    setGuideWindowStart(defaultGuideWindowStart());
    if (m_guideVisible) {
        loadGuide();
    }
}

void AppController::toggleDiagnostics()
{
    setDiagnosticsExpanded(!m_diagnosticsExpanded);
}

void AppController::setDiagnosticsExpanded(bool expanded)
{
    if (m_diagnosticsExpanded == expanded) {
        return;
    }
    m_diagnosticsExpanded = expanded;
    emit diagnosticsExpandedChanged();
}

void AppController::setGuideVisible(bool visible)
{
    if (m_guideVisible == visible) {
        return;
    }
    m_guideVisible = visible;
    emit guideVisibleChanged();
}

void AppController::setWorkspaceMode(const QString &mode)
{
    const auto normalized = mode.trimmed().toLower() == QStringLiteral("dvr")
        ? QStringLiteral("dvr")
        : QStringLiteral("live");
    if (m_workspaceMode == normalized) {
        return;
    }

    m_workspaceMode = normalized;
    emit workspaceModeChanged();

    if (m_workspaceMode == QStringLiteral("dvr")) {
        refreshDvrWorkspace();
    }
}

void AppController::refreshDvrWorkspace()
{
    loadDvrReadiness();
    loadDvrRecordings();
    loadDvrUpcoming();
    loadDvrRules();
}

void AppController::toggleRecordingGroup(const QString &groupId)
{
    if (groupId.trimmed().isEmpty()) {
        return;
    }

    const auto selectedRecording = findRecordingById(m_selectedRecordingId);
    const bool groupContainsSelection = !selectedRecording.isEmpty()
        && DvrWorkspaceHelpers::recordingGroupId(selectedRecording) == groupId;
    const bool groupIsExpanded = m_expandedRecordingGroups.contains(groupId) || groupContainsSelection;

    if (groupIsExpanded) {
        m_expandedRecordingGroups.remove(groupId);
        if (groupContainsSelection) {
            setSelectedRecordingIdInternal(QString());
        }
    } else {
        m_expandedRecordingGroups.insert(groupId);
    }
    rebuildDvrRecordingGroups();
}

void AppController::selectRecording(const QString &recordingId)
{
    const auto recording = findRecordingById(recordingId);
    if (recording.isEmpty()) {
        return;
    }

    setSelectedRecordingIdInternal(recordingId);
    m_expandedRecordingGroups.insert(DvrWorkspaceHelpers::recordingGroupId(recording));
    rebuildDvrRecordingGroups();
    rebuildSelectedRecordingDetails();
}

void AppController::requestDeleteRecording(const QString &recordingId)
{
    const auto recording = findRecordingById(recordingId);
    if (recording.isEmpty()) {
        return;
    }

    m_pendingDeleteRecordingId = recordingId;
    const auto episodeTitle = recording.value(QStringLiteral("episodeTitle")).toString();
    const auto title = recording.value(QStringLiteral("title")).toString();
    setDvrDeleteDialogTitle(
        episodeTitle.isEmpty() ? title : QStringLiteral("%1 • %2").arg(title, episodeTitle));
    setDvrDeleteDialogBusy(false);
    setDvrDeleteDialogVisible(true);
}

void AppController::confirmDeleteRecording(bool rerecord)
{
    if (m_pendingDeleteRecordingId.isEmpty()) {
        return;
    }

    const auto recording = findRecordingById(m_pendingDeleteRecordingId);
    const auto rerecordContext = DvrWorkspaceHelpers::inferRuleEditorContextForRecording(
        recording,
        m_dvrUpcomingEntries,
        m_dvrRules);

    setDvrDeleteDialogBusy(true);

    postJson(
        QStringLiteral("/api/dvr/recordings/%1/delete").arg(m_pendingDeleteRecordingId),
        QJsonObject(),
        [this, rerecord, rerecordContext](const QJsonObject &payload) {
            setDvrDeleteDialogBusy(false);
            setDvrDeleteDialogVisible(false);
            const auto outcome = payload.value(QStringLiteral("outcome")).toString();
            const auto warnings = payload.value(QStringLiteral("warnings")).toArray();
            m_pendingDeleteRecordingId.clear();

            if (outcome == QStringLiteral("missing_recording")) {
                qWarning().noquote() << "dvr delete outcome: missing recording";
                setDvrBanner(
                    true,
                    QStringLiteral("warning"),
                    QStringLiteral("Recording changed"),
                    firstWarning(warnings).isEmpty()
                        ? QStringLiteral("The selected recording is no longer available. Refresh the DVR workspace and try again.")
                        : firstWarning(warnings));
                loadDvrRecordings();
                return;
            }

            qInfo().noquote() << "dvr delete confirmed";
            setDvrBanner(
                true,
                QStringLiteral("info"),
                QStringLiteral("Recording deleted"),
                rerecord
                    ? QStringLiteral("The recording was deleted. Preparing the recovery rule path.")
                    : QStringLiteral("The recording was deleted successfully."));
            loadDvrRecordings();
            loadDvrUpcoming();
            loadDvrRules();

            if (rerecord) {
                if (rerecordContext.value(QStringLiteral("canCreateSeries")).toBool()) {
                    startSeriesRuleCreation(
                        rerecordContext,
                        QStringLiteral("Delete complete. A replacement series rule was created."));
                } else {
                    setDvrBanner(
                        true,
                        QStringLiteral("warning"),
                        QStringLiteral("Delete completed"),
                        QStringLiteral("The recording was deleted, but a replacement series rule could not be inferred from the available DVR context."));
                }
            }
        },
        [this](const QString &message) {
            qWarning().noquote() << "dvr delete failed:" << message;
            setDvrDeleteDialogBusy(false);
            setDvrBanner(true, QStringLiteral("warning"), QStringLiteral("Delete failed"), message);
        });
}

void AppController::cancelDeleteRecording()
{
    m_pendingDeleteRecordingId.clear();
    setDvrDeleteDialogBusy(false);
    setDvrDeleteDialogVisible(false);
}

void AppController::deleteRuleForUpcoming(const QString &programId)
{
    const auto upcoming = findUpcomingByProgramId(programId);
    if (upcoming.isEmpty()) {
        return;
    }

    const auto recordingRuleId = upcoming.value(QStringLiteral("recordingRuleId")).toString();
    if (recordingRuleId.isEmpty()) {
        setDvrBanner(
            true,
            QStringLiteral("warning"),
            QStringLiteral("Delete rule unavailable"),
            QStringLiteral("This upcoming item does not currently expose a recording rule id."));
        return;
    }

    const auto rule = findRuleByRecordingRuleId(recordingRuleId);
    const auto ruleKind = rule.value(QStringLiteral("kind")).toString();
    const auto title = upcoming.value(QStringLiteral("title")).toString();

    postJson(
        QStringLiteral("/api/dvr/rules/%1/delete").arg(recordingRuleId),
        QJsonObject(),
        [this, ruleKind, title](const QJsonObject &payload) {
            const auto outcome = payload.value(QStringLiteral("outcome")).toString();
            const auto warnings = payload.value(QStringLiteral("warnings")).toArray();
            if (outcome == QStringLiteral("missing_rule")) {
                setDvrBanner(
                    true,
                    QStringLiteral("warning"),
                    QStringLiteral("Recording rule changed"),
                    firstWarning(warnings).isEmpty()
                        ? QStringLiteral("The selected recording rule is no longer available. Refresh the DVR workspace and try again.")
                        : firstWarning(warnings));
                loadDvrRules();
                loadDvrUpcoming();
                return;
            }

            const auto successMessage = ruleKind == QStringLiteral("series")
                ? QStringLiteral("The series rule for %1 was deleted. Future scheduled airings for that rule will disappear after refresh.").arg(title)
                : QStringLiteral("The one-time recording rule for %1 was deleted.").arg(title);
            setDvrBanner(
                true,
                QStringLiteral("info"),
                QStringLiteral("Recording rule deleted"),
                successMessage);
            loadDvrRules();
            loadDvrUpcoming();
        },
        [this](const QString &message) {
            qWarning().noquote() << "dvr rule delete failed:" << message;
            setDvrBanner(true, QStringLiteral("warning"), QStringLiteral("Recording rule delete failed"), message);
        });
}

void AppController::openRuleEditorFromRecording(const QString &recordingId)
{
    const auto recording = findRecordingById(recordingId);
    if (recording.isEmpty()) {
        return;
    }

    m_dvrRuleEditorContext = DvrWorkspaceHelpers::inferRuleEditorContextForRecording(
        recording,
        m_dvrUpcomingEntries,
        m_dvrRules);

    const auto title = recording.value(QStringLiteral("title")).toString();
    setDvrRuleEditorState(
        QStringLiteral("Rule options for %1").arg(title),
        m_dvrRuleEditorContext.value(QStringLiteral("message")).toString(),
        m_dvrRuleEditorContext.value(QStringLiteral("canCreateSeries")).toBool(),
        false);
    setDvrRuleEditorVisible(true);
}

void AppController::openRuleEditorFromUpcoming(const QString &programId)
{
    const auto upcoming = findUpcomingByProgramId(programId);
    if (upcoming.isEmpty()) {
        return;
    }

    m_dvrRuleEditorContext = upcoming;
    m_dvrRuleEditorContext.insert(QStringLiteral("entrySource"), QStringLiteral("upcoming_item"));
    m_dvrRuleEditorContext.insert(QStringLiteral("canCreateSeries"), true);
    m_dvrRuleEditorContext.insert(QStringLiteral("canCreateOneTime"), true);

    const auto title = upcoming.value(QStringLiteral("title")).toString();
    const auto channelNumber = upcoming.value(QStringLiteral("channelNumber")).toString();
    const auto channelName = upcoming.value(QStringLiteral("channelName")).toString();
    const auto startTime = upcoming.value(QStringLiteral("startTime")).toLongLong();
    const auto when = startTime > 0
        ? QDateTime::fromSecsSinceEpoch(startTime).toString(QStringLiteral("MMM d h:mm AP"))
        : QStringLiteral("scheduled airing");
    setDvrRuleEditorState(
        QStringLiteral("Schedule %1").arg(title),
        QStringLiteral("Create a series rule or a one-time rule for %1 on %2 %3 at %4.")
            .arg(title, channelNumber, channelName, when),
        true,
        true);
    setDvrRuleEditorVisible(true);
}

void AppController::openRuleEditorFromGuide(const QVariantMap &context)
{
    const auto title = context.value(QStringLiteral("title")).toString();
    const auto seriesId = context.value(QStringLiteral("seriesId")).toString();
    const auto channelNumber = context.value(QStringLiteral("channelNumber")).toString();
    const auto channelName = context.value(QStringLiteral("channelName")).toString();
    const auto startTime = context.value(QStringLiteral("startTime")).toLongLong();
    const auto when = startTime > 0
        ? QDateTime::fromSecsSinceEpoch(startTime).toString(QStringLiteral("MMM d h:mm AP"))
        : QStringLiteral("scheduled airing");
    const bool canCreateSeries = !seriesId.isEmpty();
    const bool canCreateOneTime = canCreateSeries && !channelNumber.isEmpty() && startTime > 0;

    m_dvrRuleEditorContext = context;
    m_dvrRuleEditorContext.insert(QStringLiteral("entrySource"), QStringLiteral("guide_item"));
    m_dvrRuleEditorContext.insert(QStringLiteral("canCreateSeries"), canCreateSeries);
    m_dvrRuleEditorContext.insert(QStringLiteral("canCreateOneTime"), canCreateOneTime);

    const auto message = canCreateSeries
        ? QStringLiteral("Create a series rule or a one-time rule for %1 on %2 %3 at %4.")
              .arg(title, channelNumber, channelName, when)
        : QStringLiteral("This guide item does not currently expose DVR series metadata, so recording-rule actions are unavailable.");
    setDvrRuleEditorState(
        QStringLiteral("Schedule %1").arg(title),
        message,
        canCreateSeries,
        canCreateOneTime);
    setDvrRuleEditorVisible(true);
}

void AppController::createSeriesRuleFromGuide(const QVariantMap &context)
{
    startSeriesRuleCreation(
        context,
        QStringLiteral("Series rule created successfully."));
}

void AppController::createOneTimeRuleFromGuide(const QVariantMap &context)
{
    startOneTimeRuleCreation(
        context,
        QStringLiteral("One-time rule created successfully."));
}

void AppController::closeRuleEditor()
{
    m_dvrRuleEditorContext.clear();
    setDvrRuleEditorSubmitting(false);
    setDvrRuleEditorVisible(false);
}

void AppController::createSeriesRule()
{
    startSeriesRuleCreation(
        m_dvrRuleEditorContext,
        QStringLiteral("Series rule created successfully."));
}

void AppController::createOneTimeRule()
{
    startOneTimeRuleCreation(
        m_dvrRuleEditorContext,
        QStringLiteral("One-time rule created successfully."));
}

void AppController::getJson(const QString &path, const SuccessHandler &onSuccess, const ErrorHandler &onError)
{
    auto *reply = m_network.get(QNetworkRequest(QUrl(m_backendBaseUrl + path)));
    connect(reply, &QNetworkReply::finished, this, [this, reply, onSuccess, onError]() {
        handleJsonReply(reply, onSuccess, onError);
    });
}

void AppController::postJson(const QString &path, const QJsonObject &payload, const SuccessHandler &onSuccess, const ErrorHandler &onError)
{
    QNetworkRequest request(QUrl(m_backendBaseUrl + path));
    request.setHeader(QNetworkRequest::ContentTypeHeader, QStringLiteral("application/json"));
    auto *reply = m_network.post(request, QJsonDocument(payload).toJson(QJsonDocument::Compact));
    connect(reply, &QNetworkReply::finished, this, [this, reply, onSuccess, onError]() {
        handleJsonReply(reply, onSuccess, onError);
    });
}

void AppController::handleJsonReply(QObject *replyObject, const SuccessHandler &onSuccess, const ErrorHandler &onError)
{
    auto *reply = qobject_cast<QNetworkReply *>(replyObject);
    if (!reply) {
        onError(QStringLiteral("internal client error while processing backend response"));
        return;
    }

    const auto bytes = reply->readAll();

    if (reply->error() != QNetworkReply::NoError) {
        const auto document = QJsonDocument::fromJson(bytes);
        if (document.isObject()) {
            const auto message = document.object().value(QStringLiteral("message")).toString();
            if (!message.isEmpty()) {
                reply->deleteLater();
                onError(message);
                return;
            }
        }

        reply->deleteLater();
        onError(reply->errorString());
        return;
    }

    const auto document = QJsonDocument::fromJson(bytes);
    if (!document.isObject()) {
        reply->deleteLater();
        onError(QStringLiteral("backend returned an unexpected response shape"));
        return;
    }

    reply->deleteLater();
    onSuccess(document.object());
}

void AppController::probeBackend(bool allowStart)
{
    getJson(
        QStringLiteral("/api/health"),
        [this](const QJsonObject &) { loadBootstrap(); },
        [this, allowStart](const QString &) {
            if (allowStart) {
                startBundledBackend();
                return;
            }

            setLaunchFailure(QStringLiteral("The backend did not become ready in time."));
        });
}

void AppController::waitForBackend(int remainingAttempts)
{
    getJson(
        QStringLiteral("/api/health"),
        [this](const QJsonObject &) { loadBootstrap(); },
        [this, remainingAttempts](const QString &) {
            if (remainingAttempts <= 0) {
                setLaunchFailure(QStringLiteral("The backend could not be reached after startup was attempted."));
                return;
            }

            QTimer::singleShot(300, this, [this, remainingAttempts]() { waitForBackend(remainingAttempts - 1); });
        });
}

void AppController::startBundledBackend()
{
    if (m_backendProcess) {
        waitForBackend(15);
        return;
    }

    const auto explicitCommand = qEnvironmentVariable("HDHR_BACKEND_CMD");
    const auto program = explicitCommand.isEmpty()
        ? QStandardPaths::findExecutable(QString::fromUtf8(kDefaultBackendExecutable))
        : explicitCommand;

    if (program.isEmpty()) {
        setLaunchFailure(QStringLiteral("Backend is unavailable and no hdhomerun-backend executable was found. Set HDHR_BACKEND_CMD or install the backend."));
        return;
    }

    const auto launchDecision = resolveBackendLaunchDecision(m_backendBaseUrl);
    if (!launchDecision.canAutoStart) {
        setLaunchFailure(launchDecision.errorMessage);
        return;
    }

    m_backendProcess = new QProcess(this);
    auto environment = QProcessEnvironment::systemEnvironment();
    environment.insert(QStringLiteral("HDHR_BACKEND_PLAYER_MODE"), QStringLiteral("client"));
    environment.insert(QStringLiteral("HDHR_BACKEND_BIND"), launchDecision.bindAddress);
    m_backendProcess->setProcessEnvironment(environment);
    m_backendProcess->setProgram(program);
    m_backendProcess->start();

    if (!m_backendProcess->waitForStarted(1000)) {
        setLaunchFailure(QStringLiteral("Backend startup command was found, but the process could not be started."));
        return;
    }

    waitForBackend(15);
}

void AppController::shutdownBackendProcess()
{
    if (!m_backendProcess) {
        return;
    }

    auto *process = m_backendProcess;
    m_backendProcess = nullptr;

    disconnect(process, nullptr, this, nullptr);

    if (process->state() != QProcess::NotRunning) {
        process->terminate();
        if (!process->waitForFinished(1500)) {
            process->kill();
            process->waitForFinished(1000);
        }
    }

    delete process;
}

void AppController::loadBootstrap()
{
    setShellPhase(QStringLiteral("restoring_context"));
    setStageTitle(QStringLiteral("Restoring last context"));
    setStageSubtitle(QStringLiteral("Loading devices, channels, playback state"));

    getJson(
        QStringLiteral("/api/bootstrap"),
        [this](const QJsonObject &payload) {
            bool guideEndpointAvailable = false;
            const auto endpoints = payload.value(QStringLiteral("availableContractEndpoints")).toArray();
            for (const auto &value : endpoints) {
                const auto endpoint = value.toObject();
                if (endpoint.value(QStringLiteral("name")).toString() == QStringLiteral("guide")) {
                    guideEndpointAvailable = true;
                    break;
                }
            }
            m_guideEndpointAvailable = guideEndpointAvailable;

            auto warning = firstWarning(payload.value(QStringLiteral("warnings")).toArray());
            if (!m_guideEndpointAvailable) {
                warning = QStringLiteral("The connected backend is older than this client and does not expose guide support. Stop any backend already running on 127.0.0.1:38080 and relaunch the packaged app.");
            }

            setStageWarning(warning);
            loadDevices();
            loadPlaybackCurrent();
        },
        [this](const QString &message) { setLaunchFailure(message); });
}

void AppController::loadDevices()
{
    getJson(
        QStringLiteral("/api/devices"),
        [this](const QJsonObject &payload) {
            applyDevicesResponse(payload);
            refreshSelectedData();
        },
        [this](const QString &message) { setLaunchFailure(message); });
}

void AppController::loadLineup()
{
    getJson(
        QStringLiteral("/api/lineup"),
        [this](const QJsonObject &payload) {
            const auto channels = payload.value(QStringLiteral("channels")).toArray();
            bool hasGuideTitles = false;
            for (const auto &value : channels) {
                const auto channel = value.toObject();
                if (channel.contains(QStringLiteral("currentProgramTitle"))) {
                    hasGuideTitles = true;
                    break;
                }
            }

            setChannels(jsonArrayToVariantList(channels));

            if (m_channels.isEmpty() && !selectedDeviceRef().isEmpty()) {
                setStageTitle(QStringLiteral("No channels available"));
                setStageSubtitle(QStringLiteral("The selected device returned an empty or unavailable lineup"));
            }

            const auto warning = firstWarning(payload.value(QStringLiteral("warnings")).toArray());
            if (!warning.isEmpty()) {
                setStageWarning(warning);
            } else if (!hasGuideTitles) {
                setStageWarning(QStringLiteral("The connected backend returned channels without guide metadata. Stop any older backend already running on 127.0.0.1:38080 and relaunch the packaged app."));
            }

            if (m_guideVisible) {
                loadGuide();
            }
        },
        [this](const QString &message) {
            setChannels({});
            setStageWarning(message);
            setGuideChannels({});
        });
}

void AppController::loadDvrReadiness()
{
    if (selectedDeviceRef().isEmpty()) {
        setDvrReadinessState(QStringLiteral("selection_required"));
        setDvrBanner(
            true,
            QStringLiteral("info"),
            QStringLiteral("Select a device"),
            QStringLiteral("Select an HDHomeRun device to load DVR recordings and upcoming schedule state."));
        return;
    }

    getJson(
        QStringLiteral("/api/dvr/readiness"),
        [this](const QJsonObject &payload) { applyDvrReadinessResponse(payload); },
        [this](const QString &message) {
            qWarning().noquote() << "dvr readiness load failed:" << message;
            setDvrReadinessState(QStringLiteral("not_ready"));
            setDvrBanner(true, QStringLiteral("blocking"), QStringLiteral("DVR unavailable"), message);
        });
}

void AppController::loadDvrRecordings()
{
    if (selectedDeviceRef().isEmpty()) {
        setDvrLoading(false);
        setDvrRecordingsState(QStringLiteral("selection_required"));
        setDvrRecordingGroups({});
        setSelectedRecordingIdInternal(QString());
        setSelectedRecordingDetails({});
        return;
    }

    setDvrLoading(true);
    getJson(
        QStringLiteral("/api/dvr/recordings"),
        [this](const QJsonObject &payload) { applyDvrRecordingsResponse(payload); },
        [this](const QString &message) {
            qWarning().noquote() << "dvr recordings load failed:" << message;
            setDvrLoading(false);
            setDvrRecordingsState(QStringLiteral("unavailable"));
            m_dvrRecordings.clear();
            rebuildDvrRecordingGroups();
            setDvrBanner(true, QStringLiteral("warning"), QStringLiteral("Recordings unavailable"), message);
        });
}

void AppController::loadDvrUpcoming()
{
    if (selectedDeviceRef().isEmpty()) {
        setDvrUpcomingLoading(false);
        setDvrUpcomingState(QStringLiteral("selection_required"));
        setDvrUpcomingEntries({});
        return;
    }

    setDvrUpcomingLoading(true);
    getJson(
        QStringLiteral("/api/dvr/upcoming"),
        [this](const QJsonObject &payload) { applyDvrUpcomingResponse(payload); },
        [this](const QString &message) {
            qWarning().noquote() << "dvr upcoming load failed:" << message;
            setDvrUpcomingLoading(false);
            setDvrUpcomingState(QStringLiteral("unavailable"));
            setDvrUpcomingEntries({});
            setDvrBanner(true, QStringLiteral("warning"), QStringLiteral("Upcoming schedule unavailable"), message);
        });
}

void AppController::loadDvrRules()
{
    if (selectedDeviceRef().isEmpty()) {
        m_dvrRules.clear();
        return;
    }

    getJson(
        QStringLiteral("/api/dvr/rules"),
        [this](const QJsonObject &payload) { applyDvrRulesResponse(payload); },
        [this](const QString &message) {
            qWarning().noquote() << "dvr rules load failed:" << message;
            m_dvrRules.clear();
        });
}

void AppController::loadGuide()
{
    if (selectedDeviceRef().isEmpty()) {
        setGuideLoading(false);
        setGuideChannels({});
        return;
    }

    setGuideLoading(true);

    getJson(
        QStringLiteral("/api/guide?start=%1&durationHours=%2")
            .arg(m_guideWindowStart)
            .arg(m_guideDurationHours),
        [this](const QJsonObject &payload) { applyGuideResponse(payload); },
        [this](const QString &message) {
            setGuideLoading(false);
            setGuideChannels({});
            setStageWarning(message);
        });
}

void AppController::loadPlaybackCurrent()
{
    getJson(
        QStringLiteral("/api/playback/current"),
        [this](const QJsonObject &payload) { applyPlaybackResponse(payload); },
        [this](const QString &message) { setStageWarning(message); });
}

void AppController::loadDiagnostics()
{
    getJson(
        QStringLiteral("/api/tuners"),
        [this](const QJsonObject &payload) { applyDiagnosticsResponse(payload); },
        [this](const QString &message) {
            setDiagnosticsSummary(QStringLiteral("Diagnostics unavailable"));
            setDiagnosticsRows({});
            setStageWarning(message);
        });
}

void AppController::refreshSelectedData()
{
    if (selectedDeviceRef().isEmpty()) {
        setChannels({});
        setGuideChannels({});
        setGuideLoading(false);
        setDiagnosticsRows({});
        setDiagnosticsSummary(QStringLiteral("Select a device to load diagnostics"));
        clearDvrWorkspaceState();
        if (!m_devices.isEmpty()) {
            setShellPhase(QStringLiteral("device_selection"));
            setStageTitle(QStringLiteral("Choose a device"));
            setStageSubtitle(QStringLiteral("Select an HDHomeRun device to load the guide"));
        } else {
            setShellPhase(QStringLiteral("device_selection"));
            setStageTitle(QStringLiteral("No devices found"));
            setStageSubtitle(QStringLiteral("Retry after the backend discovers a reachable HDHomeRun tuner"));
        }
        return;
    }

    loadLineup();
    loadDiagnostics();
    refreshDvrWorkspace();
}

void AppController::rebuildDvrRecordingGroups()
{
    QVariantList groups = DvrWorkspaceHelpers::buildRecordingGroups(
        m_dvrRecordings,
        m_expandedRecordingGroups,
        m_selectedRecordingId);

    bool selectionExists = !findRecordingById(m_selectedRecordingId).isEmpty();
    if (!selectionExists) {
        setSelectedRecordingIdInternal(QString());
    }

    setDvrRecordingGroups(groups);
    rebuildSelectedRecordingDetails();
}

void AppController::rebuildSelectedRecordingDetails()
{
    const auto recording = findRecordingById(m_selectedRecordingId);
    if (recording.isEmpty()) {
        setSelectedRecordingDetails({});
        return;
    }

    QVariantMap details = recording;
    const auto inferredContext = DvrWorkspaceHelpers::inferRuleEditorContextForRecording(
        recording,
        m_dvrUpcomingEntries,
        m_dvrRules);
    details.insert(
        QStringLiteral("canCreateSeriesRule"),
        inferredContext.value(QStringLiteral("canCreateSeries")).toBool());
    details.insert(
        QStringLiteral("ruleEditorMessage"),
        inferredContext.value(QStringLiteral("message")).toString());
    details.insert(
        QStringLiteral("isCurrentPlayback"),
        m_playbackMode == QStringLiteral("recorded") && m_currentRecordingId == m_selectedRecordingId);
    setSelectedRecordingDetails(details);
}

void AppController::startSeriesRuleCreation(const QVariantMap &context, const QString &successMessage)
{
    const auto seriesId = context.value(QStringLiteral("seriesId")).toString();
    if (seriesId.isEmpty()) {
        setDvrBanner(
            true,
            QStringLiteral("warning"),
            QStringLiteral("Series rule unavailable"),
            QStringLiteral("A unique series context is not available for this DVR item yet."));
        return;
    }

    QJsonObject payload{{QStringLiteral("seriesId"), seriesId}, {QStringLiteral("options"), QJsonObject()}};
    const auto title = context.value(QStringLiteral("title")).toString();
    if (!title.isEmpty()) {
        payload.insert(QStringLiteral("title"), title);
    }

    setDvrRuleEditorSubmitting(true);
    postJson(
        QStringLiteral("/api/dvr/rules/series"),
        payload,
        [this, successMessage](const QJsonObject &payloadObject) {
            setDvrRuleEditorSubmitting(false);
            const auto outcome = payloadObject.value(QStringLiteral("outcome")).toString();
            if (outcome == QStringLiteral("invalid_airing")) {
                setDvrBanner(
                    true,
                    QStringLiteral("warning"),
                    QStringLiteral("Rule creation rejected"),
                    QStringLiteral("The backend rejected this rule context as invalid."));
                return;
            }

            qInfo().noquote() << "dvr series rule created";
            m_dvrRules = jsonArrayToVariantList(payloadObject.value(QStringLiteral("rules")).toArray());
            rebuildSelectedRecordingDetails();
            setDvrRuleEditorVisible(false);
            setDvrBanner(true, QStringLiteral("info"), QStringLiteral("Series rule created"), successMessage);
            loadDvrRules();
            loadDvrUpcoming();
        },
        [this](const QString &message) {
            qWarning().noquote() << "dvr series rule creation failed:" << message;
            setDvrRuleEditorSubmitting(false);
            setDvrBanner(true, QStringLiteral("warning"), QStringLiteral("Series rule failed"), message);
        });
}

void AppController::startOneTimeRuleCreation(const QVariantMap &context, const QString &successMessage)
{
    const auto seriesId = context.value(QStringLiteral("seriesId")).toString();
    const auto channelNumber = context.value(QStringLiteral("channelNumber")).toString();
    const auto startTime = context.value(QStringLiteral("startTime")).toLongLong();
    if (seriesId.isEmpty() || channelNumber.isEmpty() || startTime <= 0) {
        setDvrBanner(
            true,
            QStringLiteral("warning"),
            QStringLiteral("One-time rule unavailable"),
            QStringLiteral("A concrete airing is required before creating a one-time DVR rule."));
        return;
    }

    QJsonObject payload{
        {QStringLiteral("seriesId"), seriesId},
        {QStringLiteral("channelNumber"), channelNumber},
        {QStringLiteral("startTime"), startTime},
        {QStringLiteral("options"), QJsonObject()},
    };
    const auto title = context.value(QStringLiteral("title")).toString();
    if (!title.isEmpty()) {
        payload.insert(QStringLiteral("title"), title);
    }

    setDvrRuleEditorSubmitting(true);
    postJson(
        QStringLiteral("/api/dvr/rules/one-time"),
        payload,
        [this, successMessage](const QJsonObject &payloadObject) {
            setDvrRuleEditorSubmitting(false);
            const auto outcome = payloadObject.value(QStringLiteral("outcome")).toString();
            if (outcome == QStringLiteral("invalid_airing")) {
                setDvrBanner(
                    true,
                    QStringLiteral("warning"),
                    QStringLiteral("Rule creation rejected"),
                    QStringLiteral("The backend rejected this airing as invalid for one-time recording."));
                return;
            }

            qInfo().noquote() << "dvr one-time rule created";
            m_dvrRules = jsonArrayToVariantList(payloadObject.value(QStringLiteral("rules")).toArray());
            rebuildSelectedRecordingDetails();
            setDvrRuleEditorVisible(false);
            setDvrBanner(true, QStringLiteral("info"), QStringLiteral("One-time rule created"), successMessage);
            loadDvrRules();
            loadDvrUpcoming();
        },
        [this](const QString &message) {
            qWarning().noquote() << "dvr one-time rule creation failed:" << message;
            setDvrRuleEditorSubmitting(false);
            setDvrBanner(true, QStringLiteral("warning"), QStringLiteral("One-time rule failed"), message);
        });
}

void AppController::applyDevicesResponse(const QJsonObject &payload)
{
    const auto devicesArray = payload.value(QStringLiteral("devices")).toArray();
    setDevices(jsonArrayToVariantList(devicesArray));

    const auto selectedRef = payload.value(QStringLiteral("selectedDeviceRef")).toString();
    int selectedIndex = -1;
    for (int index = 0; index < m_devices.size(); ++index) {
        const auto item = m_devices.at(index).toMap();
        if (item.value(QStringLiteral("deviceRef")).toString() == selectedRef) {
            selectedIndex = index;
            break;
        }
    }
    setSelectedDeviceIndex(selectedIndex);

    const auto warning = firstWarning(payload.value(QStringLiteral("warnings")).toArray());
    if (!warning.isEmpty()) {
        setStageWarning(warning);
    }
}

void AppController::applyGuideResponse(const QJsonObject &payload)
{
    setGuideLoading(false);

    setGuideWindowStart(payload.value(QStringLiteral("windowStart")).toVariant().toLongLong());
    if (payload.contains(QStringLiteral("durationHours"))) {
        setGuideDurationHours(payload.value(QStringLiteral("durationHours")).toInt());
    }

    const auto state = payload.value(QStringLiteral("state")).toString();
    if (state == QStringLiteral("ready")) {
        setGuideChannels(jsonArrayToVariantList(payload.value(QStringLiteral("channels")).toArray()));
    } else {
        setGuideChannels({});
    }

    const auto warning = firstWarning(payload.value(QStringLiteral("warnings")).toArray());
    if (!warning.isEmpty()) {
        setStageWarning(warning);
    }
}

void AppController::applyPlaybackResponse(const QJsonObject &payload)
{
    const auto session = payload.value(QStringLiteral("sessionState")).toObject();
    const auto currentChannel = payload.value(QStringLiteral("currentChannel")).toObject();
    const auto currentRecording = payload.value(QStringLiteral("currentRecording")).toObject();
    const auto adapterState = payload.value(QStringLiteral("adapterState")).toObject();
    const auto failure = payload.value(QStringLiteral("failure")).toObject();
    const auto warnings = payload.value(QStringLiteral("warnings")).toArray();
    const auto sessionStatus = session.value(QStringLiteral("status")).toString();
    const auto sessionPlaybackMode = session.value(QStringLiteral("playbackMode")).toString();
    const auto adapterCommand = adapterState.value(QStringLiteral("lastCommand")).toString();
    const auto playbackUrl = session.value(QStringLiteral("playbackUrl")).toString();
    const bool playbackActive = sessionStatus == QStringLiteral("playing")
        || sessionStatus == QStringLiteral("starting")
        || sessionStatus == QStringLiteral("retrying_start")
        || sessionStatus == QStringLiteral("switching");

    setEmbeddedPlaybackEnabled(adapterCommand.startsWith(QStringLiteral("client_managed")));
    setPlaybackUrl(playbackUrl);

    if (playbackActive && sessionPlaybackMode == QStringLiteral("recorded") && !currentRecording.isEmpty()) {
        const auto recordingId = currentRecording.value(QStringLiteral("recordingId")).toString();
        setCurrentChannelRef(QString());
        setCurrentRecordingId(recordingId);
        setPlaybackMode(QStringLiteral("recorded"));
        setStageTitle(currentRecording.value(QStringLiteral("title")).toString());
        setStageSubtitle(formatRecordingSubtitle(currentRecording));
        if (m_workspaceMode == QStringLiteral("dvr") && !recordingId.isEmpty()) {
            setSelectedRecordingIdInternal(recordingId);
            rebuildDvrRecordingGroups();
        }
    } else if ((playbackActive || sessionStatus == QStringLiteral("stopped"))
               && (sessionPlaybackMode == QStringLiteral("live") || !currentChannel.isEmpty())
               && !currentChannel.isEmpty()) {
        setCurrentRecordingId(QString());
        setPlaybackMode(playbackActive ? QStringLiteral("live") : QStringLiteral("idle"));
        setCurrentChannelRef(currentChannel.value(QStringLiteral("channelRef")).toString());
        if (sessionStatus == QStringLiteral("stopped")) {
            setStageTitle(QStringLiteral("Playback stopped"));
            setStageSubtitle(
                QStringLiteral("Channel %1 • Ready to resume live playback")
                    .arg(currentChannel.value(QStringLiteral("guideNumber")).toString()));
        } else {
            setStageTitle(currentChannel.value(QStringLiteral("guideName")).toString());
            setStageSubtitle(QStringLiteral("Channel %1 • Persistent playback session")
                                 .arg(currentChannel.value(QStringLiteral("guideNumber")).toString()));
        }
    } else if (sessionStatus == QStringLiteral("stopped")) {
        setCurrentChannelRef(QString());
        setCurrentRecordingId(QString());
        setPlaybackMode(QStringLiteral("idle"));
        setStageTitle(QStringLiteral("Playback stopped"));
        setStageSubtitle(m_workspaceMode == QStringLiteral("dvr")
                ? QStringLiteral("Select a recording to start playback")
                : QStringLiteral("Use the guide to start live playback"));
    } else if (!selectedDeviceRef().isEmpty()) {
        setCurrentChannelRef(QString());
        setCurrentRecordingId(QString());
        setPlaybackMode(QStringLiteral("idle"));
        setStageTitle(QStringLiteral("Select a channel"));
        setStageSubtitle(QStringLiteral("Use the guide to start live playback"));
    } else {
        setCurrentChannelRef(QString());
        setCurrentRecordingId(QString());
        setPlaybackMode(QStringLiteral("idle"));
    }

    if (!failure.isEmpty()) {
        setShellPhase(QStringLiteral("playback_failed"));
        setStageFailure(failure.value(QStringLiteral("message")).toString());
    } else if (sessionStatus == QStringLiteral("playing")) {
        setShellPhase(QStringLiteral("playing"));
        setStageFailure(QString());
    } else if (sessionStatus == QStringLiteral("starting")
               || sessionStatus == QStringLiteral("retrying_start")
               || sessionStatus == QStringLiteral("switching")) {
        setShellPhase(QStringLiteral("playback_loading"));
        setStageFailure(QString());
    } else if (selectedDeviceRef().isEmpty()) {
        setShellPhase(QStringLiteral("device_selection"));
    } else {
        setShellPhase(QStringLiteral("ready"));
        setStageFailure(QString());
    }

    if (!playbackUrl.isEmpty() && !m_embeddedPlaybackEnabled) {
        setStageWarning(QStringLiteral("The backend is using external player mode. Embedded playback is available when the client launches the backend in client mode."));
    } else {
        setStageWarning(firstWarning(warnings));
    }

    rebuildSelectedRecordingDetails();
}

void AppController::applyDiagnosticsResponse(const QJsonObject &payload)
{
    const auto tuners = payload.value(QStringLiteral("tuners")).toArray();
    QVariantList activeRows;
    QVariantList inactiveRows;
    for (const auto &value : tuners) {
        const auto tuner = value.toObject();
        const auto tunerIndex = tuner.value(QStringLiteral("tunerIndex")).toInt();
        const auto isActiveContext = tuner.value(QStringLiteral("isActiveContext")).toBool();

        QStringList details;
        const auto programName = tuner.value(QStringLiteral("programName")).toString();
        const auto lockState = tuner.value(QStringLiteral("lockState")).toString();
        const auto virtualChannel = tuner.value(QStringLiteral("virtualChannel")).toString();
        const auto availability = tuner.value(QStringLiteral("availability")).toString();
        const auto warning = tuner.value(QStringLiteral("warning")).toString();

        if (!virtualChannel.isEmpty()) {
            details << virtualChannel;
        }
        if (!programName.isEmpty()) {
            details << programName;
        }
        if (!lockState.isEmpty()) {
            details << lockState;
        }
        if (tuner.contains(QStringLiteral("signalStrength"))
            && !tuner.value(QStringLiteral("signalStrength")).isNull()) {
            details << QStringLiteral("%1%% strength")
                           .arg(tuner.value(QStringLiteral("signalStrength")).toInt());
        }
        if (!warning.isEmpty()) {
            details << warning;
        } else if (!availability.isEmpty()) {
            details << availability;
        }

        QVariantMap row;
        row.insert(QStringLiteral("title"),
                   QStringLiteral("Tuner %1%2")
                       .arg(tunerIndex)
                       .arg(isActiveContext ? QStringLiteral(" • Active") : QString()));
        row.insert(QStringLiteral("detail"), details.join(QStringLiteral(" • ")));

        if (isActiveContext) {
            activeRows.append(row);
        } else {
            inactiveRows.append(row);
        }
    }

    QVariantList rows = activeRows;
    for (const auto &row : inactiveRows) {
        rows.append(row);
    }
    setDiagnosticsRows(rows);

    const auto state = payload.value(QStringLiteral("state")).toString();
    if (state == QStringLiteral("ready")) {
        setDiagnosticsSummary(QStringLiteral("Signal stable • Active tuner highlighted"));
    } else if (state == QStringLiteral("partial")) {
        setDiagnosticsSummary(QStringLiteral("Partial tuner data available"));
    } else if (state == QStringLiteral("selection_required")) {
        setDiagnosticsSummary(QStringLiteral("Select a device to inspect diagnostics"));
    } else {
        setDiagnosticsSummary(QStringLiteral("Diagnostics currently unavailable"));
    }

    const auto warning = firstWarning(payload.value(QStringLiteral("warnings")).toArray());
    if (!warning.isEmpty()) {
        setStageWarning(warning);
    }
}

void AppController::applyDvrReadinessResponse(const QJsonObject &payload)
{
    const auto state = payload.value(QStringLiteral("state")).toString();
    setDvrReadinessState(state);

    const auto conditions = payload.value(QStringLiteral("conditions")).toArray();
    const auto warnings = payload.value(QStringLiteral("warnings")).toArray();
    QString severity = QStringLiteral("info");
    QString title = QStringLiteral("DVR ready");
    QString message;

    if (!conditions.isEmpty()) {
        const auto condition = conditions.first().toObject();
        const auto conditionSeverity = condition.value(QStringLiteral("severity")).toString();
        severity = conditionSeverity == QStringLiteral("blocking") ? QStringLiteral("blocking") : QStringLiteral("warning");
        message = condition.value(QStringLiteral("message")).toString();
    } else {
        message = firstWarning(warnings);
    }

    if (state == QStringLiteral("ready")) {
        if (!message.isEmpty()) {
            setDvrBanner(true, QStringLiteral("warning"), QStringLiteral("DVR ready with warnings"), message);
        } else {
            setDvrBanner(false, QString(), QString(), QString());
        }
        return;
    }

    if (state == QStringLiteral("degraded")) {
        title = QStringLiteral("DVR partially available");
        severity = QStringLiteral("warning");
    } else if (state == QStringLiteral("selection_required")) {
        title = QStringLiteral("Select a device");
        severity = QStringLiteral("info");
        message = QStringLiteral("Select an HDHomeRun device to inspect DVR readiness.");
    } else {
        title = QStringLiteral("DVR unavailable");
        severity = severity == QStringLiteral("blocking") ? severity : QStringLiteral("blocking");
    }

    setDvrBanner(true, severity, title, message);
}

void AppController::applyDvrRecordingsResponse(const QJsonObject &payload)
{
    setDvrLoading(false);
    setDvrRecordingsState(payload.value(QStringLiteral("state")).toString());
    m_dvrRecordings = jsonArrayToVariantList(payload.value(QStringLiteral("recordings")).toArray());
    rebuildDvrRecordingGroups();

    const auto warning = firstWarning(payload.value(QStringLiteral("warnings")).toArray());
    if (!warning.isEmpty() && !m_dvrBannerVisible) {
        setDvrBanner(true, QStringLiteral("warning"), QStringLiteral("Recordings need attention"), warning);
    }
}

void AppController::applyDvrUpcomingResponse(const QJsonObject &payload)
{
    setDvrUpcomingLoading(false);
    setDvrUpcomingState(payload.value(QStringLiteral("state")).toString());
    setDvrUpcomingEntries(jsonArrayToVariantList(payload.value(QStringLiteral("entries")).toArray()));

    const auto warning = firstWarning(payload.value(QStringLiteral("warnings")).toArray());
    if (!warning.isEmpty() && !m_dvrBannerVisible) {
        setDvrBanner(true, QStringLiteral("warning"), QStringLiteral("Upcoming schedule needs attention"), warning);
    }

    rebuildSelectedRecordingDetails();
}

void AppController::applyDvrRulesResponse(const QJsonObject &payload)
{
    m_dvrRules = jsonArrayToVariantList(payload.value(QStringLiteral("rules")).toArray());
    rebuildSelectedRecordingDetails();
}

void AppController::setDevices(const QVariantList &devices)
{
    if (m_devices == devices) {
        return;
    }
    m_devices = devices;
    emit devicesChanged();
}

void AppController::setSelectedDeviceIndex(int index)
{
    if (m_selectedDeviceIndex == index) {
        return;
    }
    m_selectedDeviceIndex = index;
    emit selectedDeviceIndexChanged();
}

void AppController::setChannels(const QVariantList &channels)
{
    if (m_channels == channels) {
        return;
    }
    m_channels = channels;
    emit channelsChanged();
}

void AppController::setGuideChannels(const QVariantList &channels)
{
    if (m_guideChannels == channels) {
        return;
    }
    m_guideChannels = channels;
    emit guideChannelsChanged();
}

void AppController::setGuideLoading(bool loading)
{
    if (m_guideLoading == loading) {
        return;
    }
    m_guideLoading = loading;
    emit guideLoadingChanged();
}

void AppController::setGuideWindowStart(qint64 start)
{
    if (m_guideWindowStart == start) {
        return;
    }
    m_guideWindowStart = start;
    emit guideWindowStartChanged();
}

void AppController::setGuideDurationHours(int hours)
{
    if (m_guideDurationHours == hours) {
        return;
    }
    m_guideDurationHours = hours;
    emit guideDurationHoursChanged();
}

void AppController::setShellPhase(const QString &phase)
{
    if (m_shellPhase == phase) {
        return;
    }
    m_shellPhase = phase;
    emit shellPhaseChanged();
}

void AppController::setCurrentChannelRef(const QString &channelRef)
{
    if (m_currentChannelRef == channelRef) {
        return;
    }
    m_currentChannelRef = channelRef;
    emit currentChannelRefChanged();
}

void AppController::setStageTitle(const QString &title)
{
    if (m_stageTitle == title) {
        return;
    }
    m_stageTitle = title;
    emit stageTitleChanged();
}

void AppController::setStageSubtitle(const QString &subtitle)
{
    if (m_stageSubtitle == subtitle) {
        return;
    }
    m_stageSubtitle = subtitle;
    emit stageSubtitleChanged();
}

void AppController::setStageWarning(const QString &warning)
{
    if (m_stageWarning == warning) {
        return;
    }
    m_stageWarning = warning;
    emit stageWarningChanged();
}

void AppController::setStageFailure(const QString &failure)
{
    if (m_stageFailure == failure) {
        return;
    }
    m_stageFailure = failure;
    emit stageFailureChanged();
}

void AppController::setPlaybackUrl(const QString &playbackUrl)
{
    if (m_playbackUrl == playbackUrl) {
        return;
    }
    m_playbackUrl = playbackUrl;
    emit playbackUrlChanged();
}

void AppController::setPlaybackMode(const QString &mode)
{
    if (m_playbackMode == mode) {
        return;
    }
    m_playbackMode = mode;
    emit playbackModeChanged();
}

void AppController::setCurrentRecordingId(const QString &recordingId)
{
    if (m_currentRecordingId == recordingId) {
        return;
    }
    m_currentRecordingId = recordingId;
    emit currentRecordingIdChanged();
}

void AppController::setEmbeddedPlaybackEnabled(bool enabled)
{
    if (m_embeddedPlaybackEnabled == enabled) {
        return;
    }
    m_embeddedPlaybackEnabled = enabled;
    emit embeddedPlaybackEnabledChanged();
}

void AppController::setDiagnosticsSummary(const QString &summary)
{
    if (m_diagnosticsSummary == summary) {
        return;
    }
    m_diagnosticsSummary = summary;
    emit diagnosticsSummaryChanged();
}

void AppController::setDiagnosticsRows(const QVariantList &rows)
{
    if (m_diagnosticsRows == rows) {
        return;
    }
    m_diagnosticsRows = rows;
    emit diagnosticsRowsChanged();
}

void AppController::setDvrLoading(bool loading)
{
    if (m_dvrLoading == loading) {
        return;
    }
    m_dvrLoading = loading;
    emit dvrLoadingChanged();
}

void AppController::setDvrUpcomingLoading(bool loading)
{
    if (m_dvrUpcomingLoading == loading) {
        return;
    }
    m_dvrUpcomingLoading = loading;
    emit dvrUpcomingLoadingChanged();
}

void AppController::setDvrReadinessState(const QString &state)
{
    if (m_dvrReadinessState == state) {
        return;
    }
    m_dvrReadinessState = state;
    emit dvrReadinessStateChanged();
}

void AppController::setDvrRecordingsState(const QString &state)
{
    if (m_dvrRecordingsState == state) {
        return;
    }
    m_dvrRecordingsState = state;
    emit dvrRecordingsStateChanged();
}

void AppController::setDvrUpcomingState(const QString &state)
{
    if (m_dvrUpcomingState == state) {
        return;
    }
    m_dvrUpcomingState = state;
    emit dvrUpcomingStateChanged();
}

void AppController::setDvrBanner(bool visible, const QString &severity, const QString &title, const QString &message)
{
    if (m_dvrBannerVisible == visible
        && m_dvrBannerSeverity == severity
        && m_dvrBannerTitle == title
        && m_dvrBannerMessage == message) {
        return;
    }

    m_dvrBannerVisible = visible;
    m_dvrBannerSeverity = severity;
    m_dvrBannerTitle = title;
    m_dvrBannerMessage = message;
    emit dvrBannerChanged();
}

void AppController::setDvrRecordingGroups(const QVariantList &groups)
{
    if (m_dvrRecordingGroups == groups) {
        return;
    }
    m_dvrRecordingGroups = groups;
    emit dvrRecordingGroupsChanged();
}

void AppController::setSelectedRecordingIdInternal(const QString &recordingId)
{
    if (m_selectedRecordingId == recordingId) {
        return;
    }
    m_selectedRecordingId = recordingId;
    emit selectedRecordingIdChanged();
}

void AppController::setSelectedRecordingDetails(const QVariantMap &details)
{
    if (m_selectedRecordingDetails == details) {
        return;
    }
    m_selectedRecordingDetails = details;
    emit selectedRecordingDetailsChanged();
}

void AppController::setDvrUpcomingEntries(const QVariantList &entries)
{
    if (m_dvrUpcomingEntries == entries) {
        return;
    }
    m_dvrUpcomingEntries = entries;
    emit dvrUpcomingEntriesChanged();
}

void AppController::setDvrRuleEditorVisible(bool visible)
{
    if (m_dvrRuleEditorVisible == visible) {
        return;
    }
    m_dvrRuleEditorVisible = visible;
    emit dvrRuleEditorVisibleChanged();
}

void AppController::setDvrRuleEditorState(const QString &title, const QString &message, bool canCreateSeries, bool canCreateOneTime)
{
    if (m_dvrRuleEditorTitle == title
        && m_dvrRuleEditorMessage == message
        && m_dvrRuleEditorCanCreateSeries == canCreateSeries
        && m_dvrRuleEditorCanCreateOneTime == canCreateOneTime) {
        return;
    }

    m_dvrRuleEditorTitle = title;
    m_dvrRuleEditorMessage = message;
    m_dvrRuleEditorCanCreateSeries = canCreateSeries;
    m_dvrRuleEditorCanCreateOneTime = canCreateOneTime;
    emit dvrRuleEditorChanged();
}

void AppController::setDvrRuleEditorSubmitting(bool submitting)
{
    if (m_dvrRuleEditorSubmitting == submitting) {
        return;
    }
    m_dvrRuleEditorSubmitting = submitting;
    emit dvrRuleEditorSubmittingChanged();
}

void AppController::setDvrDeleteDialogTitle(const QString &title)
{
    if (m_dvrDeleteDialogTitle == title) {
        return;
    }
    m_dvrDeleteDialogTitle = title;
    emit dvrDeleteDialogTitleChanged();
}

void AppController::setDvrDeleteDialogVisible(bool visible)
{
    if (m_dvrDeleteDialogVisible == visible) {
        return;
    }
    m_dvrDeleteDialogVisible = visible;
    emit dvrDeleteDialogVisibleChanged();
}

void AppController::setDvrDeleteDialogBusy(bool busy)
{
    if (m_dvrDeleteDialogBusy == busy) {
        return;
    }
    m_dvrDeleteDialogBusy = busy;
    emit dvrDeleteDialogBusyChanged();
}

QString AppController::selectedDeviceRef() const
{
    if (m_selectedDeviceIndex < 0 || m_selectedDeviceIndex >= m_devices.size()) {
        return {};
    }

    return m_devices.at(m_selectedDeviceIndex).toMap().value(QStringLiteral("deviceRef")).toString();
}

QVariantMap AppController::findRecordingById(const QString &recordingId) const
{
    for (const auto &value : m_dvrRecordings) {
        const auto recording = value.toMap();
        if (recording.value(QStringLiteral("recordingId")).toString() == recordingId) {
            return recording;
        }
    }

    return {};
}

QVariantMap AppController::findUpcomingByProgramId(const QString &programId) const
{
    for (const auto &value : m_dvrUpcomingEntries) {
        const auto entry = value.toMap();
        if (entry.value(QStringLiteral("programId")).toString() == programId) {
            return entry;
        }
    }

    return {};
}

QVariantMap AppController::findRuleByRecordingRuleId(const QString &recordingRuleId) const
{
    for (const auto &value : m_dvrRules) {
        const auto rule = value.toMap();
        if (rule.value(QStringLiteral("recordingRuleId")).toString() == recordingRuleId) {
            return rule;
        }
    }

    return {};
}

void AppController::clearDvrWorkspaceState()
{
    setDvrReadinessState(QStringLiteral("selection_required"));
    setDvrRecordingsState(QStringLiteral("selection_required"));
    setDvrUpcomingState(QStringLiteral("selection_required"));
    setDvrLoading(false);
    setDvrUpcomingLoading(false);
    m_dvrRecordings.clear();
    m_dvrRules.clear();
    m_expandedRecordingGroups.clear();
    setDvrRecordingGroups({});
    setSelectedRecordingIdInternal(QString());
    setSelectedRecordingDetails({});
    setDvrUpcomingEntries({});
    setDvrBanner(
        true,
        QStringLiteral("info"),
        QStringLiteral("Select a device"),
        QStringLiteral("Select an HDHomeRun device to load the DVR workspace."));
}

void AppController::setLaunchFailure(const QString &message)
{
    setShellPhase(QStringLiteral("playback_failed"));
    setStageTitle(QStringLiteral("Backend unavailable"));
    setStageSubtitle(QStringLiteral("The client could not enter the live TV shell"));
    setStageFailure(message);
}

QVariantList AppController::jsonArrayToVariantList(const QJsonArray &array)
{
    QVariantList result;
    result.reserve(array.size());
    for (const auto &value : array) {
        result.append(value.toVariant());
    }
    return result;
}