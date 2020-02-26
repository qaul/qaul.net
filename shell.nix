with import <nixpkgs> {
  config.android_sdk.accept_license = true;
};

stdenv.mkDerivation {
  name = "qaul";
  buildInputs = with pkgs; [
    rustracer rustup clangStdenv

    cargo-watch

    # Required for the docs
    mdbook graphviz

    # Required for the Android builds
    androidenv.androidPkgs_9_0.androidsdk
  ];
}
