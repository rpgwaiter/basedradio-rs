{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-25.05";
    flake-parts.url = "github:hercules-ci/flake-parts";
    systems.url = "github:nix-systems/default";
  };

  outputs = inputs:
    inputs.flake-parts.lib.mkFlake {inherit inputs;} {
      systems = import inputs.systems;
      perSystem = {
        pkgs,
        lib,
        ...
      }: {
        packages.basedradio-api = pkgs.callPackage ./api {};
      };
    } // { 
      nixosModules.radio-api = import ./api/module.nix;
      # nixosModules.basedradio-app = import ./app/module.nix;
    };
}
