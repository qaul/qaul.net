#!/bin/bash
set -e

#-----------------------------------------------------------------NoticeStart-
# CI Scripts
#
# Copyright (c) 2024 Open Community Project Association https://ocpa.ch
# This software is published under the AGPLv3 license.
#-----------------------------------------------------------------NoticeEnd---
#
# DESCRIPTION
#   This pre-xcodebuild is meant to manually sign libqaul's dylib.
#


# Force signature on dylib
CODE_SIGNING_IDENTITY=$(security find-identity -v -p codesigning | grep "Apple Distribution: Verein" | awk '{print $2}')
codesign --force --sign "$CODE_SIGNING_IDENTITY" qaul_ui/macos/liblibqaul.dylib

exit 0