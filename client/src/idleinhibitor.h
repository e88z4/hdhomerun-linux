#pragma once

#include <QObject>

class IdleInhibitor : public QObject
{
    Q_OBJECT

public:
    explicit IdleInhibitor(QObject *parent = nullptr);
    ~IdleInhibitor() override;

    void setPlaybackActive(bool active);

private:
    void acquire();
    void release();

    bool m_playbackActive;
    unsigned int m_screenSaverCookie;
    int m_login1Fd;
};