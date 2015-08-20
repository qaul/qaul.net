
if (${PORT} STREQUAL "LINUX")
	set(QAULHELPER_DISTROFILE linux.c)
elseif (${PORT} STREQUAL "OSX")
	set(QAULHELPER_DISTROFILE osx.c)
else ()
	message(FATAL_ERROR "Something to fix")
endif ()

add_executable(qaulhelper
	qaulhelper.c
	validate.c
	${QAULHELPER_DISTROFILE}
)

install (TARGETS qaulhelper DESTINATION bin
	PERMISSIONS OWNER_READ OWNER_WRITE OWNER_EXECUTE GROUP_READ GROUP_EXECUTE WORLD_READ WORLD_EXECUTE SETUID)
