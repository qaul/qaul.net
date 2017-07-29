
set (QPLUG_VERSION_MAJOR 0)
set (QPLUG_VERSION_MINOR 1)
set (QPLUG_VERSION_STRING ${QPLUG_VERSION_MAJOR}.${QPLUG_VERSION_MINOR})

if (${PORT} STREQUAL "OSX")
	set(CMAKE_INSTALL_RPATH_USE_LINK_PATH TRUE)
	set(CMAKE_C_COMPILER gcc)
	set(CMAKE_CXX_COMPILER g++)
	set(QPLUG_LIBRARY_TYPE MODULE)
else()
	set(QPLUG_LIBRARY_TYPE SHARED)
endif ()

ADD_LIBRARY (olsrd_qaul ${QPLUG_LIBRARY_TYPE}
	qaul_ipc.c
	qaul_msg.c
	qaul_msg_LL.c
	qaul_net.c
	qaul_olsrd_plugin.c
)

set_target_properties (olsrd_qaul PROPERTIES PREFIX "")

if (${PORT} STREQUAL "OSX")
	set(CMAKE_SHARED_MODULE_CREATE_C_FLAGS "${CMAKE_SHARED_MODULE_CREATE_C_FLAGS} -flat_namespace -undefined suppress")
	set_target_properties (olsrd_qaul PROPERTIES SUFFIX ".so.${QPLUG_VERSION_STRING}")
else ()
	set_target_properties (olsrd_qaul PROPERTIES VERSION ${QPLUG_VERSION_STRING})
	set_target_properties (olsrd_qaul PROPERTIES LINK_FLAGS -Wl,--version-script=${CMAKE_CURRENT_SOURCE_DIR}/version-script.txt)
endif ()

add_dependencies (olsrd_qaul olsr libqaul)

target_include_directories (olsrd_qaul PRIVATE
    ${PROJECT_BINARY_DIR}/third_party/olsr/src/olsr/src
)

if (${PORT} STREQUAL "WINDOWS")
    target_include_directories (olsrd_qaul PRIVATE ${PROJECT_BINARY_DIR}/third_party/olsr/src/olsr/src/win32)
    target_link_libraries(olsrd_qaul PRIVATE ${CMAKE_BINARY_DIR}/third_party/olsr/src/olsr/libolsrd.a)
    target_link_libraries(olsrd_qaul PRIVATE ws2_32)
endif()

install(TARGETS olsrd_qaul DESTINATION lib/qaul/lib)

add_custom_target(olsrd_qaul_android
	COMMAND ${CMAKE_COMMAND} -DSRC=${CMAKE_BINARY_DIR}/src/olsrd-plugin/olsrd_qaul.so.0.1 -DDEST=${CMAKE_BINARY_DIR}/android/app/src/main/res/raw/ -P ${CMAKE_SOURCE_DIR}/cmake/FileCopy.cmake
	COMMAND ${CMAKE_COMMAND} -DSRC=${CMAKE_BINARY_DIR}/android/app/src/main/res/raw/olsrd_qaul.so.0.1 -DDEST=${CMAKE_BINARY_DIR}/android/app/src/main/res/raw/olsrd_qaul_so_0_1 -P ${CMAKE_SOURCE_DIR}/cmake/FileRename.cmake
	DEPENDS olsrd_qaul
)
