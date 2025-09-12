{
  pkgs,
  projectNamespace,
  ...
}:
pkgs.writeShellApplication {
  name = "root.docs-rust";

  runtimeInputs = [
    pkgs.git
    projectNamespace.rust
  ];

  text = builtins.readFile ./run.sh;
}
