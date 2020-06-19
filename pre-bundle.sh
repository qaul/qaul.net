#!/usr/bin/env bash

set -ex

# Run this script before assembling the application

BASEDIR=$(realpath $(dirname "$0"))
NOW_DIR=$(pwd)

cd $BASEDIR
cp -vr ../../webgui/dist app/src/main/webgui
cd $NOW_DIR
