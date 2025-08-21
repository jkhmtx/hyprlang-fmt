{pkgs, ...}:
pkgs.writeShellApplication {
  name = "root.lint-github-actions";

  runtimeInputs = [
    pkgs.actionlint
    pkgs.git
  ];

  text = builtins.readFile ./run.sh;
}
