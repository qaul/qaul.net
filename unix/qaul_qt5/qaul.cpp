#include "qaul.h"
#include "ui_qaul.h"
#include "../../libqaul/qaullib.h"

#include <QDebug>
#include <QByteArray>
#include <QDesktopServices>
#include <QUrl>
#include <QFileDialog>
#include <QDir>

Qaul::Qaul(QWidget *parent) :
    QMainWindow(parent),
    ui(new Ui::Qaul)
{ 
    QDir myDir;
    QString qaulLocalDirectory = QDir::homePath();
    qaulLocalDirectory += "/.qaul";
    qDebug() << "Local path: " << qaulLocalDirectory;

    // create ~/.qaul directory
    if(!myDir.exists(qaulLocalDirectory))
    {
        qDebug() << "Local path does not exist yet";
        if(myDir.mkpath(qaulLocalDirectory))
        {
            qDebug() << "Local path created";

            // copy www folder into ~/.qaul directory
            this->QaulCopyDir(QApplication::applicationDirPath() +"/www", qaulLocalDirectory +"/www");
            //QaulCopyDir(const QString &srcPath, const QString &dstPath)
        }
    }

    // initialize library
    //QString qpath = QApplication::applicationDirPath();
    Qaullib_Init(qaulLocalDirectory.toLocal8Bit().data());

    // set download folder
    QString qaulDownloadDirectory = QDir::homePath();
    qaulDownloadDirectory += "/Downloads";
    qDebug() << "download path: " << qaulDownloadDirectory;
    if(!myDir.exists(qaulDownloadDirectory))
    {
        qDebug() << "Download path does not exist yet";
        if(myDir.mkpath(qaulDownloadDirectory))
        {
            qDebug() << "Download path created";
        }
    }
    Qaullib_SetConfDownloadFolder(qaulDownloadDirectory.toLocal8Bit().data());

    // start web server
    // TODO: make QDialog::exec() as modal information for the user if web server failes
    if(!Qaullib_WebserverStart()) qDebug() << "web server startup failed";

    ui->setupUi(this);

    // configure qaul
    qaulConfigureCounter = -1;
    // how many firewall rules are set
    qaulFirewallCounter = 0;
    // wait a second to open the window
    QaulConfigureDelay(1000);
}

Qaul::~Qaul()
{
    qDebug() << "closing qaul ...";

    // stop timers
    if(qaulTimersSet) QaulStopTimers();

    // kill olsrd
    QaulOlsrdStop();

    // remove firewall rules
    QaulStopFirewall();

    // remove custom dns
    qaulConfigProcess->write("/bin/rm /etc/resolvconf/resolv.conf.d/tail \n");

    // start network manager
    qaulConfigProcess->write("/usr/bin/service network-manager start \n");
    qaulConfigProcess->write("exit \n");
    qaulConfigProcess->waitForFinished(-1);

    delete ui;
}

// -----------------------------------------------------------------
// configuration
// -----------------------------------------------------------------
void Qaul::QaulConfigure(void)
{
    // init
    if(qaulConfigureCounter == 0)
    {
        // everything is fine
        Qaullib_ConfigStart();
        qaulConfigureCounter = 10;
    }

    // check autorization
    if(qaulConfigureCounter == 10)
    {
        qaulConfigureCounter = 20;
    }

    // configure wifi
    // search interface
    if(qaulConfigureCounter == 20)
    {
        qDebug() << "[configure] search interface";
        if(QaulWifiGetInterface()) qaulConfigureCounter = 21;
        else
        {
            qDebug() << "no wifi interface found";
            // TODO: display error screen
        }
    }

    // configure interface
    if(qaulConfigureCounter == 21)
    {
        qDebug() << "[configure] configure interface";
        qaulConfigureCounter = 22;
        // open console with sudo
        qDebug() << "open console";
        qaulConfigProcess = new QProcess(this);
        connect(qaulConfigProcess, SIGNAL(started()), this, SLOT(QaulWifiConfigure()));
        connect(qaulConfigProcess, SIGNAL(readyReadStandardOutput()), this, SLOT(QaulConfigureProcessRead()));
        qaulConfigProcess->start("pkexec /bin/bash");
    }

    // wifi configures
    // wait 2 seconds
    if(qaulConfigureCounter == 29) QaulConfigureDelay(2000);

    // check if username is set
    if(qaulConfigureCounter == 30)
    {
        qDebug() << "[configure] check username";
        if(Qaullib_ExistsUsername()) qaulConfigureCounter = 40;
        else
        {
            qaulConfigureCounter--;
            QaulConfigureDelay(500);
        }
    }

    // start olsrd
    if(qaulConfigureCounter == 40)
    {
        qDebug() << "[configure] start olsrd";
        qaulConfigureCounter = 41;
        QaulOlsrdStart();
    }

    if(qaulConfigureCounter == 44) QaulConfigureDelay(2000);

    // connect ipc
    if(qaulConfigureCounter == 45)
    {
        qDebug() << "[configure] connect ipc";
        Qaullib_IpcConnect();
        qaulConfigureCounter = 46;
    }

    // start captive portal
    if(qaulConfigureCounter == 46)
    {
        Qaullib_SetConfVoIP();
        Qaullib_UDP_StartServer();
        Qaullib_CaptiveStart();
        QaulConfigureFirewall();
        qaulConfigureCounter = 50;
    }

    // start timers & finish
    if(qaulConfigureCounter == 50)
    {
        qDebug() << "[configure] timers & finish";
        QaulStartTimers();
        Qaullib_ConfigurationFinished();

        qaulConfigureCounter = 60;
    }

    // finished
}

// -----------------------------------------------------------------
void Qaul::QaulConfigureDelay(int msec)
{
    QTimer::singleShot(msec, this, SLOT(QaulConfigureDelayFired()));
}

void Qaul::QaulConfigureDelayFired(void)
{
    qDebug() << "configure delay fired";
    qaulConfigureCounter++;
    QaulConfigure();
}

// -----------------------------------------------------------------
// wifi
// -----------------------------------------------------------------
bool Qaul::QaulWifiGetInterface(void)
{
    // get all wifi interfaces
    // iwconfig
    QProcess *myProcess = new QProcess(this);
    myProcess->start("iwconfig");
    // wait until process has finished
    if(!myProcess->waitForFinished(10000))
    {
        qDebug() << "iwconfig process crashed";
        return false;
    }
    //QByteArray myOutput = myProcess->readAllStandardOutput();
    QString myString(myProcess->readAllStandardOutput());
    QRegExp myRx("^([\\S]+)");
    int myPos = 0;
    if((myPos = myRx.indexIn(myString, myPos)) == -1)
    {
        qDebug() << "iwconfig returned no interface";
        return false;
    }
    // take the first interface
    // TODO: make interface list
    qaulWifiInterface = myRx.cap(1);
    qDebug() << "interface found: " << qaulWifiInterface;

    // check if interface is active
    if(!QaulWifiInterfaceActive(qaulWifiInterface))
    {
        qDebug() << "interface not active";

        // deblock wifi
        myProcess->start("rfkill unblock all");
        if(!myProcess->waitForFinished(2000))
            qDebug() << "rfkill unblock process crashed";

        // try to activate interface if not available
        myProcess->start("nmcli nm wifi on");
        if(!myProcess->waitForFinished(10000))
            qDebug() << "nmcli process crashed";

        if(!QaulWifiInterfaceActive(qaulWifiInterface)) return false;
    }

    return true;
}

bool Qaul::QaulWifiInterfaceActive(QString interface)
{
    // get all active interfaces
    // ifconfig
    QProcess *myProcess = new QProcess(this);
    myProcess->start("ifconfig");
    // wait until process has finished
    if(!myProcess->waitForFinished(10000))
    {
        qDebug() << "ifconfig process crashed";
        return false;
    }
    //QByteArray myOutput = myProcess->readAllStandardOutput();
    QString myString(myProcess->readAllStandardOutput());
    QString myLine;
    QRegExp myRxLine("([^\\n]*)\\n");
    QRegExp myRx("^([\\S]+)");
    int myPos = 0;
    // loop through output line by line
    while((myPos = myRxLine.indexIn(myString, myPos)) != -1)
    {
        qDebug() << "[detect wifi] " << myRxLine.matchedLength() << " chars: " << myRxLine.cap(1);

        myLine = myRxLine.cap(1);
        // search for interface in this line
        if(myRx.indexIn(myLine, 0) != -1)
        {
            qDebug() << "interface compare: " << myRx.cap(1);
            if(QString::compare(myRx.cap(1), interface) == 0)
            {
                qDebug() << "comparison successful";
                return true;
            }
        }
        myPos += myRxLine.matchedLength();
    }
    qDebug() << "interface not active";
    return false;
}

void Qaul::QaulWifiConfigure(void)
{
    qDebug() << "process started";
/*
    //!!! did not work
    // stop network manager
    qaulConfigProcess->write("/usr/bin/service network-manager stop \n");
    // take wifi interface down
    qaulConfigProcess->write("/bin/ip link set ");
    qaulConfigProcess->write(qaulWifiInterface.toUtf8().constData());
    qaulConfigProcess->write(" down \n");
    // set adhoc mode
    qaulConfigProcess->write("/usr/sbin/iw dev ");
    qaulConfigProcess->write(qaulWifiInterface.toUtf8().constData());
    qaulConfigProcess->write(" mode ad-hoc \n");
    // bring wifi interface up
    qaulConfigProcess->write("/bin/ip link set ");
    qaulConfigProcess->write(qaulWifiInterface.toUtf8().constData());
    qaulConfigProcess->write(" up \n");
    // join / create network
    qaulConfigProcess->write("/usr/sbin/iw dev ");
    qaulConfigProcess->write(qaulWifiInterface.toUtf8().constData());
    qaulConfigProcess->write(" ibss join qaul.net 2462 \n");
    // set ip address
    qaulConfigProcess->write("/bin/ip addr add ");
    qaulConfigProcess->write(Qaullib_GetIP());
    qaulConfigProcess->write("/8 dev ");
    qaulConfigProcess->write(qaulWifiInterface.toUtf8().constData());
    qaulConfigProcess->write(" \n");
*/
    // -------------------------------------------------------
    // configure with iwconfig
    // -------------------------------------------------------
    // stop network manager
    qaulConfigProcess->write("/usr/bin/service network-manager stop \n");
    // take wifi interface down
    qaulConfigProcess->write("/bin/ip link set ");
    qaulConfigProcess->write(qaulWifiInterface.toUtf8().constData());
    qaulConfigProcess->write(" down \n");
    // set adhoc mode
    qaulConfigProcess->write("/sbin/iwconfig ");
    qaulConfigProcess->write(qaulWifiInterface.toUtf8().constData());
    qaulConfigProcess->write(" mode ad-hoc \n");
    // set channel
    qaulConfigProcess->write("/sbin/iwconfig ");
    qaulConfigProcess->write(qaulWifiInterface.toUtf8().constData());
    qaulConfigProcess->write(" channel 11 \n");
    // set essid
    qaulConfigProcess->write("/sbin/iwconfig ");
    qaulConfigProcess->write(qaulWifiInterface.toUtf8().constData());
    qaulConfigProcess->write(" essid 'qaul.net' \n");
    // bring wifi interface up
    qaulConfigProcess->write("/bin/ip link set ");
    qaulConfigProcess->write(qaulWifiInterface.toUtf8().constData());
    qaulConfigProcess->write(" up \n");
    // set ip address
    qaulConfigProcess->write("/bin/ip addr add ");
    qaulConfigProcess->write(Qaullib_GetIP());
    qaulConfigProcess->write("/8 dev ");
    qaulConfigProcess->write(qaulWifiInterface.toUtf8().constData());
    qaulConfigProcess->write(" broadcast 10.255.255.255 \n");

    // write wifi sucess token
    qaulConfigProcess->write("/bin/echo 'qaulTokenWifi' \n");

    // set dns server manually
    // TODO: preserve old tail / make all configuration via nm
    qaulConfigProcess->write("/bin/rm /etc/resolvconf/resolv.conf.d/tail \n");
    QString myPath = QApplication::applicationDirPath();
    QString myCmd;
    myCmd = "/bin/cp " +myPath +"/tail /etc/resolvconf/resolv.conf.d/tail \n";
    qaulConfigProcess->write(myCmd.toUtf8().constData());
    qaulConfigProcess->write("resolvconf -u \n");

    qDebug() << "process cmd written";

    //qaulConfigureCounter = 30;
    //QaulConfigure();
}

// read configuring root process output line by line
// and search for sucess tokens
void Qaul::QaulConfigureProcessRead(void)
{
    QString myString(qaulConfigProcess->readAllStandardOutput());
    QString myLine;
    QRegExp myRxLine("([^\\n]*)\\n");
    QRegExp myRxWifi("^(qaulTokenWifi)");
    QRegExp myRxOlsr("^(qaulTokenOlsr)");
    int myPos = 0;
    // loop through output line by line
    while((myPos = myRxLine.indexIn(myString, myPos)) != -1)
    {
        qDebug() << "[config process] " << myRxLine.matchedLength() << " chars: " << myRxLine.cap(1);

        // search for tokens in this line
        if(qaulConfigureCounter == 22)
        {
            myLine = myRxLine.cap(1);
            if(myRxWifi.indexIn(myLine, 0) != -1)
            {
                qDebug() << "[SUCCESS] " << myRxWifi.cap(1);
                qaulConfigureCounter = 29;
                QaulConfigure();
            }
        }
        else if(qaulConfigureCounter == 41)
        {
            myLine = myRxLine.cap(1);
            if(myRxOlsr.indexIn(myLine, 0) != -1)
            {
                qDebug() << "[SUCCESS] " << myRxOlsr.cap(1);
                qaulConfigureCounter = 44;
                QaulConfigure();
            }
        }
        myPos += myRxLine.matchedLength();
    }
}

void Qaul::QaulConfigureFirewall(void)
{
    qDebug() << "configure firewall";

    // configure iptables
    // set rules for captive portal
    qaulConfigProcess->write("iptables -t nat -I PREROUTING 1 -i ");
    qaulConfigProcess->write(qaulWifiInterface.toUtf8().constData());
    qaulConfigProcess->write(" -p tcp -d ");
    qaulConfigProcess->write(Qaullib_GetIP());
    qaulConfigProcess->write(" --dport 80 -j REDIRECT --to-port 8081 \n");
    qaulFirewallCounter++;

    qaulConfigProcess->write("iptables -t nat -I PREROUTING 1 -i ");
    qaulConfigProcess->write(qaulWifiInterface.toUtf8().constData());
    qaulConfigProcess->write(" -p udp -d ");
    qaulConfigProcess->write(Qaullib_GetIP());
    qaulConfigProcess->write(" --dport 53 -j REDIRECT --to-port 8053 \n");
    qaulFirewallCounter++;

    // start portfwd for DHCP
    // (netconfig/iptables can't handle 0.0.0.0 packages)
    QString myPath = QApplication::applicationDirPath();
    QString myCmd;
    myCmd = myPath +"/portfwd -c " +myPath +"/portfwd.conf \n";
    qaulConfigProcess->write(myCmd.toUtf8().constData());

    qDebug() << "command: " << myCmd;

    qDebug() << "firewall configured";
}

void Qaul::QaulStopFirewall(void)
{
    qDebug() << "stop firewall";

    // stop port forwarding
    qaulConfigProcess->write("killall portfwd \n");

    // remove iptables rules
    for(; qaulFirewallCounter > 0; qaulFirewallCounter--)
    {
        qaulConfigProcess->write("iptables -t nat -D PREROUTING 1 \n");
    }

    qDebug() << "firewall stopped";
}

// -----------------------------------------------------------------
// olsrd
// -----------------------------------------------------------------
void Qaul::QaulOlsrdStart(void)
{
    qDebug() << "start olsrd";

    QString myPath = QApplication::applicationDirPath();
    QString myCmd;

    myCmd  = "cd ";
    myCmd += myPath;
    myCmd += " \n";
    qDebug() << myCmd;
    qaulConfigProcess->write(myCmd.toUtf8().constData());

    myCmd = myPath;
    myCmd += "/olsrd -i ";
    myCmd += qaulWifiInterface;
    myCmd += " -f ";
    myCmd += myPath;
    myCmd += "/olsrd_linux.conf -d 0 \n";
    qDebug() << myCmd;
    qaulConfigProcess->write(myCmd.toUtf8().constData());

    /*
    // change into qaul directory
    qaulConfigProcess->write("cd ");
    qaulConfigProcess->write(myPath.toUtf8().constData());
    qaulConfigProcess->write(" \n");
    // start olsrd
    qaulConfigProcess->write(myPath.toUtf8().constData());
    qaulConfigProcess->write("/olsrd -i ");
    qaulConfigProcess->write(qaulWifiInterface.toUtf8().constData());
    qaulConfigProcess->write(" -f ");
    qaulConfigProcess->write(myPath.toUtf8().constData());
    qaulConfigProcess->write("/olsrd_linux.conf -d 0 \n");
    */

    // write olsr sucess token
    qaulConfigProcess->write("/bin/echo 'qaulTokenOlsr' \n");
}

void Qaul::QaulOlsrdStop(void)
{
    qDebug() << "stop olsrd";
    qaulConfigProcess->write("killall olsrd \n");
}

// -----------------------------------------------------------------
// timers
// -----------------------------------------------------------------
void Qaul::QaulStartTimers(void)
{
    qaulTimerEvents = new QTimer(this);
    connect(qaulTimerEvents, SIGNAL(timeout()), this, SLOT(QaulCheckEvents()));
    qaulTimerEvents->start(10);

    qaulTimerSockets = new QTimer(this);
    connect(qaulTimerSockets, SIGNAL(timeout()), this, SLOT(QaulCheckSockets()));
    qaulTimerSockets->start(100);

    qaulTimerTopology = new QTimer(this);
    connect(qaulTimerTopology, SIGNAL(timeout()), this, SLOT(QaulCheckTopology()));
    qaulTimerTopology->start(5000);

    qaulTimersSet = true;
}

void Qaul::QaulStopTimers(void)
{
    qaulTimerEvents->stop();
    qaulTimerSockets->stop();
    qaulTimerTopology->stop();
    qaulTimersSet = true;
}

void Qaul::QaulCheckEvents(void)
{
    int myEvent = Qaullib_TimedCheckAppEvent();

    if(myEvent > 0)
    {
        if(myEvent == QAUL_EVENT_CHOOSEFILE)
        {
            // open file selection
            QString fileName = QFileDialog::getOpenFileName(
                        this,
                        tr("Add File"),
                        "/home",
                        tr("Files (*.*)"));
            // file was selected
            if(fileName != NULL)
            {
                qDebug() << "file selected " << fileName;
                Qaullib_FilePicked(2, fileName.toLocal8Bit().data());
            }
            else
            {
                qDebug() << "file selection canceld ";
            }

        }
        else if(myEvent == QAUL_EVENT_OPENFILE)
        {
            // open file in default application
            QString filepath = "file://";
            filepath += Qaullib_GetAppEventOpenPath();
            qDebug() << filepath;
            QDesktopServices::openUrl(QUrl(filepath));
        }
        else if(myEvent == QAUL_EVENT_OPENURL)
        {
            // open URL in browser
            QString urlpath = Qaullib_GetAppEventOpenURL();
            qDebug() << urlpath;
            QDesktopServices::openUrl(QUrl(urlpath));
        }
        else if(myEvent == QAUL_EVENT_QUIT)
        {
            // quit application
            QApplication::quit();
        }
        else if(myEvent == QAUL_EVENT_NOTIFY && myEvent == QAUL_EVENT_RING)
        {
            // play beep
            // does not work under linux
            QApplication::beep();
        }
    }
}

void Qaul::QaulCheckSockets(void)
{
    //qDebug() << "QaulCheckSockets()";
    Qaullib_TimedSocketReceive();
}

void Qaul::QaulCheckTopology(void)
{
    Qaullib_IpcSendCom(1);
    Qaullib_TimedDownload();
}

bool Qaul::QaulCopyDir(const QString &srcPath, const QString &dstPath)
{
    //rmDir(dstPath);
    QDir parentDstDir(QFileInfo(dstPath).path());
    if (!parentDstDir.mkdir(QFileInfo(dstPath).fileName()))
        return false;

    QDir srcDir(srcPath);
    foreach(const QFileInfo &info, srcDir.entryInfoList(QDir::Dirs | QDir::Files | QDir::NoDotAndDotDot))
    {
        QString srcItemPath = srcPath + "/" + info.fileName();
        QString dstItemPath = dstPath + "/" + info.fileName();
        if (info.isDir())
        {
            if (!this->QaulCopyDir(srcItemPath, dstItemPath))
            {
                return false;
            }
        }
        else if (info.isFile())
        {
            if (!QFile::copy(srcItemPath, dstItemPath))
            {
                return false;
            }
        }
        else
        {
            qDebug() << "Unhandled item" << info.filePath() << "in cpDir";
        }
    }
    return true;
}
