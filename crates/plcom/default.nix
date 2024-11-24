{
  libiconv,
  lib,
  pkg-config,
  stdenv,
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

    buildInputs =
      [
        # Add additional build inputs here
      ]
      ++ lib.optionals stdenv.isDarwin [
        libiconv
      ];

    nativeBuildInputs = [
      # Add extra native build inputs here, etc.
      pkg-config
    ];
  };

  # Build *just* the cargo dependencies
  cargoArtifacts = craneLib.buildDepsOnly commonArgs;

  # Clippy
  clippyArtifacts = craneLib.cargoClippy (
    commonArgs
    // {
      inherit cargoArtifacts;
      # Again we apply some extra arguments only to this derivation
      # and not every where else. In this case we add some clippy flags
      # cargoClippyExtraArgs = "--all-targets -- --deny warnings";
    }
  );

in
craneLib.buildPackage (
  commonArgs
  // {
    cargoExtraArgs = "-p plcom";
    cargoArtifacts = clippyArtifacts;
  }
)
