{pkgs, ...}: let
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
in
  pkgs.stdenv.mkDerivation rec {
    pname = "basedradio-rs";
    version = "0.0.1";

    src = ./.;

    cargoDeps = pkgs.rustPlatform.importCargoLock {
      lockFile = ./Cargo.lock;
    };

    cargohook = pkgs.rustPlatform.cargoSetupHook;

    buildInputs = with pkgs; [
      dioxus-cli
      cargo
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
      # cargohook
    ];

    nativeBuildInputs = [
      pkgs.dioxus-cli
      cargohook
    ];

    buildPhase = ''
      ls -Alh .
      dx build --platform=web --features=web --release
    '';

    installPhase = ''
      mkdir $out/share
      mv ./target/dx/basedradio-rs/release/web/public $out/share/
    '';
  }
