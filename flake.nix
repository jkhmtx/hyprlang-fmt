{
  description = "A very basic flake";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";

    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = {
    nixpkgs,
    rust-overlay,
    ...
  }: let
    system = "x86_64-linux";

    pkgs = import nixpkgs {
      inherit system;
      overlays = [rust-overlay.overlays.default];
    };

    projectInputs = {
      inherit pkgs;
      projectNamespace = {
        rust = pkgs.rust-bin.nightly.latest.minimal.override {
          extensions = [
            "cargo"
            "clippy"
            "rust-src"
            "rustc"
            "rustc-codegen-cranelift-preview"
            "rustfmt"
          ];
        };
        scripts = {
          fix = import ./scripts/bin/fix/main.nix projectInputs;

          format = {
            format-nix = import ./scripts/bin/format-nix/main.nix projectInputs;
            format-rust = import ./scripts/bin/format-rust/main.nix projectInputs;
            format-shell = import ./scripts/bin/format-shell/main.nix projectInputs;
            format-yaml = import ./scripts/bin/format-yaml/main.nix projectInputs;
          };

          lint = {
            lint-github-actions = import ./scripts/bin/lint-github-actions/main.nix projectInputs;
            lint-rust = import ./scripts/bin/lint-rust/main.nix projectInputs;
            lint-shell = import ./scripts/bin/lint-shell/main.nix projectInputs;
          };

          local-ci = import ./scripts/bin/local-ci/main.nix projectInputs;
        };
      };
    };

    formatter = pkgs.alejandra;
    shell = import ./shell.nix projectInputs;
    package = import ./package.nix projectInputs;
  in {
    inherit shell;

    packages."${system}".default = package;
    formatter."${system}" = formatter;
  };
}
