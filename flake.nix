{
  description = "Multi-language development environment (rust, node)";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    {
      self,
      nixpkgs,
      rust-overlay,
    }:
    let
      supportedSystems = [
        "x86_64-linux"
        "aarch64-linux"
        "x86_64-darwin"
        "aarch64-darwin"
      ];
      forEachSupportedSystem =
        f:
        nixpkgs.lib.genAttrs supportedSystems (
          system:
          f {
            pkgs = import nixpkgs {
              inherit system;
              overlays = [
                rust-overlay.overlays.default
                self.overlays.default
              ];
            };
          }
        );
    in
    {
      overlays.default = final: prev: rec {
        rustToolchain =
          let
            rust = prev.rust-bin;
          in
          if builtins.pathExists ./rust-toolchain.toml then
            rust.fromRustupToolchainFile ./rust-toolchain.toml
          else if builtins.pathExists ./rust-toolchain then
            rust.fromRustupToolchainFile ./rust-toolchain
          else
            rust.stable.latest.default.override {
              extensions = [ "rust-src" ];
            };
        nodejs = prev.nodejs;
        yarn = prev.yarn.override {
          inherit nodejs;
        };
      };

      devShells = forEachSupportedSystem (
        { pkgs }:
        {
          default = pkgs.mkShell {
            packages = with pkgs; [
              cargo-edit
              cargo-workspaces
              node2nix
              nodejs
              pkg-config
              rust-analyzer
              rustToolchain
              yarn
              # MuPDF build dependencies (for mupdf-rs crate)
              mupdf
              freetype
              harfbuzz
              libjpeg
              jbig2dec
              openjpeg
              gumbo
              mujs
              zlib
              openssl
              fontconfig
              clang
              llvmPackages.libclang
              gperf
              gnumake
            ];

            # Required for mupdf-sys to find the libraries
            env = {
              MUPDF_LIB_DIR = "${pkgs.mupdf}/lib";
              MUPDF_INCLUDE_DIR = "${pkgs.mupdf}/include";
              LIBCLANG_PATH = "${pkgs.llvmPackages.libclang.lib}/lib";
            };
          };
        }
      );
    };
}
