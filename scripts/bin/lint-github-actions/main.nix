{pkgs, ...}:
pkgs.writeShellApplication {
  name = "lint-github-actions";

  runtimeInputs = [
    pkgs.actionlint
    pkgs.git
  ];

  text = builtins.readFile ./run.sh;
}
