#include <QtGui/QApplication>
#include "qaul.h"

int main(int argc, char *argv[])
{
    QApplication a(argc, argv);

    Qaul w;
    w.show();

    return a.exec();
}
