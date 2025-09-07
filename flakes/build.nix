{ inputs, ... }:

{
  perSystem =
    {
      pkgs,
      ...
    }:
    let
      inherit (import ../nix) mkFuscumCli;

      craneLib = inputs.crane.mkLib pkgs;

      # Common arguments for building
      formula = mkFuscumCli {
        inherit pkgs craneLib;
      };

      fuscum-cli = craneLib.buildPackage formula;
    in
    {
      packages.fuscum-cli = fuscum-cli;
      packages.default = fuscum-cli;
    };
}
