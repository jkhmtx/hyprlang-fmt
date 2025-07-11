{
  pkgs,
  projectNamespace,
  ...
}:
pkgs.writeShellApplication {
  name = "check";

  runtimeInputs = builtins.attrValues projectNamespace.scripts.format ++ builtins.attrValues projectNamespace.scripts.lint;

  text = builtins.readFile ./run.sh;
}
