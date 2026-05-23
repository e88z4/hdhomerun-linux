#include "appcontroller.h"
#include "idleinhibitor.h"

#include <QCoreApplication>
#include <QFile>
#include <QGuiApplication>
#include <QQmlApplicationEngine>
#include <QQmlContext>
#include <QTimer>

#include <memory>

namespace {
constexpr auto kHeadlessSmokeModuleDirModern = ":/qt/qml/HDHomeRun/Client/qmldir";
constexpr auto kHeadlessSmokeMainQmlModern = ":/qt/qml/HDHomeRun/Client/qml/Main.qml";
constexpr auto kHeadlessSmokeModuleDirLegacy = ":/HDHomeRun/Client/qmldir";
constexpr auto kHeadlessSmokeMainQmlLegacy = ":/HDHomeRun/Client/qml/Main.qml";

bool isHeadlessSmokeRun(bool exitAfterSet, int exitAfterMs)
{
    if (!exitAfterSet || exitAfterMs < 0) {
        return false;
    }

    const auto platform = qEnvironmentVariable("QT_QPA_PLATFORM").trimmed().toLower();
    return platform == QStringLiteral("offscreen") || platform == QStringLiteral("minimal");
}

bool validateHeadlessQmlModule()
{
    const bool modernLayout = QFile::exists(QString::fromUtf8(kHeadlessSmokeModuleDirModern))
        && QFile::exists(QString::fromUtf8(kHeadlessSmokeMainQmlModern));
    const bool legacyLayout = QFile::exists(QString::fromUtf8(kHeadlessSmokeModuleDirLegacy))
        && QFile::exists(QString::fromUtf8(kHeadlessSmokeMainQmlLegacy));

    return modernLayout || legacyLayout;
}
}

int main(int argc, char *argv[])
{
    bool exitAfterSet = false;
    const int exitAfterMs = qEnvironmentVariableIntValue("HDHR_CLIENT_EXIT_AFTER_MS", &exitAfterSet);
    const bool headlessSmokeRun = isHeadlessSmokeRun(exitAfterSet, exitAfterMs);

    std::unique_ptr<QCoreApplication> app;
    if (headlessSmokeRun) {
        app = std::make_unique<QCoreApplication>(argc, argv);
    } else {
        app = std::make_unique<QGuiApplication>(argc, argv);
    }

    QCoreApplication::setOrganizationName("felix");
    QCoreApplication::setOrganizationDomain("dev.felix");
    QCoreApplication::setApplicationName("hdhomerun-linux-player");

    AppController appController;
    std::unique_ptr<IdleInhibitor> idleInhibitor;
    std::unique_ptr<QQmlApplicationEngine> engine;
    if (!headlessSmokeRun) {
        idleInhibitor = std::make_unique<IdleInhibitor>();
        QObject::connect(&appController, &AppController::shellPhaseChanged, idleInhibitor.get(), [&appController, &idleInhibitor]() {
            idleInhibitor->setPlaybackActive(appController.shellPhase() == QStringLiteral("playing"));
        });

        engine = std::make_unique<QQmlApplicationEngine>();
        engine->rootContext()->setContextProperty(QStringLiteral("appController"), &appController);
        QObject::connect(
            engine.get(),
            &QQmlApplicationEngine::objectCreationFailed,
            app.get(),
            []() { QCoreApplication::exit(EXIT_FAILURE); },
            Qt::QueuedConnection);
        engine->loadFromModule("HDHomeRun.Client", "Main");

        idleInhibitor->setPlaybackActive(appController.shellPhase() == QStringLiteral("playing"));
    } else if (!validateHeadlessQmlModule()) {
        return EXIT_FAILURE;
    }
    appController.initialize();

    if (exitAfterSet && exitAfterMs >= 0) {
        QTimer::singleShot(exitAfterMs, app.get(), &QCoreApplication::quit);
    }

    return app->exec();
}