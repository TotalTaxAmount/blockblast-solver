{
  pkgs ? import <nixpkgs> {
    overlays = [
      (import (fetchTarball "https://github.com/oxalica/rust-overlay/archive/master.tar.gz"))
    ];
  },
}:

let
  rustToolchain = pkgs.rust-bin.stable.latest.default;
in
pkgs.mkShell {
  buildInputs = [
    rustToolchain
    pkgs.rust-analyzer
  ];
}
