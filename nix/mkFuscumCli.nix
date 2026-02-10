{
  pkgs,
  craneLib,
}:
let
  sharedArgs = import ./mkSharedArgs.nix { inherit pkgs craneLib; };

  inherit (pkgs) lib stdenv buildPackages;
  inherit (sharedArgs) individualCrateArgs;

  fuscum-cli = "${stdenv.hostPlatform.emulator buildPackages} $out/bin/fuscum-cli";
in
individualCrateArgs
// {
  pname = "fuscum-cli";
  cargoExtraArgs = "-p fuscum-cli";
  nativeBuildInputs = with pkgs; [ installShellFiles ];

  postInstall = lib.optionalString (stdenv.hostPlatform.emulatorAvailable buildPackages) ''
    installShellCompletion --cmd fuscum-cli \
      --bash <(${fuscum-cli} completions bash) \
      --fish <(${fuscum-cli} completions fish) \
      --zsh <(${fuscum-cli} completions zsh)
  '';
}
