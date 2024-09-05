{
    pkgs,
    flakeInputs,
    lib,
    self',
    rustToolchain,
    ...
}: let
  stdenv = pkgs.clangStdenv;
  craneLib = (flakeInputs.crane.mkLib pkgs).overrideToolchain rustToolchain;
  libSuffix = stdenv.hostPlatform.extensions.sharedLibrary;

  workspaceRoot = builtins.path {
    name = "reaper-root";
    path = "../../../.";
  };

  srcFilter = flakeInputs.gitignore.lib.gitignoreFilterWith {
    basePath = "../.";
    extraRules = ''
    /nix
    /flake.*
    *.nix
    *.md
    *.sh

    assets.tar.gz
    /packages

    /flake.lock
    /renovate.json
    /scripts
    /tools
    /assets
    /deployment
    /.github
    /.nx
    /.vscode
    /packages/**/src
    node_modules/
    yarn.lock
  ''
  };

  src = pkgs.nix-gitignore.gitignoreSource [extraIgnores] (lib.cleanSourceWith {
    name = "reaper-eth-engine-src";
    filter = lib.cleanSourceFilter;
    src = workspaceRoot;
  });
  deps = craneLib.vendorCargoDeps { inherit src; };

  version = pkgs.runCommand "getVersion" {} ''
    ${pkgs.dasel}/bin/dasel \
      --file ${../Cargo.toml} \
      --selector package.version\
      --write - | tr -d "\n" > $out
  '';
in {
  packages.reaper-eth-engine = craneLib.buildPackage {
    inherit src;
    pname  = "reaper-eth-engine";
    version = builtins.readFile version;
    stdenv = pkgs.clangStdenv;

    cargoBuildFlags = "-p reaper-eth-engine";
    cargoExtraArgs = "-p reaper-eth-engine";

    RUSTFLAGS = builtins.concatStringsSep " " [
      "-Arust-2018-idioms -Aunused-crate-dependencies"
      "-C linker=clang -C link-arg=-fuse-ld=lld"
    ];

    doCheck = false;
    nativeBuildInputs = [
      pkgs.pkg-config
      pkgs.openssl.dev
      pkgs.llvmPackages.bintools
    ];
  };

    # configurePhase = ''
    #   mkdir .config/cargo
    #   ln -s ${deps}/config.toml .config/cargo/config.toml
    # '';

    # buildPhase = ''
    #   cargo build --package=reaper-eth-engine --profile=prod
    # '';

    # installPhase = ''
    #   mkdir -p $out/bin $out/lib
    #   cp target/prod/reaper-eth-engine $out/bin/
    # '';

    # dontStrip = true;
};


