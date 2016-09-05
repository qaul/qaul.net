
add_executable (qauld
	main.c
	qaul_configure.c
	configure.c
	configure_nm.c
	configure_cli.c
	networkmanager_configuration.c
)

target_include_directories(qauld PRIVATE 
	${DBUS1_INCLUDE_DIRS}
)

target_link_libraries(qauld
	${DBUS1_LIBRARIES}
    pthread
    libqaul
)

install (TARGETS qauld DESTINATION bin)
