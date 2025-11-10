{ pkgs, ... }:
pkgs.mkShell {
  packages = with pkgs; [
    cargo
    clippy
    rustc
    rustfmt
    pkg-config
    openssl
    sqlite
  ];

  env.RUST_SRC_PATH = "${pkgs.rustPlatform.rustLibSrc}";
}
