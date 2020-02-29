# qaul.net Android client


## How to build

You can use the provided Dockerfile to get a complete Android build
environment:

```console
$ docker build . -t qaul.net-android
$ docker run -d /path/to/qaul.net:/qaul.net -it -rm qaul.net-android /bin/bash
```

In the container you can then do:

```console
$ cd /qaul.net/clients/android
$ ./gradlew cargoBuild
$ ./gradlew assembleDebug
```

There will be a debug APK in `/qaul.net/clients/android/app/build/outputs/apk/debug`.
