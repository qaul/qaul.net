
#find_package(ALSA REQUIRED)

set (LIBQAULDIR ${CMAKE_SOURCE_DIR}/src/libqaul)
set (PJBASE ${CMAKE_BINARY_DIR}/third_party/pjsip/src/pjsip)

# Set the OS X Bundle specific CMake variables which will be used to populate the plist for
# the application bundle
set(MACOSX_BUNDLE_INFO_STRING "${PROJECT_NAME}")
set(MACOSX_BUNDLE_ICON_FILE qaul.icns)
set(MACOSX_BUNDLE_GUI_IDENTIFIER "net.qaul.qaul")
set(MACOSX_BUNDLE_LONG_VERSION_STRING "${PROJECT_NAME} Version ${CPACK_PACKAGE_VERSION}")
set(MACOSX_BUNDLE_BUNDLE_NAME ${PROJECT_NAME})
set(MACOSX_BUNDLE_SHORT_VERSION_STRING ${CPACK_PACKAGE_VERSION})
set(MACOSX_BUNDLE_BUNDLE_VERSION ${CPACK_PACKAGE_VERSION})
set(MACOSX_BUNDLE_COPYRIGHT "qaul.net is free software, licensed under GPLv3")

# for menu .xib file
set(MACOSX_BUNDLE_NSMAIN_NIB_FILE "MainMenu")
set(MACOSX_BUNDLE_NSPRINCIPAL_CLASS "NSApplication")

FIND_PROGRAM(IBTOOL ibtool HINTS "/usr/bin" "${OSX_DEVELOPER_ROOT}/usr/bin")
IF (${IBTOOL} STREQUAL "IBTOOL-NOTFOUND")
	MESSAGE(SEND_ERROR "ibtool not found. It is needed to compile the .xib file.")
ENDIF()

set(MACOSX_RESOURCE_FILES ${PROJECT_SOURCE_DIR}/distfiles/osx/qaul.icns) 
set_source_files_properties(${MACOSX_RESOURCE_FILES} PROPERTIES MACOSX_PACKAGE_LOCATION Resources)
set(MACOSX_MENU_FILES ${PROJECT_SOURCE_DIR}/src/client/osx/en.lproj/InfoPlist.strings) 
set_source_files_properties(${MACOSX_MENU_FILES} PROPERTIES MACOSX_PACKAGE_LOCATION Resources/en.lproj)

# Mongoose
set (MONGOOSE_DIR ${CMAKE_SOURCE_DIR}/third_party/mongoose)
# mbedTLS
set (MBEDTLS_DIR ${CMAKE_SOURCE_DIR}/third_party/mbedtls)

add_executable(${PROJECT_NAME} MACOSX_BUNDLE
	main.m
	AppDelegate.h AppDelegate.m
	QaulConfigWifi.h QaulConfigWifi.m
    ${LIBQAULDIR}/qaullib.c
    ${LIBQAULDIR}/logging.c
    ${LIBQAULDIR}/qaullib_ipc.c
    ${LIBQAULDIR}/qaullib_webserver.c
    ${LIBQAULDIR}/qaullib_voip.c
    ${LIBQAULDIR}/qaullib_webclient.c
    ${LIBQAULDIR}/qaullib_threads.c
    ${LIBQAULDIR}/qaullib_user.c
    ${LIBQAULDIR}/qaullib_user_LL.c
    ${LIBQAULDIR}/qaullib_topo_LL.c
    ${LIBQAULDIR}/qaullib_appevent_LL.c
    ${LIBQAULDIR}/qaullib_messaging.c
    ${LIBQAULDIR}/qaullib_msg_LL.c
    ${LIBQAULDIR}/qaullib_validate.c
    ${LIBQAULDIR}/qaullib_filesharing.c
    ${LIBQAULDIR}/qaullib_file_LL.c
    ${LIBQAULDIR}/qaullib_exediscovery.c

	# Crypto mod
	${LIBQAULDIR}/qaullib_crypto.c
	${LIBQAULDIR}/crypto/qcry_context.c
    ${LIBQAULDIR}/crypto/qcry_arbiter.c
    ${LIBQAULDIR}/crypto/qcry_hashing.c
    ${LIBQAULDIR}/crypto/qcry_helper.c
    ${LIBQAULDIR}/crypto/qcry_keys.c
    ${LIBQAULDIR}/crypto/qcry_keystore.c

    # Validate input
    ${LIBQAULDIR}/validate.c

	${LIBQAULDIR}/qaullib_udp_communication.c
	${LIBQAULDIR}/captive/qaullib_captive.c
	${LIBQAULDIR}/captive/qaullib_captive_dhcp.c
	${LIBQAULDIR}/captive/qaullib_captive_dns.c
    ${LIBQAULDIR}/captive/whitelist.c
	${LIBQAULDIR}/qmongoose.c
	${LIBQAULDIR}/sqlite/sqlite3.c
	${LIBQAULDIR}/urlcode/urlcode.c
	${LIBQAULDIR}/olsrd/mantissa.c
	${LIBQAULDIR}/olsrd/hashing.c
	${MACOSX_RESOURCE_FILES}
	${MACOSX_MENU_FILES}
)

add_dependencies(qaul libqaul pjsip)

target_include_directories(qaul PRIVATE
    ${PJBASE}/pjlib/include
    ${PJBASE}/pjlib-util/include
    ${PJBASE}/pjmedia/include
    ${PJBASE}/pjnath/include
    ${PJBASE}/pjsip/include
    ${PJBASE}third_party/portaudio/include
    ${PJBASE}third_party/resample/include
    ${PJBASE}third_party/speex/include
    ${PJBASE}third_party/srtp/crypto/include
    ${PJBASE}third_party/srtp/include
    ${LIBQAULDIR}
    ${LIBQAULDIR}/include
    ${MONGOOSE_DIR}
    ${MBEDTLS_DIR}/include
)

#TODO system detection uname -m
#target_compile_options(qaul-gtk PRIVATE -DPJSIPTYPE=x86_64-unknown-linux-gnu)
set(PJSIPTYPE qaul)

find_library(COREDATA CoreData)
find_library(FOUNDATION Foundation)
find_library(APPKIT AppKit)
find_library(COCOA Cocoa)
find_library(WEBKIT WebKit)
find_library(COREAUDIO CoreAudio)
find_library(AUDIOUNIT AudioUnit)
find_library(AUDIOTOOLBOX AudioToolbox)
find_library(COREWLAN CoreWLAN)
find_library(SYSTEMCONFIGURATION SystemConfiguration)

target_link_libraries(qaul 
    # pthread dl m uuid avcodec opencore-amrnb
    # TODO: please check if libraries are present and do not include them accordingly
    #       see issue https://github.com/qaul/qaul.net/issues/5
    pthread dl m
    ${PJBASE}/pjsip/lib/libpjsua-${PJSIPTYPE}.a
    ${PJBASE}/pjsip/lib/libpjsip-ua-${PJSIPTYPE}.a
    ${PJBASE}/pjsip/lib/libpjsip-${PJSIPTYPE}.a
    ${PJBASE}/pjsip/lib/libpjsip-simple-${PJSIPTYPE}.a
    ${PJBASE}/pjmedia/lib/libpjmedia-codec-${PJSIPTYPE}.a
    ${PJBASE}/pjmedia/lib/libpjmedia-${PJSIPTYPE}.a
    ${PJBASE}/pjmedia/lib/libpjmedia-audiodev-${PJSIPTYPE}.a
    ${PJBASE}/pjmedia/lib/libpjmedia-videodev-${PJSIPTYPE}.a
    ${PJBASE}/pjnath/lib/libpjnath-${PJSIPTYPE}.a
    ${PJBASE}/pjlib-util/lib/libpjlib-util-${PJSIPTYPE}.a
    ${PJBASE}/third_party/lib/libg7221codec-${PJSIPTYPE}.a
    #${PJBASE}/third_party/lib/libgsmcodec-${PJSIPTYPE}.a
    ${PJBASE}/third_party/lib/libilbccodec-${PJSIPTYPE}.a
    ${PJBASE}/third_party/lib/libresample-${PJSIPTYPE}.a
    ${PJBASE}/third_party/lib/libspeex-${PJSIPTYPE}.a
    ${PJBASE}/third_party/lib/libsrtp-${PJSIPTYPE}.a
    ${PJBASE}/third_party/lib/libwebrtc-${PJSIPTYPE}.a
    ${PJBASE}/pjlib/lib/libpj-${PJSIPTYPE}.a
    # mbedtls
    ${PROJECT_BINARY_DIR}/external/mbedtls/lib/libmbedcrypto.a
    ${ALSA_LIBRARIES}
    # OSX libraries
	${COREDATA} ${FOUNDATION}
	${APPKIT} ${COCOA}
	${WEBKIT}
	${COREAUDIO} ${AUDIOUNIT} ${AUDIOTOOLBOX} ${COREWLAN} ${SYSTEMCONFIGURATION}
)

# Set a custom Info.plist file template.
set_target_properties(${PROJECT_NAME} PROPERTIES
	MACOSX_BUNDLE_INFO_PLIST ${PROJECT_SOURCE_DIR}/src/client/osx/Info-Cmake.plist
)

file(COPY ${PROJECT_SOURCE_DIR}/GUI/www DESTINATION ${CMAKE_CURRENT_BINARY_DIR}/${PROJECT_NAME}.app/Contents/Resources)
file(COPY ${PROJECT_SOURCE_DIR}/GUI/files DESTINATION ${CMAKE_CURRENT_BINARY_DIR}/${PROJECT_NAME}.app/Contents/Resources)

# compile app GUI
ADD_CUSTOM_COMMAND (
	TARGET ${PROJECT_NAME}
	POST_BUILD 
	COMMENT "Build qaul GUI .nib file"
	COMMAND ${IBTOOL} --errors --warnings --notices
	--output-format human-readable-text 
	--compile ${CMAKE_CURRENT_BINARY_DIR}/${PROJECT_NAME}.app/Contents/Resources/en.lproj/MainMenu.nib 
	${PROJECT_SOURCE_DIR}/src/client/osx/en.lproj/MainMenu.xib
)


INSTALL( 
	DIRECTORY ${CMAKE_CURRENT_BINARY_DIR}/${PROJECT_NAME}.app 
	DESTINATION /Applications 
	USE_SOURCE_PERMISSIONS
)
