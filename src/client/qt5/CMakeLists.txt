
if (${PORT} STREQUAL "LINUX")
  find_package(ALSA REQUIRED)
endif ()

# Find includes in corresponding build directories
set(CMAKE_INCLUDE_CURRENT_DIR ON)
# Instruct CMake to run moc automatically when needed.
set(CMAKE_AUTOMOC ON)

# Find the QtWidgets library
find_package(Qt5Widgets REQUIRED)
find_package(Qt5WebKit REQUIRED)

qt5_wrap_ui(UI_HEADERS ${CMAKE_SOURCE_DIR}/src/client/qt5/qaul.ui)

add_executable(qaul-qt5 main.cpp qaul.cpp
    ${UI_HEADERS}
)

qt5_use_modules(qaul-qt5 Widgets)
qt5_use_modules(qaul-qt5 WebKitWidgets)

target_include_directories (qaul-qt5 PRIVATE ${DBUS1_INCLUDE_DIRS} ${WEBKIT_INCLUDE_DIRS})

if (UUID_FOUND)
    target_link_libraries(qaul-qt5 ${UUID_LIBRARIES})
endif (UUID_FOUND)

if (OPENCORE_AMRNB_FOUND)
    target_link_libraries(qaul-qt5 ${OPENCORE_AMRNB_LIBRARIES})
endif (OPENCORE_AMRNB_FOUND)

if (OPENCORE_AMRWB_FOUND)
    target_link_libraries(qaul-qt5 ${OPENCORE_AMRWB_LIBRARIES})
endif (OPENCORE_AMRWB_FOUND)

target_link_libraries (qaul-qt5 libqaul)

target_link_libraries(qaul-qt5 ${DBUS1_LIBRARIES}
    pthread dl m
)

if (${PORT} STREQUAL "LINUX")
    target_link_libraries(qaul-qt5 ${ALSA_LIBRARIES})
endif ()

install (TARGETS qaul-qt5 DESTINATION bin)
