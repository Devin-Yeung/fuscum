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

  fileSetForCrate =
    crate:
    lib.fileset.toSource {
      root = ../.;
      fileset = lib.fileset.unions [
        ../Cargo.toml
        ../Cargo.lock
        (craneLib.fileset.commonCargoSources ../crates/fuscum)
        (craneLib.fileset.commonCargoSources crate)
        (lib.fileset.fileFilter (file: file.hasExt "html") crate)
      ];
    };
in
{
  inherit
    commonArgs
    cargoArtifacts
    individualCrateArgs
    fileSetForCrate
    ;
}
