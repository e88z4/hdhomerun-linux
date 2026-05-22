#include "channelnavigation.h"

#include <QtTest>

class ChannelNavigationTests : public QObject
{
    Q_OBJECT

private slots:
    void nextChannelSkipsUnavailableEntries();
    void previousChannelWrapsAcrossPlayableEntries();
    void navigationStartsAtFirstOrLastPlayableWhenIdle();
};

namespace {
QVariantMap channel(
    const QString &channelRef,
    const QString &guideNumber,
    const QString &guideName,
    const QString &availability)
{
    return {
        {QStringLiteral("channelRef"), channelRef},
        {QStringLiteral("guideNumber"), guideNumber},
        {QStringLiteral("guideName"), guideName},
        {QStringLiteral("availability"), availability},
    };
}
}

void ChannelNavigationTests::nextChannelSkipsUnavailableEntries()
{
    const QVariantList channels = {
        channel(QStringLiteral("channel:2.1"), QStringLiteral("2.1"), QStringLiteral("News"), QStringLiteral("playable")),
        channel(QStringLiteral("channel:4.1"), QStringLiteral("4.1"), QStringLiteral("Sports"), QStringLiteral("restricted")),
        channel(QStringLiteral("channel:5.1"), QStringLiteral("5.1"), QStringLiteral("Movies"), QStringLiteral("playable")),
    };

    QCOMPARE(findAdjacentPlayableChannelRef(channels, QStringLiteral("channel:2.1"), 1), QStringLiteral("channel:5.1"));
}

void ChannelNavigationTests::previousChannelWrapsAcrossPlayableEntries()
{
    const QVariantList channels = {
        channel(QStringLiteral("channel:2.1"), QStringLiteral("2.1"), QStringLiteral("News"), QStringLiteral("playable")),
        channel(QStringLiteral("channel:4.1"), QStringLiteral("4.1"), QStringLiteral("Sports"), QStringLiteral("restricted")),
        channel(QStringLiteral("channel:5.1"), QStringLiteral("5.1"), QStringLiteral("Movies"), QStringLiteral("playable")),
    };

    QCOMPARE(findAdjacentPlayableChannelRef(channels, QStringLiteral("channel:2.1"), -1), QStringLiteral("channel:5.1"));
}

void ChannelNavigationTests::navigationStartsAtFirstOrLastPlayableWhenIdle()
{
    const QVariantList channels = {
        channel(QStringLiteral("channel:4.1"), QStringLiteral("4.1"), QStringLiteral("Sports"), QStringLiteral("restricted")),
        channel(QStringLiteral("channel:5.1"), QStringLiteral("5.1"), QStringLiteral("Movies"), QStringLiteral("playable")),
        channel(QStringLiteral("channel:7.2"), QStringLiteral("7.2"), QStringLiteral("Drama"), QStringLiteral("playable")),
    };

    QCOMPARE(findAdjacentPlayableChannelRef(channels, QString(), 1), QStringLiteral("channel:5.1"));
    QCOMPARE(findAdjacentPlayableChannelRef(channels, QString(), -1), QStringLiteral("channel:7.2"));
}

QTEST_MAIN(ChannelNavigationTests)

#include "channelnavigation_tests.moc"