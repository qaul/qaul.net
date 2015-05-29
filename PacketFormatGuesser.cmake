
# define a set of string with may-be useful readable name
# this file is meant to be included in a CMakeLists.txt
# not as a standalone CMake script
set(SPECIFIC_SYSTEM_PREFERED_CPACK_GENERATOR "")

if(CMAKE_SYSTEM_NAME MATCHES "Linux")
  set(SPECIFIC_SYSTEM_PREFERED_CPACK_GENERATOR "TGZ")
  if(EXISTS "/etc/redhat-release")
    set(SPECIFIC_SYSTEM_PREFERED_CPACK_GENERATOR "RPM")
  endif(EXISTS "/etc/redhat-release")
  if(EXISTS "/etc/debian_version")
    set(SPECIFIC_SYSTEM_PREFERED_CPACK_GENERATOR "DEB")
  endif(EXISTS "/etc/debian_version")
  if(EXISTS "/etc/SuSE-release")
    set(SPECIFIC_SYSTEM_PREFERED_CPACK_GENERATOR "RPM")
  endif(EXISTS "/etc/SuSE-release")
endif(CMAKE_SYSTEM_NAME MATCHES "Linux")
