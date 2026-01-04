{
  description = "zotero-mcp: MCP server for Zotero integration with AI assistants";

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
      overlays.default = final: prev: {
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
              extensions = [
                "rust-src"
                "rust-analyzer"
              ];
            };
      };

      devShells = forEachSupportedSystem (
        { pkgs }:
        {
          default = pkgs.mkShell {
            packages = with pkgs; [
              # Rust toolchain
              rustToolchain
              cargo-edit
              cargo-workspaces

              # Build dependencies
              pkg-config

              # Node.js for Zotero plugin
              nodejs
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

            env = {
              # Required by rust-analyzer
              RUST_SRC_PATH = "${pkgs.rustToolchain}/lib/rustlib/src/rust/library";
              # Required for mupdf-sys to find the libraries
              MUPDF_LIB_DIR = "${pkgs.mupdf}/lib";
              MUPDF_INCLUDE_DIR = "${pkgs.mupdf}/include";
              LIBCLANG_PATH = "${pkgs.llvmPackages.libclang.lib}/lib";
            };

            shellHook = ''
              echo "zotero-mcp development environment"
              echo "Rust: $(rustc --version)"
              echo ""
              echo "Commands:"
              echo "  cargo build          - Build the project"
              echo "  cargo test           - Run unit tests"
              echo "  cargo run -p zotero-mcp-server - Run the MCP server"
              echo "  nix build            - Build release package"
              echo ""
            '';
          };
        }
      );

      packages = forEachSupportedSystem (
        { pkgs }:
        {
          default = pkgs.rustPlatform.buildRustPackage {
            pname = "zotero-mcp-server";
            version = "0.1.0";
            src = ./.;
            cargoLock.lockFile = ./Cargo.lock;

            nativeBuildInputs = with pkgs; [
              pkg-config
              clang
              llvmPackages.libclang
              gperf
              gnumake
            ];

            buildInputs =
              with pkgs;
              [
                # MuPDF and its dependencies
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
              ]
              ++ pkgs.lib.optionals pkgs.stdenv.isDarwin [
                darwin.apple_sdk.frameworks.Security
                darwin.apple_sdk.frameworks.SystemConfiguration
              ];

            env = {
              # Required for mupdf-sys to find the libraries
              MUPDF_LIB_DIR = "${pkgs.mupdf}/lib";
              MUPDF_INCLUDE_DIR = "${pkgs.mupdf}/include";
              LIBCLANG_PATH = "${pkgs.llvmPackages.libclang.lib}/lib";
            };

            # Only build the MCP server binary
            cargoBuildFlags = [
              "-p"
              "zotero-mcp-server"
            ];

            # Skip integration tests during nix build (they require Zotero running)
            checkFlags = [
              "--skip"
              "integration"
            ];

            meta = with pkgs.lib; {
              description = "MCP server for Zotero integration - enables AI assistants to read PDFs and create annotations";
              homepage = "https://github.com/stephenstubbs/zotero-mcp";
              license = licenses.mit;
              maintainers = [ ];
              mainProgram = "zotero-mcp-server";
            };
          };
        }
      );
    };
}
