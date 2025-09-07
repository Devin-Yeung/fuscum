{
  nixpkgs,
  rust-overlay,
  crane,
  localSystem,
  target,
}:
let
  inherit (import ./.) mkCrossCraneLib mkFormula mkCrossPkgs;

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

  formula = mkFormula {
    inherit pkgs craneLib;
  };
  inherit (formula) individualCrateArgs fileSetForCrate;
in
craneLib.buildPackage (
  individualCrateArgs
  // {
    pname = "fuscum-cli";
    cargoExtraArgs = "-p fuscum-cli";
    src = fileSetForCrate ../crates/fuscum-cli;
  }
  // {
    CARGO_BUILD_TARGET = target;
    CARGO_BUILD_RUSTFLAGS = "-C target-feature=+crt-static";
  }
)
