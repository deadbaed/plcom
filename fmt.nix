{
  sources ? import ./npins,
  pkgs ? import sources.nixpkgs { },
  treefmt-nix ? import sources.treefmt-nix,
}:

treefmt-nix.mkWrapper pkgs {
  projectRootFile = ".git/config";

  programs.nixfmt.enable = true;
  programs.leptosfmt.enable = true;
  programs.yamlfmt.enable = true;
}
