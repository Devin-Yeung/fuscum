{ inputs, ... }:

{
  perSystem =
    {
      pkgs,
      ...
    }:
    let

      craneLib = inputs.crane.mkLib pkgs;

      inherit (import ../nix) mkFuscumCli;

      # Common arguments for building
      formula = mkFuscumCli {
        inherit pkgs craneLib;
      };

      fuscum-cli = craneLib.buildPackage formula;
    in
    {
      checks = {
        inherit fuscum-cli;
      };
      packages.fuscum-cli = fuscum-cli;
      packages.default = fuscum-cli;
    };
}
