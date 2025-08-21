{pkgs, ...}:
pkgs.writeShellApplication {
  name = "root.format-shell";

  runtimeInputs = [
    pkgs.git
    pkgs.shfmt
  ];

  text = builtins.readFile ./run.sh;
}
