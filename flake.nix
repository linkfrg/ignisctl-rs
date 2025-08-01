{
  description = "A CLI for the Ignis widget framework, written in Rust.";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  };

  outputs = {
    self,
    nixpkgs,
    ...
  }: let
    systems = ["x86_64-linux" "aarch64-linux"];
    forAllSystems = nixpkgs.lib.genAttrs systems;
  in {
    packages = forAllSystems (system: {
      ignisctl-rs = nixpkgs.legacyPackages.${system}.callPackage ./nix {};
      default = self.packages.${system}.ignisctl-rs;
    });

    formatter = forAllSystems (system: nixpkgs.legacyPackages.${system}.alejandra);

    devShells = forAllSystems (system: let
      pkgs = nixpkgs.legacyPackages.${system};
    in {
      default = pkgs.mkShell {
        packages = with pkgs; [
          cargo
        ];
      };
    });
    overlays.default = final: prev: {inherit (self.packages.${prev.system}) ignisctl-rs;};
  };
}
