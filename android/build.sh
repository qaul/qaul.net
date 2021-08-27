#!/usr/bin/env bash

# Build rust libqaul for android targets via gradle
#./gradlew cargoBuild

# Copy library to libqaul
jniLibs=../android/libqaul/src/main/jniLibs
libName=liblibqaul.so
buildType=debug

rm -rf ${jniLibs}

mkdir ${jniLibs}
mkdir ${jniLibs}/arm64-v8a
mkdir ${jniLibs}/armeabi-v7a
mkdir ${jniLibs}/x86
mkdir ${jniLibs}/x86_64

cp ../target/aarch64-linux-android/${buildType}/${libName} ${jniLibs}/arm64-v8a/${libName}
cp ../target/armv7-linux-androideabi/${buildType}/${libName} ${jniLibs}/armeabi-v7a/${libName}
cp ../target/i686-linux-android/${buildType}/${libName} ${jniLibs}/x86/${libName}
cp ../target/${buildType}/${libName} ${jniLibs}/x86_64/${libName}


## evtl. alternative location
jniLibsBuild=../android/libqaul/build/rustJniLibs

#cp target/aarch64-linux-android/${buildType}/${libName} ${jniLibsBuild}/android/arm64-v8a/${libName}
#cp target/armv7-linux-androideabi/${buildType}/${libName} ${jniLibsBuild}/android/armeabi-v7a/${libName}
#cp target/i686-linux-android/${buildType}/${libName} ${jniLibsBuild}/android/x86/${libName}
#cp target/x86_64-linux-android/${buildType}/${libName} ${jniLibsBuild}/desktop/linux-x86-64/${libName}
