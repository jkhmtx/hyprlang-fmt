{
  pkgs,
  projectNamespace,
  ...
}:
pkgs.writeShellApplication {
  name = "fix";

  runtimeInputs = [projectNamespace.scripts.local-ci];

  text = builtins.readFile ./run.sh;
}
