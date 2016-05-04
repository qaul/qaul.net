

pkg_search_module (WEBKIT REQUIRED webkitgtk-3.0 webkit-1.0)

add_executable (qaul-gtk main.c network.c)

target_include_directories(qaul-gtk PRIVATE ${DBUS1_INCLUDE_DIRS} ${WEBKIT_INCLUDE_DIRS})

target_link_libraries(qaul-gtk ${DBUS1_LIBRARIES} ${WEBKIT_LIBRARIES}
    pthread
    libqaul
)

install (TARGETS qaul-gtk DESTINATION bin)
