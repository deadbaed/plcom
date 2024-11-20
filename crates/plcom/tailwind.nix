{
  stdenvNoCC,
  tailwindcss,
  src,
  inputFile,
}:

stdenvNoCC.mkDerivation {
  name = "plcom-css-tailwind";
  inherit src;
  buildInputs = [ tailwindcss ];
  dontUnpack = true;
  buildPhase = "${tailwindcss}/bin/tailwindcss --config ${src}/tailwind.config.js --input ${src}/${inputFile} --output $out/output.css --minify";
}
