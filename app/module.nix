{
  pkgs,
  lib,
  config,
  ...
}:
with lib; let
  cfg = config.services.basedradio-app;
in {
  options.services.basedradio-app = {
    enable = mkEnableOption "Enable BasedRadio WebApp Container";

    musicDir = mkOption {
      type = types.str;
      default = "/Music";
      description = "Local path to the radio files";
    };

    apiUrl = mkOption {
      type = types.str;
      default = "https://api.based.radio";
      description = "URL of the basedradio api";
    };

    streamUrl = mkOption {
      type = types.str;
      default = "https://cast.based.radio/vgm.mp3";
      description = "URL of the basedradio stream";
    };
  };

  config = mkIf cfg.enable {
    # networking.firewall.allowedTCPPorts = mkIf cfg.api.openFirewall [cfg.api.port];
    virtualisation.oci-containers.containers.basedradio-app = {
        image = "basedradio-app";
        imageFile = ./Dockerfile;
        ports = ["8080:8080"];
        environment = {
          STREAM_MP3 = cfg.streamUrl;
          API_URL = cfg.apiUrl
        };
      };
    };
}
