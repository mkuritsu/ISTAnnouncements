{
  lib,
  rustPlatform,
  pkg-config,
  openssl,
  sqlite,
}:
rustPlatform.buildRustPackage {
  pname = "istannouncements";
  version = "0.1.0";

  src = lib.fileset.toSource {
    root = ./..;
    fileset = lib.fileset.unions [
      ../Cargo.toml
      ../Cargo.lock
      ../src
      ../web
    ];
  };

  nativeBuildInputs = [
    pkg-config
  ];

  buildInputs = [
    openssl
    sqlite
  ];

  postInstall = ''
    mkdir -p $out/share/web
    cp -r $src/web/* $out/share/web/
  '';

  cargoHash = "sha256-/pYiB5PqQoDdvvEpwurGPkC4nl8tEUdxe8L4NPR2/b0=";
}
