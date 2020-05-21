#!/usr/bin/env bash

set -ex

BASEDIR=$(realpath $(dirname "$0"))
USER=$(id -u)
GROUP=$(id -g)

docker run --rm -it -v $BASEDIR/../../:/qaul.net qaulnet/android-build-env \
  /qaul.net/clients/android/.build_nested.sh $USER $GROUP
