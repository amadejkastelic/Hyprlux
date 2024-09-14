hyprlux: {
  pkgs,
  lib,
  config,
  ...
}: let
  time = lib.types.strMatching ''^([0-1]?[0-9]|2[0-3]):[0-5][0-9]$'';

  nightLightSubmodule = lib.types.submodule {
    options = {
      enabled = lib.mkOption {
        description = "Enabled night light";
        type = lib.types.bool;
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

    systemd.enable = lib.mkEnableOption "Use as a systemd service";

    systemd.target = lib.mkOption {
      type = lib.types.str;
      default = "graphical-session.target";
      example = "hyprland-session.target";
      description = ''
        The systemd target that will automatically start the Hyprlux service.

        When setting this value to `"hyprland-session.target"`,
        make sure to also enable {option}`wayland.windowManager.hyprland.systemd.enable`,
        otherwise the service may never be started.
      '';
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
        start_time = "20:00";
        end_time = "06:00";
        temperature = 3500;
      };
    };

    vibrance_configs = lib.mkOption {
      description = "List of vibrance configurations";
      type = lib.types.listOf vibranceSubmodule;
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

  config = lib.mkIf cfg.enable (lib.mkMerge [
    {
      home.packages = [cfg.package];

      xdg.configFile."hypr/hyprlux.toml" = {
        source = cfgFormat.generate "hyprlux.toml" {
          night_light = cfg.night_light;
          vibrance_configs = cfg.vibrance_configs;
        };
      };
    }

    (lib.mkIf cfg.systemd.enable {
      systemd.user.services.hyprlux = {
        Unit = {
          Description = "Hyprlux shader manager service";
          Documentation = "https://github.com/amadejkastelic/Hyprlux";
          PartOf = ["graphical-session.target"];
          After = ["graphical-session-pre.target"];
        };

        Service = {
          ExecStart = "${cfg.package}/bin/hyprlux";
          ExecReload = "${cfg.coreutils}/bin/kill -SIGUSR2 $MAINPID";
          Restart = "on-failure";
          KillMode = "mixed";
        };

        Install = {
          WantedBy = [cfg.systemd.target];
        };
      };
    })
  ]);
}
