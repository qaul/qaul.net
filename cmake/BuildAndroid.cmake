
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

if(${CMAKE_BINARY_DIR} STREQUAL ${CMAKE_SOURCE_DIR})
    set(JNIdepends mbedtls olsrd_qaul_android pjsip wt socat)
else()
    add_custom_target(copy_android
	COMMAND ${CMAKE_COMMAND} -DSRC=${CMAKE_SOURCE_DIR}/android/ -DDEST=${CMAKE_BINARY_DIR}/android -P ${CMAKE_SOURCE_DIR}/cmake/FileCopy.cmake
	COMMAND ${CMAKE_COMMAND} -DSRC=${CMAKE_SOURCE_DIR}/GUI/www -DDEST=${CMAKE_BINARY_DIR}/GUI -P ${CMAKE_SOURCE_DIR}/cmake/FileCopy.cmake
	COMMAND ${CMAKE_COMMAND} -DSRC=${CMAKE_SOURCE_DIR}/GUI/files -DDEST=${CMAKE_BINARY_DIR}/GUI -P ${CMAKE_SOURCE_DIR}/cmake/FileCopy.cmake)
    set(JNIdepends mbedtls olsrd_qaul_android pjsip wt socat copy_android)
endif()

add_custom_target(AndroidJNI
                  NDK_PROJECT_PATH=${CMAKE_BINARY_DIR}/android/app/src/main ${NDK_ROOT}/ndk-build -C ${CMAKE_BINARY_DIR}/android/app/src/main
                  PJSIP_PATH=${CMAKE_BINARY_DIR}/third_party/pjsip/src/pjsip
                  EXTRALIB_PATH=${EXTRALIB_PATH}
                  LIBQAUL_SRC=${CMAKE_SOURCE_DIR}/src/libqaul
                  LIBQAULUTILS_SRC=${CMAKE_SOURCE_DIR}/src/libqaulutils/src
                  LIBQAULUTILS_INCLUDE=${CMAKE_SOURCE_DIR}/src/libqaulutils/include
                  QAULCONFIG_INCLUDE=${CMAKE_BINARY_DIR}/include
                  MONGOOSE_PATH=${CMAKE_SOURCE_DIR}/third_party/mongoose

                  COMMAND ${CMAKE_COMMAND} -DSRC=${CMAKE_BINARY_DIR}/android/app/src/main/libs/armeabi/wificonfig -DDEST=${CMAKE_BINARY_DIR}/android/app/src/main/res/raw -P ${CMAKE_SOURCE_DIR}/cmake/FileCopy.cmake
                  COMMAND ${CMAKE_COMMAND} -DSRC=${CMAKE_BINARY_DIR}/android/app/src/main/libs/armeabi/tether -DDEST=${CMAKE_BINARY_DIR}/android/app/src/main/res/raw -P ${CMAKE_SOURCE_DIR}/cmake/FileCopy.cmake
                  COMMAND ${CMAKE_COMMAND} -DSRC=${EXTRALIB_PATH}/ifconfig -DDEST=${CMAKE_BINARY_DIR}/android/app/src/main/res/raw -P ${CMAKE_SOURCE_DIR}/cmake/FileCopy.cmake
                  COMMAND ${CMAKE_COMMAND} -DSRC=${EXTRALIB_PATH}/iptables -DDEST=${CMAKE_BINARY_DIR}/android/app/src/main/res/raw -P ${CMAKE_SOURCE_DIR}/cmake/FileCopy.cmake
                  DEPENDS ${JNIdepends}
                  WORKING_DIRECTORY ${CMAKE_BINARY_DIR}/android)

add_custom_target(AndroidUPDATE
                  COMMAND sed -i 's/android:versionName=".*"/android:versionName=\"${QAUL_ANDROID_VERSION_NAME}\"/g\; s/android:versionCode=".*"/android:versionCode="${QAUL_ANDROID_VERSION_CODE}"/g' ${CMAKE_BINARY_DIR}/android/app/src/main/AndroidManifest.xml
                  DEPENDS AndroidJNI
                  WORKING_DIRECTORY ${CMAKE_BINARY_DIR}/android/app/src/main)

add_custom_target(AndroidAPK ALL
                  COMMAND ANDROID_HOME=${SDK_ROOT} ANDROID_NDK_HOME=${NDK_ROOT} ./gradlew -PCMAKE_BINARY_DIR=${CMAKE_BINARY_DIR} -PEXTRALIB_PATH=${EXTRALIB_PATH} -PCMAKE_SOURCE_DIR=${CMAKE_SOURCE_DIR} build
                  DEPENDS AndroidUPDATE
                  WORKING_DIRECTORY ${CMAKE_BINARY_DIR}/android)
