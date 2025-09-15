{
  pkgs,
  projectNamespace,
  ...
}:
pkgs.writeShellApplication {
  name = "release.push-tags";

  runtimeInputs = [
    pkgs.git
    pkgs.jq
    projectNamespace.root.release.secrets.get-json
    projectNamespace.root.lint-version
  ];

  text = builtins.readFile ./run.sh;
}
