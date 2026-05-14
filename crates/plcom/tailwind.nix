{
  sources ? import ./npins,
  pkgs ? import sources.nixpkgs { },
  lib ? pkgs.lib,
  src,
  inputFile,
}:

let
  package = pkgs.tailwindcss_4;
in
pkgs.stdenvNoCC.mkDerivation {
  name = "plcom-css-tailwind";
  inherit src;
  nativeBuildInputs = [ package ];
  dontUnpack = true;
  buildPhase = "${lib.getExe package} --input ${src}/${inputFile} --output $out/output.css --cwd ${src} --minify";
}
