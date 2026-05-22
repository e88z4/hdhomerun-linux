#include "channelnavigation.h"

#include <QVariantMap>

namespace {
bool isPlayableChannel(const QVariantMap &channel)
{
    return channel.value(QStringLiteral("availability")).toString() == QStringLiteral("playable")
        && !channel.value(QStringLiteral("channelRef")).toString().trimmed().isEmpty();
}
}

QString findAdjacentPlayableChannelRef(
    const QVariantList &channels,
    const QString &currentChannelRef,
    int direction)
{
    if (channels.isEmpty() || direction == 0) {
        return {};
    }

    QList<int> playableIndices;
    playableIndices.reserve(channels.size());

    for (int index = 0; index < channels.size(); ++index) {
        const auto channel = channels.at(index).toMap();
        if (isPlayableChannel(channel)) {
            playableIndices.append(index);
        }
    }

    if (playableIndices.isEmpty()) {
        return {};
    }

    int currentPlayableIndex = -1;
    for (int index = 0; index < playableIndices.size(); ++index) {
        const auto channel = channels.at(playableIndices.at(index)).toMap();
        if (channel.value(QStringLiteral("channelRef")).toString() == currentChannelRef) {
            currentPlayableIndex = index;
            break;
        }
    }

    if (currentPlayableIndex < 0) {
        const int fallbackIndex = direction > 0 ? 0 : playableIndices.size() - 1;
        return channels.at(playableIndices.at(fallbackIndex)).toMap().value(QStringLiteral("channelRef")).toString();
    }

    const int step = direction > 0 ? 1 : -1;
    const int nextPlayableIndex = (currentPlayableIndex + step + playableIndices.size()) % playableIndices.size();
    return channels.at(playableIndices.at(nextPlayableIndex)).toMap().value(QStringLiteral("channelRef")).toString();
}