{
  pkgs,
  lib,
  ...
}:
pkgs.stdenv.mkDerivation rec {
  pname = "basedradio-app";
  version = "0.0.1";
  src = lib.cleanSource ./.;

  cargoDeps = pkgs.rustPlatform.importCargoLock {
    lockFile = ./Cargo.lock;
  };

  cargohook = pkgs.rustPlatform.cargoSetupHook;

  buildInputs = with pkgs; [
    at-spi2-atk
    atkmm
    binaryen
    cairo
    cargo
    dioxus-cli
    gdk-pixbuf
    glib
    gtk3
    harfbuzz
    librsvg
    libsoup_3
    lld_20
    nodejs
    openssl
    pango
    rustc
    wasm-bindgen-cli_0_2_106
    webkitgtk_4_1
  ];

  nativeBuildInputs = [
    pkgs.dioxus-cli
    cargohook
  ];

  buildPhase = ''
    ls -Alh .
    export PATH=${pkgs.wasm-bindgen-cli_0_2_106}/bin:$PATH
    wasm-bindgen --version
    dx --version
    dx --help
    dx doctor
    dx build --platform=web --features=web --release
  '';

  installPhase = ''
    mkdir -p $out/share
    ls -Alh
    mv ./target/dx/basedradio-rs/release/web/public $out/share/
  '';
}
