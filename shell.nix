with import <nixpkgs> {};
stdenv.mkDerivation rec {
  name = "yafetch-env";
  buildInputs = with pkgs; [
    lua51Packages.lua
  ];
  nativeBuildInputs = with pkgs; [ pkg-config ];
}
