
ExternalProject_Add(pjsip
    DEPENDS dl_pjsip
    PREFIX ${CMAKE_CURRENT_BINARY_DIR}/pjsip
    URL ${ARCHIVE_DIR}/${PJSIP_FILENAME}
    BUILD_IN_SOURCE 1
    PATCH_COMMAND patch -p1 -t -N -i ${CMAKE_CURRENT_SOURCE_DIR}/android.patch
    COMMAND patch -p1 -t -N -i ${CMAKE_CURRENT_SOURCE_DIR}/pjsip.patch
    CONFIGURE_COMMAND ANDROID_NDK_ROOT=${NDK_ROOT} APP_PLATFORM=android-${NDK_LEVEL} ./configure-android
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
    CONFIGURE_COMMAND ""
    BUILD_COMMAND make NDK_BASE=${NDK_ROOT} NDK_PLATFORM_LEVEL=${NDK_LEVEL} OS=android DEBUG=0 olsrd libs
    COMMAND ${CMAKE_COMMAND} -DSRC=<BINARY_DIR>/olsrd -DDEST=${CMAKE_BINARY_DIR}/android/res/raw -P ${CMAKE_SOURCE_DIR}/cmake/FileCopy.cmake
    COMMAND ${CMAKE_COMMAND} -DSRC=<BINARY_DIR>/lib/dyn_gw/olsrd_dyn_gw.so.0.5 -DDEST=${CMAKE_BINARY_DIR}/android/res/raw -P ${CMAKE_SOURCE_DIR}/cmake/FileCopy.cmake
    COMMAND ${CMAKE_COMMAND} -DSRC=${CMAKE_BINARY_DIR}/android/res/raw/olsrd_dyn_gw.so.0.5 -DDEST=${CMAKE_BINARY_DIR}/android/res/raw/olsrd_dyn_gw_so_0_5 -P ${CMAKE_SOURCE_DIR}/cmake/FileRename.cmake
    COMMAND ${CMAKE_COMMAND} -DSRC=<BINARY_DIR>/lib/olsrd_qaul/olsrd_qaul.so.0.1 -DDEST=${CMAKE_BINARY_DIR}/android/res/raw/ -P ${CMAKE_SOURCE_DIR}/cmake/FileCopy.cmake
    COMMAND ${CMAKE_COMMAND} -DSRC=${CMAKE_BINARY_DIR}/android/res/raw/olsrd_qaul.so.0.1 -DDEST=${CMAKE_BINARY_DIR}/android/res/raw/olsrd_qaul_so_0_1 -P ${CMAKE_SOURCE_DIR}/cmake/FileRename.cmake
    INSTALL_COMMAND ""
)

set (SOCAT_VERSION "1.7.3.1")
set (SOCAT_FILENAME "socat-${SOCAT_VERSION}.tar.bz2")
set (SOCAT_URL "http://www.dest-unreach.org/socat/download/${SOCAT_FILENAME}")
set (SOCAT_MD5 "334e46924f2b386299c9db2ac22bcd36")

add_custom_target(dl_socat
    COMMAND ${CMAKE_COMMAND} -DDL_URL=${SOCAT_URL} -DDL_FILENAME=${ARCHIVE_DIR}/${SOCAT_FILENAME} -DDL_MD5=${SOCAT_MD5} -P ${CMAKE_CURRENT_SOURCE_DIR}/download.cmake
)

ExternalProject_Add(socat
    DEPENDS dl_socat
    PREFIX ${CMAKE_CURRENT_BINARY_DIR}/socat
    URL ${ARCHIVE_DIR}/${SOCAT_FILENAME}
    BUILD_IN_SOURCE 1
    PATCH_COMMAND patch --ignore-whitespace -p1 -t -N -i ${CMAKE_CURRENT_SOURCE_DIR}/socat.patch
    CONFIGURE_COMMAND ""
    BUILD_COMMAND autoconf COMMAND ANDROID_NDK=${NDK_ROOT} ANDROID_TOOLCHAIN=arm-linux-androideabi-${ANDROID_EABI} ANDROID_PLATFORM=android-${NDK_LEVEL} ./socat_buildscript_for_android.sh
    COMMAND ${CMAKE_COMMAND} -DSRC=<BINARY_DIR>/out/socat -DDEST=${CMAKE_BINARY_DIR}/android/res/raw -P ${CMAKE_SOURCE_DIR}/cmake/FileCopy.cmake
    INSTALL_COMMAND ""
)

set (WT_VERSION "29")
set (WT_FILENAME "wireless_tools.${WT_VERSION}.tar.gz")
set (WT_URL "http://www.hpl.hp.com/personal/Jean_Tourrilhes/Linux/${WT_FILENAME}")
set (WT_MD5 "e06c222e186f7cc013fd272d023710cb")

add_custom_target(dl_wt
    COMMAND ${CMAKE_COMMAND} -DDL_URL=${WT_URL} -DDL_FILENAME=${ARCHIVE_DIR}/${WT_FILENAME} -DDL_MD5=${WT_MD5} -P ${CMAKE_CURRENT_SOURCE_DIR}/download.cmake 
)

ExternalProject_Add(wt
    DEPENDS dl_wt
    PREFIX ${CMAKE_CURRENT_BINARY_DIR}/wt
    URL ${ARCHIVE_DIR}/${WT_FILENAME}
    BUILD_IN_SOURCE 1
    PATCH_COMMAND cp wireless.22.h wireless.h
    COMMAND patch --ignore-whitespace -p1 -t -N -i ${CMAKE_CURRENT_SOURCE_DIR}/wirelesstools.patch
    CONFIGURE_COMMAND ""
    BUILD_COMMAND ${NDK_ROOT}/ndk-build NDK_PROJECT_PATH=<BINARY_DIR> APP_BUILD_SCRIPT=<BINARY_DIR>/Android.mk SYSROOT=${NDK_ROOT}/platforms/android-${NDK_LEVEL}/arch-arm
    COMMAND ${CMAKE_COMMAND} -DSRC=<BINARY_DIR>/libs/armeabi/iwconfig -DDEST=${CMAKE_BINARY_DIR}/android/res/raw -P ${CMAKE_SOURCE_DIR}/cmake/FileCopy.cmake
    INSTALL_COMMAND ""
)
