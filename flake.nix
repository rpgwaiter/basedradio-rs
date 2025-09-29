# So this doesnt work as dx add some extra deps not included in cargo.lock
# I don't want all that extra stuff in there, so this package is on hold for now
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
        packages.basedradio-app = pkgs.callPackage ./app {};
        packages.basedradio-api = pkgs.callPackage ./api {};
      };
    } // { nixosModules.radio-api = import ./api/module.nix; };
}
