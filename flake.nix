{
  # Nix: https://nixos.org/download.html
  # How to activate flakes: https://nixos.wiki/wiki/Flakes
  # For seamless integration, consider using:
  # - direnv: https://github.com/direnv/direnv
  # - nix-direnv: https://github.com/nix-community/nix-direnv
  #
  # # .envrc
  # use flake
  # dotenv .env
  #
  description = "Storm Software's Reaper development environment";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    crane = {
      url = "github:ipetkov/crane";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    flake-utils = {
      url = "github:numtide/flake-utils";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    flake-parts = {
      url = "github:hercules-ci/flake-parts";
      inputs.nixpkgs-lib.follows = "nixpkgs";
    };
    gitignore = {
      url = "github:hercules-ci/gitignore.nix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    pre-commit-hooks.url = "github:cachix/pre-commit-hooks.nix";
  };

  outputs = inputs@{
    self,
    nixpkgs,
    rust-overlay,
    flake-parts,
    crane,
    systems,
    flake-utils,
    pre-commit-hooks,
    ...
  }: let
    inherit (nixpkgs.lib) optional concatStringsSep;
    systems = flake-utils.lib.system;
    flake = flake-utils.lib.eachDefaultSystem (system: let
      overlays = [
        (self: super: rec {
          nodejs = super.nodejs_20;
          pnpm = super.nodePackages.pnpm;
          prettier = super.nodePackages.prettier;
        })
      ];
      pkgs = import nixpkgs {inherit overlays system;};
      pkgs_chromium = import nixpkgs {inherit system;};
      packages = with pkgs; [
        # TypeScript
        node2nix
        nodejs
        pnpm

        # Rust
        rustup

        # Development tools
        prettier
        git
        typos
        alejandra

        # Native SSL
        openssl
        pkg-config
        cmake

        # SQLx macros
        libiconv
      ];
    in {

        flake-parts.lib.mkFlake { inherit inputs; } {
            systems = import systems;
            perSystem = { config, system, pkgs, craneLib, ... }: {
                config._module.args.flakeInputs = inputs;
                imports = [
                    ./nix/args.nix
                    ./nix/shell.nix
                    ./apps/eth-engine/nix/eth-engine.nix
                ];
            }
        }
    })
}
