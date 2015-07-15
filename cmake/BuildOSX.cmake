
find_package (PkgConfig)
pkg_check_modules (DBUS1 REQUIRED dbus-1)

find_package (Autotools REQUIRED)

if (${PORT} STREQUAL "GTK")
    pkg_search_module (WEBKIT REQUIRED webkitgtk-3.0 webkit-1.0)
endif ()

add_subdirectory (unix/qaulhelper)
add_subdirectory (unix/qaul_osx)

#INSTALL( DIRECTORY ${PROJECT_SOURCE_DIR}/www DESTINATION ${CMAKE_INSTALL_PREFIX} )
#INSTALL( DIRECTORY ${PROJECT_SOURCE_DIR}/unix/distfiles/etc DESTINATION ${CMAKE_INSTALL_PREFIX} )

#install(FILES ${PROJECT_BINARY_DIR}/third_party/olsr/src/olsr/olsrd DESTINATION bin
#	PERMISSIONS OWNER_READ OWNER_WRITE OWNER_EXECUTE GROUP_READ GROUP_EXECUTE WORLD_READ WORLD_EXECUTE)

#install(FILES ${PROJECT_BINARY_DIR}/third_party/olsr/src/olsr/lib/olsrd_qaul/olsrd_qaul.so.0.1 DESTINATION lib
#	PERMISSIONS OWNER_READ OWNER_WRITE OWNER_EXECUTE GROUP_READ GROUP_EXECUTE WORLD_READ WORLD_EXECUTE)

include(cmake/PacketFormatGuesser.cmake)

if(PKGFORMAT MATCHES "AUTO")
  SET(CPACK_GENERATOR ${SPECIFIC_SYSTEM_PREFERED_CPACK_GENERATOR})
else()
  SET(CPACK_GENERATOR ${PKGFORMAT})
endif()

SET(CPACK_SET_DESTDIR ON)

SET(CPACK_BUNDLE_NAME qaul)
SET(CPACK_BUNDLE_PLIST ${PROJECT_SOURCE_DIR}/unix/qaul_osx/Info.plist)
SET(CPACK_BUNDLE_ICON ${PROJECT_SOURCE_DIR}/distfiles/osx/qaul.icns)

# All install must be done before this
INCLUDE(CPack)
