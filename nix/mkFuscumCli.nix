{
  pkgs,
  craneLib,
}:
let
  inherit (pkgs) lib;

  sharedArgs = import ./mkSharedArgs.nix { inherit pkgs craneLib; };

  inherit (sharedArgs) individualCrateArgs;

  src = lib.fileset.toSource {
    root = ../.;
    fileset = lib.fileset.unions [
      ../Cargo.toml
      ../Cargo.lock
      (craneLib.fileset.commonCargoSources ../crates/fuscum)
      (craneLib.fileset.commonCargoSources ../crates/fuscum-cli)
      (lib.fileset.fileFilter (file: file.hasExt "html") ../crates/fuscum-cli)
    ];
  };
in
individualCrateArgs
// {
  inherit src;
  pname = "fuscum-cli";
  cargoExtraArgs = "-p fuscum-cli";
}
