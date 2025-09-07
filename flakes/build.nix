{ inputs, ... }:

{
  perSystem =
    {
      pkgs,
      ...
    }:
    let
      inherit (import ../nix) mkFormula;

      craneLib = inputs.crane.mkLib pkgs;

      # Common arguments for building
      formula = mkFormula {
        inherit pkgs craneLib;
      };

      inherit (formula) individualCrateArgs fileSetForCrate;
      fuscum-cli = craneLib.buildPackage (
        individualCrateArgs
        // {
          pname = "fuscum-cli";
          cargoExtraArgs = "-p fuscum-cli";
          src = fileSetForCrate ../crates/fuscum-cli;
        }
      );
    in
    {
      packages.fuscum-cli = fuscum-cli;
      packages.default = fuscum-cli;
    };
}
