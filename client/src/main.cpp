#include "appcontroller.h"

#include <QCoreApplication>
#include <QGuiApplication>
#include <QQmlApplicationEngine>
#include <QQmlContext>
#include <QTimer>

#include <memory>

namespace {
bool isHeadlessSmokeRun(bool exitAfterSet, int exitAfterMs)
{
    if (!exitAfterSet || exitAfterMs < 0) {
        return false;
    }

    const auto platform = qEnvironmentVariable("QT_QPA_PLATFORM").trimmed().toLower();
    return platform == QStringLiteral("offscreen") || platform == QStringLiteral("minimal");
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
    }
    appController.initialize();

    if (exitAfterSet && exitAfterMs >= 0) {
        QTimer::singleShot(exitAfterMs, app.get(), &QCoreApplication::quit);
    }

    return app->exec();
}