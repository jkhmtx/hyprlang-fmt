{
  pkgs,
  projectNamespace,
  ...
}:
pkgs.writeShellApplication {
  name = "root.test-rust";

  runtimeInputs = [
    projectNamespace.rust
  ];

  text = builtins.readFile ./run.sh;
}
