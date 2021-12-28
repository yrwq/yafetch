with import <nixpkgs> {};
stdenv.mkDerivation rec {
  name = "yafetch-env";
  buildInputs = with pkgs; [
    lua54Packages.lua
  ];
  nativeBuildInputs = with pkgs; [ pkg-config ];
}
