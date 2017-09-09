
ADD_LIBRARY (libqaulutils STATIC
    logging.c
    validate.c
)

# since the name starts with 'lib' don't add it again
set_target_properties (libqaulutils PROPERTIES PREFIX "")

target_include_directories (libqaulutils PUBLIC ../include)

#if (${PORT} STREQUAL "LINUX")
#    target_link_libraries(libqaul LINK_PUBLIC dl m pthread ${ALSA_LIBRARIES})
#endif ()

#if (${PORT} STREQUAL "WINDOWS")
#    target_link_libraries(libqaul LINK_PUBLIC ws2_32 wsock32 winmm)
#endif ()
