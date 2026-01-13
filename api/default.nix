{
  pkgs,
  lib,
  ...
}: let
  manifest = (pkgs.lib.importTOML ./Cargo.toml).package;
in
  pkgs.rustPlatform.buildRustPackage rec {
    pname = manifest.name;
    version = manifest.version;
    cargoLock.lockFile = ./Cargo.lock;
    src = lib.cleanSource ./.;
    nativeBuildInputs = [pkgs.pkg-config];
    buildInputs = [pkgs.openssl];
  }
