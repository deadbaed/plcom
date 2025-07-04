{
  stdenvNoCC,
  tailwindcss,
  src,
  inputFile,
}:

stdenvNoCC.mkDerivation {
  name = "plcom-css-tailwind";
  inherit src;
  nativeBuildInputs = [ tailwindcss ];
  dontUnpack = true;
  buildPhase = "${tailwindcss}/bin/tailwindcss --input ${src}/${inputFile} --output $out/output.css --cwd ${src} --minify";
}
