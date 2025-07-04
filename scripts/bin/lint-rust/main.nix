{
  pkgs,
  projectNamespace,
  ...
}:
pkgs.writeShellApplication {
  name = "lint-rust";

  runtimeInputs = [
    pkgs.findutils
    pkgs.git
    projectNamespace.rust
  ];

  text = builtins.readFile ./run.sh;
}
