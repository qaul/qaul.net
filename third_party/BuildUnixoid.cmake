
ExternalProject_Add(pjsip
    DEPENDS dl_pjsip
    PREFIX ${CMAKE_CURRENT_BINARY_DIR}/pjsip
    URL "${ARCHIVE_DIR}/${PJSIP_FILENAME}"
    CONFIGURE_COMMAND ""
    BUILD_COMMAND ""
    INSTALL_COMMAND ""
)

ExternalProject_Add(oslrd
    DEPENDS dl_olsr
    PREFIX ${CMAKE_CURRENT_BINARY_DIR}/olsrd
    URL ${ARCHIVE_DIR}/${OLSR_FILENAME}
    #URL http://www.olsr.org/releases/0.6/olsrd-0.6.7.1.tar.gz
    #DOWNLOAD_DIR ${PROJECT_SOURCE_DIR}/archives
    CONFIGURE_COMMAND ""
    BUILD_COMMAND ""
    INSTALL_COMMAND ""
)
