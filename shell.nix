with import <nixpkgs> {
  config.android_sdk.accept_license = true;
  config.allowUnfree = true;
};

stdenv.mkDerivation {
  name = "qaul";
  buildInputs = with pkgs; [

    # General rust stuff
    rustracer rustup clangStdenv cargo-watch

    # Required for the docs
    mdbook graphviz

    # Required for Android integration
    cmake

    # Required for libqaul-voice
    libopus pkg-config
    steam-run

    # Required for the code coverage and stuff
    openssl
  ] ++ (with androidenv.androidPkgs_9_0; [
    # Required for Android builds
    androidsdk
    build-tools
    ndk-bundle
    platform-tools

    pkgs.openjdk
  ]);
}
