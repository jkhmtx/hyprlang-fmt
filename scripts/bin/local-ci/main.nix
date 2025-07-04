{
  pkgs,
  projectNamespace,
  ...
}:
pkgs.writeShellApplication {
  name = "local-ci";

  runtimeInputs = builtins.attrValues projectNamespace.scripts.format ++ builtins.attrValues projectNamespace.scripts.lint;

  text = builtins.readFile ./run.sh;
}
