{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    flake-parts.url = "github:hercules-ci/flake-parts";
    systems.url = "github:nix-systems/default";
  };

  outputs = inputs:
    inputs.flake-parts.lib.mkFlake {inherit inputs;} {
      systems = import inputs.systems;
      perSystem = {pkgs, ...}: let
        cargoToml = builtins.fromTOML (builtins.readFile ./Cargo.toml);
        rust-toolchain = pkgs.symlinkJoin {
          name = "rust-toolchain";
          paths = with pkgs; [
            rustc
            cargo
            cargo-watch
            rust-analyzer
            rustPlatform.rustcSrc
            cargo-dist
            cargo-tarpaulin
            cargo-insta
            cargo-machete
            cargo-edit
            wasm-bindgen-cli_0_2_100
          ];
        };

        buildInputs = with pkgs; [
          at-spi2-atk
          atkmm
          cairo
          gdk-pixbuf
          glib
          gtk3
          harfbuzz
          librsvg
          libsoup_3
          pango
          webkitgtk_4_1
          openssl
          wasm-bindgen-cli_0_2_100
          lld_20
          dioxus-cli
        ];
        nativeBuildInputs = with pkgs; [
          pkg-config
          gobject-introspection
          cargo
          cargo-tauri
          wasm-bindgen-cli_0_2_100
          nodejs
        ];
      in {
        # Rust package
        packages.default = pkgs.rustPlatform.buildRustPackage {
          inherit (cargoToml.package) name version;
          src = ./.;
          cargoLock.lockFile = ./Cargo.lock;

          buildPhase = ''
            ${pkgs.dioxus-cli}/bin/dx build --platform=web --features=web --release
          '';

          installPhase = ''
            mkdir $out/share
            mv ./target/dx/basedradio-rs/release/web/public $out/share/
          '';

          RUST_BACKTRACE = "full";

          nativeBuildInputs = nativeBuildInputs;
          buildInputs = buildInputs;
        };

        # Rust dev environment
        devShells.default = pkgs.mkShell {
          RUST_BACKTRACE = "full";
          RUST_SRC_PATH = pkgs.rustPlatform.rustLibSrc;

          packages =
            nativeBuildInputs
            ++ buildInputs
            ++ [
              rust-toolchain
            ]
            ++ (with pkgs; [
              clippy
              dioxus-cli
            ]);
        };
      };
    };
}
