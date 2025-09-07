{ inputs, ... }:

{
  perSystem =
    {
      pkgs,
      ...
    }:
    let

      craneLib = inputs.crane.mkLib pkgs;

      inherit (import ../nix) mkSharedArgs;

      sharedArgs = mkSharedArgs { inherit pkgs craneLib; };
      inherit (sharedArgs) commonArgs cargoArtifacts;
    in
    {
      checks = {
        fuscum-nextest = craneLib.cargoNextest (
          commonArgs
          // {
            inherit cargoArtifacts;
            partitions = 1;
            partitionType = "count";
            cargoNextestPartitionsExtraArgs = "--no-tests=pass";
          }
        );
      };
    };
}
