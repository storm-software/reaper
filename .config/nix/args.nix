{ flakeInputs, system, ... }:
{
  config._module.args =
    let
      overlays = [
        flakeInputs.rust-overlay.overlays.default
      ];
    in rec
    {
      pkgs = import flakeInputs.nixpkgs { inherit system overlays; };
      rustToolchain = pkgs.rust-bin.nightly.latest.default.override {
        targets = ["x86_64-unknown-linux-gnu"];
      };
    };
}
