cmake_minimum_required (VERSION 2.8)

project (qaul)

IF(CMAKE_INSTALL_PREFIX_INITIALIZED_TO_DEFAULT)
        SET(CMAKE_INSTALL_PREFIX "/opt/qaul" CACHE PATH "qaul.net install prefix" FORCE)
ENDIF(CMAKE_INSTALL_PREFIX_INITIALIZED_TO_DEFAULT)

list(APPEND CMAKE_MODULE_PATH "${CMAKE_CURRENT_LIST_DIR}/cmake")

include (ExternalProject)

# The version number.
set (Qaul_VERSION_MAJOR 0)
set (Qaul_VERSION_MINOR 0)
set (Qaul_VERSION_PATCH 0)
set (Qaul_VERSION_SUFFIX Qaul-NG_Proto)

# What to build
set(ALL_PORTS Android OpenWrt GTK QT Windows OSX iOS)
set(PORT "NOPORT" CACHE STRING "choose which Qaul port to build (one of ${ALL_PORTS})")

list(FIND ALL_PORTS ${PORT} RET)
if (${RET} EQUAL -1)
  message(FATAL_ERROR "Please choose which Qaul port to build (one of ${ALL_PORTS})")
endif ()

# What package to build
set(ALL_PKGFORMAT AUTO NSIS TGZ RPM DEB)
set(PKGFORMAT "AUTO" CACHE STRING "select what type of package should be build (one of ${ALL_PKGFORMAT})")

# -----------------------------------------------------------------------------
# Determine the operating system
# -----------------------------------------------------------------------------
if (UNIX)
  if (APPLE)
    set(Q_OS_MAC_OS_X 1)
  else ()
    set(Q_OS_UNIX 1)
  endif ()
elseif (CMAKE_SYSTEM_NAME MATCHES "Windows")
  set(Q_OS_WINDOWS 1)
else ()
  message(FATAL_ERROR "Unknown OS '${CMAKE_SYSTEM_NAME}'")
endif ()

# configure a header file to pass some of the CMake settings
# to the source code
configure_file (
  "${PROJECT_SOURCE_DIR}/QaulConfig.h.in"
  "${PROJECT_BINARY_DIR}/QaulConfig.h"
  )

# add the binary tree to the search path for include files
# so that we will find QaulConfig.h
include_directories ("${PROJECT_BINARY_DIR}")

set(UNIXOID_PORTS GTK QT)

list(FIND UNIXOID_PORTS ${PORT} RET)
if (NOT ${RET} EQUAL -1)
    include(BuildUnixoid.cmake)
elseif (${PORT} STREQUAL "Android")
    include(BuildAndroid.cmake)
elseif (${PORT} STREQUAL "OpenWrt")
    include(BuildOpenWrt.cmake)
elseif (${PORT} STREQUAL "OSX")
    include(BuildOSX.cmake)
elseif (${PORT} STREQUAL "Windows")
    include(BuildWindows.cmake)
else ()
    message(FATAL_ERROR "Dont know how to build '${PORT}'")
endif ()

add_subdirectory (third_party)
