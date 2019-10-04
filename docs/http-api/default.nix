with import <nixpkgs> {};

stdenv.mkDerivation {
  name = "http-api";
  src = ./.;
  
  buildInputs = with pkgs; [
    mdbook
  ];

  buildPhase = ''
    mdbook build
  '';
}
