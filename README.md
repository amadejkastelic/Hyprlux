# Hyprlux

A Hyprland utility program that automatically switches between shaders.

## Installation

### Nix
Add hyprlux to your flake inputs:
```nix
inputs = {
  hyprlux = {
    url = "github:amadejkastelic/Hyprlux";
  };
};
```
Then import either the home manager module or nixos module:
```nix
imports = [
  inputs.hyprlux.nixosModules.default
];
```
```nix
imports = [
  inputs.hyprlux.homeManagerModules.default
];
```
And configure it:
```nix
{inputs, ...}: {
  programs.hyprlux = {
    enable = true;

    systemd = {
      enable = true;
      target = "hyprland-session.target";
    };

    night_light = {
      enabled = true;
      start_time = "22:00";
      end_time = "06:00";
      temperature = 3500;
    };

    vibrance_configs = [
      {
        window_class = "steam_app_1172470";
        window_title = "Apex Legends";
        strength = 100;
      }
      {
        window_class = "cs2";
        window_title = "";
        strength = 100;
      }
    ];
  };
}
```
## Building
Run `cargo build`

## TODO
- [ ] Toggle night light based on location and time of day
- [x] Allow config reload
- [ ] Allow stop and resume
- [ ] Publish to aur and crate
- [x] Add nix module systemd service support
