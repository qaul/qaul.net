
add_executable (qaul-cli main.c)

target_link_libraries(qaul-cli
    pthread
    libqaul
)

if (UUID_FOUND)
    target_link_libraries(qaul-cli ${UUID_LIBRARIES})
endif (UUID_FOUND)

if (OPENCORE_AMRNB_FOUND)
    target_link_libraries(qaul-cli ${OPENCORE_AMRNB_LIBRARIES})
endif (OPENCORE_AMRNB_FOUND)

if (OPENCORE_AMRWB_FOUND)
    target_link_libraries(qaul-cli ${OPENCORE_AMRWB_LIBRARIES})
endif (OPENCORE_AMRWB_FOUND)

install (TARGETS qaul-cli DESTINATION bin)
