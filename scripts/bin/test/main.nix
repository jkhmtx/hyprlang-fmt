{
  pkgs,
  projectNamespace,
  ...
}:
pkgs.writeShellApplication {
  name = "root.test";

  runtimeInputs = [
    projectNamespace.root.test-rust
    projectNamespace.root.testbed.test-e2e
  ];

  text = builtins.readFile ./run.sh;
}
