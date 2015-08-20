
find_package(ALSA REQUIRED)

pkg_search_module (WEBKIT REQUIRED webkitgtk-3.0 webkit-1.0)
pkg_check_modules (UUID uuid)
pkg_check_modules (OPENCORE_AMRNB opencore-amrnb)
pkg_check_modules (OPENCORE_AMRWB opencore-amrwb)

add_executable (qaul-gtk main.c network.c)

target_include_directories(qaul-gtk PRIVATE ${DBUS1_INCLUDE_DIRS} ${WEBKIT_INCLUDE_DIRS})

if (UUID_FOUND)
    target_link_libraries(qaul-gtk ${UUID_LIBRARIES})
endif (UUID_FOUND)

if (OPENCORE_AMRNB_FOUND)
    target_link_libraries(qaul-gtk ${OPENCORE_AMRNB_LIBRARIES})
endif (OPENCORE_AMRNB_FOUND)

if (OPENCORE_AMRWB_FOUND)
    target_link_libraries(qaul-gtk ${OPENCORE_AMRWB_LIBRARIES})
endif (OPENCORE_AMRWB_FOUND)

target_link_libraries(qaul-gtk ${DBUS1_LIBRARIES} ${WEBKIT_LIBRARIES}
    pthread dl m
    libqaul
    ${ALSA_LIBRARIES}
)

install (TARGETS qaul-gtk DESTINATION bin)
