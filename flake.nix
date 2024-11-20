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
      localSystem:
      let
        crossSystem = "aarch64-linux";
        pkgs = import nixpkgs {
          inherit crossSystem localSystem;
          overlays = [ (import rust-overlay) ];
        };

        inherit (pkgs) lib;

        tailwindStylesheet = import ./crates/plcom/tailwind.nix {
          stdenvNoCC = pkgs.stdenvNoCC;
          tailwindcss = pkgs.tailwindcss;
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

        craneLib = (crane.mkLib pkgs).overrideToolchain (p: p.rust-bin.stable.latest.default);

        # Get metadata from Cargo.toml
        metadata = craneLib.crateNameFromCargoToml { cargoToml = ./crates/plcom/Cargo.toml; };

        # TODO: move to its own module
        crateExpression =
          {
            openssl,
            libiconv,
            lib,
            pkg-config,
            qemu,
            stdenv,
          }:
          craneLib.buildPackage (
            metadata
            // {
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
              cargoExtraArgs = "-p plcom";
              strictDeps = true;

              # Build-time tools which are target agnostic. build = host = target = your-machine.
              # Emulators should essentially also go `nativeBuildInputs`. But with some packaging issue,
              # currently it would cause some rebuild.
              # We put them here just for a workaround.
              # See: https://github.com/NixOS/nixpkgs/pull/146583
              depsBuildBuild = [
                qemu
              ];

              # Dependencies which need to be build for the current platform
              # on which we are doing the cross compilation. In this case,
              # pkg-config needs to run on the build platform so that the build
              # script can find the location of openssl. Note that we don't
              # need to specify the rustToolchain here since it was already
              # overridden above.
              nativeBuildInputs =
                [
                  pkg-config
                  stdenv.cc
                ]
                ++ lib.optionals stdenv.buildPlatform.isDarwin [
                  libiconv
                ];

              # Dependencies which need to be built for the platform on which
              # the binary will run. In this case, we need to compile openssl
              # so that it can be linked with our executable.
              buildInputs = [
                # Add additional build inputs here
                openssl
              ];

              # Tell cargo about the linker and an optional emulater. So they can be used in `cargo build`
              # and `cargo run`.
              # Environment variables are in format `CARGO_TARGET_<UPPERCASE_UNDERSCORE_RUST_TRIPLE>_LINKER`.
              # They are also be set in `.cargo/config.toml` instead.
              # See: https://doc.rust-lang.org/cargo/reference/config.html#target
              CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_LINKER = "${stdenv.cc.targetPrefix}cc";
              CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_RUNNER = "qemu-aarch64";

              # Tell cargo which target we want to build (so it doesn't default to the build system).
              # We can either set a cargo flag explicitly with a flag or with an environment variable.
              # cargoExtraArgs = "--target aarch64-unknown-linux-gnu";
              CARGO_BUILD_TARGET = "aarch64-unknown-linux-gnu";

              # These environment variables may be necessary if any of your dependencies use a
              # build-script which invokes the `cc` crate to build some other code. The `cc` crate
              # should automatically pick up on our target-specific linker above, but this may be
              # necessary if the build script needs to compile and run some extra code on the build
              # system.
              HOST_CC = "${stdenv.cc.nativePrefix}cc";
              TARGET_CC = "${stdenv.cc.targetPrefix}cc";
            }
          );

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
        # Assuming the above expression was in a file called myCrate.nix
        # this would be defined as:
        # my-crate = pkgs.callPackage ./myCrate.nix { };
        plcomBinary = pkgs.callPackage crateExpression { };

        # How to launch binary
        plcom = pkgs.writeShellScriptBin "plcom" ''
          PLCOM_ASSETS_PATH=${plcomAssets} ${plcomBinary}/bin/plcom
        '';

        # Docker image
        dockerImage = pkgs.dockerTools.buildLayeredImage {
          # Meta
          name = metadata.pname;
          tag = metadata.version;
          architecture = "arm64"; # TODO: dynamic thing
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
            #plcomClippy
            ;
        };

      }
    );
}
