{
  pkgs,
  projectNamespace,
  ...
}:
pkgs.writeShellApplication {
  name = "debug";

  runtimeInputs = [
    pkgs.git
    projectNamespace.rust
  ];

  text = builtins.readFile ./run.sh;
}
