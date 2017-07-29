add_subdirectory (src/qaulhelper)

configure_file("${PROJECT_SOURCE_DIR}/distfiles/osx/installer/make_dmg.sh.in" "${PROJECT_BINARY_DIR}/make_dmg.sh")
configure_file("${PROJECT_SOURCE_DIR}/distfiles/osx/installer/qaul.xml.in" "${PROJECT_BINARY_DIR}/qaul.xml")

INSTALL( DIRECTORY ${PROJECT_SOURCE_DIR}/distfiles/osx/etc DESTINATION ${CMAKE_INSTALL_PREFIX} )

INSTALL(FILES ${PROJECT_BINARY_DIR}/third_party/olsr/src/olsr/olsrd DESTINATION bin
	PERMISSIONS OWNER_READ OWNER_WRITE OWNER_EXECUTE GROUP_READ GROUP_EXECUTE WORLD_READ WORLD_EXECUTE)

INSTALL(FILES ${PROJECT_BINARY_DIR}/third_party/socat/src/socat/socat DESTINATION bin
	PERMISSIONS OWNER_READ OWNER_WRITE OWNER_EXECUTE GROUP_READ GROUP_EXECUTE WORLD_READ WORLD_EXECUTE)

INSTALL(FILES ${PROJECT_BINARY_DIR}/src/olsrd-plugin/olsrd_qaul.so.0.1 DESTINATION lib
	PERMISSIONS OWNER_READ OWNER_WRITE OWNER_EXECUTE GROUP_READ GROUP_EXECUTE WORLD_READ WORLD_EXECUTE)

## Since we don't patch olsr anymore, dyn_gw does not get built.
## It was most probably not working on OSX anyway.
## TODO: find another solution for the Internet sharing.
#INSTALL(FILES ${PROJECT_BINARY_DIR}/third_party/olsr/src/olsr/lib/dyn_gw/olsrd_dyn_gw.so.0.5 DESTINATION lib
#	PERMISSIONS OWNER_READ OWNER_WRITE OWNER_EXECUTE GROUP_READ GROUP_EXECUTE WORLD_READ WORLD_EXECUTE)

include(cmake/PacketFormatGuesser.cmake)

if(PKGFORMAT MATCHES "AUTO")
  SET(CPACK_GENERATOR ${SPECIFIC_SYSTEM_PREFERED_CPACK_GENERATOR})
else()
  SET(CPACK_GENERATOR ${PKGFORMAT})
endif()

SET(CPACK_SET_DESTDIR ON)

SET(CPACK_BUNDLE_NAME qaul)
SET(CPACK_BUNDLE_PLIST ${PROJECT_SOURCE_DIR}/src/client/osx/Info.plist)
SET(CPACK_BUNDLE_ICON ${PROJECT_SOURCE_DIR}/distfiles/osx/qaul.icns)

# All install must be done before this
INCLUDE(CPack)
