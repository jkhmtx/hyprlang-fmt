{
  pkgs,
  projectNamespace,
  ...
}: let
  deps = [pkgs.gcc projectNamespace.rust];
  scripts =
    [
      projectNamespace.scripts.build-and-test
      projectNamespace.scripts.check
      projectNamespace.scripts.debug
      projectNamespace.scripts.fix
      projectNamespace.scripts.local-ci
    ]
    ++ builtins.attrValues projectNamespace.scripts.format
    ++ builtins.attrValues projectNamespace.scripts.lint
    ++ builtins.attrValues projectNamespace.scripts.test;
in
  pkgs.symlinkJoin {
    name = "dev-shell";
    paths = deps ++ scripts;
  }
