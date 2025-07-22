{
  description = "A CLI for the Ignis widget framework, written in Rust.";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";

    systems.url = "github:nix-systems/default-linux";

    flake-utils = {
      url = "github:numtide/flake-utils";
      inputs.systems.follows = "systems";
    };
  };

  outputs = {
    self,
    nixpkgs,
    flake-utils,
    ...
  }:
    flake-utils.lib.eachDefaultSystem (
      system: let
        pkgs = import nixpkgs {inherit system;};
      in {
        packages = rec {
          ignisctl-rs = pkgs.callPackage ./nix {};
          default = ignisctl-rs;
        };
        apps = rec {
          ignisctl-rs = flake-utils.lib.mkApp {drv = self.packages.${system}.ignisctl-rs;};
          default = ignisctl-rs;
        };

        formatter = pkgs.alejandra;

        devShells = {
          default = pkgs.mkShell {
            packages = with pkgs; [
              cargo
            ];
          };
        };
      }
    )
    // {
      overlays.default = final: prev: {inherit (self.packages.${prev.system}) ignisctl-rs;};
    };
}
