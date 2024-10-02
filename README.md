# Hyprlux

A Hyprland utility program that automatically switches between shaders.

It currently supports two shaders with configurable strengths:
- Night light (blue light filter) - based on location or custom times
- Vibrance - toggles digital vibrance based on currently active window

## Installation

<h4>
     <sub>
          <img  src="https://cdn.simpleicons.org/nixos/white"
           height="20"
           width="20">
     </sub>
     NixOS
     </a><a href="nix"><img alt="NixOS Version" src="https://img.shields.io/badge/git-brightgreen" align="right"></a>
</h4>

<details>
  <summary>Click to expand</summary>

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
        # Manual sunset and sunrise
        start_time = "22:00";
        end_time = "06:00";
        # Automatic sunset and sunrise
        latitude = 46.056946;
        longitude = 14.505751;
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
</details>

<h4>
     <sub>
          <img  src="https://cdn.simpleicons.org/archlinux/white"
           height="20"
           width="20">
     </sub>
     Arch
     <a href="https://aur.archlinux.org/packages/hyprlux"><img alt="AUR Version" src="https://img.shields.io/aur/version/hyprlux?color=brightgreen&label=" align="right"></a>
</h4>

<details>
  <summary>Click to expand</summary>

  Install using your favorite AUR helper:
  ```bash
  paru -S hyprlux
  ```
</details>

## Configuration
Hyprlux looks for configs in the following locations (sorted by priority):
- Path passed as first argument when running the binary
- `$XDG_CONFIG_HOME/hypr/hyprlux.toml`
- `/etc/hyprlux/config.toml`

Example configurations are available in [examples](examples/).

## Building
Run `cargo build`

## TODO
- [x] Toggle night light based on location and time of day
- [x] Allow config reload
- [x] Publish to aur
- [x] Add nix module systemd service support
- [ ] Publish crate
