{
  pkgs,
  projectNamespace,
  ...
}:
pkgs.writeShellApplication {
  name = "root.release";

  runtimeInputs = [
    pkgs.git
    projectNamespace.root.lint-version
  ];

  text = builtins.readFile ./run.sh;
}
