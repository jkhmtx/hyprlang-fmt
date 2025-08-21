{
  pkgs,
  projectNamespace,
  ...
}:
pkgs.writeShellApplication {
  name = "root.fix";

  runtimeInputs = [projectNamespace.root.check];

  text = builtins.readFile ./run.sh;
}
