
if (Raspberry)
	add_definitions(-DQAUL_STOP_NETWORKING)
endif ()

add_executable (qauld
	main.c
	../../client/gtk/qaul_configure.c
	../../client/gtk/configure.c
	../../client/gtk/configure_nm.c
	../../client/gtk/configure_cli.c
	../../client/gtk/networkmanager_configuration.c
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
