{
  pkgs,
  projectNamespace,
  ...
}:
pkgs.writeShellApplication {
  name = "fix";

  runtimeInputs = [projectNamespace.scripts.check];

  text = builtins.readFile ./run.sh;
}
