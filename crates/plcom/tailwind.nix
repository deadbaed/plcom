{
  pkgs,
  src,
  inputFile,
}:

pkgs.stdenvNoCC.mkDerivation {
  name = "plcom-css-tailwind";
  inherit src;
  buildInputs = with pkgs; [ tailwindcss ];
  dontUnpack = true;
  buildPhase = "tailwindcss --config ${src}/tailwind.config.js --input ${src}/${inputFile} --output $out/output.css --minify";
}
