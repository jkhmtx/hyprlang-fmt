{
  pkgs,
  projectNamespace,
  ...
}:
pkgs.writeShellApplication {
  name = "root.docs";

  runtimeInputs = [
    projectNamespace.root.docs-help
    projectNamespace.root.docs-rust
  ];

  text = builtins.readFile ./run.sh;
}
