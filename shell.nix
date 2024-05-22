{ pkgs ? import <nixpkgs> {} }:
pkgs.mkShell {
  nativeBuildInputs = with pkgs; [ rustc cargo gcc rustfmt clippy pkg-config openssl ];

  RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";

  PKG_CONFIG_PATH = "${pkgs.openssl.dev}/lib/pkgconfig";
}
