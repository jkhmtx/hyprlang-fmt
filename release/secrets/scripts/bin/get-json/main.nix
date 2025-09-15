{pkgs, ...}:
pkgs.writeShellApplication {
  name = "release.secrets.get-json";

  runtimeInputs = [
    pkgs.sops
  ];

  runtimeEnv = {
    SOPS_CONFIG = ../../../.sops.yaml;
    RELEASE_SECRETS = ../../../secrets.yaml;
  };

  text = builtins.readFile ./run.sh;
}
