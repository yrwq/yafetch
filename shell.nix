{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  buildInputs = with pkgs; [
    gcc gnumake
    lua5_4
  ];
  nativeBuildInputs = [ pkgs.pkg-config ];
}
