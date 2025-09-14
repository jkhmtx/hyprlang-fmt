{
  pkgs,
  projectNamespace,
  ...
}: let
  rustPlatform = pkgs.makeRustPlatform {
    cargo = projectNamespace.rust;
    rustc = projectNamespace.rust;
  };
in
  rustPlatform.buildRustPackage {
    pname = "hyprlang-fmt";
    version = "1.0";

    src = pkgs.lib.sourceByRegex ../. ["src" "src/cli" "src/components" "src/grammar" "src/parsed" ".+\.rs" ".+\.pest" "^Cargo.lock$" "^Cargo.toml$"];

    cargoHash = "sha256-+jB0Wpbj3vdPZ2mZU7muigbYuOzTONz8U0+cAz31VW0=";

    meta = {
      mainProgram = "hyprlang-fmt";
      description = "A formatter for hyprlang";
      homepage = "https://github.com/jkhmtx/hyprlang-fmt";
      license = pkgs.lib.licenses.gpl3;
      maintainers = ["jakehamtexas@gmail.com"];
    };
  }
