{pkgs, ...}:
pkgs.writeShellApplication {
  name = "root.format-yaml";

  runtimeInputs = [
    pkgs.git
    pkgs.prettier
  ];

  text = builtins.readFile ./run.sh;
}
