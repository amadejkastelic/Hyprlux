{
  description = " Hyprland utility that automates vibrance and night light control  ";
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    systems.url = "github:nix-systems/default-linux";
    pre-commit-hooks = {
      url = "github:cachix/pre-commit-hooks.nix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };
  outputs =
    {
      self,
      nixpkgs,
      systems,
      ...
    }@inputs:
    let
      forAllSystems = nixpkgs.lib.genAttrs (import systems);
      pkgsFor = nixpkgs.legacyPackages;
    in
    {
      packages = forAllSystems (system: {
        default = pkgsFor.${system}.callPackage ./nix/. { };
      });

      devShells = forAllSystems (system: {
        default = pkgsFor.${system}.callPackage ./shell.nix {
          inherit (self.checks.${system}) pre-commit-check;
        };
      });

      checks = forAllSystems (system: {
        pre-commit-check = inputs.pre-commit-hooks.lib.${system}.run {
          settings.rust.check.cargoDeps = pkgsFor.${system}.rustPlatform.importCargoLock {
            lockFile = ./Cargo.lock;
          };
          src = ./.;
          hooks = {
            nixfmt-rfc-style.enable = true;
            clippy.enable = true;
            rustfmt.enable = true;
            cargo-check.enable = true;
          };
        };
      });

      nixosModules = {
        hyprlux = import ./nix/module.nix self;
        default = self.nixosModules.hyprlux;
      };
      homeManagerModules = {
        hyprlux = import ./nix/hm-module.nix self;
        default = self.homeManagerModules.hyprlux;
      };
    };
}
