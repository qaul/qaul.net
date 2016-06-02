
set (QPLUG_VERSION_MAJOR 0)
set (QPLUG_VERSION_MINOR 1)
set (QPLUG_VERSION_STRING ${QPLUG_VERSION_MAJOR}.${QPLUG_VERSION_MINOR})

ADD_LIBRARY (olsrd_qaul SHARED
    qaul_ipc.c
    qaul_msg.c
    qaul_msg_LL.c
    qaul_net.c
    qaul_olsrd_plugin.c
)

set_target_properties (olsrd_qaul PROPERTIES PREFIX "")
set_target_properties (olsrd_qaul PROPERTIES VERSION ${QPLUG_VERSION_STRING})
set_target_properties (olsrd_qaul PROPERTIES LINK_FLAGS -Wl,--version-script=${CMAKE_CURRENT_SOURCE_DIR}/version-script.txt)

add_dependencies (olsrd_qaul olsr)

target_include_directories (olsrd_qaul PRIVATE
    ${PROJECT_BINARY_DIR}/third_party/olsr/src/olsr/src
)

install(TARGETS olsrd_qaul DESTINATION lib/qaul/lib)

add_custom_target(olsrd_qaul_android
	COMMAND ${CMAKE_COMMAND} -DSRC=${CMAKE_BINARY_DIR}/src/olsr-plugin/olsrd_qaul.so.0.1 -DDEST=${CMAKE_BINARY_DIR}/android/app/src/main/res/raw/ -P ${CMAKE_SOURCE_DIR}/cmake/FileCopy.cmake
	COMMAND ${CMAKE_COMMAND} -DSRC=${CMAKE_BINARY_DIR}/android/app/src/main/res/raw/olsrd_qaul.so.0.1 -DDEST=${CMAKE_BINARY_DIR}/android/app/src/main/res/raw/olsrd_qaul_so_0_1 -P ${CMAKE_SOURCE_DIR}/cmake/FileRename.cmake
	DEPENDS olsrd_qaul
)
