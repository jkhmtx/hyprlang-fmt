{pkgs, ...}:
pkgs.writeShellApplication {
  name = "root.testbed.test-e2e";

  runtimeInputs = [
    pkgs.diffutils
    pkgs.git
  ];

  runtimeEnv = {
    HYPRLAND_CONF = ./hyprland.conf;
    HYPRIDLE_CONF = ./hypridle.conf;
  };

  text = builtins.readFile ./run.sh;
}
