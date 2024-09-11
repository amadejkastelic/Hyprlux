{pkgs ? import <nixpkgs> {}}:
pkgs.mkShell {
  RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
  inputsFrom = [(pkgs.callPackage ./nix/default.nix {})];
  buildInputs = with pkgs; [
    rust-analyzer
    rustfmt
    clippy
  ];
}
