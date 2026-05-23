#include "appcontroller.h"
#include "backendlaunchconfig.h"
#include "channelnavigation.h"

#include <QCoreApplication>
#include <QProcessEnvironment>
#include <QJsonDocument>
#include <QNetworkReply>
#include <QNetworkRequest>
#include <QStandardPaths>
#include <QStringList>
#include <QTimer>
#include <QUrl>
#include <QDateTime>

namespace {
constexpr auto kDefaultBackendUrl = "http://127.0.0.1:38080";
constexpr auto kDefaultBackendExecutable = "hdhomerun-backend";

qint64 defaultGuideWindowStart()
{
    const auto now = QDateTime::currentSecsSinceEpoch();
    return now - (now % 1800);
}
}

AppController::AppController(QObject *parent)
    : QObject(parent)
    , m_backendProcess(nullptr)
    , m_backendBaseUrl(qEnvironmentVariable("HDHR_BACKEND_URL", kDefaultBackendUrl))
    , m_selectedDeviceIndex(-1)
    , m_guideVisible(false)
    , m_guideLoading(false)
    , m_guideWindowStart(defaultGuideWindowStart())
    , m_guideDurationHours(24)
    , m_guideEndpointAvailable(true)
    , m_shellPhase(QStringLiteral("launching"))
    , m_embeddedPlaybackEnabled(false)
    , m_diagnosticsExpanded(true)
    , m_diagnosticsSummary(QStringLiteral("Diagnostics unavailable"))
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
bool AppController::embeddedPlaybackEnabled() const { return m_embeddedPlaybackEnabled; }
bool AppController::diagnosticsExpanded() const { return m_diagnosticsExpanded; }
QString AppController::diagnosticsSummary() const { return m_diagnosticsSummary; }
QVariantList AppController::diagnosticsRows() const { return m_diagnosticsRows; }

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

    setGuideVisible(!m_guideVisible);
    if (m_guideVisible) {
        loadGuide();
    }
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
    setStageSubtitle(QStringLiteral("Loading devices, channels, and playback state"));

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

            const auto warnings = payload.value(QStringLiteral("warnings")).toArray();
            QString warning = warnings.isEmpty() ? QString() : warnings.first().toString();
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

            const auto warnings = payload.value(QStringLiteral("warnings")).toArray();
            if (!warnings.isEmpty()) {
                setStageWarning(warnings.first().toString());
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
        setGuideVisible(false);
        setGuideLoading(false);
        setDiagnosticsRows({});
        setDiagnosticsSummary(QStringLiteral("Select a device to load diagnostics"));
        if (!m_devices.isEmpty()) {
            setShellPhase(QStringLiteral("device_selection"));
            setStageTitle(QStringLiteral("Choose a device"));
            setStageSubtitle(QStringLiteral("Select an HDHomeRun device to load the channel rail"));
        } else {
            setShellPhase(QStringLiteral("device_selection"));
            setStageTitle(QStringLiteral("No devices found"));
            setStageSubtitle(QStringLiteral("Retry after the backend discovers a reachable HDHomeRun tuner"));
        }
        return;
    }

    loadLineup();
    loadDiagnostics();
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

    const auto warnings = payload.value(QStringLiteral("warnings")).toArray();
    if (!warnings.isEmpty()) {
        setStageWarning(warnings.first().toString());
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

    const auto warnings = payload.value(QStringLiteral("warnings")).toArray();
    if (!warnings.isEmpty()) {
        setStageWarning(warnings.first().toString());
    }
}

void AppController::applyPlaybackResponse(const QJsonObject &payload)
{
    const auto session = payload.value(QStringLiteral("sessionState")).toObject();
    const auto currentChannel = payload.value(QStringLiteral("currentChannel")).toObject();
    const auto adapterState = payload.value(QStringLiteral("adapterState")).toObject();
    const auto failure = payload.value(QStringLiteral("failure")).toObject();
    const auto warnings = payload.value(QStringLiteral("warnings")).toArray();
    const auto sessionStatus = session.value(QStringLiteral("status")).toString();
    const auto adapterCommand = adapterState.value(QStringLiteral("lastCommand")).toString();
    const auto playbackUrl = session.value(QStringLiteral("playbackUrl")).toString();

    setEmbeddedPlaybackEnabled(adapterCommand.startsWith(QStringLiteral("client_managed")));
    setPlaybackUrl(playbackUrl);

    if (!currentChannel.isEmpty()) {
        setCurrentChannelRef(currentChannel.value(QStringLiteral("channelRef")).toString());
        setStageTitle(currentChannel.value(QStringLiteral("guideName")).toString());
        setStageSubtitle(QStringLiteral("Channel %1 • Persistent playback session")
                             .arg(currentChannel.value(QStringLiteral("guideNumber")).toString()));
    } else if (!selectedDeviceRef().isEmpty()) {
        setCurrentChannelRef(QString());
        setStageTitle(QStringLiteral("Select a channel"));
        setStageSubtitle(QStringLiteral("Use the channel rail to start live playback"));
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
        return;
    }

    setStageWarning(warnings.isEmpty() ? QString() : warnings.first().toString());
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

    const auto warnings = payload.value(QStringLiteral("warnings")).toArray();
    if (!warnings.isEmpty()) {
        setStageWarning(warnings.first().toString());
    }
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

QString AppController::selectedDeviceRef() const
{
    if (m_selectedDeviceIndex < 0 || m_selectedDeviceIndex >= m_devices.size()) {
        return {};
    }

    return m_devices.at(m_selectedDeviceIndex).toMap().value(QStringLiteral("deviceRef")).toString();
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