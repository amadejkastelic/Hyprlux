{
  description = " Hyprland utility that automates vibrance and night light control  ";
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    systems.url = "github:nix-systems/default-linux";
  };
  outputs = {
    self,
    nixpkgs,
    systems,
  }: let
    forAllSystems = nixpkgs.lib.genAttrs (import systems);
    pkgsFor = nixpkgs.legacyPackages;
  in {
    packages = forAllSystems (system: {
      default = pkgsFor.${system}.callPackage ./nix/. {};
    });
    devShells = forAllSystems (system: {
      default = pkgsFor.${system}.callPackage ./nix/shell.nix {};
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
