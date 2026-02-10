{
  description = "Fuscum: A Rust implementation of MOSS for code plagiarism detection";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    crane.url = "github:ipetkov/crane";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    {
      nixpkgs,
      crane,
      flake-utils,
      rust-overlay,
      ...
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ (import rust-overlay) ];
        };
        craneLib = (crane.mkLib pkgs).overrideToolchain (
          p: p.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml
        );
        inherit (pkgs) lib;

        unfilteredRoot = ./.;
        src = lib.fileset.toSource {
          root = unfilteredRoot;
          fileset = lib.fileset.unions [
            (craneLib.fileset.commonCargoSources unfilteredRoot)
            (lib.fileset.fileFilter (file: file.hasExt "snap") unfilteredRoot)
            (lib.fileset.fileFilter (file: file.hasExt "html") unfilteredRoot)
            (lib.fileset.maybeMissing ./fixtures)
          ];
        };

        commonArgs = {
          inherit src;
          pname = "fuscum-cli";
          cargoExtraArgs = "-p fuscum-cli";
          strictDeps = true;
          nativeBuildInputs = with pkgs; [ installShellFiles ];
          buildInputs = lib.optionals pkgs.stdenv.isDarwin [
            pkgs.libiconv
          ];
        };

        cargoArtifacts = craneLib.buildDepsOnly commonArgs;

        fuscum-cli = craneLib.buildPackage (
          commonArgs
          // {
            inherit cargoArtifacts;
            postInstall =
              let
                fuscum-cli-bin = "${pkgs.stdenv.hostPlatform.emulator pkgs.buildPackages} $out/bin/fuscum-cli";
              in
              lib.optionalString (pkgs.stdenv.hostPlatform.emulatorAvailable pkgs.buildPackages) ''
                installShellCompletion --cmd fuscum-cli \
                  --bash <(${fuscum-cli-bin} completions bash) \
                  --fish <(${fuscum-cli-bin} completions fish) \
                  --zsh <(${fuscum-cli-bin} completions zsh)
              '';
          }
        );
      in
      {
        checks = {
          inherit fuscum-cli;

          fuscum-clippy = craneLib.cargoClippy (
            commonArgs
            // {
              inherit cargoArtifacts;
              cargoClippyExtraArgs = "--all-targets -- --deny warnings";
            }
          );

          fuscum-doc = craneLib.cargoDoc (
            commonArgs
            // {
              inherit cargoArtifacts;
              env.RUSTDOCFLAGS = "--deny warnings";
            }
          );

          fuscum-fmt = craneLib.cargoFmt {
            inherit src;
          };

          fuscum-toml-fmt = craneLib.taploFmt {
            src = lib.sources.sourceFilesBySuffices src [ ".toml" ];
          };

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

        packages = {
          inherit fuscum-cli;
          default = fuscum-cli;
        };
      }
    );
}
