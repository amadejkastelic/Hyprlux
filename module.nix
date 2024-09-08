hyprlux: {
  pkgs,
  lib,
  config,
  ...
}: let
  timeRegex = ''^([0-1]?[0-9]|2[0-3]):[0-5][0-9]$'';
  time = lib.types.strMatching timeRegex;

  vibrance = lib.mkOption {
    description = "Vibrance configuration";
    type = with lib.types;
      attrsOf (submodule {
        options = {
          window_class = lib.mkOption {
            window_class = "Window class name or regex";
            type = str;
            default = "";
          };
          window_title = lib.mkOption {
            window_class = "Window title name or regex";
            type = str;
            default = "";
          };
          strength = lib.mkOption {
            description = "Vibrance strength (1-100)";
            type = int;
            default = 100;
          };
        };
      });
    default = {};
    example = {
      window_class = "^(steam_app_)(.*)$";
      window_title = "";
      strength = 100;
    };
  };

  cfg = config.programs.hyprlux;
  cfgFormat = pkgs.formats.toml {};

  pkg = hyprlux.packages.${pkgs.system}.default;
in {
  options.programs.hyprlux = {
    enable = lib.mkEnableOption "Enable hyprlux";

    package =
      lib.mkPackageOption pkgs "hyprlux" {}
      // {
        default = pkg;
      };

    night_light = lib.mkOption {
      description = "Night light configuration";
      type = with lib.types;
        attrsOf (submodule {
          options = {
            enable = lib.mkOption {
              description = "Enable night light";
              type = bool;
              default = false;
            };
            start_time = lib.mkOption {
              description = "When to start night light";
              type = time;
              default = "20:00";
            };
            end_time = lib.mkOption {
              description = "When to end night light";
              type = time;
              default = "06:00";
            };
            temperature = lib.mkOption {
              description = "Night light temperature";
              type = int;
              default = 3500;
            };
          };
        });
      default = {};
      example = {
        enabled = true;
        start_time = "20:00";
        end_time = "06:00";
        temperature = 3500;
      };
    };

    vibrance_configs = lib.mkOption {
      description = "List of vibrance configurations";
      type = lib.types.listOf vibrance;
      default = [];
      example = [
        {
          window_class = "^(steam_app_)(.*)$";
          window_title = "";
          strength = 100;
        }
        {
          window_class = "Firefox";
          window_title = "";
          strength = 10;
        }
      ];
    };
  };

  config = lib.mkIf cfg.enable {
    home.packages = [
      cfg.package
    ];

    xdg.configFile."hyprland/hyprlux.toml" = {
      source = cfgFormat.generate "hyprlux.toml" cfg.settings;
    };
  };
}
