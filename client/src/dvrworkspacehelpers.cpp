#include "dvrworkspacehelpers.h"

#include <algorithm>

namespace {

QString normalizedTitle(const QString &title)
{
    const auto trimmed = title.trimmed();
    return trimmed.isEmpty() ? QStringLiteral("untitled recording") : trimmed;
}

qint64 recordStartTime(const QVariantMap &recording)
{
    return recording.value(QStringLiteral("recordStartTime")).toLongLong();
}

bool caseInsensitiveEqual(const QString &left, const QString &right)
{
    return left.trimmed().compare(right.trimmed(), Qt::CaseInsensitive) == 0;
}

}

namespace DvrWorkspaceHelpers {

QString recordingGroupId(const QVariantMap &recording)
{
    return normalizedTitle(recording.value(QStringLiteral("title")).toString()).toLower();
}

QVariantList buildRecordingGroups(
    const QVariantList &recordings,
    const QSet<QString> &expandedGroupIds,
    const QString &selectedRecordingId)
{
    struct GroupAccumulator {
        QString title;
        QString groupId;
        qint64 latestRecordTime = 0;
        QVariantList episodes;
    };

    QList<GroupAccumulator> groups;
    QHash<QString, int> groupIndexes;

    for (const auto &value : recordings) {
        const auto recording = value.toMap();
        const auto groupId = recordingGroupId(recording);
        const auto title = normalizedTitle(recording.value(QStringLiteral("title")).toString());

        int groupIndex = groupIndexes.value(groupId, -1);
        if (groupIndex < 0) {
            groupIndex = groups.size();
            groups.append(GroupAccumulator{title, groupId, 0, {}});
            groupIndexes.insert(groupId, groupIndex);
        }

        QVariantMap episode = recording;
        episode.insert(
            QStringLiteral("selected"),
            recording.value(QStringLiteral("recordingId")).toString() == selectedRecordingId);

        groups[groupIndex].latestRecordTime = std::max(groups[groupIndex].latestRecordTime, recordStartTime(recording));
        groups[groupIndex].episodes.append(episode);
    }

    std::sort(groups.begin(), groups.end(), [](const GroupAccumulator &left, const GroupAccumulator &right) {
        if (left.latestRecordTime != right.latestRecordTime) {
            return left.latestRecordTime > right.latestRecordTime;
        }

        return left.title.compare(right.title, Qt::CaseInsensitive) < 0;
    });

    QVariantList result;
    result.reserve(groups.size());
    for (auto &group : groups) {
        std::sort(group.episodes.begin(), group.episodes.end(), [](const QVariant &left, const QVariant &right) {
            const auto leftMap = left.toMap();
            const auto rightMap = right.toMap();
            const auto leftStart = recordStartTime(leftMap);
            const auto rightStart = recordStartTime(rightMap);
            if (leftStart != rightStart) {
                return leftStart > rightStart;
            }

            return leftMap.value(QStringLiteral("recordingId")).toString()
                < rightMap.value(QStringLiteral("recordingId")).toString();
        });

        bool containsSelection = false;
        for (const auto &episodeValue : group.episodes) {
            if (episodeValue.toMap().value(QStringLiteral("selected")).toBool()) {
                containsSelection = true;
                break;
            }
        }

        QVariantMap item;
        item.insert(QStringLiteral("seriesTitle"), group.title);
        item.insert(QStringLiteral("groupId"), group.groupId);
        item.insert(QStringLiteral("expanded"), containsSelection || expandedGroupIds.contains(group.groupId));
        item.insert(QStringLiteral("latestRecordTime"), group.latestRecordTime);
        item.insert(QStringLiteral("episodeCount"), group.episodes.size());
        item.insert(QStringLiteral("episodes"), group.episodes);
        result.append(item);
    }

    return result;
}

QVariantMap inferRuleEditorContextForRecording(
    const QVariantMap &recording,
    const QVariantList &upcomingItems,
    const QVariantList &rules)
{
    QVariantMap context;
    const auto title = normalizedTitle(recording.value(QStringLiteral("title")).toString());
    const auto episodeTitle = recording.value(QStringLiteral("episodeTitle")).toString();

    context.insert(QStringLiteral("entrySource"), QStringLiteral("recording_details"));
    context.insert(QStringLiteral("title"), title);
    context.insert(QStringLiteral("episodeTitle"), episodeTitle);
    context.insert(QStringLiteral("canCreateOneTime"), false);

    QSet<QString> candidateSeriesIds;

    for (const auto &value : upcomingItems) {
        const auto item = value.toMap();
        if (caseInsensitiveEqual(item.value(QStringLiteral("title")).toString(), title)) {
            const auto seriesId = item.value(QStringLiteral("seriesId")).toString();
            if (!seriesId.isEmpty()) {
                candidateSeriesIds.insert(seriesId);
            }
        }
    }

    if (candidateSeriesIds.isEmpty()) {
        for (const auto &value : rules) {
            const auto item = value.toMap();
            if (caseInsensitiveEqual(item.value(QStringLiteral("title")).toString(), title)) {
                const auto seriesId = item.value(QStringLiteral("seriesId")).toString();
                if (!seriesId.isEmpty()) {
                    candidateSeriesIds.insert(seriesId);
                }
            }
        }
    }

    if (candidateSeriesIds.size() == 1) {
        const auto seriesId = *candidateSeriesIds.constBegin();
        context.insert(QStringLiteral("seriesId"), seriesId);
        context.insert(QStringLiteral("canCreateSeries"), true);
        context.insert(
            QStringLiteral("message"),
            QStringLiteral("Create a series rule for %1 from this recording context.").arg(title));
    } else {
        context.insert(QStringLiteral("canCreateSeries"), false);
        context.insert(
            QStringLiteral("message"),
            QStringLiteral("Series rule creation from recording details becomes available when a unique DVR schedule context can be inferred."));
    }

    return context;
}

}