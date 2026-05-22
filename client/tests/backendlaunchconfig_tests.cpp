#include "backendlaunchconfig.h"

#include <QtTest>

class BackendLaunchConfigTests : public QObject
{
    Q_OBJECT

private slots:
    void resolvesDefaultLoopbackBindAddress();
    void resolvesCustomLoopbackPort();
    void rejectsNonLocalHosts();
    void rejectsHttpsUrls();
};

void BackendLaunchConfigTests::resolvesDefaultLoopbackBindAddress()
{
    const auto decision = resolveBackendLaunchDecision(QStringLiteral("http://127.0.0.1:38080"));

    QVERIFY(decision.canAutoStart);
    QCOMPARE(decision.bindAddress, QStringLiteral("127.0.0.1:38080"));
    QVERIFY(decision.errorMessage.isEmpty());
}

void BackendLaunchConfigTests::resolvesCustomLoopbackPort()
{
    const auto decision = resolveBackendLaunchDecision(QStringLiteral("http://localhost:39090"));

    QVERIFY(decision.canAutoStart);
    QCOMPARE(decision.bindAddress, QStringLiteral("localhost:39090"));
    QVERIFY(decision.errorMessage.isEmpty());
}

void BackendLaunchConfigTests::rejectsNonLocalHosts()
{
    const auto decision = resolveBackendLaunchDecision(QStringLiteral("http://192.168.1.20:38080"));

    QVERIFY(!decision.canAutoStart);
    QVERIFY(decision.bindAddress.isEmpty());
    QVERIFY(!decision.errorMessage.isEmpty());
}

void BackendLaunchConfigTests::rejectsHttpsUrls()
{
    const auto decision = resolveBackendLaunchDecision(QStringLiteral("https://127.0.0.1:38080"));

    QVERIFY(!decision.canAutoStart);
    QVERIFY(decision.bindAddress.isEmpty());
    QVERIFY(!decision.errorMessage.isEmpty());
}

QTEST_MAIN(BackendLaunchConfigTests)

#include "backendlaunchconfig_tests.moc"