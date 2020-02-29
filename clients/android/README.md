# qaul.net Android client


## How to build

You can use the provided Dockerfile to get a complete Android build
environment:

```console
$ docker build . -t qaul.net-android
$ docker run -d /path/to/qaul.net:/qaul.net -it -rm qaul.net-android /bin/bash
```

Or you can just build the apk with

```console
$ docker run -d /path/to/qaul.net:/qaul.net -it -rm qaul.net-android ./gradlew assembleDebug
```
