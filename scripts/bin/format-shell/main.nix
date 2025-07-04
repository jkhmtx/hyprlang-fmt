{pkgs, ...}:
pkgs.writeShellApplication {
  name = "format-shell";

  runtimeInputs = [
    pkgs.git
    pkgs.shfmt
  ];

  text = builtins.readFile ./run.sh;
}
