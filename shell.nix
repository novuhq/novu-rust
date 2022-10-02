{ pkgs ? import <nixpkgs> { } }:

pkgs.mkShell {
  PKG_CONFIG_PATH = "${pkgs.openssl.dev}/lib/pkgconfig";
  # nativeBuildInputs is usually what you want -- tools you need to run
  nativeBuildInputs = [
    pkgs.rustc
    pkgs.cargo
    pkgs.pkg-config
  ];
}
