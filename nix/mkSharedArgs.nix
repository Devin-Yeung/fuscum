{
  pkgs,
  craneLib,
}:
let
  inherit (pkgs) lib;

  unfilteredRoot = ../.; # The original, unfiltered source
  src = lib.fileset.toSource {
    root = unfilteredRoot;
    fileset = lib.fileset.unions [
      # Default files from crane (Rust and cargo files)
      (craneLib.fileset.commonCargoSources unfilteredRoot)
      # Also keep any snapshots files
      (lib.fileset.fileFilter (file: file.hasExt "snap") unfilteredRoot)
      # Keep any HTML templates
      (lib.fileset.fileFilter (file: file.hasExt "html") unfilteredRoot)
      # Example of a folder for images, icons, etc
      (lib.fileset.maybeMissing ../fixtures)
    ];
  };

  commonArgs = {
    inherit src;
    strictDeps = true;
    buildInputs = [
      # Add extra build inputs if needed
    ]
    ++ lib.optionals pkgs.stdenv.isDarwin [
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
