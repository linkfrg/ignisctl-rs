{rustPlatform}:
rustPlatform.buildRustPackage (
  finalAttrs: let
    CargoToml = builtins.fromTOML (builtins.readFile ../Cargo.toml);
  in {
    pname = "ignisctl-rs";
    version = CargoToml.package.version;

    src = ./..;
    cargoLock = {
      lockFile = ../Cargo.lock;
    };

    meta = {
      description = "A CLI for the Ignis widget framework, written in Rust.";
      homepage = "https://github.com/linkfrg/ignisctl-rs";
      maintainers = [
        {
          name = "linkfrg";
          email = "linkfrg.dev@proton.me";
        }
      ];
    };
  }
)
