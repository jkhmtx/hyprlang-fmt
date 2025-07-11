{
  pkgs,
  package,
  ...
}:
pkgs.writeShellApplication {
  name = "test-e2e";

  runtimeInputs = [
    package
    pkgs.diffutils
    pkgs.git
  ];

  runtimeEnv = {
    HYPRLAND_CONF = ./hyprland.conf;
  };

  text = builtins.readFile ./run.sh;
}
