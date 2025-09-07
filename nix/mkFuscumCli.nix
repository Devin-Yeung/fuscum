{
  pkgs,
  craneLib,
}:
let
  sharedArgs = import ./mkSharedArgs.nix { inherit pkgs craneLib; };

  inherit (sharedArgs) individualCrateArgs;
in
individualCrateArgs
// {
  pname = "fuscum-cli";
  cargoExtraArgs = "-p fuscum-cli";
}
