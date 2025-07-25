self:
{
  config,
  lib,
  pkgs,
  ...
}:
let
  cfg = config.services.istannouncements;
  pkg = self.packages.${pkgs.system}.default;
in
{
  options.services.istannouncements = {
    enable = lib.mkEnableOption "Enable istannouncements service";

    openFirewall = lib.mkEnableOption "Open the firewall port";

    port = lib.mkOption {
      type = lib.types.int;
      description = "The port to use to run the web server";
      default = 8000;
    };

    username = lib.mkOption {
      type = lib.types.string;
      description = "The name of the username in the annoucement message";
      default = "Fenix IST";
    };

    avatar_url = lib.mkOption {
      type = lib.types.string;
      description = "The url for the image";
      default = "https://fenix.tecnico.ulisboa.pt/api/bennu-oauth/applications/570015174623432/logo?cb=1725362687682";
    };

    webhook_url_file = lib.mkOption {
      type = lib.types.string;
      description = "The file containing the webhook url to where to send the announcement message";
    };

    mention_role = lib.mkOption {
      type = lib.types.int;
      description = "The discord role id to mention";
      default = 1280425739124215943;
    };

    poll_time = lib.mkOption {
      type = lib.types.int;
      description = "The amount of time in milis before every poll";
      default = 600000;
    };

    database_url = lib.mkOption {
      type = lib.types.string;
      description = "The path/url for the sqlite database to store data";
      default = "sqlite:///var/lib/istannouncements/istannouncements.db";
    };

    web_dir = lib.mkOption {
      type = lib.types.string;
      description = "The path to the directory containing the webpage files to server";
      default = "${pkg}/share/web";
    };

    log_level = lib.mkOption {
      type = lib.types.string;
      description = "The log level to use for the logger";
      default = "error";
    };
  };

  config =
    let
      parsed-config = pkgs.writeText "generated-istannouncements-config" ''
        username = "${cfg.username}"
        avatar_url = "${cfg.avatar_url}"
        webhook_url_file = "${cfg.webhook_url_file}"
        mention_role = ${builtins.toString cfg.mention_role}
        poll_time = ${builtins.toString cfg.poll_time}
        database_url = "${cfg.database_url}"
        web_dir = "${cfg.web_dir}"
        port = ${builtins.toString cfg.port}
      '';
    in
    lib.mkIf cfg.enable {
      environment.systemPackages = [
        pkg
      ];

      networking.firewall.allowedTCPPorts = lib.mkIf cfg.openFirewall [ cfg.port ];

      users.groups.istannouncements = { };

      users.users.istannouncements = {
        isSystemUser = true;
        group = "istannouncements";
      };

      systemd.services.istannouncements = {
        enable = true;
        after = [ "network.target" ];
        wantedBy = [ "default.target" ];
        description = "ISTAnnouncement's systemd service";
        serviceConfig = {
          Type = "simple";
          User = "istannouncements";
          Restart = "on-failure";
          Environment = ''RUST_LOG=${cfg.log_level}'';
          ExecStart = "${pkg}/bin/ist_announcements --config ${parsed-config}";
        };
      };

      systemd.tmpfiles.rules = [
        "d /var/lib/istannouncements 0770 istannouncements istannouncements"
      ];
    };
}
