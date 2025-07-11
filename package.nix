{pkgs, ...}:
pkgs.rustPlatform.buildRustPackage {
  pname = "hyprlang-fmt";
  version = "0.1";

  src = pkgs.lib.cleanSource ./.;

  cargoHash = "sha256-+jB0Wpbj3vdPZ2mZU7muigbYuOzTONz8U0+cAz31VW0=";

  meta = {
    mainProgram = "hyprlang-fmt";
    description = "A formatter for hyprlang";
    homepage = "https://github.com/jkhmtx/hyprlang-fmt";
    license = pkgs.lib.licenses.gpl3;
    maintainers = ["jakehamtexas@gmail.com"];
  };
}
