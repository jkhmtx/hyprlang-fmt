{pkgs, ...}:
pkgs.writeShellApplication {
  name = "format-yaml";

  runtimeInputs = [
    pkgs.git
    pkgs.prettier
  ];

  text = builtins.readFile ./run.sh;
}
