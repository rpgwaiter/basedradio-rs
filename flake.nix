{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
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
        packages.basedradio-app = pkgs.callPackage ./app {};
      };
    }
    // {
      nixosModules.radio-api = import ./api/module.nix;
      overlays.default = final: prev: {
        inherit
          (self.outputs.packages.x86_64-linux)
          basedradio-app
          basedradio-api
          ;
      };
      # nixosModules.basedradio-app = import ./app/module.nix;
    };
}
