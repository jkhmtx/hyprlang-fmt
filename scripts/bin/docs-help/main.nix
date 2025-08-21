{
  pkgs,
  projectNamespace,
  ...
}:
pkgs.writeShellApplication {
  name = "root.docs-help";

  runtimeInputs = [
    pkgs.git
    projectNamespace.rust
  ];

  text = builtins.readFile ./run.sh;
}
