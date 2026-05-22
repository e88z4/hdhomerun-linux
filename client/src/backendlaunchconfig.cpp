#include "backendlaunchconfig.h"

#include <QHostAddress>
#include <QUrl>

namespace {
constexpr int kDefaultBackendPort = 38080;

QString normalizedBindHost(const QString &host)
{
    if (host.compare(QStringLiteral("localhost"), Qt::CaseInsensitive) == 0) {
        return QStringLiteral("127.0.0.1");
    }

    return host;
}

bool isLoopbackHost(const QString &host)
{
    if (host.compare(QStringLiteral("localhost"), Qt::CaseInsensitive) == 0) {
        return true;
    }

    QHostAddress address;
    return address.setAddress(host) && address.isLoopback();
}

QString formatBindAddress(const QString &host, int port)
{
    if (host.contains(QLatin1Char(':'))) {
        return QStringLiteral("[%1]:%2").arg(host).arg(port);
    }

    return QStringLiteral("%1:%2").arg(host).arg(port);
}
}

BackendLaunchDecision resolveBackendLaunchDecision(const QString &backendBaseUrl)
{
    const QUrl url(backendBaseUrl);
    if (!url.isValid() || url.host().isEmpty()) {
        return {
            false,
            {},
            QStringLiteral("HDHR_BACKEND_URL must be a valid local HTTP URL when the client auto-starts the backend."),
        };
    }

    if (!url.scheme().isEmpty() && url.scheme() != QStringLiteral("http")) {
        return {
            false,
            {},
            QStringLiteral("HDHR_BACKEND_URL must use http when the client auto-starts the backend."),
        };
    }

    const QString host = url.host();
    if (!isLoopbackHost(host)) {
        return {
            false,
            {},
            QStringLiteral("HDHR_BACKEND_URL points to a non-local address. Start that backend manually or update the URL before using client auto-start."),
        };
    }

    return {
        true,
        formatBindAddress(normalizedBindHost(host), url.port(kDefaultBackendPort)),
        {},
    };
}