#pragma once

#include <QSet>
#include <QString>
#include <QVariantList>
#include <QVariantMap>

namespace DvrWorkspaceHelpers {

QString recordingGroupId(const QVariantMap &recording);
QVariantList buildRecordingGroups(
    const QVariantList &recordings,
    const QSet<QString> &expandedGroupIds,
    const QString &selectedRecordingId);
QVariantMap inferRuleEditorContextForRecording(
    const QVariantMap &recording,
    const QVariantList &upcomingItems,
    const QVariantList &rules);

}