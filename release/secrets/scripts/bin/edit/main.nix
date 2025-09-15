{pkgs, ...}:
pkgs.writeShellApplication {
  name = "release.secrets.edit";

  runtimeInputs = [
    pkgs.sops
  ];

  runtimeEnv = {
    SOPS_CONFIG = ../../../.sops.yaml;
  };

  text = builtins.readFile ./run.sh;
}
