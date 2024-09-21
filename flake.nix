{
  inputs = {
    nixpkgs.url      = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url  = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
        rustpkg = pkgs.rust-bin.selectLatestNightlyWith (toolchain: toolchain.default.override {
          extensions = [ "rust-src" "rust-analyzer" "rustfmt" "llvm-tools-preview" ];
          targets = [ "x86_64-unknown-none" ];
        });
      in {
        RUST_BACKTRACE = 1;
        devShells.default = with pkgs; mkShell {
          buildInputs = [
            cargo-bootimage
            fish
            pkg-config
            rustpkg
            qemu
          ];
        };
      }
    );
}
