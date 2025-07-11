{
  pkgs,
  projectNamespace,
  ...
}:
pkgs.writeShellApplication {
  name = "build-and-test";

  runtimeInputs = builtins.attrValues projectNamespace.scripts.test;

  text = builtins.readFile ./run.sh;
}
