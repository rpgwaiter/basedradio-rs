{pkgs, ...}: let
  manifest = (pkgs.lib.importTOML ./Cargo.toml).package;
in
  pkgs.rustPlatform.buildRustPackage rec {
    pname = manifest.name;
    version = manifest.version;
    cargoLock.lockFile = ./Cargo.lock;

    # nativeBuildInputs = [
    #   pkgs.pkg-config
    #   pkgs.glib # for glib-compile-resources
    #   # wrapGAppsHook3
    # ];

    # buildInputs = [pkgs.dbus pkgs.glib.dev];

    # cargohook = pkgs.rustPlatform.cargoSetupHook;
    src = ./.;
    # cargoBuildFlags = ["--package" "api"];
  }
