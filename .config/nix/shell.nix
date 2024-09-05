{ self', pkgs, rustToolchain, ... }:

let
  devToolchain = rustToolchain.override { extensions = [ "rust-analyzer" "rust-src" ]; };
  nodejs = pkgs.nodejs_latest;
  pnpm = pkgs.pnpm;
  prettier = pkgs.prettier;
  typeos = pkgs.typos;
  alejandra = pkgs.alejandra;
in
{
    checks = {
        pre-commit-check = pre-commit-hooks.lib.${system}.run {
            src = ../.;
            hooks = {
                typos.enable = true; # Source code spell checker
                alejandra.enable = true; # Nix linter
                prettier.enable = true; # Markdown & TS formatter
            };
            settings = {
                typos = {
                    write = true; # Automatically fix typos
                    ignored-words = [];
                };
                prettier = {
                    write = true; # Automatically format files
                    configPath = "@storm-software/prettier/config.json";
                };
            };
        };
    };

    devShells.default = pkgs.mkShell {
        packages = with pkgs; [
            # TypeScript
            node2nix
            nodejs
            nodejs_20
            nodejs_20.pkgs.typescript-language-server
            nodejs_20.pkgs.pnpm

            pnpm

            # Rust
            rustup
            llvmPackages_latest.bintools
            binaryen

            # Native SSL
            openssl
            pkg-config
            cmake

            # SQLx macros
            libiconv
        ];

        inputsFrom = [ self'.packages.reaper-eth-engine ];
        shellHook = ''
            echo "node `${pkgs.nodejs}/bin/node --version`"
            ${checks.${system}.pre-commit-check.shellHook}
            '';
    };
}
