{
  self,
}:
{
  config,
  lib,
  pkgs,
  ...
}:

let
  cfg = config.programs.zdir;
in
{

  options.programs.zdir = {
    enable = lib.mkEnableOption "Enable zdir and zdir-librarian";
    package = lib.mkOption {
      type = lib.types.package;
      default = self.packages.${pkgs.stdenv.hostPlatform.system}.zdir;
      description = "The zdir package to use";
    };
    enableNushellIntegration = lib.mkEnableOption "Enable Nushell integration";
  };

  config = lib.mkIf cfg.enable {
    home.packages = [ cfg.package ];

    systemd.user.services.zdir-librarian = {
      Unit.Description = "zdir-librarian";
      Service.ExecStart = "${cfg.package}/bin/zdir-librarian";
      Install.WantedBy = [ "default.target" ];
    };

    programs.nushell = lib.mkIf cfg.enableNushellIntegration {
      extraConfig = ''
        source ${cfg.package}/share/zdir/zdir.nu
      '';
    };
  };

}
