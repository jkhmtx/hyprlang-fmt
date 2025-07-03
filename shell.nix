{
  pkgs,
  rustStable,
  ...
}: let
  paths = [pkgs.gcc rustStable];
in
  pkgs.symlinkJoin {
    name = "dev-shell";
    inherit paths;
  }
