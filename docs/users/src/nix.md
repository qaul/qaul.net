# Nix builds

While it is possible to install dependencies with platform specific
tools (such as `apt` on Debian, etc), it is far more recommended to
use [nix](https://nixos.org) to build qaul.net instead.

While the actual build isn't handled by nix (yet), it makes aquiring
the dependencies a lot easier.  Follow the instructions on how to
install nix on your platform [here][nix-instructions]


[nix-instructions]: https://nixos.org/download.html

## Fetch dependencies

The `shell.nix` in the qaul.net repo root defines dependencies.  Fetch
them into your environment by running `nix-shell` in the repo root
(this might take a while).

Afterwards you can simple run `cargo build --bin qaul-hubd --release`
to build a new hubd binary.

The output artefact will be written to `./target/release/qaul-hubd`.


## Lorri & direnv

You can enable automatic environment loading when you enter the
qaul.net repository, by configuring [lorri] and [direnv] on your system.

[lorri]: https://github.com/target/lorri
[direnv]: https://direnv.net/

```console
 ❤ (uwu) ~/p/code> cd qaul.net
direnv: loading ~/projects/code/qaul.net/.envrc
direnv: export +AR +AR_FOR_TARGET +AS +AS_FOR_TARGET +CC
        // ... snip ...
 ❤ (uwu) ~/p/c/qaul.net> cargo build                           lorri-keep-env-hack-qaul
 ...
```
