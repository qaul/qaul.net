with import <nixpkgs> {};

stdenv.mkDerivation {
  name = "qaul";
  buildInputs = with pkgs; [
    rustracer rustup clangStdenv

    cargo-watch

    # Required for the docs
    mdbook graphviz
  ];
}
