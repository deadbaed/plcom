{
  sources ? import ../../npins,
  pkgs ? import sources.nixpkgs {
    overlays = [
      (import sources.rust-overlay)
    ];
  },
  lib ? pkgs.lib,
  crane ? import sources.crane { inherit pkgs; },
  common ? import ../.. { inherit sources pkgs; },
}:

let
  craneLib = crane.overrideToolchain common.rust;

  src = craneLib.cleanCargoSource ./.;

  commonArgs = {
    inherit src;
    strictDeps = true;

    buildInputs = [
      # Add additional build inputs here
    ]
    ++ lib.optionals pkgs.stdenv.isDarwin [
      # Additional darwin specific inputs can be set here
      pkgs.libiconv
    ];
  };
  # Build *just* the cargo dependencies, so we can reuse
  # all of that work (e.g. via cachix) when running in CI
  cargoArtifacts = craneLib.buildDepsOnly commonArgs;

  # Build the actual crate itself, reusing the dependency
  # artifacts from above.
  gen-wallpapers = craneLib.buildPackage (
    commonArgs
    // {
      inherit cargoArtifacts;
    }
  );
in
{
  inherit cargoArtifacts;
  package = gen-wallpapers;
}
