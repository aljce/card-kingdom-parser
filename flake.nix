{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-23.11";
    nixpkgs-mozilla.url = "github:mozilla/nixpkgs-mozilla";
  };
  outputs = { self, nixpkgs, nixpkgs-mozilla }: {
    devShell."x86_64-linux" = let
      pkgs = import nixpkgs { system = "x86_64-linux"; overlays = [ nixpkgs-mozilla.overlay ]; };
    in pkgs.mkShell {
      buildInputs = [ pkgs.latest.rustChannels.nightly.rust ];
    };
  };
}