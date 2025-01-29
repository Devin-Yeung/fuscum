{
  description = "Fuscum built with Nix";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let pkgs = nixpkgs.legacyPackages.${system};
      in {
        # Define the package
        packages = rec {
          default = fuscum-cli;

          fuscum-cli = pkgs.rustPlatform.buildRustPackage {
            pname = "fuscum-cli";

            version = "0.1.0";
            src = ./.;

            cargoLock = {
              lockFile = ./Cargo.lock; # why i dont need to provide a hash???
            };

          };
        };

        # Optionally add a development shell
        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [ cargo rustc rust-analyzer ];
        };

        # Add apps so the binary can be run with `nix run`
        apps = rec {
          default = fuscum-cli;
          fuscum-cli = flake-utils.lib.mkApp {
            drv = self.packages.${system}.fuscum-cli;
            name = "fuscum-cli";
          };
        };
      });
}
