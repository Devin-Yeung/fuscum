{
  nixpkgs,
  rust-overlay,
  crane,
  localSystem,
  target,
}:
let
  inherit (import ./.) mkCrossCraneLib mkCrossPkgs;

  pkgs = mkCrossPkgs {
    inherit
      nixpkgs
      rust-overlay
      localSystem
      target
      ;
  };

  craneLib = mkCrossCraneLib {
    inherit crane;
    inherit pkgs target;
  };

  inherit (import ../nix) mkFuscumCli;

  # Common arguments for building
  formula = mkFuscumCli {
    inherit pkgs craneLib;
  };

in
craneLib.buildPackage (
  formula
  // {
    CARGO_BUILD_TARGET = target;
    CARGO_BUILD_RUSTFLAGS = "-C target-feature=+crt-static";
  }
)
