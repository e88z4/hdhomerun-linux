#pragma once

#include <QString>
#include <QVariantList>

QString findAdjacentPlayableChannelRef(
    const QVariantList &channels,
    const QString &currentChannelRef,
    int direction);