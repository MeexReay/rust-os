{
  description = "Rust OS flake";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url  = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
        requires = with pkgs; [
          nasm
          grub2
          gnumake
        ];
      in
      {
        devShells.default = with pkgs; mkShell {
          buildInputs = requires ++ [
            (rust-bin.nightly.latest.default.override {
              extensions = [ "rust-src" ];
              targets = [ "x86_64-unknown-none" ];
            })
          ];
        };
      }
    );
}
