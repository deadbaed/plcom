{
  description = "philippeloctaux dot com";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    crane.url = "github:ipetkov/crane";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    {
      nixpkgs,
      crane,
      flake-utils,
      rust-overlay,
      ...
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ (import rust-overlay) ];
        };

        # Gather assets
        plcomAssets = import ./crates/plcom/assets.nix {
          stdenvNoCC = pkgs.stdenvNoCC;
          tailwindcss = pkgs.tailwindcss_4;
          tailwindProjectRoot = ./crates/plcom;
          src = ./public;
        };

        # Which rust toolchain to use
        craneLib = (crane.mkLib pkgs).overrideToolchain (p: p.rust-bin.stable.latest.default);

        # Build binary
        plcomBinary = pkgs.callPackage ./crates/plcom/default.nix {
          libiconv = pkgs.libiconv;
          lib = pkgs.lib;
          pkg-config = pkgs.pkg-config;
          stdenv = pkgs.stdenv;
          craneLib = craneLib;
        };

        # How to launch binary
        plcom = pkgs.writeShellScriptBin "plcom" ''
          PLCOM_ASSETS_PATH=${plcomAssets} ${plcomBinary}/bin/plcom
        '';

      in
      {
        packages = {
          inherit plcom;
          default = plcom;
        };

        checks = {
          # Build the crate as part of `nix flake check` for convenience
          inherit plcomBinary;
        };

      }
    );
}
