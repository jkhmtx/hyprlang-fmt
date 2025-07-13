{
  pkgs,
  projectNamespace,
  ...
}:
pkgs.writeShellApplication {
  name = "local-ci";

  runtimeInputs = [
    projectNamespace.scripts.build-and-test
    projectNamespace.scripts.check
  ];

  text = builtins.readFile ./run.sh;
}
