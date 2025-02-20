{
  description = "Development shell for quickview";
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixpkgs-unstable";
    flake-parts = {
      url = "github:hercules-ci/flake-parts";
      inputs.nixpkgs-lib.follows = "nixpkgs";
    };
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    devenv = {
      url = "github:cachix/devenv";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };
  outputs =
    inputs@{ flake-parts, nixpkgs, ... }:
    flake-parts.lib.mkFlake { inherit inputs; } {
      imports = [
        inputs.devenv.flakeModule
      ];
      systems = nixpkgs.lib.systems.flakeExposed;
      perSystem =
        { pkgs, ... }:
        {
          devenv.shells.default = {
            packages = with pkgs; [
              just
              libxkbcommon
              wayland
            ];
            env.LD_LIBRARY_PATH = "${pkgs.libxkbcommon}/lib:${pkgs.wayland}/lib";
            languages.rust = {
              enable = true;
              channel = "nightly";
              components = [
                "rustc"
                "cargo"
                "clippy"
                "rustfmt"
                "rust-src"
                "rust-analyzer"
              ];
              mold.enable = true;
            };
            git-hooks.hooks = {
              rustfmt.enable = true;
              clippy.enable = true;
            };
          };
        };
    };
}
