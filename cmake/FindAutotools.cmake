
#--------------------------------------------------------------------------------
# Copyright (c) 2012-2013, Lars Baehren <lbaehren@gmail.com>
# All rights reserved.
#
# Redistribution and use in source and binary forms, with or without modification,
# are permitted provided that the following conditions are met:
#
#  * Redistributions of source code must retain the above copyright notice, this
#    list of conditions and the following disclaimer.
#  * Redistributions in binary form must reproduce the above copyright notice,
#    this list of conditions and the following disclaimer in the documentation
#    and/or other materials provided with the distribution.
#
# THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS"
# AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
# IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
# DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE
# FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
# DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
# SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER
# CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY,
# OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
# OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
#--------------------------------------------------------------------------------


# - Check for the presence of AUTOTOOLS
#
# The following variables are set when AUTOTOOLS is found:
#  AUTOTOOLS_FOUND      = Set to true, if all components of AUTOTOOLS have been found.
include(FindPackageHandleStandardArgs)

if (NOT AUTOTOOLS_FOUND)

  if (NOT AUTOTOOLS_ROOT_DIR)
    set (AUTOTOOLS_ROOT_DIR ${CMAKE_INSTALL_PREFIX})
  endif (NOT AUTOTOOLS_ROOT_DIR)

  ##_____________________________________________________________________________
  ## Check for the executable

  foreach (_program
      autoupdate
      autoscan
      autoreconf
      autom4te
      autoheader
      autoconf
      )

    string (TOUPPER ${_program} _varProgram)

    find_program (${_varProgram}_EXECUTABLE ${_program}
      HINTS ${AUTOTOOLS_ROOT_DIR} ${CMAKE_INSTALL_PREFIX}
      PATH_SUFFIXES bin
      )

  endforeach (_program)

  ##_____________________________________________________________________________
  ## Actions taken when all components have been found

  find_package_handle_standard_args (AUTOTOOLS DEFAULT_MSG AUTOCONF_EXECUTABLE)

  if (AUTOTOOLS_FOUND)
    if (NOT AUTOTOOLS_FIND_QUIETLY)
      message (STATUS "Found components for AUTOTOOLS")
      message (STATUS "AUTOTOOLS_ROOT_DIR  = ${AUTOTOOLS_ROOT_DIR}"  )
      message (STATUS "AUTOCONF_EXECUTABLE = ${AUTOCONF_EXECUTABLE}" )
      message (STATUS "AUTOSCAN_EXECUTABLE = ${AUTOSCAN_EXECUTABLE}" )
    endif (NOT AUTOTOOLS_FIND_QUIETLY)
  else (AUTOTOOLS_FOUND)
    if (AUTOTOOLS_FIND_REQUIRED)
      message (FATAL_ERROR "Could not find AUTOTOOLS!")
    endif (AUTOTOOLS_FIND_REQUIRED)
  endif (AUTOTOOLS_FOUND)

  ##_____________________________________________________________________________
  ## Mark advanced variables

  mark_as_advanced (
    AUTOTOOLS_ROOT_DIR
    AUTOCONF_EXECUTABLE
    AUTOSCAN_EXECUTABLE
    )

endif (NOT AUTOTOOLS_FOUND)
