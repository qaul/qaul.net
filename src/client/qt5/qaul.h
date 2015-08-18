#ifndef QAUL_H
#define QAUL_H

#include <QMainWindow>
#include <QTimer>
#include <QString>
#include <QStringList>
#include <QProcess>

namespace Ui {
    class Qaul;
}

class Qaul : public QMainWindow
{
    Q_OBJECT

public:
    explicit Qaul(QWidget *parent = 0);
    ~Qaul();

private:
    Ui::Qaul *ui;

    int qaulConfigureCounter;
    void QaulConfigure(void);
    void QaulConfigureDelay(int msec);

    QProcess *qaulConfigProcess;

    QStringList qaulWifiInterfaceList;
    QString qaulWifiInterface;
    bool QaulWifiGetInterface(void);
    bool QaulWifiSearchInterfaces(void);
    bool QaulWifiInterfaceActive(QString interface);

    void QaulOlsrdStart(void);
    void QaulOlsrdStop(void);

    bool qaulTimersSet;
    QTimer *qaulTimerEvents;
    QTimer *qaulTimerSockets;
    QTimer *qaulTimerTopology;
    void QaulStartTimers(void);
    void QaulStopTimers(void);

    int  qaulFirewallCounter;
    void QaulConfigureFirewall(void);
    void QaulStopFirewall(void);

private slots:
    void QaulWifiConfigure(void);
    void QaulConfigureProcessRead(void);
    void QaulCheckEvents(void);
    void QaulCheckSockets(void);
    void QaulCheckTopology(void);
    void QaulConfigureDelayFired(void);
    bool QaulCopyDir(const QString &srcPath, const QString &dstPath);
};

#endif // QAUL_H
