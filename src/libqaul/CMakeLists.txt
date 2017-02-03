
if (VOIP STREQUAL "NO")
	set (VOIP_SOURCES
		qaullib_voip_dummy.c
	)
else ()
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
#		${PJBASE}third_party/portaudio/include
#		${PJBASE}third_party/resample/include
#		${PJBASE}third_party/speex/include
#		${PJBASE}third_party/srtp/crypto/include
#		${PJBASE}third_party/srtp/include
	)

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
		${PJBASE}/pjlib/lib/libpj-${PJSIPTYPE}.a
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

ADD_LIBRARY (libqaul ${LIBTYPE}
    logging.c
    qaullib.c
    qaullib_ipc.c
    qaullib_webserver.c
    qaullib_webclient.c
    qaullib_threads.c
    qaullib_user.c
    qaullib_user_LL.c
    qaullib_topo_LL.c
    qaullib_appevent_LL.c
    qaullib_messaging.c
    qaullib_msg_LL.c
    qaullib_validate.c
    qaullib_filesharing.c
    qaullib_file_LL.c
    qaullib_exediscovery.c
    qaullib_crypto.c
    qaullib_udp_communication.c
    captive/qaullib_captive.c
    captive/qaullib_captive_dhcp.c
    captive/qaullib_captive_dns.c
    mongoose/mongoose.c
    sqlite/sqlite3.c
    urlcode/urlcode.c
    olsrd/mantissa.c
    olsrd/hashing.c
    ${VOIP_SOURCES}
    ${WIN_SOURCES}
)

# since the name starts with 'lib' don't add it again
set_target_properties (libqaul PROPERTIES PREFIX "")

if (VOIP STREQUAL "NO")
else ()
	add_dependencies(libqaul
		${VOIP_DEPENDENCIES}
	)
endif ()

target_include_directories (libqaul PUBLIC include)

target_include_directories (libqaul PRIVATE
	${VOIP_INCLUDE}
)

target_link_libraries(libqaul LINK_PUBLIC
	${VOIP_LINK_LIBRARIES}
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
