
add_executable (qaul-cli main.c)

target_link_libraries(qaul-cli
    pthread
    libqaul
)

if (UUID_FOUND)
    target_link_libraries(qaul-cli ${UUID_LIBRARIES})
endif (UUID_FOUND)

install (TARGETS qaul-cli DESTINATION bin)
