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
      self,
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

        inherit (pkgs) lib;

        tailwindStylesheet = import ./crates/plcom/tailwind.nix {
          pkgs = import nixpkgs { inherit system; };
          src = ./crates/plcom;
          inputFile = "css/main.css";
        };

        plcomAssets = pkgs.stdenvNoCC.mkDerivation {
          name = "plcom-assets";

          # Local folder as a source
          src = ./public;

          # Build inputs (external derivation dependencies)
          buildInputs = [ tailwindStylesheet ];

          installPhase = ''
            mkdir -p $out
            cp -r $src/* $out/
            cp ${tailwindStylesheet}/output.css $out/style.css
          '';
        };

        # in
        craneLib = crane.mkLib pkgs;

        # Get metadata from Cargo.toml
        metadata = craneLib.crateNameFromCargoToml { cargoToml = ./crates/plcom/Cargo.toml; };

        # Common derivation arguments used for all builds
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
            with pkgs;
            [
              # Add additional build inputs here
              openssl
            ]
            ++ lib.optionals pkgs.stdenv.isDarwin [
              pkgs.libiconv
            ];

          nativeBuildInputs = with pkgs; [
            # Add extra native build inputs here, etc.
            pkg-config
          ];

        } // metadata;

        # Build *just* the cargo dependencies
        cargoArtifacts = craneLib.buildDepsOnly commonArgs;

        # Clippy
        plcomClippy = craneLib.cargoClippy (
          commonArgs
          // {
            inherit cargoArtifacts;
            # Again we apply some extra arguments only to this derivation
            # and not every where else. In this case we add some clippy flags
            # cargoClippyExtraArgs = "--all-targets -- --deny warnings";
          }
        );

        # Build crate
        plcomBinary = craneLib.buildPackage (
          commonArgs
          // {
            cargoExtraArgs = "-p plcom";
            cargoArtifacts = plcomClippy;
          }
        );

        # How to launch binary
        plcom = pkgs.writeShellScriptBin "plcom" ''
          PLCOM_ASSETS_PATH=${plcomAssets} ${plcomBinary}/bin/plcom
        '';

        # Docker image
        dockerImage = pkgs.dockerTools.buildLayeredImage {
          # Meta
          name = metadata.pname;
          tag = metadata.version;
          created = builtins.substring 0 8 self.lastModifiedDate;

          # Content of image
          contents = pkgs.buildEnv {
            name = "image-root";
            paths = [
              plcomBinary
              plcomAssets
            ];

            pathsToLink = [ "/bin" ];
          };

          # Container config
          config = {
            Cmd = [ "${plcomBinary}/bin/plcom" ];
            Env = [
              "PLCOM_ASSETS_PATH=${plcomAssets}"
            ];
          };
        };
      in
      {
        apps.default = {
          type = "app";
          program = "${plcom}/bin/plcom";
        };

        packages = {
          inherit plcom dockerImage;
          default = plcom;
        };

        checks = {
          inherit
            # Build the crate as part of `nix flake check` for convenience
            plcom
            plcomClippy
            ;
        };

      }
    );
}
