{
  sources ? import ./npins,
  pkgs ? import sources.nixpkgs {
    overlays = [
      (import sources.rust-overlay)
    ];
  },
  common ? import ./. { inherit sources pkgs; },
  fmt ? import ./fmt.nix { inherit sources pkgs; },
  gen-wallpapers ? import ./crates/gen-wallpapers { inherit sources pkgs; },
  plcom ? import ./package.nix { inherit sources pkgs common; },
  supervisord ? import sources.nix-supervisord { inherit pkgs; },
}:

let
  paths = supervisord.mkPaths { };
  programs = [
    {
      name = "tailwind";
      command = "${pkgs.lib.getExe pkgs.tailwindcss_4} --input ./crates/plcom/css/main.css > ./public/style.css";
    }
    {
      name = "website";
      command = "  cargo run --manifest-path ./crates/plcom/Cargo.toml";
      environment = {
        PLCOM_ASSETS = "./public";
        PLCOM_BLOG_FEED = "/dev/null";
      };
    }
  ];
  supervisordProject = supervisord.mkSupervisor {
    inherit paths programs;
    project_name = "plcom";
  };
in
pkgs.mkShellNoCC {
  packages = with pkgs; [
    # formatter
    fmt

    # nix
    npins
    nixd
    nixfmt
    nix-output-monitor

    # rust
    common.rust
    rust-analyzer
    taplo

    # workspace
    gen-wallpapers.package

    # supervisord for local development
    supervisordProject.supervisord-wrapper
    supervisordProject.supervisorctl-wrapper
    supervisordProject.supervisord-kill
  ];
  shellHook = supervisordProject.shellHook;
}
