{ config, lib, ... }:

let
  cfg = config.services.plcom;
in
{
  options.services.plcom = {
    enable = lib.mkEnableOption "plcom website";

    port = lib.mkOption {
      type = lib.types.port;
      default = 8000;
    };

    package = lib.mkOption {
      type = lib.types.package;
    };
  };

  config = lib.mkIf cfg.enable {
    systemd.services.plcom = {
      wantedBy = [ "multi-user.target" ];
      after = [ "network.target" ];

      serviceConfig = {
        Type = "simple";
        ExecStart = "${cfg.package}/bin/plcom";
        Restart = "on-failure";
        Environment = "ROCKET_PORT=${toString cfg.port}";
        DynamicUser = "yes";
      };
    };

    services.nginx.virtualHosts."philippeloctaux.com" = {
      forceSSL = true;
      enableACME = true;
      locations."/" = {
        proxyPass = "http://localhost:${toString cfg.port}";
      };
      locations."/wallpapers/files/" = {
        alias = "/var/www/wallpapers/";
        tryFiles = "$uri $uri/ =404";
        extraConfig = ''
          autoindex on;
        '';
      };
    };
  };
}
