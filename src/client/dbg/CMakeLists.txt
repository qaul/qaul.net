add_executable (qaul-dbg q_debug.c)

target_include_directories(qaul-dbg PRIVATE ${DBUS1_INCLUDE_DIRS} ${WEBKIT_INCLUDE_DIRS})

target_link_libraries(qaul-dbg ${DBUS1_LIBRARIES} ${WEBKIT_LIBRARIES}
        pthread
        libqaul
        )

install (TARGETS qaul-dbg DESTINATION bin)
