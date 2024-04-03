with import <nixpkgs> {};
  pkgs.mkShell {
    buildInputs = [
      pkgs.cargo
      pkgs.rustc
      pkgs.rustfmt
      pkgs.clippy
      pkgs.openssl
      pkgs.pkg-config
    ];
    RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
    PKG_CONFIG_PATH = "${pkgs.openssl.dev.outPath}/lib/pkgconfig:" + "$PKG_CONFIG_PATH";
  }
