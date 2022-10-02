{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/master";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay, ... }:
    flake-utils.lib.eachSystem
      [ "x86_64-linux" ]
      (system:
        let
          overlays = [ (import rust-overlay) ];
          pkgs = import nixpkgs {
            inherit system overlays;
          };
        in
        rec
        {
          devShell = pkgs.mkShell rec {
            buildInputs = with pkgs; [
              rust-bin.stable.latest.default
              rust-analyzer
              cargo-deny
            ];

            LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath buildInputs;
          };
        });
}
