with import <nixpkgs> {};
  stdenv.mkDerivation {
    name = "hyprlux-shell";
    buildInputs = [
      rustc
      cargo
      rustfmt
      clippy
      rust-analyzer
    ];
  }
