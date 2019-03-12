
find_package (PkgConfig)

option (VOIP "Enable VOIP" ON)
option (DBUS "Use DBus for IPC" ON)
option (USE_SYSTEM_OLSRD "Use sytems olsrd package" OFF)
option (PORTFWD "Build portfwd tool" ON)

if (DBUS)
    pkg_check_modules (DBUS1 REQUIRED dbus-1)
endif ()

pkg_check_modules (UUID uuid)

if (VOIP)
    pkg_check_modules (OPENCORE_AMRNB opencore-amrnb)
    pkg_check_modules (OPENCORE_AMRWB opencore-amrwb)
endif ()

if (NOT USE_SYSTEM_OLSRD)
    find_package (Autotools REQUIRED)
    find_package (BISON REQUIRED) # olsr
    find_package (FLEX REQUIRED) # olsr
endif ()

add_subdirectory (src/qaulhelper)

configure_file (
  "${PROJECT_SOURCE_DIR}/distfiles/linux/share/qaul.desktop.in"
  "${PROJECT_BINARY_DIR}/distfiles/linux/share/qaul.desktop"
)

configure_file (
  "${PROJECT_SOURCE_DIR}/distfiles/linux/etc/olsrd_linux.conf.in"
  "${PROJECT_BINARY_DIR}/distfiles/linux/etc/olsrd_linux.conf"
)

configure_file (
  "${PROJECT_SOURCE_DIR}/distfiles/linux/etc/olsrd_linux_gw.conf.in"
  "${PROJECT_BINARY_DIR}/distfiles/linux/etc/olsrd_linux_gw.conf"
)

configure_file (
  "${PROJECT_SOURCE_DIR}/distfiles/linux/bin/qaul.in"
  "${PROJECT_BINARY_DIR}/distfiles/linux/bin/${GUINAME}"
)

if (NOT ${QAUL_BINDIR} STREQUAL "NONE")
  install(FILES ${PROJECT_BINARY_DIR}/distfiles/linux/bin/${GUINAME} DESTINATION ${QAUL_BINDIR}
	  PERMISSIONS OWNER_READ OWNER_WRITE OWNER_EXECUTE GROUP_READ GROUP_EXECUTE WORLD_READ WORLD_EXECUTE)
endif()

INSTALL( DIRECTORY ${PROJECT_SOURCE_DIR}/GUI/www DESTINATION lib/qaul )
INSTALL( DIRECTORY ${PROJECT_SOURCE_DIR}/GUI/files DESTINATION lib/qaul )
INSTALL( DIRECTORY ${PROJECT_SOURCE_DIR}/distfiles/linux/etc DESTINATION lib/qaul
         PATTERN "*.in" EXCLUDE )
INSTALL( DIRECTORY ${PROJECT_BINARY_DIR}/distfiles/linux/etc DESTINATION lib/qaul )

install(FILES ${PROJECT_BINARY_DIR}/third_party/olsr/src/olsr/olsrd DESTINATION lib/qaul/bin
	PERMISSIONS OWNER_READ OWNER_WRITE OWNER_EXECUTE GROUP_READ GROUP_EXECUTE WORLD_READ WORLD_EXECUTE)

if (PORTFWD)
install(FILES ${PROJECT_BINARY_DIR}/third_party/portfwd/src/portfwd/src/portfwd DESTINATION lib/qaul/bin
	PERMISSIONS OWNER_READ OWNER_WRITE OWNER_EXECUTE GROUP_READ GROUP_EXECUTE WORLD_READ WORLD_EXECUTE)
endif()

if (NOT USE_SYSTEM_OLSRD)
install(FILES ${PROJECT_BINARY_DIR}/third_party/olsr/src/olsr/lib/dyn_gw/olsrd_dyn_gw.so.0.5 DESTINATION lib/qaul/lib
	PERMISSIONS OWNER_READ OWNER_WRITE OWNER_EXECUTE GROUP_READ GROUP_EXECUTE WORLD_READ WORLD_EXECUTE)
endif()

include(cmake/PacketFormatGuesser.cmake)

if(PKGFORMAT MATCHES "AUTO")
  SET(CPACK_GENERATOR ${SPECIFIC_SYSTEM_PREFERED_CPACK_GENERATOR})
else()
  SET(CPACK_GENERATOR ${PKGFORMAT})
endif()

SET(CPACK_SET_DESTDIR ON)

SET(CPACK_SOURCE_IGNORE_FILES
"~$"
"^${PROJECT_SOURCE_DIR}/archives/[^\\\\.]"
"^${PROJECT_SOURCE_DIR}/build/"
)

############################
# CPACK Installer
############################
# Configure Debian Installer
SET(CPACK_DEBIAN_PACKAGE_MAINTAINER  "qaul.net community <contact@qaul.net>")
#SET(CPACK_DEBIAN_PACKAGE_NAME        "qaul.net ${VERSION_SUFFIX}")
SET(CPACK_DEBIAN_PACKAGE_VERSION     "${CPACK_PACKAGE_VERSION}${CPACK_PACKAGE_REVISION}")
SET(CPACK_DEBIAN_PACKAGE_DESCRIPTION "Wifi-mesh communication app,
 decentralized, Internet independent,
 text messaging, chat,
 file sharing,
 VoIP, voice calls,
 olsr")
SET(CPACK_DEBIAN_PACKAGE_PRIORITY    "optional")
SET(CPACK_DEBIAN_PACKAGE_SECTION     "web")
SET(CPACK_DEBIAN_PACKAGE_DEPENDS     "libwebkitgtk-3.0-0")
#SET(CPACK_PACKAGE_FILE_NAME          "${CPACK_DEBIAN_PACKAGE_NAME}-${CPACK_DEBIAN_PACKAGE_VERSION}_${CMAKE_SYSTEM_PROCESSOR}")
SET(CPACK_PACKAGE_FILE_NAME          "${CMAKE_PROJECT_NAME}_${CPACK_PACKAGE_VERSION}_${CMAKE_SYSTEM_PROCESSOR}")
SET(CPACK_DEBIAN_PACKAGE_HOMEPAGE    "http://qaul.net")

# copy application icon
INSTALL(FILES ${PROJECT_SOURCE_DIR}/distfiles/linux/share/qaul_app_icon.png DESTINATION ${QAUL_ICONDIR})

# add qaul to applications menu 
INSTALL(FILES ${PROJECT_BINARY_DIR}/distfiles/linux/share/qaul.desktop DESTINATION ${QAUL_DESKTOPDIR})

# All install must be done before this
INCLUDE(CPack)
