
set (ARCHIVE_DIR ${PROJECT_SOURCE_DIR}/archives)

set (PJSIP_VERSION "2.3")
set (PJSIP_FILENAME "pjproject-${PJSIP_VERSION}.tar.bz2")
set (PJSIP_URL "http://www.pjsip.org/release/2.3/${PJSIP_FILENAME}")
set (PJSIP_MD5 "8440e43242c439ae5ec30b5b85005fce")

set (OLSR_VERSION "0.6.7.1")
set (OLSR_FILENAME "olsrd-${OLSR_VERSION}.tar.bz2")
set (OLSR_URL "http://www.olsr.org/releases/0.6/${OLSR_FILENAME}")
set (OLSR_MD5 "d4411d4ed94b399a3778b8036dee1214")

add_custom_target(dl_pjsip
    COMMAND ${CMAKE_COMMAND} -DDL_URL=${PJSIP_URL} -DDL_FILENAME=${ARCHIVE_DIR}/${PJSIP_FILENAME} -DDL_MD5=${PJSIP_MD5} -P ${CMAKE_CURRENT_SOURCE_DIR}/download.cmake 
)

add_custom_target(dl_olsr
    COMMAND ${CMAKE_COMMAND} -DDL_URL=${OLSR_URL} -DDL_FILENAME=${ARCHIVE_DIR}/${OLSR_FILENAME} -DDL_MD5=${OLSR_MD5} -P ${CMAKE_CURRENT_SOURCE_DIR}/download.cmake 
)

if (${PORT} STREQUAL "Android")
    include(BuildAndroid.cmake)
elseif (${PORT} STREQUAL "OpenWrt")
    include(BuildOpenWrt.cmake)
else ()
    include(BuildUnixoid.cmake)
endif ()
