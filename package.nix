{
  system ? builtins.currentSystem,
  sources ? import ./npins,
  pkgs ? import sources.nixpkgs {
    inherit system;
    overlays = [
      (import sources.rust-overlay)
    ];
  },
  crane ? import sources.crane { inherit pkgs; },
  common ? import ./. { inherit sources pkgs; },
}:

let
  # Gather assets
  plcomAssets = import ./crates/plcom/assets.nix {
    inherit pkgs;
    tailwindProjectRoot = ./crates/plcom;
    src = ./public;
  };

  craneLib = crane.overrideToolchain common.rust;

  # Build binary
  plcomCrane = pkgs.callPackage ./crates/plcom/default.nix {
    inherit pkgs craneLib;
  };

  # How to launch binary
  wrappedPackage = pkgs.writeShellScriptBin "plcom" ''
    PLCOM_ASSETS=${plcomAssets} PLCOM_BLOG_FEED=/var/www/blog/current/www/atom.xml ${plcomCrane.package}/bin/plcom
  '';
in

{
  inherit (plcomCrane) cargoArtifacts package;
  inherit wrappedPackage;
}
