{
  pkgs,
  projectNamespace,
  ...
}:
pkgs.writeShellApplication {
  name = "format-rust";

  runtimeInputs = [
    pkgs.findutils
    pkgs.git
    projectNamespace.rust
  ];

  text = builtins.readFile ./run.sh;
}
