{
  sources ? import ./npins,
  pkgs ? import sources.nixpkgs {
    overlays = [
      (import sources.rust-overlay)
    ];
  },
}:
{
  rust = pkgs.rust-bin.stable."1.93.1".default.override {
    extensions = [ "rust-src" ];
  };
}
