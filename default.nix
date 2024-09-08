{pkgs ? import <nixpkgs> {}}: let
  manifest = (pkgs.lib.importTOML ./Cargo.toml).package;
in
  pkgs.rustPlatform.buildRustPackage {
    pname = manifest.name;
    version = manifest.version;
    cargoLock.lockFile = ./Cargo.lock;
    cargoLock.outputHashes = {
      "hyprland-0.4.0-alpha.3" = "";
    };
    src = pkgs.lib.cleanSource ./.;
  }
