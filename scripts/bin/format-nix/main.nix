{pkgs, ...}:
pkgs.writeShellApplication {
  name = "format-nix";

  runtimeInputs = [
    pkgs.alejandra
    pkgs.git
  ];

  text = builtins.readFile ./run.sh;
}
