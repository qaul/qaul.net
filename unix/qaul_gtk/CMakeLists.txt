
find_package(ALSA REQUIRED)

set (LIBQAULDIR ${CMAKE_SOURCE_DIR}/libqaul)
set (PJBASE ${CMAKE_BINARY_DIR}/third_party/pjsip/src/pjsip)

add_executable(qaul-gtk main.c network.c
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
)

add_dependencies(qaul-gtk pjsip)

target_include_directories(qaul-gtk PRIVATE ${DBUS1_INCLUDE_DIRS} ${WEBKIT_INCLUDE_DIRS}
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

#TODO system detection uname -m
#target_compile_options(qaul-gtk PRIVATE -DPJSIPTYPE=x86_64-unknown-linux-gnu)
set(PJSIPTYPE x86_64-unknown-linux-gnu)

target_link_libraries(qaul-gtk ${DBUS1_LIBRARIES} ${WEBKIT_LIBRARIES}
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
    ${ALSA_LIBRARIES}
)

install (TARGETS qaul-gtk DESTINATION bin)
