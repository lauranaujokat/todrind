let
  unstable = import (fetchTarball "https://nixos.org/channels/nixos-23.11/nixexprs.tar.xz") { };
in
with import <nixpkgs> {};
  pkgs.mkShell {
    buildInputs = [
      unstable.cargo
      unstable.rustc
      unstable.rustfmt
      unstable.clippy
      pkgs.openssl
      pkgs.pkg-config
    ];
    RUST_SRC_PATH = "${unstable.rust.packages.stable.rustPlatform.rustLibSrc}";
    PKG_CONFIG_PATH = "${pkgs.openssl.dev.outPath}/lib/pkgconfig:" + "$PKG_CONFIG_PATH";
  }
