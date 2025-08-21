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

    package = import ./nix/package.nix projectInputs;

    projectInputs = {
      inherit package;
      inherit pkgs;
      projectNamespace = import ./nix/project.nix projectInputs;
    };

    devShell = import ./nix/dev-shell.nix projectInputs;
  in {
    inherit devShell;
    inherit (projectInputs.projectNamespace) root;
    packages."${system}".default = package;
  };
}
