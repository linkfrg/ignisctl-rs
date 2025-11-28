{
  description = "A CLI for the Ignis widget framework, written in Rust.";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";

    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = {
    self,
    nixpkgs,
    rust-overlay,
    ...
  }: let
    systems = ["x86_64-linux" "aarch64-linux"];
    overlays = [rust-overlay.overlays.default];
    forAllSystems = nixpkgs.lib.genAttrs systems;
  in {
    packages = forAllSystems (system: {
      ignisctl-rs = nixpkgs.legacyPackages.${system}.callPackage ./nix {};
      default = self.packages.${system}.ignisctl-rs;
    });

    formatter = forAllSystems (system: nixpkgs.legacyPackages.${system}.alejandra);

    devShells = forAllSystems (system: let
      pkgs = import nixpkgs {inherit system overlays;};
    in {
      default = pkgs.mkShell {
        packages = with pkgs; [
          rust-bin.stable.latest.default
        ];
      };
    });
    overlays.default = final: prev: {inherit (self.packages.${prev.system}) ignisctl-rs;};
  };
}
