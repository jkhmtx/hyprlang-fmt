{
  pkgs,
  projectNamespace,
  ...
}: let
  inherit (pkgs.lib.lists) flatten;
  inherit (pkgs.lib) isDerivation mapAttrsToList;

  scripts = let
    flattenToScriptDrvs = scripts:
      flatten ((mapAttrsToList (_: value:
        if isDerivation value
        then value
        else flattenToScriptDrvs value))
      scripts);
  in (flattenToScriptDrvs projectNamespace);
in
  pkgs.symlinkJoin {
    name = "dev-shell";
    paths = [pkgs.gcc projectNamespace.rust] ++ scripts;
  }
