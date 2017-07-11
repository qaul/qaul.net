

pkg_search_module (WEBKIT REQUIRED webkitgtk-3.0 webkit-1.0)

if (Raspberry)
	add_definitions(-DQAUL_STOP_NETWORKING)
endif ()

add_executable (qaul-gtk 
	main.c
	qaul_configure.c
	configure.c
	configure_nm.c
	configure_cli.c
	networkmanager_configuration.c
)

target_include_directories(qaul-gtk PRIVATE
	${DBUS1_INCLUDE_DIRS}
	${WEBKIT_INCLUDE_DIRS}
)

target_link_libraries(qaul-gtk
	${DBUS1_LIBRARIES}
	${WEBKIT_LIBRARIES}
    pthread
    libqaul
)

install (TARGETS qaul-gtk DESTINATION bin)
