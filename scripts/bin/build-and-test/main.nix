{
  pkgs,
  projectNamespace,
  ...
}:
pkgs.writeShellApplication {
  name = "root.build-and-test";

  runtimeInputs = [projectNamespace.root.test];

  text = builtins.readFile ./run.sh;
}
