
add_executable (qauld main.c)

target_link_libraries(qauld
    pthread
    libqaul
)

install (TARGETS qauld DESTINATION bin)
