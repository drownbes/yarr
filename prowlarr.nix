{ config, lib, pkgs, name, ... }:
{
  options = {
    package = lib.mkPackageOption pkgs "prowlarr" { };
  };
  config = {
      outputs.settings = {
        processes.${name} = {
          command = ''
            ${lib.getExe config.package} -nobrowser -data=$(realpath ${config.dataDir})
          '';
        };
      };
  };
}
