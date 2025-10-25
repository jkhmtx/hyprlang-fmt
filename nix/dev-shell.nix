{
  pkgs,
  projectNamespace,
  ...
}:
pkgs.symlinkJoin {
  name = "dev-shell";
  paths = [pkgs.gcc projectNamespace.rust];
}
