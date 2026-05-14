{
  sources ? import ./npins,
  pkgs ? import sources.nixpkgs { },
  tailwindProjectRoot,
  src,
}:

let
  tailwindStylesheet = import ./tailwind.nix {
    inherit sources pkgs;
    src = tailwindProjectRoot;
    inputFile = "css/main.css";
  };

in
pkgs.stdenvNoCC.mkDerivation {
  name = "plcom-assets";
  src = src;
  buildInputs = [ tailwindStylesheet ];
  installPhase = ''
    mkdir -p $out
    cp -r $src/* $out/
    cp ${tailwindStylesheet}/output.css $out/style.css
  '';
}
