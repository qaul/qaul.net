#-------------------------------------------------
#
# Project created by QtCreator 2012-05-13T23:44:54
#
#-------------------------------------------------

QT       += core gui webkit

TARGET = qaul
TEMPLATE = app


SOURCES += main.cpp\
        qaul.cpp

HEADERS  += qaul.h

FORMS    += qaul.ui

LIBS     += \
    -L../../libqaul -lqaul \
    \
    -L../../pjproject-2.0.1/pjlib/lib \
    -L../../pjproject-2.0.1/pjlib-util/lib \
    -L../../pjproject-2.0.1/pjmedia/lib \
    -L../../pjproject-2.0.1/pjnath/lib \
    -L../../pjproject-2.0.1/pjsip/lib \
    -L../../pjproject-2.0.1/third_party/lib \
    -lpjsua-i686-pc-linux-gnu \
    -lpjsip-ua-i686-pc-linux-gnu \
    -lpjsip-simple-i686-pc-linux-gnu \
    -lpjsip-i686-pc-linux-gnu \
    -lpjmedia-codec-i686-pc-linux-gnu \
    -lpjmedia-videodev-i686-pc-linux-gnu \
    -lpjmedia-i686-pc-linux-gnu \
    -lpjmedia-audiodev-i686-pc-linux-gnu \
    -lpjnath-i686-pc-linux-gnu \
    -lpjlib-util-i686-pc-linux-gnu \
    -lresample-i686-pc-linux-gnu \
    -lmilenage-i686-pc-linux-gnu \
    -lsrtp-i686-pc-linux-gnu \
    -lgsmcodec-i686-pc-linux-gnu \
    -lspeex-i686-pc-linux-gnu \
    -lilbccodec-i686-pc-linux-gnu \
    -lg7221codec-i686-pc-linux-gnu \
    -lportaudio-i686-pc-linux-gnu \
    -lpj-i686-pc-linux-gnu \
    \
    -L/usr/local/lib \
    -lavformat \
    -lavcodec \
    -lswscale \
    -lavutil \
    -lasound \
    -lcrypto \
    -lssl


OTHER_FILES += \
    olsrd_linux.conf \
    portfwd.conf \
    tail \
    copy_files.sh

copyfiles.commands  = ../qaul/copy_files.sh
QMAKE_EXTRA_TARGETS += copyfiles
POST_TARGETDEPS += copyfiles
