
add_executable (qaul-cli main.c)

target_link_libraries(qaul-cli
    pthread
    libqaul
)

install (TARGETS qaul-cli DESTINATION bin)
