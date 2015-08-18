
if (${PORT} STREQUAL "LINUX")
  find_package(ALSA REQUIRED)
  pkg_check_modules (UUID uuid)
  pkg_check_modules (OPENCORE_AMRNB opencore-amrnb)
  pkg_check_modules (OPENCORE_AMRWB opencore-amrwb)
endif ()

# Find includes in corresponding build directories
set(CMAKE_INCLUDE_CURRENT_DIR ON)
# Instruct CMake to run moc automatically when needed.
set(CMAKE_AUTOMOC ON)

# Find the QtWidgets library
find_package(Qt5Widgets REQUIRED)
find_package(Qt5WebKit REQUIRED)


set (LIBQAULDIR ${CMAKE_SOURCE_DIR}/libqaul)
set (PJBASE ${CMAKE_BINARY_DIR}/third_party/pjsip/src/pjsip)

qt5_wrap_ui(UI_HEADERS ${CMAKE_SOURCE_DIR}/src/client/qt5/qaul.ui)

if (${PORT} STREQUAL "WINDOWS")
  set (EXTRA_SOURCES
    ${LIBQAULDIR}/win32/functions.c
    ${LIBQAULDIR}/win32/pthread.c
  )
endif ()

add_executable(qaul-qt5 main.cpp qaul.cpp
    ${UI_HEADERS}
    ${LIBQAULDIR}/qaullib.c
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
    ${LIBQAULDIR}/qaullib_crypto.c
    ${LIBQAULDIR}/qaullib_udp_communication.c
    ${LIBQAULDIR}/captive/qaullib_captive.c
    ${LIBQAULDIR}/captive/qaullib_captive_dhcp.c
    ${LIBQAULDIR}/captive/qaullib_captive_dns.c
    ${LIBQAULDIR}/mongoose/mongoose.c
    ${LIBQAULDIR}/sqlite/sqlite3.c
    ${LIBQAULDIR}/urlcode/urlcode.c
    ${LIBQAULDIR}/polarssl/sha1.c
    ${LIBQAULDIR}/olsrd/mantissa.c
    ${LIBQAULDIR}/olsrd/hashing.c
    ${EXTRA_SOURCES}
)

qt5_use_modules(qaul-qt5 Widgets)
qt5_use_modules(qaul-qt5 WebKitWidgets)

add_dependencies(qaul-qt5 pjsip)

target_include_directories(qaul-qt5 PRIVATE ${DBUS1_INCLUDE_DIRS} ${WEBKIT_INCLUDE_DIRS}
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
)

set(PJSIPTYPE qaul)

if (UUID_FOUND)
    target_link_libraries(qaul-qt5 ${UUID_LIBRARIES})
endif (UUID_FOUND)

if (OPENCORE_AMRNB_FOUND)
    target_link_libraries(qaul-qt5 ${OPENCORE_AMRNB_LIBRARIES})
endif (OPENCORE_AMRNB_FOUND)

if (OPENCORE_AMRWB_FOUND)
    target_link_libraries(qaul-qt5 ${OPENCORE_AMRWB_LIBRARIES})
endif (OPENCORE_AMRWB_FOUND)

target_link_libraries(qaul-qt5 ${DBUS1_LIBRARIES} 
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
    ${PJBASE}/third_party/lib/libgsmcodec-${PJSIPTYPE}.a
    ${PJBASE}/third_party/lib/libilbccodec-${PJSIPTYPE}.a
    ${PJBASE}/third_party/lib/libportaudio-${PJSIPTYPE}.a
    ${PJBASE}/third_party/lib/libresample-${PJSIPTYPE}.a
    ${PJBASE}/third_party/lib/libspeex-${PJSIPTYPE}.a
    ${PJBASE}/third_party/lib/libsrtp-${PJSIPTYPE}.a
    ${PJBASE}/pjlib/lib/libpj-${PJSIPTYPE}.a
)

if (${PORT} STREQUAL "LINUX")
	target_link_libraries(qaul-qt5 ${ALSA_LIBRARIES})
endif ()


if (${PORT} STREQUAL "WINDOWS")
    target_link_libraries(qaul-qt5 ws2_32 wsock32 winmm)
endif ()

install (TARGETS qaul-qt5 DESTINATION bin)
