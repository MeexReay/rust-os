{
  description = "Rust OS flake";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
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
        rustToolchain = with pkgs;
          rust-bin.selectLatestNightlyWith (toolchain: toolchain.default.override {
            extensions = [ "rust-src" ];
            targets = [ "x86_64-unknown-none" ];
          });
        requires = with pkgs; [
          nasm
          grub2
          gnumake
          rustToolchain
        ];
      in {
        devShells.default = with pkgs; mkShell {
          buildInputs = requires;
        };
      }
    );
}
