
if(NOT NDK_ROOT)
    message(FATAL_ERROR "The path to the Android NDK needs to be specified: -DNDK_ROOT=/path/to/ndk")
endif()

if(NOT SDK_ROOT)
    message(FATAL_ERROR "The path to the Android SDK needs to be specified: -DSDK_ROOT=/path/to/sdk")
endif()


if(NOT EXTRALIB_PATH)
    message(FATAL_ERROR "The path to the extra libraries needs to be specified: -DEXTRALIB_PATH=/path/to/extra/lib")
endif()

if(NOT NDK_LEVEL)
    message(STATUS "Use default Android Target version 9 (-DNDK_LEVEL=9).")
    set(NDK_LEVEL 9)
endif()

if(NOT ANDROID_EABI)
    message(STATUS "Use default Android EABI version 4.9 (-DANDROID_EABI=\"4.9\").")
    set(ANDROID_EABI "4.9")
endif()

add_custom_target(copy_android
    ${CMAKE_COMMAND} -DSRC=${CMAKE_SOURCE_DIR}/android/app/src/main -DDEST=${CMAKE_BINARY_DIR}/android/app/src -P ${CMAKE_SOURCE_DIR}/cmake/FileCopy.cmake
    ${CMAKE_COMMAND} -DSRC=${CMAKE_SOURCE_DIR}/GUI/www -DDEST=${CMAKE_BINARY_DIR} -P ${CMAKE_SOURCE_DIR}/cmake/FileCopy.cmake
    ${CMAKE_COMMAND} -DSRC=${CMAKE_SOURCE_DIR}/GUI/files -DDEST=${CMAKE_BINARY_DIR} -P ${CMAKE_SOURCE_DIR}/cmake/FileCopy.cmake
)

add_custom_target(AndroidJNI
                  NDK_PROJECT_PATH=${CMAKE_BINARY_DIR}/android/app/src/main ${NDK_ROOT}/ndk-build -C ${CMAKE_BINARY_DIR}/android/app/src/main
                  PJSIP_PATH=${CMAKE_BINARY_DIR}/third_party/pjsip/src/pjsip
                  EXTRALIB_PATH=${EXTRALIB_PATH}
                  LIBQAUL_SRC=${CMAKE_SOURCE_DIR}/src/libqaul
                  QAULCONFIG_INCLUDE=${CMAKE_BINARY_DIR}/include

                  COMMAND ${CMAKE_COMMAND} -DSRC=${CMAKE_BINARY_DIR}/android/app/src/main/libs/armeabi/wificonfig -DDEST=${CMAKE_BINARY_DIR}/android/app/src/main/res/raw -P ${CMAKE_SOURCE_DIR}/cmake/FileCopy.cmake
                  COMMAND ${CMAKE_COMMAND} -DSRC=${CMAKE_BINARY_DIR}/android/app/src/main/libs/armeabi/tether -DDEST=${CMAKE_BINARY_DIR}/android/app/src/main/res/raw -P ${CMAKE_SOURCE_DIR}/cmake/FileCopy.cmake
                  COMMAND ${CMAKE_COMMAND} -DSRC=${EXTRALIB_PATH}/ifconfig -DDEST=${CMAKE_BINARY_DIR}/android/app/src/main/res/raw -P ${CMAKE_SOURCE_DIR}/cmake/FileCopy.cmake
                  COMMAND ${CMAKE_COMMAND} -DSRC=${EXTRALIB_PATH}/iptables -DDEST=${CMAKE_BINARY_DIR}/android/app/src/main/res/raw -P ${CMAKE_SOURCE_DIR}/cmake/FileCopy.cmake
                  DEPENDS olsr pjsip wt socat copy_android
                  WORKING_DIRECTORY ${CMAKE_BINARY_DIR}/android)

add_custom_target(AndroidUPDATE ${SDK_ROOT}/tools/android update project -t android-17 -p ${CMAKE_BINARY_DIR}/android/app/src/main
                  DEPENDS AndroidJNI
                  WORKING_DIRECTORY ${CMAKE_BINARY_DIR}/android/app/src/main)

add_custom_target(AndroidAPK ALL ant -Dsource.dir=java debug
                  DEPENDS AndroidUPDATE
                  WORKING_DIRECTORY ${CMAKE_BINARY_DIR}/android/app/src/main)
