#include "appcontroller.h"

#include <QGuiApplication>
#include <QQmlApplicationEngine>
#include <QQmlContext>
#include <QTimer>

int main(int argc, char *argv[])
{
    QGuiApplication app(argc, argv);
    app.setOrganizationName("felix");
    app.setOrganizationDomain("dev.felix");
    app.setApplicationName("hdhomerun-linux-player");

    AppController appController;
    QQmlApplicationEngine engine;
    engine.rootContext()->setContextProperty(QStringLiteral("appController"), &appController);
    QObject::connect(
        &engine,
        &QQmlApplicationEngine::objectCreationFailed,
        &app,
        []() { QCoreApplication::exit(EXIT_FAILURE); },
        Qt::QueuedConnection);
    engine.loadFromModule("HDHomeRun.Client", "Main");
    appController.initialize();

    bool exitAfterSet = false;
    const int exitAfterMs = qEnvironmentVariableIntValue("HDHR_CLIENT_EXIT_AFTER_MS", &exitAfterSet);
    if (exitAfterSet && exitAfterMs >= 0) {
        QTimer::singleShot(exitAfterMs, &app, &QCoreApplication::quit);
    }

    return app.exec();
}