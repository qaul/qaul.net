with import <nixpkgs> {
  config.android_sdk.accept_license = true;
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
    
    # Required for Android builds
    # androidenv.androidPkgs_9_0.androidsdk

    # Required for libqaul-voice
    libopus pkg-config

    # Required for the code coverage and stuff
    openssl
  ];
}
