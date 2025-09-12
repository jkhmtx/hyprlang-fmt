{
  pkgs,
  projectNamespace,
  ...
}:
pkgs.writeShellApplication {
  name = "root.check";

  runtimeInputs = [
    projectNamespace.root.docs
    projectNamespace.root.format
    projectNamespace.root.lint
  ];

  text = builtins.readFile ./run.sh;
}
