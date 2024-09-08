# Hyprlux

A Hyprland utility program that automatically switches between shaders.

## Installation

### Nix
Add hyprflux to your flake inputs:
```nix
inputs = {
  hyprlux = {
    url = "github:amadejkastelic/Hyprlux";
  };
};
```
Then import it and use it as a module:
```nix
{inputs, ...}: {
  imports = [
    inputs.hyprlux.nixosModules.default
  ];

  programs.hyprlux = {
    enable = true;

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
[ ] Toggle night light based on location and time of day
