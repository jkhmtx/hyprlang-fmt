{
  pkgs,
  projectNamespace,
  ...
}:
pkgs.writeShellApplication {
  name = "root.test";

  runtimeInputs = [
    projectNamespace.root.testbed.test-e2e
  ];

  text = builtins.readFile ./run.sh;
}
