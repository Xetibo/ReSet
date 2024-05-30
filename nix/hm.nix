self: { config
      , pkgs
      , lib
      , hm
      , ...
      }:
let
  cfg = config.programs.reset;
  defaultPackage = self.packages.${pkgs.stdenv.hostPlatform.system}.default;
in
{
  meta.maintainers = with lib.maintainers; [ DashieTM ];
  options.programs.reset = with lib; {
    enable = mkEnableOption "reset";

    package = mkOption {
      type = with types; nullOr package;
      default = defaultPackage;
      defaultText = lib.literalExpression ''
        reset.packages.''${pkgs.stdenv.hostPlatform.system}.default
      '';
      description = mdDoc ''
        Package to run
      '';
    };

    config = {
      plugins = mkOption {
        type = with types; nullOr (listOf package);
        default = null;
        description = mdDoc ''
          List of plugins to use, represented as a list of packages.
        '';
      };

      plugin_config = mkOption {
        # TODO: define type 
        # type = with types; nullOr (listOf toml);
        default = { };
        description = mdDoc ''
          Toml values passed to the configuration for plugins to use. 
        '';
      };
    };

  };
  config =
    let
      fetchedPlugins =
        if cfg.config.plugins == null
        then [ ]
        else
          builtins.map
            (entry:
              if lib.types.package.check entry
              then "lib${entry.pname}.so"
              else "")
            cfg.config.plugins;
      path =
        if cfg.config.plugins == null
        then ""
        else
          "${lib.lists.last cfg.config.plugins}/lib";
    in
    lib.mkIf cfg.enable {
      home.packages = lib.optional (cfg.package != null) cfg.package;

      xdg.configFile."reset/ReSet.toml".source = (pkgs.formats.toml { }).generate "reset"
        {
          plugins = fetchedPlugins;
          plugin_path = path;
        }; #++ (pkgs.formats.toml cfg.config.plugin_config);
    };
}
