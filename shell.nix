{ pkgs ? import <nixpkgs> { } }:

let
  rust-overlay = (import (builtins.fetchTarball "https://github.com/oxalica/rust-overlay/archive/master.tar.gz"));
  pkgs = (import <nixpkgs> {
    overlays = [ rust-overlay ];
  });
in
pkgs.mkShell {
  buildInputs = [
    (pkgs.rust-bin.stable.latest.default.override {
      extensions = [ "rust-src" ];
    })
  ];
  PKG_CONFIG_PATH = "${pkgs.openssl.dev}/lib/pkgconfig";
  # nativeBuildInputs is usually what you want -- tools you need to run
  nativeBuildInputs = [
    pkgs.pkg-config
    pkgs.rust-analyzer
  ];
}
