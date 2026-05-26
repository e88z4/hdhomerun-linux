#include <QtTest>

#include "dvrworkspacehelpers.h"

class DvrWorkspaceHelpersTests : public QObject
{
    Q_OBJECT

private slots:
    void buildsSeriesGroupsRecentFirst();
    void infersUniqueSeriesContextForRecording();
    void keepsSeriesCreationDisabledForAmbiguousRecordingContext();
    void fallsBackToRulesForUniqueSeriesContext();
    void keepsExplicitlyExpandedGroupVisibleWithoutSelection();
};

void DvrWorkspaceHelpersTests::buildsSeriesGroupsRecentFirst()
{
    QVariantMap olderEpisode{
        {QStringLiteral("recordingId"), QStringLiteral("alpha-1")},
        {QStringLiteral("title"), QStringLiteral("Alpha")},
        {QStringLiteral("episodeTitle"), QStringLiteral("Pilot")},
        {QStringLiteral("recordStartTime"), 1000},
    };
    QVariantMap newerEpisode{
        {QStringLiteral("recordingId"), QStringLiteral("alpha-2")},
        {QStringLiteral("title"), QStringLiteral("Alpha")},
        {QStringLiteral("episodeTitle"), QStringLiteral("Finale")},
        {QStringLiteral("recordStartTime"), 5000},
    };
    QVariantMap otherSeries{
        {QStringLiteral("recordingId"), QStringLiteral("beta-1")},
        {QStringLiteral("title"), QStringLiteral("Beta")},
        {QStringLiteral("recordStartTime"), 3000},
    };

    const QVariantList groups = DvrWorkspaceHelpers::buildRecordingGroups(
        {olderEpisode, newerEpisode, otherSeries},
        {},
        QStringLiteral("alpha-2"));

    QCOMPARE(groups.size(), 2);
    const auto firstGroup = groups.first().toMap();
    QCOMPARE(firstGroup.value(QStringLiteral("seriesTitle")).toString(), QStringLiteral("Alpha"));
    QVERIFY(firstGroup.value(QStringLiteral("expanded")).toBool());

    const auto episodes = firstGroup.value(QStringLiteral("episodes")).toList();
    QCOMPARE(episodes.size(), 2);
    QCOMPARE(episodes.first().toMap().value(QStringLiteral("recordingId")).toString(), QStringLiteral("alpha-2"));
    QVERIFY(episodes.first().toMap().value(QStringLiteral("selected")).toBool());
}

void DvrWorkspaceHelpersTests::infersUniqueSeriesContextForRecording()
{
    const QVariantMap recording{
        {QStringLiteral("recordingId"), QStringLiteral("rec-1")},
        {QStringLiteral("title"), QStringLiteral("The King of Queens")},
        {QStringLiteral("episodeTitle"), QStringLiteral("Shrink Wrap")},
    };
    const QVariantMap upcoming{
        {QStringLiteral("programId"), QStringLiteral("program-1")},
        {QStringLiteral("seriesId"), QStringLiteral("series-1")},
        {QStringLiteral("title"), QStringLiteral("The King of Queens")},
    };

    const QVariantMap context = DvrWorkspaceHelpers::inferRuleEditorContextForRecording(
        recording,
        {upcoming},
        {});

    QVERIFY(context.value(QStringLiteral("canCreateSeries")).toBool());
    QCOMPARE(context.value(QStringLiteral("seriesId")).toString(), QStringLiteral("series-1"));
    QVERIFY(!context.value(QStringLiteral("canCreateOneTime")).toBool());
}

void DvrWorkspaceHelpersTests::keepsSeriesCreationDisabledForAmbiguousRecordingContext()
{
    const QVariantMap recording{
        {QStringLiteral("recordingId"), QStringLiteral("rec-1")},
        {QStringLiteral("title"), QStringLiteral("News")},
    };
    const QVariantMap upcomingA{
        {QStringLiteral("programId"), QStringLiteral("program-a")},
        {QStringLiteral("seriesId"), QStringLiteral("series-a")},
        {QStringLiteral("title"), QStringLiteral("News")},
    };
    const QVariantMap upcomingB{
        {QStringLiteral("programId"), QStringLiteral("program-b")},
        {QStringLiteral("seriesId"), QStringLiteral("series-b")},
        {QStringLiteral("title"), QStringLiteral("News")},
    };

    const QVariantMap context = DvrWorkspaceHelpers::inferRuleEditorContextForRecording(
        recording,
        {upcomingA, upcomingB},
        {});

    QVERIFY(!context.value(QStringLiteral("canCreateSeries")).toBool());
    QVERIFY(context.value(QStringLiteral("seriesId")).toString().isEmpty());
}

void DvrWorkspaceHelpersTests::fallsBackToRulesForUniqueSeriesContext()
{
    const QVariantMap recording{
        {QStringLiteral("recordingId"), QStringLiteral("rec-1")},
        {QStringLiteral("title"), QStringLiteral("Example Show")},
    };
    const QVariantMap rule{
        {QStringLiteral("recordingRuleId"), QStringLiteral("rule-1")},
        {QStringLiteral("seriesId"), QStringLiteral("series-7")},
        {QStringLiteral("title"), QStringLiteral("Example Show")},
    };

    const QVariantMap context = DvrWorkspaceHelpers::inferRuleEditorContextForRecording(
        recording,
        {},
        {rule});

    QVERIFY(context.value(QStringLiteral("canCreateSeries")).toBool());
    QCOMPARE(context.value(QStringLiteral("seriesId")).toString(), QStringLiteral("series-7"));
}

void DvrWorkspaceHelpersTests::keepsExplicitlyExpandedGroupVisibleWithoutSelection()
{
    const QVariantMap episode{
        {QStringLiteral("recordingId"), QStringLiteral("alpha-1")},
        {QStringLiteral("title"), QStringLiteral("Alpha")},
        {QStringLiteral("recordStartTime"), 1000},
    };

    const QVariantList groups = DvrWorkspaceHelpers::buildRecordingGroups(
        {episode},
        {QStringLiteral("alpha")},
        QString());

    QCOMPARE(groups.size(), 1);
    QVERIFY(groups.first().toMap().value(QStringLiteral("expanded")).toBool());
}

QTEST_MAIN(DvrWorkspaceHelpersTests)

#include "dvrworkspacehelpers_tests.moc"