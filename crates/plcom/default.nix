{
  sources ? import ./npins,
  pkgs ? import sources.nixpkgs { },
  lib ? pkgs.lib,
  craneLib,
}:

let
  commonArgs = {
    src = lib.cleanSourceWith {
      src = craneLib.path ./.; # The original, unfiltered source
      filter =
        path: type:
        # Assets for codegen
        (lib.hasSuffix ".json" path)
        ||
          # Default filter from crane (allow .rs files)
          (craneLib.filterCargoSources path type);
    };

    strictDeps = true;

    buildInputs = [
      # Add additional build inputs here
    ]
    ++ lib.optionals pkgs.stdenv.isDarwin [
      pkgs.libiconv
    ];

    nativeBuildInputs = [
      # Add extra native build inputs here, etc.
      pkgs.pkg-config
    ];
  };

  # Build *just* the cargo dependencies
  cargoArtifacts = craneLib.buildDepsOnly commonArgs;

  # Build the actual crate itself, reusing the dependency
  # artifacts from above.
  plcom = craneLib.buildPackage (
    commonArgs
    // {
      cargoExtraArgs = "-p plcom";
      cargoArtifacts = cargoArtifacts;
    }
  );
in
{
  inherit cargoArtifacts;
  package = plcom;

}
