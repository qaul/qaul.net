
if (${PORT} STREQUAL "LINUX")
	set(QAULHELPER_DISTROFILE linux.c)
	set(QAULHELPER_DESTINATION lib/qaul/bin)
elseif (${PORT} STREQUAL "OSX")
	set(QAULHELPER_DISTROFILE osx.c)
	set(QAULHELPER_DESTINATION bin)
else ()
	message(FATAL_ERROR "Something to fix")
endif ()

if (Raspberry)
	add_definitions(-DQAUL_STOP_NETWORKING)
endif ()

add_executable(qaulhelper
	qaulhelper.c
	${QAULHELPER_DISTROFILE}
)

target_link_libraries(qaulhelper
	libqaulutils
)

install (TARGETS qaulhelper DESTINATION ${QAULHELPER_DESTINATION}
	PERMISSIONS OWNER_READ OWNER_WRITE OWNER_EXECUTE GROUP_READ GROUP_EXECUTE WORLD_READ WORLD_EXECUTE SETUID)
