{
  pkgs,
  craneLib,
}:
let
  inherit (pkgs) lib;

  src = craneLib.cleanCargoSource ../.;

  commonArgs = {
    inherit src;
    strictDeps = true;
    buildInputs = [
      # Add extra build inputs if needed
    ]
    ++ pkgs.lib.optionals pkgs.stdenv.isDarwin [
      pkgs.libiconv
    ];
  };

  cargoArtifacts = craneLib.buildDepsOnly commonArgs;

  individualCrateArgs = commonArgs // {
    inherit cargoArtifacts;
    inherit (craneLib.crateNameFromCargoToml { inherit src; }) version;
  };
in
{
  inherit
    commonArgs
    cargoArtifacts
    individualCrateArgs
    ;
}
