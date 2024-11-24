{
  stdenvNoCC,
  tailwindcss,
  tailwindProjectRoot,
  src,
}:

let
  tailwindStylesheet = import ./tailwind.nix {
    stdenvNoCC = stdenvNoCC;
    tailwindcss = tailwindcss;
    src = tailwindProjectRoot;
    inputFile = "css/main.css";
  };

in
stdenvNoCC.mkDerivation {
  name = "plcom-assets";
  src = src;
  buildInputs = [ tailwindStylesheet ];
  installPhase = ''
    mkdir -p $out
    cp -r $src/* $out/
    cp ${tailwindStylesheet}/output.css $out/style.css
  '';
}
