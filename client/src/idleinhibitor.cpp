#include "idleinhibitor.h"

#include <QCoreApplication>

#ifdef Q_OS_LINUX
#include <QDBusConnection>
#include <QDBusInterface>
#include <QDBusReply>
#include <QDBusUnixFileDescriptor>

#include <unistd.h>
#endif

namespace {
constexpr auto kInhibitReason = "Playing live TV";

QString inhibitReason()
{
    return QString::fromUtf8(kInhibitReason);
}
}

IdleInhibitor::IdleInhibitor(QObject *parent)
    : QObject(parent)
    , m_playbackActive(false)
    , m_screenSaverCookie(0)
    , m_login1Fd(-1)
{
}

IdleInhibitor::~IdleInhibitor()
{
    release();
}

void IdleInhibitor::setPlaybackActive(bool active)
{
    if (m_playbackActive == active) {
        return;
    }

    m_playbackActive = active;
    if (m_playbackActive) {
        acquire();
    } else {
        release();
    }
}

void IdleInhibitor::acquire()
{
#ifdef Q_OS_LINUX
    if (m_screenSaverCookie == 0 && QDBusConnection::sessionBus().isConnected()) {
        QDBusInterface screenSaver(
            QStringLiteral("org.freedesktop.ScreenSaver"),
            QStringLiteral("/ScreenSaver"),
            QStringLiteral("org.freedesktop.ScreenSaver"),
            QDBusConnection::sessionBus());

        if (screenSaver.isValid()) {
            const QDBusReply<unsigned int> reply = screenSaver.call(
                QStringLiteral("Inhibit"),
                QCoreApplication::applicationName(),
                inhibitReason());
            if (reply.isValid()) {
                m_screenSaverCookie = reply.value();
            }
        }
    }

    if (m_login1Fd < 0 && QDBusConnection::systemBus().isConnected()) {
        QDBusInterface login1(
            QStringLiteral("org.freedesktop.login1"),
            QStringLiteral("/org/freedesktop/login1"),
            QStringLiteral("org.freedesktop.login1.Manager"),
            QDBusConnection::systemBus());

        if (login1.isValid()) {
            const QDBusReply<QDBusUnixFileDescriptor> reply = login1.call(
                QStringLiteral("Inhibit"),
                QStringLiteral("idle:sleep"),
                QCoreApplication::applicationName(),
                inhibitReason(),
                QStringLiteral("block"));
            if (reply.isValid()) {
                m_login1Fd = reply.value().fileDescriptor();
            }
        }
    }
#endif
}

void IdleInhibitor::release()
{
#ifdef Q_OS_LINUX
    if (m_screenSaverCookie != 0 && QDBusConnection::sessionBus().isConnected()) {
        QDBusInterface screenSaver(
            QStringLiteral("org.freedesktop.ScreenSaver"),
            QStringLiteral("/ScreenSaver"),
            QStringLiteral("org.freedesktop.ScreenSaver"),
            QDBusConnection::sessionBus());
        if (screenSaver.isValid()) {
            screenSaver.call(QStringLiteral("UnInhibit"), m_screenSaverCookie);
        }
        m_screenSaverCookie = 0;
    }

    if (m_login1Fd >= 0) {
        ::close(m_login1Fd);
        m_login1Fd = -1;
    }
#endif
}