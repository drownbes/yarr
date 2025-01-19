{ config, lib, pkgs, name, ... }:
{
  options = with lib; {
    package = lib.mkPackageOption pkgs "transmission_4" { };
  };
  config = {
      outputs.settings = {
        processes.${name} = {
          command = ''
            ${config.package}/bin/transmission-daemon -f --config-dir ${config.dataDir} -w ${config.dataDir}/download
          '';
        };
      };
  };
}
