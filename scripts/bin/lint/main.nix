{
  pkgs,
  projectNamespace,
  ...
}:
pkgs.writeShellApplication {
  name = "root.lint";

  runtimeInputs = [
    projectNamespace.root.lint-github-actions
    projectNamespace.root.lint-rust
    projectNamespace.root.lint-shell
  ];

  text = builtins.readFile ./run.sh;
}
