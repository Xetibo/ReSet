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
        type = with types; listOf package;
        default = null;
        description = mdDoc ''
          List of plugins to use, represented as a list of packages.
        '';
      };

      plugin_config = mkOption {
        type = with types; attrs;
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
        if cfg.config.plugins == [ ]
        then [ ]
        else
          builtins.map
            (entry:
              if lib.types.package.check entry
              then "lib${lib.replaceStrings ["-"] ["_"] entry.pname}.so"
              else "")
            cfg.config.plugins;
    in
    lib.mkIf
      cfg.enable
      {
        home.packages = lib.optional (cfg.package != null) cfg.package ++ cfg.config.plugins;
        home.file = builtins.listToAttrs (builtins.map
          (pkg: {
            name = ".config/reset/plugins/lib${lib.replaceStrings ["-"] ["_"] pkg.pname}.so";
            value = {
              source = "${pkg}/lib/lib${lib.replaceStrings ["-"] ["_"] pkg.pname}.so";
            };
          })
          cfg.config.plugins);

        xdg.configFile."reset/ReSet.toml".source = (pkgs.formats.toml cfg.config.plugin_config).generate "reset"
          {
            plugins = fetchedPlugins;
          };
      };
}
