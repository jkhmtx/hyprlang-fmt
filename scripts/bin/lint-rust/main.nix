{
  pkgs,
  projectNamespace,
  ...
}:
pkgs.writeShellApplication {
  name = "root.lint-rust";

  runtimeInputs = [
    pkgs.findutils
    pkgs.git
    projectNamespace.rust
  ];

  text = builtins.readFile ./run.sh;
}
