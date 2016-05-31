
ExternalProject_Add(pjsip
    DEPENDS dl_pjsip
    PREFIX ${CMAKE_CURRENT_BINARY_DIR}/pjsip
    URL ${ARCHIVE_DIR}/${PJSIP_FILENAME}
    BUILD_IN_SOURCE 1
    PATCH_COMMAND patch -p1 -t -N -i ${CMAKE_CURRENT_SOURCE_DIR}/pjsip.patch
    CONFIGURE_COMMAND ./aconfigure --prefix=${CMAKE_INSTALL_PREFIX} --disable-ffmpeg --disable-ssl --disable-video --disable-gsm-codec ${PJSIP_CONFIGURE_OPTIONS}
    BUILD_COMMAND make dep COMMAND make
    INSTALL_COMMAND ""
)

ExternalProject_Add(olsr
    DEPENDS dl_olsr
    PREFIX ${CMAKE_CURRENT_BINARY_DIR}/olsr
    URL ${ARCHIVE_DIR}/${OLSR_FILENAME}
    BUILD_IN_SOURCE 1
    PATCH_COMMAND patch -p1 -t -N -i ${CMAKE_CURRENT_SOURCE_DIR}/olsr.patch
    COMMAND ${CMAKE_COMMAND} -DSRC=${CMAKE_SOURCE_DIR}/src/olsrd_qaul -DDEST=${CMAKE_CURRENT_BINARY_DIR}/olsr/src/olsr/lib -P ${CMAKE_SOURCE_DIR}/cmake/FileCopy.cmake
    COMMAND patch -p1 -t -N -i ${CMAKE_CURRENT_SOURCE_DIR}/olsr_OSX10.10.patch
    CONFIGURE_COMMAND ""
    BUILD_COMMAND make DEBUG=0 olsrd libs
    INSTALL_COMMAND ""
)

set (SOCAT_VERSION "1.7.3.0")
set (SOCAT_FILENAME "socat-${SOCAT_VERSION}.tar.bz2")
set (SOCAT_URL "http://www.dest-unreach.org/socat/download/${SOCAT_FILENAME}")
set (SOCAT_MD5 "b607edb65bc6c57f4a43f06247504274")

add_custom_target(dl_socat
    COMMAND ${CMAKE_COMMAND} -DDL_URL=${SOCAT_URL} -DDL_FILENAME=${ARCHIVE_DIR}/${SOCAT_FILENAME} -DDL_MD5=${SOCAT_MD5} -P ${CMAKE_CURRENT_SOURCE_DIR}/download.cmake 
)

ExternalProject_Add(socat
    DEPENDS dl_socat
    PREFIX ${CMAKE_CURRENT_BINARY_DIR}/socat
    URL ${ARCHIVE_DIR}/${SOCAT_FILENAME}
    BUILD_IN_SOURCE 1
    CONFIGURE_COMMAND ./configure --disable-openssl --disable-readline
    BUILD_COMMAND make
    INSTALL_COMMAND ""
)
