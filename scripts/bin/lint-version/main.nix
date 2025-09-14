{pkgs, ...}:
pkgs.writeShellApplication {
  name = "root.lint-version";

  runtimeInputs = [
    pkgs.gawk
    pkgs.git
    pkgs.gnused
  ];

  text = builtins.readFile ./run.sh;
}
