{
  pkgs,
  projectNamespace,
  ...
}:
pkgs.writeShellApplication {
  name = "root.docs";

  runtimeInputs = [
    projectNamespace.root.docs-help
  ];

  text = builtins.readFile ./run.sh;
}
