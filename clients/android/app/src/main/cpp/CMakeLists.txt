cmake_minimum_required(VERSION 3.4.1)

include(../../../../../../librobot)

add_library(native-lib SHARED
                       native-lib.cpp )

find_library( # Sets the name of the path variable.
              log-lib

              # Specifies the name of the NDK library that
              # you want CMake to locate.
              log )

target_link_libraries(native-lib
                       ${log-lib} )

