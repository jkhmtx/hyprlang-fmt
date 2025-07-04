{
  pkgs,
  projectNamespace,
  ...
}: let
  deps = [pkgs.gcc projectNamespace.rust];
  scripts =
    [
      projectNamespace.scripts.fix
      projectNamespace.scripts.local-ci
    ]
    ++ builtins.attrValues projectNamespace.scripts.format
    ++ builtins.attrValues projectNamespace.scripts.lint;
in
  pkgs.symlinkJoin {
    name = "dev-shell";
    paths = deps ++ scripts;
  }
