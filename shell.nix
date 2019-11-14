with import <nixpkgs> {};

stdenv.mkDerivation {
  name = "qaul";
  buildInputs = with pkgs; [
    rustracer rustup clangStdenv
  ];
}
