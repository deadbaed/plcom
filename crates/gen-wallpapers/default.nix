{
  sources ? import ../../npins,
  pkgs ? import sources.nixpkgs {
    overlays = [
      (import sources.rust-overlay)
    ];
  },
  crane ? import ./crane.nix { inherit sources pkgs; },
}:

{
  inherit (crane) package;
}
