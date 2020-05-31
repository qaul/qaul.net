# qauldroid

Arguably the primary client of the project exists on Android.  Being
able to create mesh networks with people around you, without needing
dedicated infrastructure was born out of the idea that a lot of people
have phones, and that creating small scale networks with phones around
you, without having to rely to SIM cards, cell towers or pre-setup
WiFi networks can enable people to avoid censorship and surveillence.

At the moment the qaul.net android client is a prototype!

## How to build

Building the Android client is a multi-step process.  Namely, it has
two external build dependencies: the Rust code, built via an
additional gradle task, and the ember.js ui, built entirely
independently.

Because cross-compiling for Android (on arm) can be a bit of a pain,
we provide a docker build environment.  The following command will
pull down the image, and build the Rust code.

### Full docker build

In this case you build the entire app inside docker.

(TODO, not fully tested)


### Partial docker build

In this case you only build the Rust code inside the docker env.  The
Java/ Kotlin dependencies are much easier to fetch as long as you have
the Android SDK installed.  But tools like Android Studio will also
help you configure them.

Run the following command to just build the Rust code.

```console
$ clients/android/build.sh
```

This script will take care of permission issues caused by the
container building everything as root already.

Finally you can finish the assembly process either in Android Studio
(and run it live on your phone or an emulator), or just use the
command-line to build an app for publishing.

```console
$ cd clients/android
$ clients/android/gradlew assemble
```

A finished APK will appear in in `clients/android/app/build/outputs/apk/release`.

If you have questions about the build process, or if you're hitting
some problem, feel free to e-mail us on the mailing list, or just our
IRC channel on freenode: #qaul.net
