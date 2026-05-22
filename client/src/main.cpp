#include "appcontroller.h"

#include <QCoreApplication>
#include <QFile>
#include <QGuiApplication>
#include <QQmlApplicationEngine>
#include <QQmlContext>
#include <QTimer>

#include <memory>

namespace {
constexpr auto kHeadlessSmokeModuleDir = ":/qt/qml/HDHomeRun/Client/qmldir";
constexpr auto kHeadlessSmokeMainQml = ":/qt/qml/HDHomeRun/Client/qml/Main.qml";

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
    return QFile::exists(QString::fromUtf8(kHeadlessSmokeModuleDir))
        && QFile::exists(QString::fromUtf8(kHeadlessSmokeMainQml));
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
    std::unique_ptr<QQmlApplicationEngine> engine;
    if (!headlessSmokeRun) {
        engine = std::make_unique<QQmlApplicationEngine>();
        engine->rootContext()->setContextProperty(QStringLiteral("appController"), &appController);
        QObject::connect(
            engine.get(),
            &QQmlApplicationEngine::objectCreationFailed,
            app.get(),
            []() { QCoreApplication::exit(EXIT_FAILURE); },
            Qt::QueuedConnection);
        engine->loadFromModule("HDHomeRun.Client", "Main");
    } else if (!validateHeadlessQmlModule()) {
        return EXIT_FAILURE;
    }
    appController.initialize();

    if (exitAfterSet && exitAfterMs >= 0) {
        QTimer::singleShot(exitAfterMs, app.get(), &QCoreApplication::quit);
    }

    return app->exec();
}