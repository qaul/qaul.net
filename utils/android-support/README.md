# qaul.net android support

This library provides android specific bindings for the qaul.net
ecosystem (`libqaul`, `ratman`, `ratman-configure`, `netmod-wd`,
`netmod-tcp`, and the primary services)

You don't need to build this library by yourself, this is handled by
the android build harness.  If you want to develop on this crate (for
testing or other reasons), a normal Rust development setup will be
enough, as long as you can cross-compile the crate for the `android`
OS (because compilation is feature gated in some places).

The easiest way to do this is to use the `qaulnet/android-build-env`
docker image (available on the docker hub).
