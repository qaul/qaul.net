# Mongoose
set (MONGOOSE_DIR ${CMAKE_CURRENT_SOURCE_DIR}/../../third_party/mongoose)

if (VOIP)
    if (${PORT} STREQUAL "LINUX")
        find_package(ALSA REQUIRED)
    endif ()

    set (PJBASE ${CMAKE_BINARY_DIR}/third_party/pjsip/src/pjsip)
    set (PJSIPTYPE qaul)
    
    set (VOIP_SOURCES
        qaullib_voip.c
        )
    
    set (VOIP_DEPENDENCIES
        pjsip
        )
    
    set (VOIP_INCLUDE
        ${PJBASE}/pjlib/include
        ${PJBASE}/pjlib-util/include
        ${PJBASE}/pjmedia/include
        ${PJBASE}/pjnath/include
        ${PJBASE}/pjsip/include
        #       ${PJBASE}third_party/portaudio/include
        #       ${PJBASE}third_party/resample/include
        #       ${PJBASE}third_party/speex/include
        #       ${PJBASE}third_party/srtp/crypto/include
        #       ${PJBASE}third_party/srtp/include
        )

    if (Raspberry)
        set (VOIP_LINK_LIBRARIES
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
            ${PJBASE}/third_party/lib/libresample-${PJSIPTYPE}.a
            ${PJBASE}/third_party/lib/libspeex-${PJSIPTYPE}.a
            ${PJBASE}/third_party/lib/libsrtp-${PJSIPTYPE}.a
            #${PJBASE}/third_party/lib/libwebrtc-${PJSIPTYPE}.a
            ${PJBASE}/pjlib/lib/libpj-${PJSIPTYPE}.a
            )
    else ()
        set (VOIP_LINK_LIBRARIES
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
            ${PJBASE}/third_party/lib/libresample-${PJSIPTYPE}.a
            ${PJBASE}/third_party/lib/libspeex-${PJSIPTYPE}.a
            ${PJBASE}/third_party/lib/libsrtp-${PJSIPTYPE}.a
            #${PJBASE}/third_party/lib/libwebrtc-${PJSIPTYPE}.a
            ${PJBASE}/pjlib/lib/libpj-${PJSIPTYPE}.a
            )
    endif ()
else()
    set (VOIP_SOURCES
        qaullib_voip_dummy.c
        )
endif ()

if (${PORT} STREQUAL "WINDOWS")
  set (LIBTYPE SHARED)
else ()
  set (LIBTYPE STATIC)
endif ()

if (${PORT} STREQUAL "WINDOWS")
  set (WIN_SOURCES
    win32/functions.c
    win32/pthread.c
    )
endif ()

if (${PORT} STREQUAL "ANDROID")
  set (ANDROIDA_SOURCES
    mg_backend_apk.c
    )
endif ()


ADD_LIBRARY (libqaul ${LIBTYPE}

    # library API (temporary)
    public/qaul/qaul.h
    public/qaul/query.h
    public/qaul/error.h

    ### Internal module headers ###
    include/qaul/mod/auth.h
    include/qaul/mod/crypto.h
    include/qaul/mod/database.h
    include/qaul/mod/filesystem.h
    include/qaul/mod/key_store.h
    include/qaul/mod/network.h
    include/qaul/mod/protocol.h
    include/qaul/mod/structures.h
        include/qaul/mod/userstore.h
    include/qaul/mod/voip.h

    ### Global API implementations ###
    api/authentication.c
    api/config.c
    api/core.c
    api/files.c
    api/messages.c
    api/users.c
    api/voip.c

    ### Authentication module ###
    authentication/auth.c

    ### Crypto core module ###
    crypto-core/crypto.c
    
    ### Database module ###
    database/database.c

    ### Filesystem module ###
    filesystem/filesystem.c
    
    ### Keystore module ###
    keystore/keystore.c
    
    ### Network module ###
    network/network.c
    
    ### Protocol module ###
    protocol/protocol.c
    protocol/messaging.c
    protocol/files.c
    
    ### Userstore module ###
    userstore/userstore.c
    userstore/keystore.c

    ### Voip module ###
)

add_dependencies(libqaul mbedtls)
add_dependencies(libqaul libcuckoo)

ExternalProject_Get_Property(mbedtls install_dir)
target_link_libraries(libqaul PUBLIC
    ${install_dir}/mbedtls/lib/libmbedcrypto.a
    )
target_include_directories(libqaul PUBLIC ${install_dir}/mbedtls/include)

ExternalProject_Get_Property(libcuckoo install_dir)
target_link_libraries(libqaul PUBLIC
    ${install_dir}/libcuckoo/lib/libcuckoo.a
    )
target_include_directories(libqaul PUBLIC ${install_dir}/libcuckoo/include)

target_link_libraries(libqaul PUBLIC
    libqaulutils
    ${VOIP_LINK_LIBRARIES}
    )

target_include_directories(libqaul PRIVATE include)

# since the name starts with 'lib' don't add it again
set_target_properties (libqaul PROPERTIES PREFIX "")

if (VOIP)
    add_dependencies(libqaul
        ${VOIP_DEPENDENCIES}
        )
endif ()

target_include_directories (libqaul PUBLIC include)

target_include_directories (libqaul PRIVATE
    ${MONGOOSE_DIR}
    ${VOIP_INCLUDE}
    )

if (UUID_FOUND)
    target_link_libraries(libqaul LINK_PRIVATE ${UUID_LIBRARIES})
endif (UUID_FOUND)

if (OPENCORE_AMRNB_FOUND)
    target_link_libraries(libqaul LINK_PUBLIC ${OPENCORE_AMRNB_LIBRARIES})
endif (OPENCORE_AMRNB_FOUND)

if (OPENCORE_AMRWB_FOUND)
    target_link_libraries(libqaul LINK_PUBLIC ${OPENCORE_AMRWB_LIBRARIES})
endif (OPENCORE_AMRWB_FOUND)

if (${PORT} STREQUAL "LINUX")
    target_link_libraries(libqaul LINK_PUBLIC dl m pthread ${ALSA_LIBRARIES})
endif ()

if (${PORT} STREQUAL "WINDOWS")
    target_link_libraries(libqaul LINK_PUBLIC ws2_32 wsock32 winmm)
endif ()

# TODO lack of api requires this :(
target_include_directories(libqaul PUBLIC ${CMAKE_CURRENT_SOURCE_DIR})
target_include_directories(libqaul PUBLIC public)