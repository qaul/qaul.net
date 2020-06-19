#!/usr/bin/env bash

set -e

BASEDIR=$(realpath $(dirname "$0"))
USER=$(id -u)
GROUP=$(id -g)

if [ $1 = "dev" ]; then
    echo "Attaching shell for repeated builds."
    echo "Don't invoke gradle yourself! Use 'client/android/.build_nested.sh' instead!"
    echo "Don't forget to run 'export USER=$USER GROUP=$GROUP'!"
    docker run --rm -it -v $BASEDIR/../../:/qaul.net qaulnet/android-build-env /bin/bash
  
else
    echo "Running one-shot-build"
    docker run --rm -it -v $BASEDIR/../../:/qaul.net qaulnet/android-build-env \
           /qaul.net/clients/android/.build_nested.sh $USER $GROUP
fi
