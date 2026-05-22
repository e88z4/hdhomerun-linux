#pragma once

#include <QString>

struct BackendLaunchDecision {
    bool canAutoStart;
    QString bindAddress;
    QString errorMessage;
};

BackendLaunchDecision resolveBackendLaunchDecision(const QString &backendBaseUrl);