# -----------------------------------------------------------------------------
# Determine the operating system
# -----------------------------------------------------------------------------
#if (UNIX)
#  if (APPLE)
#    set(Q_OS_MAC_OS_X 1)
#  else ()
#    set(Q_OS_UNIX 1)
#  endif ()
#elseif (CMAKE_SYSTEM_NAME MATCHES "Windows")
#  set(Q_OS_WINDOWS 1)
#else ()
#  message(FATAL_ERROR "Unknown OS '${CMAKE_SYSTEM_NAME}'")
#endif ()


# mandatory stuff
#find_package (PkgConfig)
#pkg_check_modules (DBUS1 REQUIRED dbus-1)
#pkg_search_module (WEBKIT REQUIRED webkitgtk-3.0 webkit-1.0)

#
# configure a header file to pass some of the CMake settings
# to the source code
#if (${PORT} STREQUAL "Android")
#    message(FATAL_ERROR "Dont know how to build '${PORT}'")
#elseif (${PORT} STREQUAL "OpenWrt")
#    message(FATAL_ERROR "Dont know how to build '${PORT}'")
#elseif (${PORT} STREQUAL "GTK")
#    message(FATAL_ERROR "Dont know how to build '${PORT}'")
#elseif (${PORT} STREQUAL "QT")
#    message(FATAL_ERROR "Dont know how to build '${PORT}'")
#else ()
#    message(FATAL_ERROR "Dont know how to build '${PORT}'")
#endif ()

#message(FATAL_ERROR "Dont know how to build '${PORT}'")
#set (OLSRD_SRCDIR ${PROJECT_SOURCE_DIR}/../olsrd-0.6.6.2)

#ExternalProject_Add(oslrd
#  SOURCE_DIR ${OLSRD_SRCDIR}
#  PREFIX ${CMAKE_CURRENT_BINARY_DIR}/olsrd/
#  CONFIGURE_COMMAND ""
#  BUILD_COMMAND make -I${OLSRD_SRCDIR} -f ${OLSRD_SRCDIR}/Makefile TOPDIR=${OLSRD_SRCDIR}
#  INSTALL_COMMAND make install PREFIX=${CMAKE_CURRENT_BINARY_DIR}
#)

#ExternalProject_Add(oslrd_plugin
#  SOURCE_DIR ${PROJECT_SOURCE_DIR}/../olsrd-0.6.6.2/lib/olsrd_qaul/
#  PREFIX ${CMAKE_CURRENT_BINARY_DIR}/olsrd/
#  CONFIGURE_COMMAND ""
#  BUILD_COMMAND make
#  INSTALL_COMMAND make install PREFIX=${CMAKE_CURRENT_BINARY_DIR}
#)

# Add sub-directories
#add_subdirectory (qaulhelper)
#add_subdirectory (third_party)
