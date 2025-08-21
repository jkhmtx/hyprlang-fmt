{pkgs, ...}:
pkgs.writeShellApplication {
  name = "root.lint-shell";

  runtimeInputs = [
    pkgs.git
    pkgs.findutils
    pkgs.shellcheck
  ];

  text = builtins.readFile ./run.sh;
}
