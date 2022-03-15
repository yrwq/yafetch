{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  buildInputs = with pkgs; [
    rustup
    rustc
    cargo

    lua5_4
    openssl
  ];

  nativeBuildInputs = [ pkgs.pkg-config ];
}
