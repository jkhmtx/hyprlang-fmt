{pkgs, ...}:
pkgs.writeShellApplication {
  name = "root.format-nix";

  runtimeInputs = [
    pkgs.alejandra
    pkgs.git
  ];

  text = builtins.readFile ./run.sh;
}
