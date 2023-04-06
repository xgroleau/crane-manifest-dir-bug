{
  description =
    "Example that failes to build using CARGO_MANIFEST_DIR variable";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    crane = {
      url = "github:ipetkov/crane";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs = {
        nixpkgs.follows = "nixpkgs";
        flake-utils.follows = "flake-utils";
      };
    };
  };

  outputs = { self, nixpkgs, flake-utils, crane, rust-overlay, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs { inherit system overlays; };

        rustpkg = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
        craneLib = (crane.mkLib pkgs).overrideToolchain rustpkg;

        commonArgs = {
          nativeBuildInputs = with pkgs; [ pkg-config protobuf ];
        };

        # Package
        example = craneLib.buildPackage (commonArgs // {
          src = craneLib.cleanCargoSource (craneLib.path ./.);
        });

      in with pkgs; {
        checks = { inherit example; };

        devShells.default = mkShell {
          inputsFrom = builtins.attrValues self.checks.${system};
          LD_LIBRARY_PATH = lib.makeLibraryPath ([ systemd ]);
        };

        packages = {
          default = example;
          inherit example;
        };
      });
}
