{
  pkgs,
  projectNamespace,
  ...
}:
pkgs.writeShellApplication {
  name = "root.debug";

  meta.description = "Runs the formattter on the testbed files and writes the results to the 'debug' directory";

  runtimeInputs = [
    pkgs.git
    projectNamespace.rust
  ];

  text = builtins.readFile ./run.sh;
}
