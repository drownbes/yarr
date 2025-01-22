{
  inputs = {
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    flake-utils.url = "github:numtide/flake-utils";
    nixpkgs.url = "nixpkgs/nixos-unstable";
    process-compose-flake.url = "github:Platonic-Systems/process-compose-flake";
    services-flake.url = "github:juspay/services-flake";
  };

  outputs = inputs@{ self, fenix, flake-utils, nixpkgs,
    process-compose-flake,
    services-flake
    }: 
    flake-utils.lib.eachDefaultSystem (system: let
      overlay = final: prev: {
          beef_market = self.packages.${system}.beef_market;
      };
      pkgs = nixpkgs.legacyPackages.${system}.extend overlay;
      toolchain = with fenix.packages.${system}; combine [
        stable.completeToolchain
        targets.wasm32-unknown-unknown.stable.rust-std
      ];

      pcs = import process-compose-flake.lib {inherit pkgs;};
      services = pcs.evalModules {
        modules = [
          services-flake.processComposeModules.default
          (inputs.services-flake.lib.multiService ./prowlarr.nix)
          (inputs.services-flake.lib.multiService ./transmission.nix)
          {
            cli.options.port = 8084;
            services.prowlarr."prowlarr".enable = true;
            services.transmission."transmission".enable = true;
          }
        ];
      };

      rustPlatform = (pkgs.makeRustPlatform {
        cargo = toolchain;
        rustc = toolchain;
      });

      prowarrApiSrc = pkgs.stdenv.mkDerivation {
        name = "prowarr-rust-bindings";
        src = pkgs.fetchurl {
          url = "https://raw.githubusercontent.com/Prowlarr/Prowlarr/develop/src/Prowlarr.Api.V1/openapi.json";
          sha256 = "sha256-NwSrozhjreNsQTxQvFwZVX/FxhGz85XTSUgTBwFmwEc=";
        };
        
        unpackPhase = ''
          cp "$src" $(stripHash "$src")
        '';

        buildInputs = [
          pkgs.openapi-generator-cli
        ];

        buildPhase = ''
          ls -la
          openapi-generator-cli generate --package-name prowarr-api -i openapi.json -g rust -o $out
        '';
      };
    in {

      packages = {
        inherit prowarrApiSrc services;
      };
      

      devShells.default = pkgs.mkShell {
        shellHook = ''
          export SHELL="${pkgs.bashInteractive}/bin/bash"
          source "${toolchain}/etc/bash_completion.d/cargo"
        '';
        nativeBuildInputs = [
            pkgs.pkg-config
        ];
        inputsFrom = [
          services.config.services.outputs.devShell
        ];
        buildInputs = with pkgs;[
          services.config.outputs.package
          prowarrApiSrc
          toolchain
          just
          openssl
          openssl.dev
          sqlx-cli
          sqlite
          sqlite-vec
          litecli
          openapi-generator-cli
          nodejs
          nodePackages.sass
          transmission_4
        ];
      };
    } // {inherit pcs;}); 
}
