hyprlux:
{
  pkgs,
  lib,
  config,
  ...
}:
let
  time = lib.types.strMatching ''^([0-1]?[0-9]|2[0-3]):[0-5][0-9]$'';

  nightLightSubmodule = lib.types.submodule {
    options = {
      enabled = lib.mkOption {
        description = "Enabled night light";
        type = lib.types.bool;
        default = false;
      };
      latitude = lib.mkOption {
        description = "Your latitude";
        type =
          with lib.types;
          nullOr (oneOf [
            int
            float
          ]);
        default = null;
      };
      longitude = lib.mkOption {
        description = "Your longitude";
        type =
          with lib.types;
          nullOr (oneOf [
            int
            float
          ]);
        default = null;
      };
      start_time = lib.mkOption {
        description = "When to start night light";
        type = lib.types.nullOr time;
        default = "20:00";
      };
      end_time = lib.mkOption {
        description = "When to end night light";
        type = lib.types.nullOr time;
        default = "06:00";
      };
      temperature = lib.mkOption {
        description = "Night light temperature";
        type = lib.types.int;
        default = 3500;
      };
    };
  };

  vibranceSubmodule = lib.types.submodule {
    options = {
      window_class = lib.mkOption {
        description = "Window class name or regex";
        type = lib.types.str;
        default = "";
      };
      window_title = lib.mkOption {
        description = "Window title name or regex";
        type = lib.types.str;
        default = "";
      };
      strength = lib.mkOption {
        description = "Vibrance strength (1-100)";
        type = lib.types.int;
        default = 100;
      };
    };
  };

  cfg = config.programs.hyprlux;
  cfgFormat = pkgs.formats.toml { };

  pkg = hyprlux.packages.${pkgs.stdenv.hostPlatform.system}.default;
in
{
  options.programs.hyprlux = {
    enable = lib.mkEnableOption "Enable hyprlux";

    package = lib.mkPackageOption pkgs "hyprlux" { } // {
      default = pkg;
    };

    night_light = lib.mkOption {
      type = nightLightSubmodule;
      description = "Night light settings";
      default = {
        enabled = false;
        start_time = "20:00";
        end_time = "06:00";
        temperature = 3500;
      };
      example = {
        enabled = true;
        latitude = 46.056946;
        longitude = 14.505751;
        temperature = 3500;
      };
    };

    vibrance_configs = lib.mkOption {
      description = "List of vibrance configurations";
      type = lib.types.listOf vibranceSubmodule;
      default = [ ];
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

    hot_reload = lib.mkOption {
      description = "Listen for config changes";
      type = lib.types.bool;
      default = false;
      example = true;
    };
  };

  config = lib.mkIf cfg.enable {
    environment.systemPackages = [
      cfg.package
    ];

    environment.etc."hyprlux/config.toml" = {
      source = cfgFormat.generate "config.toml" {
        night_light = lib.attrsets.filterAttrs (n: v: v != null) cfg.night_light;
        vibrance_configs = cfg.vibrance_configs;
        hot_reload = cfg.hot_reload;
      };
    };
  };
}
