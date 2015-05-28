
find_package (PkgConfig)
pkg_check_modules (DBUS1 REQUIRED dbus-1)

find_package (Autotools REQUIRED)

if (${PORT} STREQUAL "GTK")
    pkg_search_module (WEBKIT REQUIRED webkitgtk-3.0 webkit-1.0)
endif ()

add_subdirectory (unix/qaulhelper)
add_subdirectory (unix/qaul_gtk)

INSTALL( DIRECTORY ${PROJECT_SOURCE_DIR}/www DESTINATION ${CMAKE_INSTALL_PREFIX} )
INSTALL( DIRECTORY ${PROJECT_SOURCE_DIR}/unix/distfiles/etc DESTINATION ${CMAKE_INSTALL_PREFIX} )

install(FILES ${PROJECT_BINARY_DIR}/third_party/olsr/src/olsr/olsrd DESTINATION bin
	PERMISSIONS OWNER_READ OWNER_WRITE OWNER_EXECUTE GROUP_READ GROUP_EXECUTE WORLD_READ WORLD_EXECUTE)

install(FILES ${PROJECT_BINARY_DIR}/third_party/portfwd/src/portfwd/src/portfwd DESTINATION bin
	PERMISSIONS OWNER_READ OWNER_WRITE OWNER_EXECUTE GROUP_READ GROUP_EXECUTE WORLD_READ WORLD_EXECUTE)

install(FILES ${PROJECT_BINARY_DIR}/third_party/olsr/src/olsr/lib/olsrd_qaul/olsrd_qaul.so.0.1 DESTINATION lib
	PERMISSIONS OWNER_READ OWNER_WRITE OWNER_EXECUTE GROUP_READ GROUP_EXECUTE WORLD_READ WORLD_EXECUTE)

include(SystemSpecificInformations.cmake)

if(PKGFORMAT MATCHES "AUTO")
  SET(CPACK_GENERATOR ${SPECIFIC_SYSTEM_PREFERED_CPACK_GENERATOR})
else()
  SET(CPACK_GENERATOR ${PKGFORMAT})
endif()

SET(CPACK_SET_DESTDIR ON)


############################
# CPACK Installer
############################
# Configure Debian Installer
SET(CPACK_DEBIAN_PACKAGE_MAINTAINER  "qaul.net community <contact@qaul.net>")
SET(CPACK_DEBIAN_PACKAGE_NAME        "qaul.net ${VERSION_SUFFIX}")
SET(CPACK_DEBIAN_PACKAGE_VERSION     "${CPACK_PACKAGE_VERSION}${CPACK_PACKAGE_REVISION}")
SET(CPACK_DEBIAN_PACKAGE_DESCRIPTION "tools for the next revolution
 independent WIFI mesh network,
 text messaging, chat,
 file sharing,
 VoIP, voice calls,
 olsr")
SET(CPACK_DEBIAN_PACKAGE_PRIORITY    "optional")
SET(CPACK_DEBIAN_PACKAGE_SECTION     "web")
SET(CPACK_DEBIAN_PACKAGE_DEPENDS     "libwebkit-3.0-0")
#SET(CPACK_PACKAGE_FILE_NAME "${CPACK_DEBIAN_PACKAGE_NAME}-${CPACK_DEBIAN_PACKAGE_VERSION}_${CMAKE_SYSTEM_PROCESSOR}")
SET(CPACK_DEBIAN_PACKAGE_HOMEPAGE    "http://qaul.net")

# copy application icon
INSTALL(FILES ${PROJECT_SOURCE_DIR}/unix/distfiles/share/qaul_app_icon.png DESTINATION /opt/qaul/)

# add qaul to applications menu 
INSTALL(FILES ${PROJECT_SOURCE_DIR}/unix/distfiles/share/qaul.desktop DESTINATION /usr/share/applications/)

# All install must be done before this
INCLUDE(CPack)
